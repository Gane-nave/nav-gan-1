/**
 * G.A.N.E — Tunnel Recovery Engine
 * ===================================
 * Dead-reckoning + map matching during GNSS blackout zones:
 * - Tunnels, underground parking, covered highways
 * - Dense urban canyons with total signal loss
 * - Indoor transitions (mall parking → road)
 *
 * Algorithm:
 * 1. Detect GNSS loss (satellite count drops below threshold)
 * 2. Switch to IMU-based dead reckoning (accelerometer + gyroscope)
 * 3. Apply road constraint (snap to known road geometry)
 * 4. Estimate position using last known speed + heading
 * 5. On GNSS recovery, smooth transition back to satellite fix
 *
 * Accuracy target: <15m drift per 1km of tunnel
 */

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export interface TunnelState {
  isInTunnel: boolean;
  entryTime: number;
  entryLat: number;
  entryLon: number;
  entryHeading: number;
  entrySpeed: number;
  currentLat: number;
  currentLon: number;
  currentHeading: number;
  currentSpeed: number;
  distanceTraveled: number;
  driftEstimate: number;       // estimated position error in meters
  confidenceScore: number;     // 0-1
  imuSamples: number;
  recoveryPhase: 'none' | 'dead_reckoning' | 'recovery_blend' | 'recovered';
  lastGnssTime: number;
  blackoutDurationMs: number;
}

interface IMUSample {
  timestamp: number;
  accelX: number;             // m/s² (forward)
  accelY: number;             // m/s² (lateral)
  accelZ: number;             // m/s² (vertical)
  gyroX: number;              // rad/s (pitch)
  gyroY: number;              // rad/s (roll)
  gyroZ: number;              // rad/s (yaw)
}

interface RoadSegment {
  startLat: number;
  startLon: number;
  endLat: number;
  endLon: number;
  heading: number;            // degrees
  speedLimit: number;         // m/s
  length: number;             // meters
}

// ═══════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════

const GNSS_LOSS_THRESHOLD = 3;           // satellites below this = tunnel mode
const GNSS_RECOVERY_THRESHOLD = 5;       // satellites above this = recovery
const RECOVERY_BLEND_DURATION_MS = 3000; // smooth transition duration
const MAX_DEAD_RECKONING_MS = 600_000;   // 10 minutes max DR
const DRIFT_RATE_M_PER_KM = 12;          // expected drift per km
const EARTH_RADIUS_M = 6_371_000;
const DEG_TO_RAD = Math.PI / 180;
const RAD_TO_DEG = 180 / Math.PI;

// ═══════════════════════════════════════════════════
// TUNNEL RECOVERY ENGINE
// ═══════════════════════════════════════════════════

export class TunnelRecoveryEngine {
  private state: TunnelState;
  private imuBuffer: IMUSample[] = [];
  private readonly MAX_IMU_BUFFER = 1000;
  private roadSegments: RoadSegment[] = [];
  private listeners: Array<(state: TunnelState) => void> = [];

  // Kalman-like state for speed estimation
  private speedEstimate: number = 0;
  private speedVariance: number = 1;
  private headingEstimate: number = 0;
  private headingVariance: number = 0.01;

  constructor() {
    this.state = this.createInitialState();
  }

  private createInitialState(): TunnelState {
    return {
      isInTunnel: false,
      entryTime: 0,
      entryLat: 0,
      entryLon: 0,
      entryHeading: 0,
      entrySpeed: 0,
      currentLat: 0,
      currentLon: 0,
      currentHeading: 0,
      currentSpeed: 0,
      distanceTraveled: 0,
      driftEstimate: 0,
      confidenceScore: 1,
      imuSamples: 0,
      recoveryPhase: 'none',
      lastGnssTime: 0,
      blackoutDurationMs: 0,
    };
  }

  /**
   * Process GNSS update — detect tunnel entry/exit
   */
  processGNSSUpdate(
    lat: number, lon: number, heading: number, speed: number,
    satelliteCount: number, timestamp: number
  ): TunnelState {
    if (satelliteCount < GNSS_LOSS_THRESHOLD && !this.state.isInTunnel) {
      // Enter tunnel mode
      this.enterTunnel(lat, lon, heading, speed, timestamp);
    } else if (satelliteCount >= GNSS_RECOVERY_THRESHOLD && this.state.isInTunnel) {
      // Begin recovery blend
      this.beginRecovery(lat, lon, heading, speed, timestamp);
    } else if (!this.state.isInTunnel) {
      // Normal GNSS operation — update last known position
      this.state.lastGnssTime = timestamp;
      this.state.currentLat = lat;
      this.state.currentLon = lon;
      this.state.currentHeading = heading;
      this.state.currentSpeed = speed;
      this.speedEstimate = speed;
      this.headingEstimate = heading;
    }

    return this.getState();
  }

  /**
   * Process IMU sample during tunnel mode
   */
  processIMU(sample: IMUSample): TunnelState {
    if (!this.state.isInTunnel) return this.getState();

    this.imuBuffer.push(sample);
    if (this.imuBuffer.length > this.MAX_IMU_BUFFER) {
      this.imuBuffer.shift();
    }
    this.state.imuSamples++;

    // Dead reckoning step
    const dt = this.imuBuffer.length >= 2
      ? (sample.timestamp - this.imuBuffer[this.imuBuffer.length - 2].timestamp) / 1000
      : 0.01; // 100Hz default

    if (dt <= 0 || dt > 1) return this.getState(); // skip bad samples

    // Update heading from gyroscope (yaw rate)
    const yawRate = sample.gyroZ; // rad/s
    this.headingEstimate += yawRate * dt * RAD_TO_DEG;
    this.headingEstimate = ((this.headingEstimate % 360) + 360) % 360;
    this.headingVariance += Math.abs(yawRate) * dt * 0.01;

    // Update speed from accelerometer (forward axis)
    const forwardAccel = sample.accelX;
    // Simple complementary filter for speed
    const processNoise = 0.5;
    const measurementNoise = 2.0;
    const kalmanGain = this.speedVariance / (this.speedVariance + measurementNoise);
    const predictedSpeed = this.speedEstimate + forwardAccel * dt;
    this.speedEstimate = predictedSpeed + kalmanGain * (predictedSpeed - this.speedEstimate);
    this.speedVariance = (1 - kalmanGain) * this.speedVariance + processNoise * dt;

    // Clamp speed to reasonable bounds
    this.speedEstimate = Math.max(0, Math.min(this.speedEstimate, 50)); // 0-180 km/h

    // Update position using dead reckoning
    const distance = this.speedEstimate * dt;
    this.state.distanceTraveled += distance;

    const headingRad = this.headingEstimate * DEG_TO_RAD;
    const dLat = (distance * Math.cos(headingRad)) / EARTH_RADIUS_M * RAD_TO_DEG;
    const dLon = (distance * Math.sin(headingRad)) /
      (EARTH_RADIUS_M * Math.cos(this.state.currentLat * DEG_TO_RAD)) * RAD_TO_DEG;

    this.state.currentLat += dLat;
    this.state.currentLon += dLon;
    this.state.currentHeading = this.headingEstimate;
    this.state.currentSpeed = this.speedEstimate;

    // Update drift estimate
    this.state.driftEstimate = (this.state.distanceTraveled / 1000) * DRIFT_RATE_M_PER_KM;

    // Update confidence (degrades over time)
    const blackoutMs = sample.timestamp - this.state.entryTime;
    this.state.blackoutDurationMs = blackoutMs;
    this.state.confidenceScore = Math.max(0.1,
      1.0 - (blackoutMs / MAX_DEAD_RECKONING_MS) * 0.9
    );

    // Apply road constraint if available
    this.applyRoadConstraint();

    this.notifyListeners();
    return this.getState();
  }

  /**
   * Set nearby road segments for map matching constraint
   */
  setRoadSegments(segments: RoadSegment[]): void {
    this.roadSegments = segments;
  }

  /**
   * Enter tunnel mode
   */
  private enterTunnel(lat: number, lon: number, heading: number, speed: number, timestamp: number): void {
    this.state.isInTunnel = true;
    this.state.entryTime = timestamp;
    this.state.entryLat = lat;
    this.state.entryLon = lon;
    this.state.entryHeading = heading;
    this.state.entrySpeed = speed;
    this.state.currentLat = lat;
    this.state.currentLon = lon;
    this.state.currentHeading = heading;
    this.state.currentSpeed = speed;
    this.state.distanceTraveled = 0;
    this.state.driftEstimate = 0;
    this.state.confidenceScore = 1;
    this.state.imuSamples = 0;
    this.state.recoveryPhase = 'dead_reckoning';
    this.state.blackoutDurationMs = 0;

    this.speedEstimate = speed;
    this.speedVariance = 1;
    this.headingEstimate = heading;
    this.headingVariance = 0.01;
    this.imuBuffer = [];

    this.notifyListeners();
  }

  /**
   * Begin recovery blend — smooth transition from DR to GNSS
   */
  private beginRecovery(
    gnssLat: number, gnssLon: number, gnssHeading: number,
    gnssSpeed: number, timestamp: number
  ): void {
    this.state.recoveryPhase = 'recovery_blend';

    // Smooth blend from DR position to GNSS position
    const blendSteps = 10;
    const latStep = (gnssLat - this.state.currentLat) / blendSteps;
    const lonStep = (gnssLon - this.state.currentLon) / blendSteps;

    // Apply final position immediately (in real implementation, this would be animated)
    this.state.currentLat = gnssLat;
    this.state.currentLon = gnssLon;
    this.state.currentHeading = gnssHeading;
    this.state.currentSpeed = gnssSpeed;
    this.state.isInTunnel = false;
    this.state.recoveryPhase = 'recovered';
    this.state.confidenceScore = 1;
    this.state.lastGnssTime = timestamp;

    this.speedEstimate = gnssSpeed;
    this.headingEstimate = gnssHeading;

    this.notifyListeners();

    // Reset to normal after blend duration
    setTimeout(() => {
      this.state.recoveryPhase = 'none';
      this.notifyListeners();
    }, RECOVERY_BLEND_DURATION_MS);
  }

  /**
   * Apply road constraint — snap position to nearest road segment
   */
  private applyRoadConstraint(): void {
    if (this.roadSegments.length === 0) return;

    let bestDist = Infinity;
    let bestLat = this.state.currentLat;
    let bestLon = this.state.currentLon;
    let bestHeading = this.state.currentHeading;

    for (const seg of this.roadSegments) {
      const projected = this.projectPointOnSegment(
        this.state.currentLat, this.state.currentLon,
        seg.startLat, seg.startLon, seg.endLat, seg.endLon
      );

      if (projected.distance < bestDist) {
        bestDist = projected.distance;
        bestLat = projected.lat;
        bestLon = projected.lon;
        bestHeading = seg.heading;
      }
    }

    // Only snap if within reasonable distance (50m)
    if (bestDist < 50) {
      // Weighted blend: 70% road constraint, 30% DR
      const roadWeight = 0.7;
      this.state.currentLat = this.state.currentLat * (1 - roadWeight) + bestLat * roadWeight;
      this.state.currentLon = this.state.currentLon * (1 - roadWeight) + bestLon * roadWeight;

      // Heading from road is more reliable
      const headingDiff = bestHeading - this.state.currentHeading;
      if (Math.abs(headingDiff) < 45) {
        this.state.currentHeading = this.state.currentHeading * 0.3 + bestHeading * 0.7;
      }

      // Reduce drift estimate when road-constrained
      this.state.driftEstimate *= 0.6;
    }
  }

  /**
   * Project a point onto a line segment, return closest point and distance
   */
  private projectPointOnSegment(
    pLat: number, pLon: number,
    aLat: number, aLon: number,
    bLat: number, bLon: number
  ): { lat: number; lon: number; distance: number } {
    const dx = bLon - aLon;
    const dy = bLat - aLat;
    const lenSq = dx * dx + dy * dy;

    if (lenSq === 0) {
      return { lat: aLat, lon: aLon, distance: this.haversine(pLat, pLon, aLat, aLon) };
    }

    let t = ((pLon - aLon) * dx + (pLat - aLat) * dy) / lenSq;
    t = Math.max(0, Math.min(1, t));

    const projLat = aLat + t * dy;
    const projLon = aLon + t * dx;

    return {
      lat: projLat,
      lon: projLon,
      distance: this.haversine(pLat, pLon, projLat, projLon),
    };
  }

  private haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const dLat = (lat2 - lat1) * DEG_TO_RAD;
    const dLon = (lon2 - lon1) * DEG_TO_RAD;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * DEG_TO_RAD) * Math.cos(lat2 * DEG_TO_RAD) *
      Math.sin(dLon / 2) ** 2;
    return EARTH_RADIUS_M * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  // ─── Public API ───

  getState(): TunnelState {
    return { ...this.state };
  }

  subscribe(listener: (state: TunnelState) => void): () => void {
    this.listeners.push(listener);
    return () => {
      this.listeners = this.listeners.filter(l => l !== listener);
    };
  }

  private notifyListeners(): void {
    for (const listener of this.listeners) {
      try { listener(this.state); } catch { /* ignore */ }
    }
  }

  reset(): void {
    this.state = this.createInitialState();
    this.imuBuffer = [];
    this.roadSegments = [];
    this.notifyListeners();
  }

  destroy(): void {
    this.listeners = [];
    this.imuBuffer = [];
    this.roadSegments = [];
  }
}
