/**
 * G.A.N.E — Analytics Pipeline (Server-Side)
 * =============================================
 * Real-time analytics aggregation for navigation data.
 *
 * METRICS:
 *   - Active users / devices
 *   - Trips completed / in-progress
 *   - Average speed by road type
 *   - Incident frequency by type
 *   - Crowd intelligence coverage
 *   - System health (latency, error rates)
 *
 * AGGREGATION:
 *   - 1-minute rolling windows
 *   - 5-minute snapshots
 *   - Hourly summaries
 *   - Daily reports
 */

import { z } from 'zod';
import { publicProcedure, protectedProcedure, router } from '../_core/trpc';

// ─── Types ───────────────────────────────────────────────

export interface AnalyticsSnapshot {
  timestamp: number;
  windowMs: number;
  activeDevices: number;
  activeTrips: number;
  completedTrips: number;
  avgSpeedKmh: number;
  avgEtaAccuracy: number;           // 0-1
  incidentCount: number;
  crowdCoverage: number;            // 0-1 (fraction of cells with data)
  rerouteCount: number;
  apiLatencyMs: number;
  errorRate: number;                // 0-1
  telemetryRate: number;            // samples/sec
}

export interface AnalyticsEvent {
  type: string;
  deviceId: string;
  userId?: number;
  data: Record<string, unknown>;
  timestamp: number;
}

// ─── Analytics Store ────────────────────────────────────

class AnalyticsPipelineStore {
  private events: AnalyticsEvent[] = [];
  private snapshots: AnalyticsSnapshot[] = [];
  private maxEvents = 10000;
  private maxSnapshots = 288; // 24h at 5-min intervals

  // Counters (reset per window)
  private counters = {
    activeDevices: new Set<string>(),
    activeTrips: 0,
    completedTrips: 0,
    speedSamples: [] as number[],
    etaAccuracySamples: [] as number[],
    incidents: 0,
    reroutes: 0,
    apiCalls: 0,
    apiErrors: 0,
    apiLatencies: [] as number[],
    telemetrySamples: 0,
  };

  private snapshotTimer: ReturnType<typeof setInterval> | null = null;

  start() {
    // Take snapshot every 5 minutes
    this.snapshotTimer = setInterval(() => this.takeSnapshot(), 5 * 60 * 1000);
    // Take initial snapshot
    this.takeSnapshot();
  }

  stop() {
    if (this.snapshotTimer) {
      clearInterval(this.snapshotTimer);
      this.snapshotTimer = null;
    }
  }

  // ─── Event Ingestion ──────────────────────────────────

  trackEvent(event: AnalyticsEvent) {
    this.events.push(event);
    if (this.events.length > this.maxEvents) {
      this.events = this.events.slice(-this.maxEvents);
    }

    // Update counters
    this.counters.activeDevices.add(event.deviceId);

    switch (event.type) {
      case 'trip_start':
        this.counters.activeTrips++;
        break;
      case 'trip_complete':
        this.counters.activeTrips = Math.max(0, this.counters.activeTrips - 1);
        this.counters.completedTrips++;
        break;
      case 'speed_sample':
        if (typeof event.data.speedKmh === 'number') {
          this.counters.speedSamples.push(event.data.speedKmh);
        }
        break;
      case 'eta_accuracy':
        if (typeof event.data.accuracy === 'number') {
          this.counters.etaAccuracySamples.push(event.data.accuracy);
        }
        break;
      case 'incident':
        this.counters.incidents++;
        break;
      case 'reroute':
        this.counters.reroutes++;
        break;
      case 'api_call':
        this.counters.apiCalls++;
        if (typeof event.data.latencyMs === 'number') {
          this.counters.apiLatencies.push(event.data.latencyMs);
        }
        if (event.data.error) {
          this.counters.apiErrors++;
        }
        break;
      case 'telemetry':
        this.counters.telemetrySamples++;
        break;
    }
  }

  // ─── Snapshot ─────────────────────────────────────────

  private takeSnapshot() {
    const now = Date.now();
    const avgSpeed = this.counters.speedSamples.length > 0
      ? this.counters.speedSamples.reduce((s, v) => s + v, 0) / this.counters.speedSamples.length
      : 0;

    const avgEtaAccuracy = this.counters.etaAccuracySamples.length > 0
      ? this.counters.etaAccuracySamples.reduce((s, v) => s + v, 0) / this.counters.etaAccuracySamples.length
      : 0;

    const avgLatency = this.counters.apiLatencies.length > 0
      ? this.counters.apiLatencies.reduce((s, v) => s + v, 0) / this.counters.apiLatencies.length
      : 0;

    const errorRate = this.counters.apiCalls > 0
      ? this.counters.apiErrors / this.counters.apiCalls
      : 0;

    const snapshot: AnalyticsSnapshot = {
      timestamp: now,
      windowMs: 5 * 60 * 1000,
      activeDevices: this.counters.activeDevices.size,
      activeTrips: this.counters.activeTrips,
      completedTrips: this.counters.completedTrips,
      avgSpeedKmh: Math.round(avgSpeed * 10) / 10,
      avgEtaAccuracy: Math.round(avgEtaAccuracy * 1000) / 1000,
      incidentCount: this.counters.incidents,
      crowdCoverage: 0, // Will be updated from crowd intelligence
      rerouteCount: this.counters.reroutes,
      apiLatencyMs: Math.round(avgLatency),
      errorRate: Math.round(errorRate * 10000) / 10000,
      telemetryRate: Math.round(this.counters.telemetrySamples / 300 * 10) / 10,
    };

    this.snapshots.push(snapshot);
    if (this.snapshots.length > this.maxSnapshots) {
      this.snapshots = this.snapshots.slice(-this.maxSnapshots);
    }

    // Reset per-window counters
    this.counters.activeDevices = new Set();
    this.counters.completedTrips = 0;
    this.counters.speedSamples = [];
    this.counters.etaAccuracySamples = [];
    this.counters.incidents = 0;
    this.counters.reroutes = 0;
    this.counters.apiCalls = 0;
    this.counters.apiErrors = 0;
    this.counters.apiLatencies = [];
    this.counters.telemetrySamples = 0;
  }

  // ─── Queries ──────────────────────────────────────────

  getLatestSnapshot(): AnalyticsSnapshot | null {
    return this.snapshots.length > 0 ? this.snapshots[this.snapshots.length - 1] : null;
  }

  getSnapshots(count: number = 12): AnalyticsSnapshot[] {
    return this.snapshots.slice(-count);
  }

  getRecentEvents(count: number = 50): AnalyticsEvent[] {
    return this.events.slice(-count);
  }

  getDailySummary(): {
    totalTrips: number;
    totalDevices: number;
    avgSpeed: number;
    totalIncidents: number;
    totalReroutes: number;
    uptimePercent: number;
  } {
    const dayAgo = Date.now() - 24 * 60 * 60 * 1000;
    const daySnapshots = this.snapshots.filter(s => s.timestamp > dayAgo);

    if (daySnapshots.length === 0) {
      return { totalTrips: 0, totalDevices: 0, avgSpeed: 0, totalIncidents: 0, totalReroutes: 0, uptimePercent: 100 };
    }

    return {
      totalTrips: daySnapshots.reduce((s, snap) => s + snap.completedTrips, 0),
      totalDevices: Math.max(...daySnapshots.map(s => s.activeDevices)),
      avgSpeed: Math.round(
        daySnapshots.reduce((s, snap) => s + snap.avgSpeedKmh, 0) / daySnapshots.length * 10
      ) / 10,
      totalIncidents: daySnapshots.reduce((s, snap) => s + snap.incidentCount, 0),
      totalReroutes: daySnapshots.reduce((s, snap) => s + snap.rerouteCount, 0),
      uptimePercent: Math.round(
        (1 - daySnapshots.reduce((s, snap) => s + snap.errorRate, 0) / daySnapshots.length) * 10000
      ) / 100,
    };
  }
}

export const analyticsPipelineStore = new AnalyticsPipelineStore();

// ─── tRPC Router ────────────────────────────────────────

export const analyticsRouter = router({
  /**
   * Track an analytics event.
   */
  track: publicProcedure
    .input(z.object({
      type: z.string().min(1).max(64),
      deviceId: z.string().min(1).max(64),
      data: z.record(z.string(), z.unknown()).default({}),
    }))
    .mutation(({ input, ctx }) => {
      analyticsPipelineStore.trackEvent({
        type: input.type,
        deviceId: input.deviceId,
        userId: ctx.user?.id,
        data: input.data,
        timestamp: Date.now(),
      });
      return { accepted: true };
    }),

  /**
   * Get latest analytics snapshot.
   */
  latest: publicProcedure
    .query(() => {
      return analyticsPipelineStore.getLatestSnapshot();
    }),

  /**
   * Get historical snapshots.
   */
  history: publicProcedure
    .input(z.object({
      count: z.number().min(1).max(288).default(12),
    }))
    .query(({ input }) => {
      return analyticsPipelineStore.getSnapshots(input.count);
    }),

  /**
   * Get daily summary.
   */
  daily: publicProcedure
    .query(() => {
      return analyticsPipelineStore.getDailySummary();
    }),

  /**
   * Get recent events.
   */
  events: protectedProcedure
    .input(z.object({
      count: z.number().min(1).max(200).default(50),
    }))
    .query(({ input }) => {
      return analyticsPipelineStore.getRecentEvents(input.count);
    }),
});
