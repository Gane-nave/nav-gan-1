/**
 * G.A.N.E — Real-Time Traffic Intelligence Pipeline
 * 
 * THE core competitive module vs Waze/Google Maps.
 * 
 * Architecture:
 *   INPUT:  raw_telemetry (device speed, position, heading)
 *   PROCESS: segment → aggregate → congestion model → emit
 *   OUTPUT: per-segment traffic state every 5 seconds
 * 
 * Pipeline loop (5s cycle):
 *   1. Fetch telemetry from last 60s rolling window
 *   2. Map each point → road segment (grid-based or graph-based)
 *   3. Aggregate metrics per segment (avg speed, vehicle count, heading variance)
 *   4. Compute congestion level + trend
 *   5. Emit to in-memory store + WebSocket channel + tRPC API
 * 
 * Performance targets:
 *   - Cycle time: <200ms
 *   - Segment resolution: ~111m grid cells
 *   - Update frequency: every 5 seconds
 *   - Memory: O(active_segments) ≈ bounded by active area
 */

import { getDb } from "../db";
import { rawTelemetry } from "../../drizzle/schema";
import { sql, gte, and } from "drizzle-orm";
import { wsBridge } from "./wsbridge";

// ─── Types ───────────────────────────────────────────────

export interface SegmentTraffic {
  segmentId: string;
  avgSpeed: number;
  medianSpeed: number;
  minSpeed: number;
  maxSpeed: number;
  speedVariance: number;
  vehicleCount: number;
  congestionLevel: "free" | "low" | "medium" | "high" | "gridlock";
  congestionIndex: number;       // 0.0 (free) → 1.0 (gridlock)
  trend: "improving" | "stable" | "worsening";
  travelTimeDelta: number;       // seconds deviation from free-flow
  avgHeading: number;
  headingVariance: number;       // high = intersection or merge
  centerLat: number;
  centerLon: number;
  updatedAt: number;             // Unix ms
}

export interface TrafficSnapshot {
  timestamp: number;
  cycleMs: number;               // how long this cycle took
  segmentCount: number;
  totalVehicles: number;
  segments: SegmentTraffic[];
  congestionDistribution: {
    free: number;
    low: number;
    medium: number;
    high: number;
    gridlock: number;
  };
}

interface RawPoint {
  deviceId: string;
  lat: number;
  lon: number;
  velocity: number | null;
  heading: number | null;
  timestamp: Date;
}

interface SegmentAccumulator {
  speeds: number[];
  headings: number[];
  devices: Set<string>;
  latSum: number;
  lonSum: number;
  count: number;
}

// ─── Configuration ───────────────────────────────────────

export interface TrafficPipelineConfig {
  cycleIntervalMs: number;       // default: 5000
  windowSeconds: number;         // default: 60 (rolling window)
  gridResolution: number;        // default: 1000 (lat*1000 → ~111m cells)
  freeFlowSpeedKmh: number;      // default: 80 (baseline for congestion calc)
  minPointsForSegment: number;   // default: 1 (minimum data points to emit segment)
  maxHistorySnapshots: number;   // default: 720 (1 hour at 5s intervals)
  enableWsBroadcast: boolean;    // default: true
}

const DEFAULT_CONFIG: TrafficPipelineConfig = {
  cycleIntervalMs: 5000,
  windowSeconds: 60,
  gridResolution: 1000,
  freeFlowSpeedKmh: 80,
  minPointsForSegment: 1,
  maxHistorySnapshots: 720,
  enableWsBroadcast: true,
};

// ─── Congestion Model ────────────────────────────────────

function computeCongestion(
  avgSpeed: number,
  freeFlowSpeed: number
): { level: SegmentTraffic["congestionLevel"]; index: number } {
  const ratio = avgSpeed / freeFlowSpeed;
  
  if (ratio >= 0.8) return { level: "free", index: Math.max(0, 1 - ratio) };
  if (ratio >= 0.6) return { level: "low", index: 0.2 + (0.8 - ratio) * 0.5 };
  if (ratio >= 0.4) return { level: "medium", index: 0.4 + (0.6 - ratio) * 0.5 };
  if (ratio >= 0.15) return { level: "high", index: 0.6 + (0.4 - ratio) * 0.8 };
  return { level: "gridlock", index: Math.min(1.0, 0.8 + (0.15 - ratio) * 1.3) };
}

function computeTrend(
  currentIndex: number,
  previousIndex: number | undefined
): SegmentTraffic["trend"] {
  if (previousIndex === undefined) return "stable";
  const delta = currentIndex - previousIndex;
  if (delta > 0.05) return "worsening";
  if (delta < -0.05) return "improving";
  return "stable";
}

function median(arr: number[]): number {
  if (arr.length === 0) return 0;
  const sorted = [...arr].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 !== 0 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

function variance(arr: number[], mean: number): number {
  if (arr.length < 2) return 0;
  return arr.reduce((sum, v) => sum + (v - mean) ** 2, 0) / (arr.length - 1);
}

function circularMean(angles: number[]): number {
  if (angles.length === 0) return 0;
  const sinSum = angles.reduce((s, a) => s + Math.sin(a * Math.PI / 180), 0);
  const cosSum = angles.reduce((s, a) => s + Math.cos(a * Math.PI / 180), 0);
  return ((Math.atan2(sinSum / angles.length, cosSum / angles.length) * 180 / Math.PI) + 360) % 360;
}

function circularVariance(angles: number[]): number {
  if (angles.length < 2) return 0;
  const sinSum = angles.reduce((s, a) => s + Math.sin(a * Math.PI / 180), 0) / angles.length;
  const cosSum = angles.reduce((s, a) => s + Math.cos(a * Math.PI / 180), 0) / angles.length;
  return 1 - Math.sqrt(sinSum ** 2 + cosSum ** 2);
}

// ─── Grid Segmentation ──────────────────────────────────

function pointToSegmentId(lat: number, lon: number, resolution: number): string {
  const latBucket = Math.floor(lat * resolution);
  const lonBucket = Math.floor(lon * resolution);
  return `${latBucket}_${lonBucket}`;
}

function segmentIdToCenter(segmentId: string, resolution: number): { lat: number; lon: number } {
  const [latB, lonB] = segmentId.split("_").map(Number);
  return {
    lat: (latB + 0.5) / resolution,
    lon: (lonB + 0.5) / resolution,
  };
}

// ─── Pipeline Engine ─────────────────────────────────────

export class TrafficPipeline {
  private config: TrafficPipelineConfig;
  private interval: ReturnType<typeof setInterval> | null = null;
  private running = false;

  // In-memory traffic state (hot store)
  private currentState = new Map<string, SegmentTraffic>();
  private previousCongestionIndex = new Map<string, number>();
  
  // History ring buffer for trend analysis
  private history: TrafficSnapshot[] = [];
  
  // Metrics
  private metrics = {
    totalCycles: 0,
    totalPointsProcessed: 0,
    avgCycleMs: 0,
    maxCycleMs: 0,
    lastCycleMs: 0,
    errors: 0,
    startedAt: 0,
  };

  constructor(config: Partial<TrafficPipelineConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  start() {
    if (this.running) return;
    this.running = true;
    this.metrics.startedAt = Date.now();
    
    console.log(`[TrafficPipeline] Starting (cycle=${this.config.cycleIntervalMs}ms, window=${this.config.windowSeconds}s, grid=${this.config.gridResolution})`);
    
    // Run first cycle immediately
    this.runCycle().catch(err => console.error("[TrafficPipeline] Initial cycle error:", err));
    
    // Schedule recurring cycles
    this.interval = setInterval(() => {
      this.runCycle().catch(err => console.error("[TrafficPipeline] Cycle error:", err));
    }, this.config.cycleIntervalMs);
  }

  stop() {
    this.running = false;
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
    console.log(`[TrafficPipeline] Stopped after ${this.metrics.totalCycles} cycles`);
  }

  // ─── Core Pipeline Cycle ───────────────────────────────

  async runCycle(): Promise<TrafficSnapshot | null> {
    const cycleStart = performance.now();
    
    try {
      // Step 1: Fetch telemetry from rolling window
      const points = await this.fetchTelemetry();
      
      if (points.length === 0) {
        // No data — emit empty snapshot
        const snapshot = this.buildSnapshot([], cycleStart);
        this.recordMetrics(snapshot, cycleStart);
        return snapshot;
      }

      // Step 2: Map points to segments
      const segmentMap = this.segmentize(points);

      // Step 3: Aggregate per segment
      const segments = this.aggregate(segmentMap);

      // Step 4: Build snapshot
      const snapshot = this.buildSnapshot(segments, cycleStart);

      // Step 5: Update state
      this.updateState(segments, snapshot);

      // Step 6: Broadcast via WebSocket
      if (this.config.enableWsBroadcast) {
        this.broadcast(snapshot);
      }

      // Step 7: Record metrics
      this.recordMetrics(snapshot, cycleStart);

      return snapshot;

    } catch (err) {
      this.metrics.errors++;
      console.error("[TrafficPipeline] Cycle failed:", err);
      return null;
    }
  }

  // ─── Step 1: Fetch Telemetry ───────────────────────────

  private async fetchTelemetry(): Promise<RawPoint[]> {
    const db = await getDb();
    if (!db) return [];

    const windowStart = new Date(Date.now() - this.config.windowSeconds * 1000);

    const rows = await db
      .select({
        deviceId: rawTelemetry.deviceId,
        lat: rawTelemetry.lat,
        lon: rawTelemetry.lon,
        velocity: rawTelemetry.velocity,
        heading: rawTelemetry.heading,
        timestamp: rawTelemetry.timestamp,
      })
      .from(rawTelemetry)
      .where(
        and(
          gte(rawTelemetry.timestamp, windowStart),
          gte(rawTelemetry.confidenceScore, 0.5)
        )
      )
      .limit(50000); // Safety cap

    return rows as RawPoint[];
  }

  // ─── Step 2: Segmentize ────────────────────────────────

  private segmentize(points: RawPoint[]): Map<string, SegmentAccumulator> {
    const segments = new Map<string, SegmentAccumulator>();

    for (const point of points) {
      const segId = pointToSegmentId(point.lat, point.lon, this.config.gridResolution);
      
      let acc = segments.get(segId);
      if (!acc) {
        acc = { speeds: [], headings: [], devices: new Set(), latSum: 0, lonSum: 0, count: 0 };
        segments.set(segId, acc);
      }

      const speed = point.velocity ?? 0;
      acc.speeds.push(speed * 3.6); // m/s → km/h
      if (point.heading !== null) acc.headings.push(point.heading);
      acc.devices.add(point.deviceId);
      acc.latSum += point.lat;
      acc.lonSum += point.lon;
      acc.count++;
    }

    return segments;
  }

  // ─── Step 3: Aggregate ─────────────────────────────────

  private aggregate(segmentMap: Map<string, SegmentAccumulator>): SegmentTraffic[] {
    const results: SegmentTraffic[] = [];
    const now = Date.now();

    for (const [segId, acc] of Array.from(segmentMap.entries())) {
      if (acc.count < this.config.minPointsForSegment) continue;

      const avgSpeed = acc.speeds.reduce((s, v) => s + v, 0) / acc.speeds.length;
      const medSpeed = median(acc.speeds);
      const minSpeed = Math.min(...acc.speeds);
      const maxSpeed = Math.max(...acc.speeds);
      const speedVar = variance(acc.speeds.map((s: number) => s), avgSpeed);

      const { level, index } = computeCongestion(avgSpeed, this.config.freeFlowSpeedKmh);
      const prevIndex = this.previousCongestionIndex.get(segId);
      const trend = computeTrend(index, prevIndex);

      // Travel time delta: how much slower than free flow (seconds per 111m cell)
      const freeFlowMs = (111 / this.config.freeFlowSpeedKmh) * 3600; // seconds at free flow
      const currentMs = avgSpeed > 0 ? (111 / avgSpeed) * 3600 : freeFlowMs * 10;
      const travelTimeDelta = Math.max(0, currentMs - freeFlowMs);

      const center = segmentIdToCenter(segId, this.config.gridResolution);

      results.push({
        segmentId: segId,
        avgSpeed: Math.round(avgSpeed * 10) / 10,
        medianSpeed: Math.round(medSpeed * 10) / 10,
        minSpeed: Math.round(minSpeed * 10) / 10,
        maxSpeed: Math.round(maxSpeed * 10) / 10,
        speedVariance: Math.round(speedVar * 100) / 100,
        vehicleCount: acc.devices.size,
        congestionLevel: level,
        congestionIndex: Math.round(index * 1000) / 1000,
        trend,
        travelTimeDelta: Math.round(travelTimeDelta * 10) / 10,
        avgHeading: Math.round(circularMean(acc.headings) * 10) / 10,
        headingVariance: Math.round(circularVariance(acc.headings) * 1000) / 1000,
        centerLat: center.lat,
        centerLon: center.lon,
        updatedAt: now,
      });
    }

    return results;
  }

  // ─── Step 4: Build Snapshot ────────────────────────────

  private buildSnapshot(segments: SegmentTraffic[], cycleStart: number): TrafficSnapshot {
    const distribution = { free: 0, low: 0, medium: 0, high: 0, gridlock: 0 };
    let totalVehicles = 0;

    for (const seg of segments) {
      distribution[seg.congestionLevel]++;
      totalVehicles += seg.vehicleCount;
    }

    return {
      timestamp: Date.now(),
      cycleMs: Math.round((performance.now() - cycleStart) * 100) / 100,
      segmentCount: segments.length,
      totalVehicles,
      segments,
      congestionDistribution: distribution,
    };
  }

  // ─── Step 5: Update State ──────────────────────────────

  private updateState(segments: SegmentTraffic[], snapshot: TrafficSnapshot) {
    // Update current state map
    this.currentState.clear();
    for (const seg of segments) {
      this.currentState.set(seg.segmentId, seg);
      this.previousCongestionIndex.set(seg.segmentId, seg.congestionIndex);
    }

    // Add to history ring buffer
    this.history.push(snapshot);
    if (this.history.length > this.config.maxHistorySnapshots) {
      this.history.shift();
    }
  }

  // ─── Step 6: Broadcast ────────────────────────────────

  private broadcast(snapshot: TrafficSnapshot) {
    try {
      wsBridge.broadcastToChannel("traffic_updates", {
        type: "traffic_snapshot",
        payload: {
          timestamp: snapshot.timestamp,
          cycleMs: snapshot.cycleMs,
          segmentCount: snapshot.segmentCount,
          totalVehicles: snapshot.totalVehicles,
          congestionDistribution: snapshot.congestionDistribution,
          segments: snapshot.segments,
        },
      });
    } catch {
      // Non-critical — WS may not be initialized yet
    }
  }

  // ─── Step 7: Metrics ──────────────────────────────────

  private recordMetrics(snapshot: TrafficSnapshot, cycleStart: number) {
    const cycleMs = performance.now() - cycleStart;
    this.metrics.totalCycles++;
    this.metrics.totalPointsProcessed += snapshot.totalVehicles;
    this.metrics.lastCycleMs = Math.round(cycleMs * 100) / 100;
    this.metrics.maxCycleMs = Math.max(this.metrics.maxCycleMs, cycleMs);
    this.metrics.avgCycleMs = Math.round(
      ((this.metrics.avgCycleMs * (this.metrics.totalCycles - 1)) + cycleMs) / this.metrics.totalCycles * 100
    ) / 100;
  }

  // ─── Public API ───────────────────────────────────────

  /** Get current traffic state for all segments */
  getCurrentState(): SegmentTraffic[] {
    return Array.from(this.currentState.values());
  }

  /** Get traffic for a specific segment */
  getSegment(segmentId: string): SegmentTraffic | undefined {
    return this.currentState.get(segmentId);
  }

  /** Get segments within a bounding box */
  getSegmentsInBounds(
    minLat: number, maxLat: number,
    minLon: number, maxLon: number
  ): SegmentTraffic[] {
    return Array.from(this.currentState.values()).filter(seg =>
      seg.centerLat >= minLat && seg.centerLat <= maxLat &&
      seg.centerLon >= minLon && seg.centerLon <= maxLon
    );
  }

  /** Get segments near a point */
  getSegmentsNear(lat: number, lon: number, radiusKm: number): SegmentTraffic[] {
    const latDelta = radiusKm / 111.32;
    const lonDelta = radiusKm / (111.32 * Math.cos(lat * Math.PI / 180));
    return this.getSegmentsInBounds(
      lat - latDelta, lat + latDelta,
      lon - lonDelta, lon + lonDelta
    );
  }

  /** Get congestion index for a route (array of lat/lon points) */
  getRouteCongestion(points: { lat: number; lon: number }[]): {
    avgCongestion: number;
    maxCongestion: number;
    segments: SegmentTraffic[];
    estimatedDelaySeconds: number;
  } {
    const routeSegments: SegmentTraffic[] = [];
    const seen = new Set<string>();

    for (const point of points) {
      const segId = pointToSegmentId(point.lat, point.lon, this.config.gridResolution);
      if (seen.has(segId)) continue;
      seen.add(segId);
      
      const seg = this.currentState.get(segId);
      if (seg) routeSegments.push(seg);
    }

    if (routeSegments.length === 0) {
      return { avgCongestion: 0, maxCongestion: 0, segments: [], estimatedDelaySeconds: 0 };
    }

    const avgCongestion = routeSegments.reduce((s, seg) => s + seg.congestionIndex, 0) / routeSegments.length;
    const maxCongestion = Math.max(...routeSegments.map(s => s.congestionIndex));
    const estimatedDelaySeconds = routeSegments.reduce((s, seg) => s + seg.travelTimeDelta, 0);

    return {
      avgCongestion: Math.round(avgCongestion * 1000) / 1000,
      maxCongestion: Math.round(maxCongestion * 1000) / 1000,
      segments: routeSegments,
      estimatedDelaySeconds: Math.round(estimatedDelaySeconds),
    };
  }

  /** Get pipeline metrics */
  getMetrics() {
    return {
      ...this.metrics,
      uptimeMs: this.metrics.startedAt > 0 ? Date.now() - this.metrics.startedAt : 0,
      currentSegments: this.currentState.size,
      historyLength: this.history.length,
      isRunning: this.running,
    };
  }

  /** Get historical snapshots */
  getHistory(limit = 60): TrafficSnapshot[] {
    return this.history.slice(-limit);
  }
}

// ─── Singleton ──────────────────────────────────────────

export const trafficPipeline = new TrafficPipeline();
