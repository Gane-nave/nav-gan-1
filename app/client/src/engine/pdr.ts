/**
 * G.A.N.E — Predictive Dead Reckoning (PDR) Engine
 * ==================================================
 * Fallback navigation when GNSS signal is lost (tunnels, urban canyons).
 * Uses inertial navigation with step detection, heading estimation,
 * and map-matching constraints for bounded drift.
 *
 * Integrates with ESKF for seamless handoff.
 */

import type { Vec3, IMUMeasurement, ESKFState } from './eskf';

// ─── Types ───
export interface PDRConfig {
  stepLength: number;          // average step length in meters (calibrated)
  stepDetectionThreshold: number; // acceleration magnitude threshold
  headingSmoothing: number;    // exponential smoothing factor (0-1)
  driftBoundMeters: number;    // max allowed drift before EMERGENCY
  mapMatchingEnabled: boolean;
}

export interface PDRState {
  position: Vec3;
  heading: number;             // degrees (0=North, CW)
  speed: number;               // m/s estimated
  stepCount: number;
  driftEstimate: number;       // meters of estimated drift
  lastStepTimestamp: number;
  isActive: boolean;
  confidence: number;          // 0-1
}

export interface RoadSegment {
  startLat: number;
  startLon: number;
  endLat: number;
  endLon: number;
  bearing: number;             // degrees
  name: string;
}

// ─── Step Detection (Peak Detection Algorithm) ───
class StepDetector {
  private buffer: number[] = [];
  private readonly windowSize = 20;
  private readonly minStepInterval = 250; // ms
  private lastStepTime = 0;
  private threshold: number;

  constructor(threshold: number) {
    this.threshold = threshold;
  }

  /** Returns true if a step is detected */
  detect(accelMagnitude: number, timestamp: number): boolean {
    this.buffer.push(accelMagnitude);
    if (this.buffer.length > this.windowSize) this.buffer.shift();
    if (this.buffer.length < this.windowSize) return false;

    // Check minimum interval
    if (timestamp - this.lastStepTime < this.minStepInterval) return false;

    // Peak detection: current sample is local maximum above threshold
    const mid = Math.floor(this.windowSize / 2);
    const current = this.buffer[mid];
    if (current < this.threshold) return false;

    let isPeak = true;
    for (let i = 0; i < this.windowSize; i++) {
      if (i !== mid && this.buffer[i] >= current) {
        isPeak = false;
        break;
      }
    }

    if (isPeak) {
      this.lastStepTime = timestamp;
      return true;
    }
    return false;
  }

  /** Adaptive threshold based on recent activity */
  adaptThreshold(recentAccelRange: number): void {
    this.threshold = Math.max(1.5, recentAccelRange * 0.4);
  }
}

// ─── Complementary Heading Filter ───
class HeadingFilter {
  private heading: number = 0;
  private gyroHeading: number = 0;
  private alpha: number;

  constructor(initialHeading: number, smoothing: number) {
    this.heading = initialHeading;
    this.gyroHeading = initialHeading;
    this.alpha = smoothing;
  }

  /** Update heading with gyroscope data */
  updateGyro(gyroZ: number, dt: number): void {
    // Integrate gyroscope (yaw rate)
    this.gyroHeading += gyroZ * (180 / Math.PI) * dt;
    this.gyroHeading = ((this.gyroHeading % 360) + 360) % 360;
    // Complementary filter: trust gyro more for short-term
    this.heading = this.alpha * this.gyroHeading + (1 - this.alpha) * this.heading;
    this.heading = ((this.heading % 360) + 360) % 360;
  }

  /** Correct heading with magnetometer or map-matching */
  correctHeading(trueHeading: number, weight: number = 0.1): void {
    // Handle wrap-around
    let diff = trueHeading - this.heading;
    if (diff > 180) diff -= 360;
    if (diff < -180) diff += 360;
    this.heading += diff * weight;
    this.heading = ((this.heading % 360) + 360) % 360;
    this.gyroHeading = this.heading;
  }

  getHeading(): number {
    return this.heading;
  }
}

// ─── PDR Engine ───
const DEFAULT_CONFIG: PDRConfig = {
  stepLength: 0.75,
  stepDetectionThreshold: 2.0,
  headingSmoothing: 0.95,
  driftBoundMeters: 200,
  mapMatchingEnabled: true,
};

export class PDREngine {
  private config: PDRConfig;
  private state: PDRState;
  private stepDetector: StepDetector;
  private headingFilter: HeadingFilter;
  private lastTimestamp: number = 0;
  private accelHistory: number[] = [];
  private nearbyRoads: RoadSegment[] = [];

  // Velocity estimation from step frequency
  private recentStepTimestamps: number[] = [];

  constructor(config?: Partial<PDRConfig>) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.stepDetector = new StepDetector(this.config.stepDetectionThreshold);
    this.headingFilter = new HeadingFilter(0, this.config.headingSmoothing);

    this.state = {
      position: { x: 0, y: 0, z: 0 },
      heading: 0,
      speed: 0,
      stepCount: 0,
      driftEstimate: 0,
      lastStepTimestamp: 0,
      isActive: false,
      confidence: 0.8,
    };
  }

  /** Initialize PDR from last known ESKF state */
  initFromESKF(eskfState: ESKFState): void {
    this.state.position = { ...eskfState.position };
    // Extract heading from quaternion
    const q = eskfState.orientation;
    const yaw = Math.atan2(2 * (q.w * q.z + q.x * q.y), 1 - 2 * (q.y * q.y + q.z * q.z));
    this.state.heading = ((yaw * 180 / Math.PI) + 360) % 360;
    this.headingFilter = new HeadingFilter(this.state.heading, this.config.headingSmoothing);
    this.state.isActive = true;
    this.state.driftEstimate = 0;
    this.state.confidence = 0.8;
    this.lastTimestamp = eskfState.timestamp;
  }

  /** Process IMU data for dead reckoning */
  processIMU(imu: IMUMeasurement): void {
    if (!this.state.isActive) return;

    const dt = this.lastTimestamp > 0 ? (imu.timestamp - this.lastTimestamp) / 1000 : 0;
    this.lastTimestamp = imu.timestamp;
    if (dt <= 0 || dt > 1) return;

    // Acceleration magnitude (for step detection)
    const accelMag = Math.sqrt(imu.accel.x ** 2 + imu.accel.y ** 2 + imu.accel.z ** 2);
    this.accelHistory.push(accelMag);
    if (this.accelHistory.length > 100) this.accelHistory.shift();

    // Update heading from gyroscope
    this.headingFilter.updateGyro(imu.gyro.z, dt);
    this.state.heading = this.headingFilter.getHeading();

    // Step detection
    if (this.stepDetector.detect(accelMag, imu.timestamp)) {
      this.state.stepCount++;
      this.state.lastStepTimestamp = imu.timestamp;
      this.recentStepTimestamps.push(imu.timestamp);

      // Keep only last 10 steps for frequency estimation
      if (this.recentStepTimestamps.length > 10) this.recentStepTimestamps.shift();

      // Estimate step length using Weinberg model: L = K * (a_max - a_min)^0.25
      const recentAccel = this.accelHistory.slice(-20);
      const aMax = Math.max(...recentAccel);
      const aMin = Math.min(...recentAccel);
      const weinbergK = 0.5; // calibration constant
      const adaptiveStepLength = weinbergK * Math.pow(aMax - aMin, 0.25);
      const stepLength = Math.max(0.3, Math.min(1.5, adaptiveStepLength || this.config.stepLength));

      // Update position
      const headingRad = this.state.heading * Math.PI / 180;
      this.state.position.x += stepLength * Math.sin(headingRad); // East
      this.state.position.y += stepLength * Math.cos(headingRad); // North

      // Estimate speed from step frequency
      if (this.recentStepTimestamps.length >= 2) {
        const timeSpan = (this.recentStepTimestamps[this.recentStepTimestamps.length - 1] -
          this.recentStepTimestamps[0]) / 1000;
        const stepFreq = (this.recentStepTimestamps.length - 1) / timeSpan;
        this.state.speed = stepLength * stepFreq;
      }

      // Accumulate drift estimate (grows with each step)
      this.state.driftEstimate += stepLength * 0.05; // ~5% drift per step

      // Adaptive threshold
      this.stepDetector.adaptThreshold(aMax - aMin);
    }

    // Map matching (snap to nearest road if enabled)
    if (this.config.mapMatchingEnabled && this.nearbyRoads.length > 0) {
      this.applyMapMatching();
    }

    // Update confidence based on drift
    this.state.confidence = Math.max(0.05,
      0.8 * (1 - this.state.driftEstimate / this.config.driftBoundMeters));
  }

  /** Vehicle mode: use accelerometer integration instead of step detection */
  processIMUVehicle(imu: IMUMeasurement): void {
    if (!this.state.isActive) return;

    const dt = this.lastTimestamp > 0 ? (imu.timestamp - this.lastTimestamp) / 1000 : 0;
    this.lastTimestamp = imu.timestamp;
    if (dt <= 0 || dt > 1) return;

    // Heading from gyroscope
    this.headingFilter.updateGyro(imu.gyro.z, dt);
    this.state.heading = this.headingFilter.getHeading();

    // Simple velocity integration (with damping to prevent unbounded growth)
    const DAMPING = 0.98;
    const headingRad = this.state.heading * Math.PI / 180;

    // Forward acceleration (body frame Y axis typically)
    const forwardAccel = imu.accel.y - 9.81 * Math.sin(0); // simplified
    this.state.speed = Math.max(0, this.state.speed * DAMPING + forwardAccel * dt);

    // Position update
    this.state.position.x += this.state.speed * Math.sin(headingRad) * dt;
    this.state.position.y += this.state.speed * Math.cos(headingRad) * dt;

    // Drift grows with time
    this.state.driftEstimate += this.state.speed * dt * 0.02;
    this.state.confidence = Math.max(0.05,
      0.8 * (1 - this.state.driftEstimate / this.config.driftBoundMeters));
  }

  /** Set nearby road segments for map matching */
  setNearbyRoads(roads: RoadSegment[]): void {
    this.nearbyRoads = roads;
  }

  /** Apply map matching to constrain drift */
  private applyMapMatching(): void {
    if (this.nearbyRoads.length === 0) return;

    // Find nearest road segment
    let minDist = Infinity;
    let bestRoad: RoadSegment | null = null;
    let bestProjection: Vec3 | null = null;

    for (const road of this.nearbyRoads) {
      const { dist, projection } = this.pointToSegmentDistance(
        this.state.position, road
      );
      if (dist < minDist) {
        minDist = dist;
        bestRoad = road;
        bestProjection = projection;
      }
    }

    // Snap to road if within 20m
    if (bestRoad && bestProjection && minDist < 20) {
      const snapWeight = Math.min(0.3, 1 / (minDist + 1));
      this.state.position.x += (bestProjection.x - this.state.position.x) * snapWeight;
      this.state.position.y += (bestProjection.y - this.state.position.y) * snapWeight;

      // Correct heading toward road bearing
      this.headingFilter.correctHeading(bestRoad.bearing, snapWeight * 0.5);

      // Reduce drift estimate when map-matched
      this.state.driftEstimate *= 0.95;
    }
  }

  /** Point-to-segment distance calculation */
  private pointToSegmentDistance(point: Vec3, road: RoadSegment): { dist: number; projection: Vec3 } {
    // Simplified: use road bearing and a reference point
    const roadStart: Vec3 = { x: 0, y: 0, z: 0 }; // would be converted from lat/lon
    const bearingRad = road.bearing * Math.PI / 180;
    const roadDir: Vec3 = { x: Math.sin(bearingRad), y: Math.cos(bearingRad), z: 0 };

    // Project point onto road line
    const dx = point.x - roadStart.x;
    const dy = point.y - roadStart.y;
    const t = dx * roadDir.x + dy * roadDir.y;

    const projection: Vec3 = {
      x: roadStart.x + t * roadDir.x,
      y: roadStart.y + t * roadDir.y,
      z: point.z,
    };

    const dist = Math.sqrt((point.x - projection.x) ** 2 + (point.y - projection.y) ** 2);
    return { dist, projection };
  }

  /** Get current PDR state */
  getState(): PDRState {
    return { ...this.state };
  }

  /** Deactivate PDR (when GNSS is restored) */
  deactivate(): void {
    this.state.isActive = false;
  }

  /** Check if drift bound is exceeded */
  isDriftExceeded(): boolean {
    return this.state.driftEstimate >= this.config.driftBoundMeters;
  }
}
