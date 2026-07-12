/**
 * G.A.N.E — Navigation Manager (Continuity Manager)
 * ====================================================
 * Orchestrates all navigation engines:
 *   - ESKF (primary sensor fusion)
 *   - PDR (dead reckoning fallback)
 *   - Visual Odometry (GPS-free navigation)
 *   - FDE (spoofing/jamming detection)
 *   - Spatial Audio (directional cues)
 *
 * Implements the FSM: BOOTING → OPTIMAL_FUSION → DEGRADED_MODE → EMERGENCY_BOUNDED
 * Handles seamless transitions between modes with zero navigation interruption.
 */

import { ESKFEngine, geodeticToENU, enuToGeodetic } from './eskf';
import type { GNSSMeasurement, IMUMeasurement, VisionMeasurement, ESKFState } from './eskf';
import { PDREngine } from './pdr';
import type { RoadSegment } from './pdr';
import { VisualOdometryEngine } from './visualOdometry';
import type { VOResult } from './visualOdometry';
import { SpatialAudioEngine } from './spatialAudio';
import type { AudioCue } from './spatialAudio';

// ─── FSM States ───
export type NavigationMode =
  | 'BOOTING'
  | 'OPTIMAL_FUSION'
  | 'DEGRADED_MODE'
  | 'EMERGENCY_BOUNDED';

// ─── UI Profile Types ───
export type UIProfile = 'private' | 'sports' | 'ems' | 'logistics';

// ─── Events ───
export interface NavigationEvent {
  type: 'gnss_acquired' | 'gnss_lost' | 'gnss_spoofed' | 'imu_update' |
    'vision_update' | 'mode_change' | 'anomaly_detected' | 'route_update' |
    'arrival' | 'recalculating' | 'offline' | 'online';
  timestamp: number;
  data?: unknown;
}

// ─── Telemetry Packet (for WebSocket transmission) ───
export interface TelemetryPacket {
  deviceId: string;
  timestamp: number;
  lat: number;
  lon: number;
  alt: number;
  velocity: number;
  heading: number;
  confidence: number;
  mode: NavigationMode;
  satellites: number;
  batteryLevel: number;
  sensorStatus: {
    gnss: boolean;
    imu: boolean;
    vision: boolean;
    network: boolean;
  };
}

// ─── Navigation State ───
export interface NavigationState {
  mode: NavigationMode;
  position: { lat: number; lon: number; alt: number };
  velocity: number;           // m/s
  heading: number;            // degrees
  confidence: number;         // 0-1
  accuracy: { horizontal: number; vertical: number }; // meters
  satellites: number;
  isOnline: boolean;
  isNavigating: boolean;
  currentRoute: RouteInfo | null;
  uiProfile: UIProfile;
  engineStatus: {
    eskf: boolean;
    pdr: boolean;
    vo: boolean;
    spatialAudio: boolean;
  };
  lastGNSSTime: number;
  spoofingDetected: boolean;
  anomaliesNearby: number;
}

export interface RouteInfo {
  origin: { lat: number; lon: number; name: string };
  destination: { lat: number; lon: number; name: string };
  distance: number;           // meters
  duration: number;           // seconds
  eta: Date;
  steps: RouteStep[];
  currentStepIndex: number;
}

export interface RouteStep {
  instruction: string;
  instructionHe: string;      // Hebrew
  distance: number;
  duration: number;
  bearing: number;
  maneuver: string;
  position: { lat: number; lon: number };
}

// ─── Event Bus ───
type EventHandler = (event: NavigationEvent) => void;

class EventBus {
  private handlers: Map<string, EventHandler[]> = new Map();

  on(type: string, handler: EventHandler): void {
    const existing = this.handlers.get(type) || [];
    existing.push(handler);
    this.handlers.set(type, existing);
  }

  off(type: string, handler: EventHandler): void {
    const existing = this.handlers.get(type) || [];
    this.handlers.set(type, existing.filter(h => h !== handler));
  }

  emit(event: NavigationEvent): void {
    const handlers = this.handlers.get(event.type) || [];
    handlers.forEach(h => {
      try { h(event); } catch (e) { console.error('[EventBus] Handler error:', e); }
    });
    // Also emit to wildcard listeners
    const wildcardHandlers = this.handlers.get('*') || [];
    wildcardHandlers.forEach(h => {
      try { h(event); } catch (e) { console.error('[EventBus] Handler error:', e); }
    });
  }
}

// ─── Navigation Manager ───
export class NavigationManager {
  // Engines
  private eskf: ESKFEngine;
  private pdr: PDREngine;
  private vo: VisualOdometryEngine;
  private spatialAudio: SpatialAudioEngine;

  // State
  private state: NavigationState;
  private eventBus: EventBus;
  private deviceId: string;

  // Timing
  private lastGNSSTimestamp = 0;
  private gnssTimeoutMs = 5000;     // 5s without GNSS → degraded
  private gnssEmergencyMs = 60000;  // 60s → emergency
  private telemetryInterval: ReturnType<typeof setInterval> | null = null;

  // Callbacks
  private onStateChange: ((state: NavigationState) => void) | null = null;
  private onTelemetry: ((packet: TelemetryPacket) => void) | null = null;

  // Bound event handlers for cleanup
  private boundOnline = () => { this.state.isOnline = true; this.eventBus.emit({ type: 'online', timestamp: Date.now() }); };
  private boundOffline = () => { this.state.isOnline = false; this.eventBus.emit({ type: 'offline', timestamp: Date.now() }); };

  constructor(deviceId?: string) {
    this.deviceId = deviceId || this.generateDeviceId();
    this.eskf = new ESKFEngine();
    this.pdr = new PDREngine();
    this.vo = new VisualOdometryEngine();
    this.spatialAudio = new SpatialAudioEngine();
    this.eventBus = new EventBus();

    this.state = {
      mode: 'BOOTING',
      position: { lat: 32.0853, lon: 34.7818, alt: 50 },
      velocity: 0,
      heading: 0,
      confidence: 0,
      accuracy: { horizontal: 100, vertical: 100 },
      satellites: 0,
      isOnline: navigator.onLine,
      isNavigating: false,
      currentRoute: null,
      uiProfile: 'private',
      engineStatus: { eskf: false, pdr: false, vo: false, spatialAudio: false },
      lastGNSSTime: 0,
      spoofingDetected: false,
      anomaliesNearby: 0,
    };

    // Listen for online/offline events
    if (typeof window !== 'undefined') {
      window.addEventListener('online', this.boundOnline);
      window.addEventListener('offline', this.boundOffline);
    }
  }

  /** Initialize all engines */
  async init(): Promise<void> {
    await this.spatialAudio.init();
    this.state.engineStatus.spatialAudio = true;
    this.state.engineStatus.eskf = true;

    // Start telemetry broadcast (1Hz)
    this.telemetryInterval = setInterval(() => {
      this.broadcastTelemetry();
    }, 1000);

    // Transition from BOOTING
    setTimeout(() => {
      if (this.state.mode === 'BOOTING') {
        this.transitionMode('OPTIMAL_FUSION');
      }
    }, 3000);

    this.notifyStateChange();
  }

  /** Process GNSS measurement */
  processGNSS(gnss: GNSSMeasurement): void {
    const result = this.eskf.updateGNSS(gnss);

    if (result.accepted) {
      this.lastGNSSTimestamp = gnss.timestamp;
      this.state.satellites = gnss.satellites;
      this.state.spoofingDetected = false;

      // Update position from ESKF
      const geoState = this.eskf.getGeodeticState();
      this.state.position = { lat: geoState.lat, lon: geoState.lon, alt: geoState.alt };
      this.state.velocity = geoState.speed;
      this.state.heading = geoState.heading;
      this.state.confidence = geoState.confidence;
      this.state.lastGNSSTime = gnss.timestamp;

      const uncertainty = this.eskf.getPositionUncertainty();
      this.state.accuracy = { horizontal: uncertainty.horizontal, vertical: uncertainty.vertical };

      // Deactivate PDR if it was running
      if (this.pdr.getState().isActive) {
        this.pdr.deactivate();
        this.state.engineStatus.pdr = false;
      }

      // Transition to optimal if we were degraded
      if (this.state.mode === 'DEGRADED_MODE' && gnss.satellites >= 4 && gnss.accuracy < 10) {
        this.transitionMode('OPTIMAL_FUSION');
      }

      this.eventBus.emit({ type: 'gnss_acquired', timestamp: gnss.timestamp, data: gnss });
    } else {
      // GNSS rejected (spoofing/jamming detected)
      this.state.spoofingDetected = true;
      this.eventBus.emit({ type: 'gnss_spoofed', timestamp: gnss.timestamp, data: { chiSquare: result.chiSquare } });

      // Play alert
      this.spatialAudio.playCue({
        id: 'spoofing-alert',
        type: 'hazard',
        bearing: 0,
        distance: 0,
        priority: 1,
        message: 'אזהרה: זוהה שיבוש GPS',
      });
    }

    // Check route progress
    if (this.state.isNavigating && this.state.currentRoute) {
      this.updateRouteProgress();
    }

    this.notifyStateChange();
  }

  /** Process IMU measurement */
  processIMU(imu: IMUMeasurement): void {
    // Always feed ESKF
    this.eskf.predictIMU(imu);

    // Check GNSS timeout
    const gnssAge = Date.now() - this.lastGNSSTimestamp;
    if (gnssAge > this.gnssTimeoutMs && this.state.mode === 'OPTIMAL_FUSION') {
      this.activateDegradedMode();
    }
    if (gnssAge > this.gnssEmergencyMs && this.state.mode === 'DEGRADED_MODE') {
      this.transitionMode('EMERGENCY_BOUNDED');
    }

    // Feed PDR if active
    if (this.pdr.getState().isActive) {
      this.pdr.processIMUVehicle(imu);
      const pdrState = this.pdr.getState();
      this.state.confidence = pdrState.confidence;
    }

    // Feed VO with speed estimate
    this.vo.setIMUSpeed(this.state.velocity);

    this.eventBus.emit({ type: 'imu_update', timestamp: imu.timestamp });
  }

  /** Process Vision measurement (from camera) */
  processVision(imageData: Uint8Array | null, timestamp: number): void {
    const result = this.vo.processFrame(imageData, timestamp);

    if (result && result.confidence > 0.3) {
      // Feed VO result to ESKF as a vision measurement
      const visionMeas: VisionMeasurement = {
        deltaPosition: result.translation,
        confidence: result.confidence,
        featureCount: result.inlierCount,
        timestamp: result.timestamp,
      };
      this.eskf.updateVision(visionMeas);
      this.state.engineStatus.vo = true;

      this.eventBus.emit({ type: 'vision_update', timestamp, data: result });
    }
  }

  /** Activate degraded mode (GNSS lost) */
  private activateDegradedMode(): void {
    this.transitionMode('DEGRADED_MODE');

    // Initialize PDR from last known ESKF state
    const eskfState = this.eskf.getState();
    this.pdr.initFromESKF(eskfState);
    this.state.engineStatus.pdr = true;

    // Signal GNSS loss to ESKF
    this.eskf.signalGNSSLoss();

    this.eventBus.emit({ type: 'gnss_lost', timestamp: Date.now() });

    // Audio alert
    this.spatialAudio.playCue({
      id: 'gnss-lost',
      type: 'alert',
      bearing: 0,
      distance: 0,
      priority: 2,
      message: 'אות GPS אבד. ניווט ממשיך במצב חיישנים',
    });
  }

  /** FSM mode transition */
  private transitionMode(newMode: NavigationMode): void {
    const oldMode = this.state.mode;
    if (oldMode === newMode) return;

    this.state.mode = newMode;
    this.eventBus.emit({
      type: 'mode_change',
      timestamp: Date.now(),
      data: { from: oldMode, to: newMode },
    });

    this.notifyStateChange();
  }

  /** Start navigation to a destination */
  startNavigation(route: RouteInfo): void {
    this.state.isNavigating = true;
    this.state.currentRoute = route;

    // Play departure audio
    this.spatialAudio.playCue({
      id: 'nav-start',
      type: 'arrival',
      bearing: route.steps[0]?.bearing ?? 0,
      distance: 5,
      priority: 1,
      message: `מתחיל ניווט. ${Math.round(route.distance / 1000)} קילומטר, ${Math.round(route.duration / 60)} דקות`,
    });

    this.notifyStateChange();
  }

  /** Update route progress and trigger audio cues */
  private updateRouteProgress(): void {
    if (!this.state.currentRoute) return;

    const route = this.state.currentRoute;
    const pos = this.state.position;

    // Find current step
    let minDist = Infinity;
    let closestStep = route.currentStepIndex;

    for (let i = route.currentStepIndex; i < route.steps.length; i++) {
      const step = route.steps[i];
      const dist = this.haversineDistance(pos.lat, pos.lon, step.position.lat, step.position.lon);
      if (dist < minDist) {
        minDist = dist;
        closestStep = i;
      }
    }

    // Advance step if we passed it
    if (closestStep > route.currentStepIndex) {
      route.currentStepIndex = closestStep;
    }

    // Upcoming turn alert (200m before)
    const nextStep = route.steps[route.currentStepIndex];
    if (nextStep) {
      const distToNext = this.haversineDistance(pos.lat, pos.lon, nextStep.position.lat, nextStep.position.lon);

      if (distToNext < 200 && distToNext > 50) {
        this.spatialAudio.playCue({
          id: `turn-${route.currentStepIndex}`,
          type: 'turn',
          bearing: nextStep.bearing,
          distance: distToNext,
          priority: 1,
          message: nextStep.instructionHe,
        });
      }
    }

    // Check arrival
    const lastStep = route.steps[route.steps.length - 1];
    if (lastStep) {
      const distToEnd = this.haversineDistance(pos.lat, pos.lon, lastStep.position.lat, lastStep.position.lon);
      if (distToEnd < 30) {
        this.state.isNavigating = false;
        this.spatialAudio.playCue({
          id: 'arrival',
          type: 'arrival',
          bearing: 0,
          distance: 0,
          priority: 1,
          message: 'הגעת ליעד',
        });
        this.eventBus.emit({ type: 'arrival', timestamp: Date.now() });
      }
    }
  }

  /** Set UI profile */
  setUIProfile(profile: UIProfile): void {
    this.state.uiProfile = profile;
    this.notifyStateChange();
  }

  /** Set nearby road segments for map matching */
  setNearbyRoads(roads: RoadSegment[]): void {
    this.pdr.setNearbyRoads(roads);
  }

  /** Play a navigation audio cue */
  playAudioCue(cue: AudioCue): void {
    this.spatialAudio.playCue(cue);
  }

  /** Subscribe to navigation events */
  on(type: string, handler: EventHandler): void {
    this.eventBus.on(type, handler);
  }

  /** Unsubscribe from events */
  off(type: string, handler: EventHandler): void {
    this.eventBus.off(type, handler);
  }

  /** Set state change callback */
  setOnStateChange(callback: (state: NavigationState) => void): void {
    this.onStateChange = callback;
  }

  /** Set telemetry callback (for WebSocket transmission) */
  setOnTelemetry(callback: (packet: TelemetryPacket) => void): void {
    this.onTelemetry = callback;
  }

  /** Get Visual Odometry engine for CameraBridge connection */
  getVOEngine(): VisualOdometryEngine {
    return this.vo;
  }

  /** Get current navigation state */
  getState(): NavigationState {
    return { ...this.state };
  }

  /** Broadcast telemetry packet */
  private broadcastTelemetry(): void {
    if (!this.onTelemetry) return;

    const packet: TelemetryPacket = {
      deviceId: this.deviceId,
      timestamp: Date.now(),
      lat: this.state.position.lat,
      lon: this.state.position.lon,
      alt: this.state.position.alt,
      velocity: this.state.velocity,
      heading: this.state.heading,
      confidence: this.state.confidence,
      mode: this.state.mode,
      satellites: this.state.satellites,
      batteryLevel: 100, // would come from Battery API
      sensorStatus: {
        gnss: Date.now() - this.lastGNSSTimestamp < this.gnssTimeoutMs,
        imu: true,
        vision: this.vo.getState().isActive,
        network: this.state.isOnline,
      },
    };

    this.onTelemetry(packet);
  }

  /** Notify state change */
  private notifyStateChange(): void {
    if (this.onStateChange) {
      this.onStateChange(this.getState());
    }
  }

  /** Haversine distance (meters) */
  private haversineDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const R = 6371000;
    const toRad = Math.PI / 180;
    const dLat = (lat2 - lat1) * toRad;
    const dLon = (lon2 - lon1) * toRad;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * toRad) * Math.cos(lat2 * toRad) * Math.sin(dLon / 2) ** 2;
    return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  /** Generate unique device ID */
  private generateDeviceId(): string {
    const stored = typeof localStorage !== 'undefined' ? localStorage.getItem('gane-device-id') : null;
    if (stored) return stored;
    const id = 'gane-' + Date.now().toString(36) + '-' + Math.random().toString(36).slice(2, 8);
    if (typeof localStorage !== 'undefined') localStorage.setItem('gane-device-id', id);
    return id;
  }

  /** Cleanup */
  destroy(): void {
    if (this.telemetryInterval) clearInterval(this.telemetryInterval);
    // Remove online/offline listeners
    if (typeof window !== 'undefined') {
      window.removeEventListener('online', this.boundOnline);
      window.removeEventListener('offline', this.boundOffline);
    }
    this.spatialAudio.destroy();
    this.vo.deactivate();
    this.pdr.deactivate();
  }
}

// ─── Singleton export ───
let _instance: NavigationManager | null = null;

export function getNavigationManager(): NavigationManager {
  if (!_instance) {
    _instance = new NavigationManager();
  }
  return _instance;
}

export function resetNavigationManager(): void {
  _instance?.destroy();
  _instance = null;
}
