/**
 * G.A.N.E — Trip Replay System
 * ================================
 * Client-side trip replay engine for reviewing completed trips.
 *
 * Features:
 * - Timeline scrubbing with variable speed (0.5x → 16x)
 * - Event overlay (alerts, geofence, waypoints)
 * - Telemetry visualization (speed, heading, GNSS quality)
 * - Pause/resume/seek
 * - Export replay as JSON
 */

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export interface ReplayPoint {
  timestamp: number;
  lat: number;
  lon: number;
  speed: number;
  heading: number;
  accuracy?: number;
  altitude?: number;
}

export interface ReplayEvent {
  timestamp: number;
  type: string;
  title: string;
  lat?: number;
  lon?: number;
  payload?: Record<string, unknown>;
}

export interface ReplayData {
  tripId: string;
  points: ReplayPoint[];
  events: ReplayEvent[];
  startTime: number;
  endTime: number;
  totalDistanceM: number;
  avgSpeedKmh: number;
}

export type ReplayState = 'idle' | 'loading' | 'playing' | 'paused' | 'finished';

export interface ReplayFrame {
  point: ReplayPoint;
  progress: number;          // 0..1
  elapsedMs: number;
  currentSpeed: number;
  nearbyEvents: ReplayEvent[];
}

type ReplayListener = (frame: ReplayFrame) => void;
type StateListener = (state: ReplayState) => void;

// ═══════════════════════════════════════════════════
// REPLAY ENGINE
// ═══════════════════════════════════════════════════

export class TripReplayEngine {
  private data: ReplayData | null = null;
  private state: ReplayState = 'idle';
  private playbackSpeed = 1;
  private currentIndex = 0;
  private animationId: number | null = null;
  private lastFrameTime = 0;
  private accumulatedTime = 0;

  private frameListeners: Set<ReplayListener> = new Set();
  private stateListeners: Set<StateListener> = new Set();

  // ── Lifecycle ──

  /**
   * Load replay data from trip events
   */
  loadReplay(data: ReplayData): void {
    this.data = data;
    this.currentIndex = 0;
    this.accumulatedTime = 0;
    this.setState('paused');
  }

  /**
   * Build ReplayData from raw trip events
   */
  static buildFromEvents(
    tripId: string,
    events: Array<{
      eventType: string;
      timestampDevice: number;
      lat?: number | null;
      lon?: number | null;
      accuracy?: number | null;
      payload?: unknown;
    }>
  ): ReplayData {
    const points: ReplayPoint[] = [];
    const replayEvents: ReplayEvent[] = [];

    let totalDistance = 0;
    let prevLat = 0, prevLon = 0;

    for (const evt of events) {
      if (evt.lat != null && evt.lon != null) {
        const speed = typeof evt.payload === 'object' && evt.payload !== null
          ? ((evt.payload as Record<string, unknown>).avgSpeed as number) || 0
          : 0;

        if (prevLat !== 0 && prevLon !== 0) {
          totalDistance += haversine(prevLat, prevLon, evt.lat, evt.lon);
        }
        prevLat = evt.lat;
        prevLon = evt.lon;

        points.push({
          timestamp: evt.timestampDevice,
          lat: evt.lat,
          lon: evt.lon,
          speed,
          heading: 0,
          accuracy: evt.accuracy || undefined,
        });
      }

      replayEvents.push({
        timestamp: evt.timestampDevice,
        type: evt.eventType,
        title: evt.eventType.replace(/_/g, ' '),
        lat: evt.lat || undefined,
        lon: evt.lon || undefined,
        payload: typeof evt.payload === 'object' ? evt.payload as Record<string, unknown> : undefined,
      });
    }

    // Sort by timestamp
    points.sort((a, b) => a.timestamp - b.timestamp);
    replayEvents.sort((a, b) => a.timestamp - b.timestamp);

    const startTime = points[0]?.timestamp || 0;
    const endTime = points[points.length - 1]?.timestamp || 0;
    const durationH = (endTime - startTime) / 3_600_000;

    return {
      tripId,
      points,
      events: replayEvents,
      startTime,
      endTime,
      totalDistanceM: totalDistance,
      avgSpeedKmh: durationH > 0 ? (totalDistance / 1000) / durationH : 0,
    };
  }

  // ── Playback Controls ──

  play(): void {
    if (!this.data || this.data.points.length === 0) return;
    if (this.state === 'finished') {
      this.currentIndex = 0;
      this.accumulatedTime = 0;
    }
    this.setState('playing');
    this.lastFrameTime = performance.now();
    this.tick();
  }

  pause(): void {
    this.setState('paused');
    if (this.animationId !== null) {
      cancelAnimationFrame(this.animationId);
      this.animationId = null;
    }
  }

  stop(): void {
    this.pause();
    this.currentIndex = 0;
    this.accumulatedTime = 0;
    this.setState('idle');
  }

  /**
   * Seek to a position (0..1)
   */
  seek(progress: number): void {
    if (!this.data) return;
    const clamped = Math.max(0, Math.min(1, progress));
    this.currentIndex = Math.floor(clamped * (this.data.points.length - 1));
    this.accumulatedTime = clamped * (this.data.endTime - this.data.startTime);
    this.emitFrame();
  }

  setSpeed(speed: number): void {
    this.playbackSpeed = Math.max(0.25, Math.min(32, speed));
  }

  getSpeed(): number {
    return this.playbackSpeed;
  }

  getState(): ReplayState {
    return this.state;
  }

  getData(): ReplayData | null {
    return this.data;
  }

  // ── Event System ──

  onFrame(listener: ReplayListener): () => void {
    this.frameListeners.add(listener);
    return () => this.frameListeners.delete(listener);
  }

  onStateChange(listener: StateListener): () => void {
    this.stateListeners.add(listener);
    return () => this.stateListeners.delete(listener);
  }

  // ── Internal ──

  private tick = (): void => {
    if (this.state !== 'playing' || !this.data) return;

    const now = performance.now();
    const delta = (now - this.lastFrameTime) * this.playbackSpeed;
    this.lastFrameTime = now;
    this.accumulatedTime += delta;

    // Find the point that matches accumulated time
    const targetTime = this.data.startTime + this.accumulatedTime;

    while (
      this.currentIndex < this.data.points.length - 1 &&
      this.data.points[this.currentIndex + 1].timestamp <= targetTime
    ) {
      this.currentIndex++;
    }

    if (this.currentIndex >= this.data.points.length - 1) {
      this.setState('finished');
      this.emitFrame();
      return;
    }

    this.emitFrame();
    this.animationId = requestAnimationFrame(this.tick);
  };

  private emitFrame(): void {
    if (!this.data) return;
    const point = this.data.points[this.currentIndex];
    if (!point) return;

    const progress = this.data.points.length > 1
      ? this.currentIndex / (this.data.points.length - 1)
      : 0;

    // Find events near current timestamp (±5s)
    const nearbyEvents = this.data.events.filter(e =>
      Math.abs(e.timestamp - point.timestamp) < 5000
    );

    const frame: ReplayFrame = {
      point,
      progress,
      elapsedMs: point.timestamp - this.data.startTime,
      currentSpeed: point.speed,
      nearbyEvents,
    };

    for (const listener of Array.from(this.frameListeners)) {
      try { listener(frame); } catch { /* ignore */ }
    }
  }

  private setState(state: ReplayState): void {
    if (this.state === state) return;
    this.state = state;
    for (const listener of Array.from(this.stateListeners)) {
      try { listener(state); } catch { /* ignore */ }
    }
  }

  /**
   * Export current replay data as JSON
   */
  exportJSON(): string | null {
    if (!this.data) return null;
    return JSON.stringify(this.data, null, 2);
  }

  destroy(): void {
    this.stop();
    this.frameListeners.clear();
    this.stateListeners.clear();
    this.data = null;
  }
}

// ── Utility ──

function haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const R = 6_371_000;
  const dLat = (lat2 - lat1) * Math.PI / 180;
  const dLon = (lon2 - lon1) * Math.PI / 180;
  const a = Math.sin(dLat / 2) ** 2 +
    Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) *
    Math.sin(dLon / 2) ** 2;
  return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

// Singleton
export const tripReplayEngine = new TripReplayEngine();
