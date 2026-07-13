/**
 * G.A.N.E — Real-Time Reroute Engine
 * ====================================
 * Continuous route recompute loop with <500ms target.
 *
 * TRIGGERS:
 *   1. Deviation from route > threshold (50m default)
 *   2. Congestion spike on current route
 *   3. Incident detected on route
 *   4. ETA drift exceeds tolerance
 *   5. Manual reroute request
 *
 * LOGIC:
 *   - Hysteresis: prevent route flapping (min 30s between reroutes)
 *   - Confidence scoring per alternative route
 *   - Multi-option ranking (time, distance, reliability)
 *   - Deviation detection via perpendicular distance to route polyline
 *
 * OUTPUT:
 *   - route_update event
 *   - alternative routes with scores
 *   - reroute reason + confidence
 */

// ─── Types ───────────────────────────────────────────────

export interface RerouteConfig {
  deviationThresholdM: number;       // meters off-route to trigger (default: 50)
  congestionThreshold: number;       // congestion index 0-1 to trigger (default: 0.7)
  etaDriftThresholdS: number;        // seconds of ETA drift to trigger (default: 120)
  hysteresisMs: number;              // minimum ms between reroutes (default: 30000)
  maxAlternatives: number;           // max alternative routes to compute (default: 3)
  checkIntervalMs: number;           // how often to check for reroute (default: 2000)
  enabled: boolean;
}

export interface RerouteReason {
  type: 'deviation' | 'congestion' | 'incident' | 'eta_drift' | 'manual' | 'road_closure';
  details: string;
  severity: number;                  // 0-1
  detectedAt: number;               // Unix ms
}

export interface RouteAlternative {
  id: string;
  path: { lat: number; lon: number }[];
  distanceM: number;
  durationS: number;
  trafficDelayS: number;
  congestionIndex: number;           // avg congestion on this route
  score: number;                     // composite score (lower = better)
  savings: {
    timeS: number;                   // time saved vs current route
    distanceM: number;               // distance difference
  };
}

export interface RerouteResult {
  shouldReroute: boolean;
  reason: RerouteReason | null;
  alternatives: RouteAlternative[];
  bestAlternative: RouteAlternative | null;
  computeTimeMs: number;
  confidence: number;                // 0-1
  timestamp: number;
}

export interface RerouteState {
  isActive: boolean;
  lastRerouteAt: number;
  totalReroutes: number;
  consecutiveDeviations: number;
  currentDeviationM: number;
  currentCongestion: number;
  currentEtaDriftS: number;
  lastCheckAt: number;
  lastResult: RerouteResult | null;
}

export interface RoutePoint {
  lat: number;
  lon: number;
}

export interface ActiveRoute {
  points: RoutePoint[];
  distanceM: number;
  durationS: number;
  originalEtaS: number;
  startedAt: number;
}

export interface TrafficSegment {
  segmentId: string;
  congestionIndex: number;
  avgSpeed: number;
  centerLat: number;
  centerLon: number;
}

export interface Incident {
  id: string;
  type: string;
  lat: number;
  lon: number;
  radiusM: number;
  severity: number;
  isActive: boolean;
}

// ─── Callbacks ──────────────────────────────────────────

type RerouteCallback = (result: RerouteResult) => void;
type DeviationCallback = (deviationM: number) => void;

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: RerouteConfig = {
  deviationThresholdM: 50,
  congestionThreshold: 0.7,
  etaDriftThresholdS: 120,
  hysteresisMs: 30000,
  maxAlternatives: 3,
  checkIntervalMs: 2000,
  enabled: true,
};

const DEG_TO_RAD = Math.PI / 180;
const EARTH_RADIUS_M = 6371000;

// ─── Geometry Helpers ───────────────────────────────────

function haversineDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const dLat = (lat2 - lat1) * DEG_TO_RAD;
  const dLon = (lon2 - lon1) * DEG_TO_RAD;
  const a = Math.sin(dLat / 2) ** 2 +
    Math.cos(lat1 * DEG_TO_RAD) * Math.cos(lat2 * DEG_TO_RAD) *
    Math.sin(dLon / 2) ** 2;
  return EARTH_RADIUS_M * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

/**
 * Perpendicular distance from point P to line segment AB.
 * Used for deviation detection.
 */
function pointToSegmentDistance(
  pLat: number, pLon: number,
  aLat: number, aLon: number,
  bLat: number, bLon: number
): number {
  const ab = haversineDistance(aLat, aLon, bLat, bLon);
  if (ab < 0.1) return haversineDistance(pLat, pLon, aLat, aLon);

  // Project P onto AB using dot product approximation
  const apLat = pLat - aLat;
  const apLon = pLon - aLon;
  const abLat = bLat - aLat;
  const abLon = bLon - aLon;

  const t = Math.max(0, Math.min(1,
    (apLat * abLat + apLon * abLon) / (abLat * abLat + abLon * abLon)
  ));

  const projLat = aLat + t * abLat;
  const projLon = aLon + t * abLon;

  return haversineDistance(pLat, pLon, projLat, projLon);
}

/**
 * Find minimum distance from a point to a polyline.
 * Returns the minimum perpendicular distance to any segment.
 */
function pointToPolylineDistance(
  lat: number, lon: number,
  polyline: RoutePoint[]
): number {
  if (polyline.length === 0) return Infinity;
  if (polyline.length === 1) return haversineDistance(lat, lon, polyline[0].lat, polyline[0].lon);

  let minDist = Infinity;
  for (let i = 0; i < polyline.length - 1; i++) {
    const dist = pointToSegmentDistance(
      lat, lon,
      polyline[i].lat, polyline[i].lon,
      polyline[i + 1].lat, polyline[i + 1].lon
    );
    if (dist < minDist) minDist = dist;
  }
  return minDist;
}

// ─── Reroute Engine ─────────────────────────────────────

export class RerouteEngine {
  private config: RerouteConfig;
  private state: RerouteState;
  private interval: ReturnType<typeof setInterval> | null = null;

  // Current navigation state
  private currentPosition: RoutePoint | null = null;
  private activeRoute: ActiveRoute | null = null;
  private trafficSegments: TrafficSegment[] = [];
  private activeIncidents: Incident[] = [];
  private currentEtaS = 0;

  // Callbacks
  private onReroute: RerouteCallback | null = null;
  private onDeviation: DeviationCallback | null = null;

  // Metrics
  private metrics = {
    totalChecks: 0,
    totalReroutes: 0,
    avgComputeMs: 0,
    maxComputeMs: 0,
    deviationTriggers: 0,
    congestionTriggers: 0,
    incidentTriggers: 0,
    etaDriftTriggers: 0,
    manualTriggers: 0,
    hysteresisBlocks: 0,
  };

  constructor(config: Partial<RerouteConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      isActive: false,
      lastRerouteAt: 0,
      totalReroutes: 0,
      consecutiveDeviations: 0,
      currentDeviationM: 0,
      currentCongestion: 0,
      currentEtaDriftS: 0,
      lastCheckAt: 0,
      lastResult: null,
    };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  start() {
    if (this.state.isActive) return;
    this.state.isActive = true;

    this.interval = setInterval(() => {
      if (this.config.enabled && this.activeRoute && this.currentPosition) {
        this.checkAndReroute();
      }
    }, this.config.checkIntervalMs);
  }

  stop() {
    this.state.isActive = false;
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
  }

  // ─── Input Updates ────────────────────────────────────

  updatePosition(lat: number, lon: number) {
    this.currentPosition = { lat, lon };
  }

  setActiveRoute(route: ActiveRoute) {
    this.activeRoute = route;
    this.state.consecutiveDeviations = 0;
    this.state.currentDeviationM = 0;
  }

  clearRoute() {
    this.activeRoute = null;
    this.state.consecutiveDeviations = 0;
    this.state.currentDeviationM = 0;
  }

  updateTraffic(segments: TrafficSegment[]) {
    this.trafficSegments = segments;
  }

  updateIncidents(incidents: Incident[]) {
    this.activeIncidents = incidents;
  }

  updateCurrentEta(etaS: number) {
    this.currentEtaS = etaS;
  }

  // ─── Callbacks ────────────────────────────────────────

  setOnReroute(callback: RerouteCallback) {
    this.onReroute = callback;
  }

  setOnDeviation(callback: DeviationCallback) {
    this.onDeviation = callback;
  }

  // ─── Core Check Loop ─────────────────────────────────

  checkAndReroute(): RerouteResult {
    const startTime = performance.now();
    this.metrics.totalChecks++;
    this.state.lastCheckAt = Date.now();

    const result: RerouteResult = {
      shouldReroute: false,
      reason: null,
      alternatives: [],
      bestAlternative: null,
      computeTimeMs: 0,
      confidence: 0,
      timestamp: Date.now(),
    };

    if (!this.activeRoute || !this.currentPosition) {
      result.computeTimeMs = performance.now() - startTime;
      return result;
    }

    // Check hysteresis
    const timeSinceLastReroute = Date.now() - this.state.lastRerouteAt;
    if (timeSinceLastReroute < this.config.hysteresisMs && this.state.lastRerouteAt > 0) {
      this.metrics.hysteresisBlocks++;
      result.computeTimeMs = performance.now() - startTime;
      return result;
    }

    // Check all triggers
    const reason = this.evaluateTriggers();

    if (reason) {
      result.shouldReroute = true;
      result.reason = reason;

      // Compute alternatives
      result.alternatives = this.computeAlternatives();
      result.bestAlternative = result.alternatives[0] || null;
      result.confidence = this.computeConfidence(reason, result.alternatives);

      // Update state
      this.state.lastRerouteAt = Date.now();
      this.state.totalReroutes++;
      this.state.lastResult = result;
      this.metrics.totalReroutes++;

      // Notify
      if (this.onReroute) {
        this.onReroute(result);
      }
    }

    result.computeTimeMs = Math.round((performance.now() - startTime) * 100) / 100;
    this.updateMetrics(result.computeTimeMs);
    this.state.lastResult = result;

    return result;
  }

  // ─── Trigger Evaluation ───────────────────────────────

  private evaluateTriggers(): RerouteReason | null {
    // Priority order: incident > deviation > congestion > eta_drift

    // 1. Check for incidents on route
    const incidentReason = this.checkIncidents();
    if (incidentReason) {
      this.metrics.incidentTriggers++;
      return incidentReason;
    }

    // 2. Check deviation from route
    const deviationReason = this.checkDeviation();
    if (deviationReason) {
      this.metrics.deviationTriggers++;
      return deviationReason;
    }

    // 3. Check congestion spike
    const congestionReason = this.checkCongestion();
    if (congestionReason) {
      this.metrics.congestionTriggers++;
      return congestionReason;
    }

    // 4. Check ETA drift
    const etaReason = this.checkEtaDrift();
    if (etaReason) {
      this.metrics.etaDriftTriggers++;
      return etaReason;
    }

    return null;
  }

  private checkDeviation(): RerouteReason | null {
    if (!this.currentPosition || !this.activeRoute) return null;

    const deviation = pointToPolylineDistance(
      this.currentPosition.lat,
      this.currentPosition.lon,
      this.activeRoute.points
    );

    this.state.currentDeviationM = Math.round(deviation);

    if (this.onDeviation) {
      this.onDeviation(deviation);
    }

    if (deviation > this.config.deviationThresholdM) {
      this.state.consecutiveDeviations++;

      // Require 2+ consecutive deviations to avoid GPS jitter
      if (this.state.consecutiveDeviations >= 2) {
        return {
          type: 'deviation',
          details: `Off-route by ${Math.round(deviation)}m (threshold: ${this.config.deviationThresholdM}m)`,
          severity: Math.min(1, deviation / (this.config.deviationThresholdM * 3)),
          detectedAt: Date.now(),
        };
      }
    } else {
      this.state.consecutiveDeviations = 0;
    }

    return null;
  }

  private checkCongestion(): RerouteReason | null {
    if (!this.activeRoute || this.trafficSegments.length === 0) return null;

    // Check congestion on route segments
    const routeSegments = this.getSegmentsOnRoute();
    if (routeSegments.length === 0) return null;

    const avgCongestion = routeSegments.reduce((s, seg) => s + seg.congestionIndex, 0) / routeSegments.length;
    const maxCongestion = Math.max(...routeSegments.map(s => s.congestionIndex));

    this.state.currentCongestion = avgCongestion;

    if (maxCongestion > this.config.congestionThreshold) {
      return {
        type: 'congestion',
        details: `Congestion spike: avg=${(avgCongestion * 100).toFixed(0)}%, max=${(maxCongestion * 100).toFixed(0)}%`,
        severity: maxCongestion,
        detectedAt: Date.now(),
      };
    }

    return null;
  }

  private checkIncidents(): RerouteReason | null {
    if (!this.activeRoute || this.activeIncidents.length === 0) return null;

    for (const incident of this.activeIncidents) {
      if (!incident.isActive) continue;

      // Check if incident is near the route
      const distToRoute = pointToPolylineDistance(
        incident.lat, incident.lon,
        this.activeRoute.points
      );

      if (distToRoute < incident.radiusM + 100) { // 100m buffer
        return {
          type: 'incident',
          details: `${incident.type} detected ${Math.round(distToRoute)}m from route`,
          severity: incident.severity,
          detectedAt: Date.now(),
        };
      }
    }

    return null;
  }

  private checkEtaDrift(): RerouteReason | null {
    if (!this.activeRoute || this.currentEtaS <= 0) return null;

    const drift = this.currentEtaS - this.activeRoute.originalEtaS;
    this.state.currentEtaDriftS = drift;

    if (drift > this.config.etaDriftThresholdS) {
      return {
        type: 'eta_drift',
        details: `ETA drifted by ${Math.round(drift)}s (threshold: ${this.config.etaDriftThresholdS}s)`,
        severity: Math.min(1, drift / (this.config.etaDriftThresholdS * 3)),
        detectedAt: Date.now(),
      };
    }

    return null;
  }

  // ─── Alternative Route Computation ────────────────────

  private computeAlternatives(): RouteAlternative[] {
    if (!this.activeRoute || !this.currentPosition) return [];

    const alternatives: RouteAlternative[] = [];
    const destination = this.activeRoute.points[this.activeRoute.points.length - 1];
    if (!destination) return [];

    // Generate synthetic alternative routes
    // In production, this would call the RouteGraphEngine
    // For now, we generate plausible alternatives with traffic-aware scoring

    const directDist = haversineDistance(
      this.currentPosition.lat, this.currentPosition.lon,
      destination.lat, destination.lon
    );

    for (let i = 0; i < this.config.maxAlternatives; i++) {
      const detourFactor = 1 + (i * 0.15); // 0%, 15%, 30% longer
      const speedFactor = 1 - (i * 0.1);   // progressively less congested

      const altDistance = directDist * detourFactor * (1.2 + Math.random() * 0.3);
      const avgSpeedKmh = (60 + Math.random() * 40) * speedFactor;
      const altDuration = (altDistance / 1000) / avgSpeedKmh * 3600;
      const trafficDelay = altDuration * (0.05 + Math.random() * 0.15);
      const congestion = Math.max(0, this.state.currentCongestion * (1 - i * 0.3));

      // Generate intermediate points for the alternative
      const midLat = (this.currentPosition.lat + destination.lat) / 2 + (Math.random() - 0.5) * 0.01 * (i + 1);
      const midLon = (this.currentPosition.lon + destination.lon) / 2 + (Math.random() - 0.5) * 0.01 * (i + 1);

      const path: RoutePoint[] = [
        { lat: this.currentPosition.lat, lon: this.currentPosition.lon },
        { lat: midLat, lon: midLon },
        { lat: destination.lat, lon: destination.lon },
      ];

      const totalDuration = altDuration + trafficDelay;
      const currentDuration = this.activeRoute.durationS;

      // Composite score: lower = better
      // Weighted: 60% time, 25% congestion, 15% distance
      const timeScore = totalDuration / 60;
      const congestionScore = congestion * 100;
      const distScore = altDistance / 1000;
      const score = timeScore * 0.6 + congestionScore * 0.25 + distScore * 0.15;

      alternatives.push({
        id: `alt_${Date.now()}_${i}`,
        path,
        distanceM: Math.round(altDistance),
        durationS: Math.round(totalDuration),
        trafficDelayS: Math.round(trafficDelay),
        congestionIndex: Math.round(congestion * 1000) / 1000,
        score: Math.round(score * 100) / 100,
        savings: {
          timeS: Math.round(currentDuration - totalDuration),
          distanceM: Math.round(this.activeRoute.distanceM - altDistance),
        },
      });
    }

    // Sort by score (lower = better)
    alternatives.sort((a, b) => a.score - b.score);

    return alternatives;
  }

  // ─── Helper Methods ───────────────────────────────────

  private getSegmentsOnRoute(): TrafficSegment[] {
    if (!this.activeRoute) return [];

    return this.trafficSegments.filter(seg => {
      const dist = pointToPolylineDistance(
        seg.centerLat, seg.centerLon,
        this.activeRoute!.points
      );
      return dist < 200; // Within 200m of route
    });
  }

  private computeConfidence(reason: RerouteReason, alternatives: RouteAlternative[]): number {
    let confidence = 0.5;

    // Higher confidence for clear triggers
    if (reason.type === 'deviation') confidence += 0.3;
    if (reason.type === 'incident') confidence += 0.25;
    if (reason.type === 'congestion') confidence += 0.15;
    if (reason.type === 'eta_drift') confidence += 0.1;

    // Higher confidence if alternatives save significant time
    if (alternatives.length > 0 && alternatives[0].savings.timeS > 60) {
      confidence += 0.1;
    }

    // Higher confidence with more severity
    confidence += reason.severity * 0.1;

    return Math.min(1, Math.round(confidence * 100) / 100);
  }

  private updateMetrics(computeMs: number) {
    const n = this.metrics.totalChecks;
    this.metrics.avgComputeMs = Math.round(
      ((this.metrics.avgComputeMs * (n - 1)) + computeMs) / n * 100
    ) / 100;
    this.metrics.maxComputeMs = Math.max(this.metrics.maxComputeMs, computeMs);
  }

  // ─── Manual Reroute ───────────────────────────────────

  requestReroute(reason = 'User requested reroute'): RerouteResult {
    this.metrics.manualTriggers++;

    const startTime = performance.now();
    const alternatives = this.computeAlternatives();

    const result: RerouteResult = {
      shouldReroute: true,
      reason: {
        type: 'manual',
        details: reason,
        severity: 0.5,
        detectedAt: Date.now(),
      },
      alternatives,
      bestAlternative: alternatives[0] || null,
      computeTimeMs: Math.round((performance.now() - startTime) * 100) / 100,
      confidence: 0.9,
      timestamp: Date.now(),
    };

    this.state.lastRerouteAt = Date.now();
    this.state.totalReroutes++;
    this.state.lastResult = result;

    if (this.onReroute) {
      this.onReroute(result);
    }

    return result;
  }

  // ─── Public API ───────────────────────────────────────

  getState(): RerouteState {
    return { ...this.state };
  }

  getMetrics() {
    return { ...this.metrics };
  }

  getConfig(): RerouteConfig {
    return { ...this.config };
  }

  updateConfig(partial: Partial<RerouteConfig>) {
    this.config = { ...this.config, ...partial };
  }

  destroy() {
    this.stop();
    this.onReroute = null;
    this.onDeviation = null;
    this.activeRoute = null;
    this.currentPosition = null;
  }
}
