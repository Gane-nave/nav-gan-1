/**
 * G.A.N.E — Sensor Bridge
 * ========================
 * Connects real browser sensor APIs to the navigation engine pipeline:
 *   - Geolocation API → GNSS measurements → NavigationManager.processGNSS()
 *   - DeviceMotion API → IMU measurements → NavigationManager.processIMU()
 *   - DeviceOrientation API → compass heading → NavigationManager state
 *
 * Handles permission requests, sensor availability detection, and graceful
 * degradation when sensors are unavailable.
 */

import type { GNSSMeasurement, IMUMeasurement } from './eskf';
import { getNavigationManager } from './navigationManager';

// ─── Types ───

export interface SensorStatus {
  geolocation: 'unavailable' | 'denied' | 'prompt' | 'granted' | 'active';
  deviceMotion: 'unavailable' | 'denied' | 'prompt' | 'granted' | 'active';
  deviceOrientation: 'unavailable' | 'denied' | 'prompt' | 'granted' | 'active';
  magnetometer: 'unavailable' | 'active';
}

export interface SensorBridgeConfig {
  enableGeolocation: boolean;
  enableDeviceMotion: boolean;
  enableDeviceOrientation: boolean;
  geolocationOptions: PositionOptions;
  imuSampleRateHz: number;
}

const DEFAULT_CONFIG: SensorBridgeConfig = {
  enableGeolocation: false, // Disabled by default — RealDataContext provides geolocation
  enableDeviceMotion: true,
  enableDeviceOrientation: true,
  geolocationOptions: {
    enableHighAccuracy: true,
    maximumAge: 0,
    timeout: 10000,
  },
  imuSampleRateHz: 50, // 50Hz IMU sampling
};

// ─── Sensor Bridge ───

export class SensorBridge {
  private config: SensorBridgeConfig;
  private status: SensorStatus;
  private watchId: number | null = null;
  private imuThrottleMs: number;
  private lastIMUTimestamp = 0;
  private lastOrientationTimestamp = 0;
  private isRunning = false;

  // Bound handlers for cleanup
  private boundDeviceMotion: ((e: DeviceMotionEvent) => void) | null = null;
  private boundDeviceOrientation: ((e: DeviceOrientationEvent) => void) | null = null;

  // Callbacks
  private onStatusChange: ((status: SensorStatus) => void) | null = null;
  private onGNSSUpdate: ((pos: GeolocationPosition) => void) | null = null;
  private onIMUUpdate: ((imu: IMUMeasurement) => void) | null = null;
  private onHeadingUpdate: ((heading: number) => void) | null = null;
  private onError: ((error: string) => void) | null = null;

  // Stats
  private gnssCount = 0;
  private imuCount = 0;
  private orientationCount = 0;
  private startTime = 0;

  constructor(config?: Partial<SensorBridgeConfig>) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.imuThrottleMs = 1000 / this.config.imuSampleRateHz;
    this.status = {
      geolocation: 'unavailable',
      deviceMotion: 'unavailable',
      deviceOrientation: 'unavailable',
      magnetometer: 'unavailable',
    };

    // Detect available sensors
    this.detectSensors();
  }

  /** Detect which sensors are available in this browser */
  private detectSensors(): void {
    if (typeof navigator !== 'undefined' && 'geolocation' in navigator) {
      this.status.geolocation = 'prompt';
    }
    if (typeof DeviceMotionEvent !== 'undefined') {
      this.status.deviceMotion = 'prompt';
    }
    if (typeof DeviceOrientationEvent !== 'undefined') {
      this.status.deviceOrientation = 'prompt';
    }
  }

  /** Request all sensor permissions and start streaming */
  async start(): Promise<SensorStatus> {
    if (this.isRunning) return this.status;
    this.isRunning = true;
    this.startTime = Date.now();

    // Start Geolocation
    if (this.config.enableGeolocation && this.status.geolocation !== 'unavailable') {
      await this.startGeolocation();
    }

    // Start DeviceMotion (requires permission on iOS 13+)
    if (this.config.enableDeviceMotion && this.status.deviceMotion !== 'unavailable') {
      await this.startDeviceMotion();
    }

    // Start DeviceOrientation
    if (this.config.enableDeviceOrientation && this.status.deviceOrientation !== 'unavailable') {
      await this.startDeviceOrientation();
    }

    this.notifyStatusChange();
    return this.status;
  }

  /** Stop all sensor streams */
  stop(): void {
    this.isRunning = false;

    // Stop geolocation
    if (this.watchId !== null) {
      navigator.geolocation.clearWatch(this.watchId);
      this.watchId = null;
      if (this.status.geolocation === 'active') {
        this.status.geolocation = 'granted';
      }
    }

    // Stop DeviceMotion
    if (this.boundDeviceMotion) {
      window.removeEventListener('devicemotion', this.boundDeviceMotion);
      this.boundDeviceMotion = null;
      if (this.status.deviceMotion === 'active') {
        this.status.deviceMotion = 'granted';
      }
    }

    // Stop DeviceOrientation
    if (this.boundDeviceOrientation) {
      window.removeEventListener('deviceorientation', this.boundDeviceOrientation);
      this.boundDeviceOrientation = null;
      if (this.status.deviceOrientation === 'active') {
        this.status.deviceOrientation = 'granted';
      }
    }

    this.notifyStatusChange();
  }

  /** Start Geolocation API watchPosition */
  private async startGeolocation(): Promise<void> {
    try {
      // Check permission status if available
      if ('permissions' in navigator) {
        try {
          const perm = await navigator.permissions.query({ name: 'geolocation' });
          this.status.geolocation = perm.state as 'denied' | 'prompt' | 'granted';
          if (perm.state === 'denied') {
            this.onError?.('Geolocation permission denied');
            return;
          }
        } catch {
          // permissions API not fully supported, proceed anyway
        }
      }

      this.watchId = navigator.geolocation.watchPosition(
        (position) => this.handleGeolocationSuccess(position),
        (error) => this.handleGeolocationError(error),
        this.config.geolocationOptions,
      );

      this.status.geolocation = 'active';
    } catch (err) {
      this.status.geolocation = 'denied';
      this.onError?.(`Geolocation failed: ${err}`);
    }
  }

  /** Handle successful geolocation reading */
  private handleGeolocationSuccess(position: GeolocationPosition): void {
    this.gnssCount++;
    this.status.geolocation = 'active';

    const gnss: GNSSMeasurement = {
      lat: position.coords.latitude,
      lon: position.coords.longitude,
      alt: position.coords.altitude ?? 0,
      accuracy: position.coords.accuracy,
      hdop: position.coords.accuracy / 5, // Approximate HDOP from accuracy
      satellites: this.estimateSatellites(position.coords.accuracy),
      timestamp: position.timestamp,
    };

    // Feed into NavigationManager
    const navManager = getNavigationManager();
    navManager.processGNSS(gnss);

    this.onGNSSUpdate?.(position);
  }

  /** Handle geolocation error */
  private handleGeolocationError(error: GeolocationPositionError): void {
    switch (error.code) {
      case error.PERMISSION_DENIED:
        this.status.geolocation = 'denied';
        this.onError?.('Geolocation permission denied by user');
        break;
      case error.POSITION_UNAVAILABLE:
        this.onError?.('Geolocation position unavailable');
        break;
      case error.TIMEOUT:
        this.onError?.('Geolocation request timed out');
        break;
    }
    this.notifyStatusChange();
  }

  /** Start DeviceMotion API */
  private async startDeviceMotion(): Promise<void> {
    try {
      // iOS 13+ requires explicit permission request
      if (typeof (DeviceMotionEvent as unknown as { requestPermission?: () => Promise<string> }).requestPermission === 'function') {
        const permission = await (DeviceMotionEvent as unknown as { requestPermission: () => Promise<string> }).requestPermission();
        if (permission !== 'granted') {
          this.status.deviceMotion = 'denied';
          this.onError?.('DeviceMotion permission denied');
          return;
        }
      }

      this.boundDeviceMotion = (event: DeviceMotionEvent) => this.handleDeviceMotion(event);
      window.addEventListener('devicemotion', this.boundDeviceMotion);
      this.status.deviceMotion = 'active';
    } catch (err) {
      this.status.deviceMotion = 'denied';
      this.onError?.(`DeviceMotion failed: ${err}`);
    }
  }

  /** Handle DeviceMotion event → IMU measurement */
  private handleDeviceMotion(event: DeviceMotionEvent): void {
    const now = Date.now();
    // Throttle to configured sample rate
    if (now - this.lastIMUTimestamp < this.imuThrottleMs) return;
    this.lastIMUTimestamp = now;
    this.imuCount++;

    const accel = event.accelerationIncludingGravity;
    const rotation = event.rotationRate;

    if (!accel) return;

    const imu: IMUMeasurement = {
      accel: {
        x: accel.x ?? 0,
        y: accel.y ?? 0,
        z: accel.z ?? 0,
      },
      gyro: {
        x: rotation ? (rotation.alpha ?? 0) * (Math.PI / 180) : 0, // Convert deg/s to rad/s
        y: rotation ? (rotation.beta ?? 0) * (Math.PI / 180) : 0,
        z: rotation ? (rotation.gamma ?? 0) * (Math.PI / 180) : 0,
      },
      timestamp: now,
      temperature: 25, // Browser doesn't expose temperature; use nominal
    };

    // Feed into NavigationManager
    const navManager = getNavigationManager();
    navManager.processIMU(imu);

    this.onIMUUpdate?.(imu);
  }

  /** Start DeviceOrientation API (compass heading) */
  private async startDeviceOrientation(): Promise<void> {
    try {
      // iOS 13+ requires explicit permission request
      if (typeof (DeviceOrientationEvent as unknown as { requestPermission?: () => Promise<string> }).requestPermission === 'function') {
        const permission = await (DeviceOrientationEvent as unknown as { requestPermission: () => Promise<string> }).requestPermission();
        if (permission !== 'granted') {
          this.status.deviceOrientation = 'denied';
          this.onError?.('DeviceOrientation permission denied');
          return;
        }
      }

      this.boundDeviceOrientation = (event: DeviceOrientationEvent) => this.handleDeviceOrientation(event);
      window.addEventListener('deviceorientation', this.boundDeviceOrientation, true);
      this.status.deviceOrientation = 'active';
    } catch (err) {
      this.status.deviceOrientation = 'denied';
      this.onError?.(`DeviceOrientation failed: ${err}`);
    }
  }

  /** Handle DeviceOrientation event → compass heading */
  private handleDeviceOrientation(event: DeviceOrientationEvent): void {
    const now = Date.now();
    // Throttle to 10Hz for orientation
    if (now - this.lastOrientationTimestamp < 100) return;
    this.lastOrientationTimestamp = now;
    this.orientationCount++;

    // webkitCompassHeading is available on iOS Safari
    const heading = (event as DeviceOrientationEvent & { webkitCompassHeading?: number }).webkitCompassHeading
      ?? (event.alpha !== null ? (360 - event.alpha) % 360 : null);

    if (heading !== null) {
      this.status.magnetometer = 'active';
      this.onHeadingUpdate?.(heading);
    }
  }

  /** Estimate satellite count from accuracy (heuristic) */
  private estimateSatellites(accuracy: number): number {
    if (accuracy < 3) return 12;
    if (accuracy < 5) return 10;
    if (accuracy < 10) return 8;
    if (accuracy < 20) return 6;
    if (accuracy < 50) return 4;
    return 3;
  }

  /** Get current sensor status */
  getStatus(): SensorStatus {
    return { ...this.status };
  }

  /** Get sensor statistics */
  getStats(): { gnssCount: number; imuCount: number; orientationCount: number; uptimeMs: number } {
    return {
      gnssCount: this.gnssCount,
      imuCount: this.imuCount,
      orientationCount: this.orientationCount,
      uptimeMs: this.isRunning ? Date.now() - this.startTime : 0,
    };
  }

  /** Check if any sensor is active */
  isActive(): boolean {
    return this.isRunning;
  }

  // ─── Event handlers ───

  setOnStatusChange(cb: (status: SensorStatus) => void): void { this.onStatusChange = cb; }
  setOnGNSSUpdate(cb: (pos: GeolocationPosition) => void): void { this.onGNSSUpdate = cb; }
  setOnIMUUpdate(cb: (imu: IMUMeasurement) => void): void { this.onIMUUpdate = cb; }
  setOnHeadingUpdate(cb: (heading: number) => void): void { this.onHeadingUpdate = cb; }
  setOnError(cb: (error: string) => void): void { this.onError = cb; }

  private notifyStatusChange(): void {
    this.onStatusChange?.(this.getStatus());
  }

  /** Inject external GNSS measurement (from RealDataContext) */
  injectGNSS(lat: number, lon: number, alt: number, accuracy: number, speed: number, heading: number, timestamp: number): void {
    this.gnssCount++;
    this.status.geolocation = 'active';

    const gnss: GNSSMeasurement = {
      lat,
      lon,
      alt,
      accuracy,
      hdop: accuracy / 5,
      satellites: this.estimateSatellites(accuracy),
      timestamp,
    };

    const navManager = getNavigationManager();
    navManager.processGNSS(gnss);
    this.notifyStatusChange();
  }

  /** Cleanup */
  destroy(): void {
    this.stop();
    this.onStatusChange = null;
    this.onGNSSUpdate = null;
    this.onIMUUpdate = null;
    this.onHeadingUpdate = null;
    this.onError = null;
  }
}

// ─── Singleton ───
let _instance: SensorBridge | null = null;

export function getSensorBridge(config?: Partial<SensorBridgeConfig>): SensorBridge {
  if (!_instance) {
    _instance = new SensorBridge(config);
  }
  return _instance;
}

export function resetSensorBridge(): void {
  _instance?.destroy();
  _instance = null;
}
