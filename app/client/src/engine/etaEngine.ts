/**
 * G.A.N.E — ETA Correction Engine
 * =================================
 * Real-time ETA computation with learning from historical trips.
 *
 * INPUT:
 *   - Historical trip segment latencies
 *   - Live traffic data (segment speeds)
 *   - Current route segments
 *
 * MODEL:
 *   - Per-segment latency learning (exponential moving average)
 *   - Time-of-day weighting (rush hour, night, weekend)
 *   - Route composition model (sum of segment ETAs)
 *   - Confidence based on data freshness and sample size
 *
 * OUTPUT:
 *   - Corrected ETA (Date)
 *   - ETA confidence (0-1)
 *   - Per-segment breakdown
 *   - Drift from original ETA
 */

// ─── Types ───────────────────────────────────────────────

export interface ETAConfig {
  smoothingAlpha: number;           // EMA alpha for segment learning (default: 0.3)
  minSamplesForConfidence: number;  // min samples before high confidence (default: 5)
  maxSegmentAgeMs: number;          // max age of segment data before stale (default: 300000 = 5min)
  updateIntervalMs: number;        // how often to recompute ETA (default: 5000)
  rushHourMultiplier: number;      // multiplier during rush hours (default: 1.3)
  nightMultiplier: number;         // multiplier during night (default: 0.85)
  weekendMultiplier: number;       // multiplier during weekends (default: 0.9)
  enabled: boolean;
}

export interface SegmentETA {
  segmentId: string;
  distanceM: number;
  baseSpeedKmh: number;            // free-flow speed
  currentSpeedKmh: number;         // live traffic speed
  historicalSpeedKmh: number;      // learned from history
  estimatedTimeS: number;          // computed segment time
  congestionIndex: number;         // 0-1
  sampleCount: number;             // historical samples
  lastUpdated: number;             // Unix ms
}

export interface ETAResult {
  eta: Date;
  etaUnixMs: number;
  totalDurationS: number;
  remainingDurationS: number;
  remainingDistanceM: number;
  confidence: number;              // 0-1
  driftFromOriginalS: number;      // positive = later than expected
  segments: SegmentETA[];
  timeOfDayFactor: number;
  computeTimeMs: number;
  timestamp: number;
}

export interface ETAState {
  isActive: boolean;
  lastComputeAt: number;
  totalComputes: number;
  currentEta: ETAResult | null;
  avgComputeMs: number;
  segmentCount: number;
}

export interface RouteSegment {
  segmentId: string;
  startLat: number;
  startLon: number;
  endLat: number;
  endLon: number;
  distanceM: number;
  freeFlowSpeedKmh: number;
  roadType: string;
}

export interface TrafficUpdate {
  segmentId: string;
  avgSpeed: number;
  congestionLevel: 'low' | 'medium' | 'high';
  vehicleCount: number;
  timestamp: number;
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: ETAConfig = {
  smoothingAlpha: 0.3,
  minSamplesForConfidence: 5,
  maxSegmentAgeMs: 300000,
  updateIntervalMs: 5000,
  rushHourMultiplier: 1.3,
  nightMultiplier: 0.85,
  weekendMultiplier: 0.9,
  enabled: true,
};

const DEG_TO_RAD = Math.PI / 180;
const EARTH_RADIUS_M = 6371000;

// ─── Helpers ────────────────────────────────────────────

function haversineDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const dLat = (lat2 - lat1) * DEG_TO_RAD;
  const dLon = (lon2 - lon1) * DEG_TO_RAD;
  const a = Math.sin(dLat / 2) ** 2 +
    Math.cos(lat1 * DEG_TO_RAD) * Math.cos(lat2 * DEG_TO_RAD) *
    Math.sin(dLon / 2) ** 2;
  return EARTH_RADIUS_M * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

// ─── Segment History Store ──────────────────────────────

interface SegmentHistory {
  segmentId: string;
  emaSpeedKmh: number;            // exponential moving average
  sampleCount: number;
  lastUpdated: number;
  hourlyFactors: number[];        // 24 entries, multiplier per hour
}

// ─── ETA Engine ─────────────────────────────────────────

export class ETAEngine {
  private config: ETAConfig;
  private state: ETAState;
  private interval: ReturnType<typeof setInterval> | null = null;

  // Route data
  private routeSegments: RouteSegment[] = [];
  private currentSegmentIndex = 0;
  private originalDurationS = 0;
  private routeStartedAt = 0;

  // Position
  private currentLat = 0;
  private currentLon = 0;
  private currentSpeedKmh = 0;

  // Traffic data
  private trafficMap: Map<string, TrafficUpdate> = new Map();

  // Historical learning
  private segmentHistory: Map<string, SegmentHistory> = new Map();

  // Callbacks
  private onETAUpdate: ((result: ETAResult) => void) | null = null;

  constructor(config: Partial<ETAConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      isActive: false,
      lastComputeAt: 0,
      totalComputes: 0,
      currentEta: null,
      avgComputeMs: 0,
      segmentCount: 0,
    };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  start() {
    if (this.state.isActive) return;
    this.state.isActive = true;

    this.interval = setInterval(() => {
      if (this.config.enabled && this.routeSegments.length > 0) {
        this.compute();
      }
    }, this.config.updateIntervalMs);
  }

  stop() {
    this.state.isActive = false;
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
  }

  // ─── Input Updates ────────────────────────────────────

  setRoute(segments: RouteSegment[], originalDurationS: number) {
    this.routeSegments = segments;
    this.originalDurationS = originalDurationS;
    this.currentSegmentIndex = 0;
    this.routeStartedAt = Date.now();
    this.state.segmentCount = segments.length;
  }

  updatePosition(lat: number, lon: number, speedKmh: number) {
    this.currentLat = lat;
    this.currentLon = lon;
    this.currentSpeedKmh = speedKmh;

    // Advance segment index based on position
    this.advanceSegmentIndex();
  }

  updateTraffic(updates: TrafficUpdate[]) {
    for (const update of updates) {
      this.trafficMap.set(update.segmentId, update);
    }
  }

  /**
   * Learn from a completed trip segment.
   * Called when a segment is traversed with actual time data.
   */
  learnSegment(segmentId: string, actualSpeedKmh: number) {
    const existing = this.segmentHistory.get(segmentId);
    const now = Date.now();
    const hour = new Date().getHours();

    if (existing) {
      // Exponential moving average update
      existing.emaSpeedKmh = this.config.smoothingAlpha * actualSpeedKmh +
        (1 - this.config.smoothingAlpha) * existing.emaSpeedKmh;
      existing.sampleCount++;
      existing.lastUpdated = now;

      // Update hourly factor
      const baseSpeed = existing.emaSpeedKmh;
      if (baseSpeed > 0) {
        existing.hourlyFactors[hour] = this.config.smoothingAlpha * (actualSpeedKmh / baseSpeed) +
          (1 - this.config.smoothingAlpha) * existing.hourlyFactors[hour];
      }
    } else {
      const hourlyFactors = new Array(24).fill(1.0);
      hourlyFactors[hour] = 1.0;

      this.segmentHistory.set(segmentId, {
        segmentId,
        emaSpeedKmh: actualSpeedKmh,
        sampleCount: 1,
        lastUpdated: now,
        hourlyFactors,
      });
    }
  }

  setOnETAUpdate(callback: (result: ETAResult) => void) {
    this.onETAUpdate = callback;
  }

  // ─── Core Computation ─────────────────────────────────

  compute(): ETAResult {
    const startTime = performance.now();
    this.state.totalComputes++;

    const now = Date.now();
    const timeOfDayFactor = this.getTimeOfDayFactor();
    const segmentETAs: SegmentETA[] = [];
    let totalRemainingS = 0;
    let totalRemainingM = 0;

    // Compute ETA for each remaining segment
    for (let i = this.currentSegmentIndex; i < this.routeSegments.length; i++) {
      const seg = this.routeSegments[i];
      const segETA = this.computeSegmentETA(seg, timeOfDayFactor, i === this.currentSegmentIndex);
      segmentETAs.push(segETA);
      totalRemainingS += segETA.estimatedTimeS;
      totalRemainingM += segETA.distanceM;
    }

    // If on current segment, adjust for partial traversal
    if (segmentETAs.length > 0 && this.currentSegmentIndex < this.routeSegments.length) {
      const currentSeg = this.routeSegments[this.currentSegmentIndex];
      const distToEnd = haversineDistance(
        this.currentLat, this.currentLon,
        currentSeg.endLat, currentSeg.endLon
      );
      const segDist = currentSeg.distanceM;
      if (segDist > 0) {
        const fraction = Math.min(1, distToEnd / segDist);
        segmentETAs[0].estimatedTimeS *= fraction;
        segmentETAs[0].distanceM *= fraction;
        totalRemainingS = segmentETAs.reduce((s, seg) => s + seg.estimatedTimeS, 0);
        totalRemainingM = segmentETAs.reduce((s, seg) => s + seg.distanceM, 0);
      }
    }

    // Compute total duration (elapsed + remaining)
    const elapsedS = (now - this.routeStartedAt) / 1000;
    const totalDurationS = elapsedS + totalRemainingS;

    // Compute confidence
    const confidence = this.computeConfidence(segmentETAs);

    // Drift from original
    const driftFromOriginalS = totalDurationS - this.originalDurationS;

    const computeTimeMs = Math.round((performance.now() - startTime) * 100) / 100;

    const result: ETAResult = {
      eta: new Date(now + totalRemainingS * 1000),
      etaUnixMs: now + totalRemainingS * 1000,
      totalDurationS: Math.round(totalDurationS),
      remainingDurationS: Math.round(totalRemainingS),
      remainingDistanceM: Math.round(totalRemainingM),
      confidence,
      driftFromOriginalS: Math.round(driftFromOriginalS),
      segments: segmentETAs,
      timeOfDayFactor: Math.round(timeOfDayFactor * 1000) / 1000,
      computeTimeMs,
      timestamp: now,
    };

    this.state.lastComputeAt = now;
    this.state.currentEta = result;
    this.updateAvgCompute(computeTimeMs);

    if (this.onETAUpdate) {
      this.onETAUpdate(result);
    }

    return result;
  }

  // ─── Per-Segment ETA ──────────────────────────────────

  private computeSegmentETA(
    seg: RouteSegment,
    timeOfDayFactor: number,
    isCurrent: boolean
  ): SegmentETA {
    const now = Date.now();
    const traffic = this.trafficMap.get(seg.segmentId);
    const history = this.segmentHistory.get(seg.segmentId);

    // Determine speed to use (priority: live traffic > history > free flow)
    let effectiveSpeedKmh = seg.freeFlowSpeedKmh;
    let source: 'live' | 'history' | 'freeflow' = 'freeflow';

    // Live traffic data
    let currentSpeedKmh = seg.freeFlowSpeedKmh;
    if (traffic && (now - traffic.timestamp) < this.config.maxSegmentAgeMs) {
      currentSpeedKmh = traffic.avgSpeed * 3.6; // m/s to km/h
      effectiveSpeedKmh = currentSpeedKmh;
      source = 'live';
    }

    // Historical data
    let historicalSpeedKmh = seg.freeFlowSpeedKmh;
    if (history && history.sampleCount >= 2) {
      const hour = new Date().getHours();
      historicalSpeedKmh = history.emaSpeedKmh * history.hourlyFactors[hour];

      if (source === 'freeflow') {
        effectiveSpeedKmh = historicalSpeedKmh;
        source = 'history';
      } else {
        // Blend live and historical (70% live, 30% historical)
        effectiveSpeedKmh = effectiveSpeedKmh * 0.7 + historicalSpeedKmh * 0.3;
      }
    }

    // Apply time-of-day factor
    effectiveSpeedKmh *= timeOfDayFactor;

    // If currently on this segment and have real speed, use it
    if (isCurrent && this.currentSpeedKmh > 5) {
      effectiveSpeedKmh = effectiveSpeedKmh * 0.5 + this.currentSpeedKmh * 0.5;
    }

    // Clamp speed
    effectiveSpeedKmh = Math.max(5, Math.min(200, effectiveSpeedKmh));

    // Compute time
    const estimatedTimeS = (seg.distanceM / 1000) / effectiveSpeedKmh * 3600;

    // Congestion index
    const congestionIndex = Math.max(0, Math.min(1,
      1 - (effectiveSpeedKmh / seg.freeFlowSpeedKmh)
    ));

    return {
      segmentId: seg.segmentId,
      distanceM: seg.distanceM,
      baseSpeedKmh: seg.freeFlowSpeedKmh,
      currentSpeedKmh: Math.round(currentSpeedKmh * 10) / 10,
      historicalSpeedKmh: Math.round(historicalSpeedKmh * 10) / 10,
      estimatedTimeS: Math.round(estimatedTimeS * 10) / 10,
      congestionIndex: Math.round(congestionIndex * 1000) / 1000,
      sampleCount: history?.sampleCount || 0,
      lastUpdated: traffic?.timestamp || history?.lastUpdated || 0,
    };
  }

  // ─── Time-of-Day Factor ───────────────────────────────

  private getTimeOfDayFactor(): number {
    const now = new Date();
    const hour = now.getHours();
    const day = now.getDay(); // 0=Sun, 6=Sat

    // Weekend
    if (day === 0 || day === 6) {
      return this.config.weekendMultiplier;
    }

    // Rush hours (7-9 AM, 4-7 PM)
    if ((hour >= 7 && hour <= 9) || (hour >= 16 && hour <= 19)) {
      return 1 / this.config.rushHourMultiplier; // Slower = lower factor
    }

    // Night (10 PM - 5 AM)
    if (hour >= 22 || hour <= 5) {
      return 1 / this.config.nightMultiplier; // Faster = higher factor
    }

    // Normal hours
    return 1.0;
  }

  // ─── Confidence ───────────────────────────────────────

  private computeConfidence(segments: SegmentETA[]): number {
    if (segments.length === 0) return 0;

    let totalWeight = 0;
    let weightedConfidence = 0;

    for (const seg of segments) {
      const weight = seg.distanceM;

      // Base confidence from data source
      let segConfidence = 0.4; // free-flow baseline

      // Live traffic data boosts confidence
      const now = Date.now();
      if (seg.lastUpdated > 0 && (now - seg.lastUpdated) < this.config.maxSegmentAgeMs) {
        segConfidence += 0.3;
      }

      // Historical data boosts confidence
      if (seg.sampleCount >= this.config.minSamplesForConfidence) {
        segConfidence += 0.25;
      } else if (seg.sampleCount >= 2) {
        segConfidence += 0.1;
      }

      // Penalize high congestion (less predictable)
      if (seg.congestionIndex > 0.7) {
        segConfidence -= 0.1;
      }

      segConfidence = Math.max(0.1, Math.min(1, segConfidence));
      weightedConfidence += segConfidence * weight;
      totalWeight += weight;
    }

    return totalWeight > 0
      ? Math.round((weightedConfidence / totalWeight) * 100) / 100
      : 0.5;
  }

  // ─── Segment Index Advancement ────────────────────────

  private advanceSegmentIndex() {
    if (this.routeSegments.length === 0) return;

    // Find closest segment end point
    let minDist = Infinity;
    let bestIdx = this.currentSegmentIndex;

    for (let i = this.currentSegmentIndex; i < this.routeSegments.length; i++) {
      const seg = this.routeSegments[i];
      const dist = haversineDistance(
        this.currentLat, this.currentLon,
        seg.endLat, seg.endLon
      );

      if (dist < minDist) {
        minDist = dist;
        bestIdx = i;
      }

      // Don't look too far ahead
      if (i > this.currentSegmentIndex + 5) break;
    }

    // If we're close to a segment end, advance past it
    if (minDist < 50 && bestIdx > this.currentSegmentIndex) {
      // Learn from traversed segments
      for (let i = this.currentSegmentIndex; i < bestIdx; i++) {
        const seg = this.routeSegments[i];
        if (this.currentSpeedKmh > 0) {
          this.learnSegment(seg.segmentId, this.currentSpeedKmh);
        }
      }
      this.currentSegmentIndex = bestIdx;
    }
  }

  private updateAvgCompute(computeMs: number) {
    const n = this.state.totalComputes;
    this.state.avgComputeMs = Math.round(
      ((this.state.avgComputeMs * (n - 1)) + computeMs) / n * 100
    ) / 100;
  }

  // ─── Public API ───────────────────────────────────────

  getState(): ETAState {
    return { ...this.state };
  }

  getConfig(): ETAConfig {
    return { ...this.config };
  }

  updateConfig(partial: Partial<ETAConfig>) {
    this.config = { ...this.config, ...partial };
  }

  getSegmentHistory(): Map<string, SegmentHistory> {
    return new Map(this.segmentHistory);
  }

  /**
   * Format ETA for display.
   * Returns human-readable string like "14:32" or "2h 15m"
   */
  formatETA(result?: ETAResult): string {
    const eta = result || this.state.currentEta;
    if (!eta) return '--:--';

    const remaining = eta.remainingDurationS;

    if (remaining < 60) {
      return 'פחות מדקה'; // Less than a minute
    }

    if (remaining < 3600) {
      const mins = Math.round(remaining / 60);
      return `${mins} דק'`; // X minutes
    }

    const hours = Math.floor(remaining / 3600);
    const mins = Math.round((remaining % 3600) / 60);
    return `${hours} שע' ${mins} דק'`; // X hours Y minutes
  }

  /**
   * Format remaining distance for display.
   */
  formatDistance(result?: ETAResult): string {
    const eta = result || this.state.currentEta;
    if (!eta) return '--';

    const dist = eta.remainingDistanceM;

    if (dist < 1000) {
      return `${Math.round(dist)} מ'`; // meters
    }

    return `${(dist / 1000).toFixed(1)} ק"מ`; // km
  }

  destroy() {
    this.stop();
    this.onETAUpdate = null;
    this.routeSegments = [];
    this.trafficMap.clear();
  }
}
