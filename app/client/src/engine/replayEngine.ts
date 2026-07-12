/**
 * G.A.N.E — Replay System Engine
 * =================================
 * 
 * Full-fidelity trip replay with deterministic event reconstruction.
 * 
 * Capabilities:
 * 1. Event Loader — load events from trip recordings (position, sensor, incident)
 * 2. Timeline Reconstructor — rebuild exact timeline from sparse events
 * 3. Determinism Checker — verify replay produces identical outputs
 * 4. Playback Controller — play, pause, seek, speed control
 * 5. Frame Interpolator — smooth position/heading between sparse samples
 * 6. Annotation Layer — add markers, notes, bookmarks during replay
 */

// ─── Core Types ─────────────────────────────────────────

export type ReplayEventType =
  | 'position_fix'
  | 'sensor_reading'
  | 'route_change'
  | 'incident_report'
  | 'speed_change'
  | 'heading_change'
  | 'signal_quality'
  | 'mode_switch'
  | 'alert_triggered'
  | 'user_interaction'
  | 'system_event';

export interface ReplayEventRecord {
  id: string;
  type: ReplayEventType;
  timestamp: number; // ms since epoch
  data: Record<string, unknown>;
  source: string;
  /** Sequence number for ordering within same timestamp */
  seq: number;
  /** Hash of event data for determinism verification */
  hash: string;
}

export interface ReplayTimeline {
  tripId: string;
  startTime: number;
  endTime: number;
  duration: number; // ms
  eventCount: number;
  events: ReplayEventRecord[];
  /** Sorted index for binary search */
  timeIndex: number[];
}

export interface PlaybackState {
  status: 'idle' | 'loading' | 'playing' | 'paused' | 'seeking' | 'complete';
  currentTime: number;
  playbackSpeed: number; // 0.25x to 16x
  currentEventIndex: number;
  progress: number; // 0-1
  looping: boolean;
}

export interface InterpolatedFrame {
  timestamp: number;
  lat: number;
  lng: number;
  heading: number;
  speed: number;
  altitude: number;
  accuracy: number;
  /** Interpolation factor between prev and next event (0-1) */
  t: number;
  /** Whether this is an exact event or interpolated */
  isInterpolated: boolean;
}

export interface ReplayAnnotation {
  id: string;
  timestamp: number;
  type: 'marker' | 'note' | 'bookmark' | 'issue' | 'highlight';
  text: string;
  color: string;
  createdBy: string;
  createdAt: number;
}

export interface DeterminismResult {
  isIdentical: boolean;
  totalEvents: number;
  matchedEvents: number;
  mismatches: DeterminismMismatch[];
  confidence: number; // 0-1
}

export interface DeterminismMismatch {
  eventIndex: number;
  field: string;
  expected: unknown;
  actual: unknown;
  severity: 'critical' | 'warning' | 'info';
}

export interface ReplayConfig {
  /** Default playback speed */
  defaultSpeed: number;
  /** Frame rate for interpolation (fps) */
  frameRate: number;
  /** Maximum events to load at once */
  maxBatchSize: number;
  /** Enable determinism checking */
  enableDeterminismCheck: boolean;
  /** Interpolation method */
  interpolation: 'linear' | 'cubic' | 'hermite';
  /** Auto-loop when reaching end */
  autoLoop: boolean;
}

const DEFAULT_CONFIG: ReplayConfig = {
  defaultSpeed: 1.0,
  frameRate: 30,
  maxBatchSize: 50000,
  enableDeterminismCheck: true,
  interpolation: 'hermite',
  autoLoop: false,
};

// ─── Utility Functions ──────────────────────────────────

function simpleHash(data: string): string {
  let hash = 0;
  for (let i = 0; i < data.length; i++) {
    const char = data.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32bit integer
  }
  return Math.abs(hash).toString(36).padStart(8, '0');
}

function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

function lerpAngle(a: number, b: number, t: number): number {
  let diff = b - a;
  while (diff > 180) diff -= 360;
  while (diff < -180) diff += 360;
  return a + diff * t;
}

/** Hermite interpolation for smoother curves */
function hermite(p0: number, m0: number, p1: number, m1: number, t: number): number {
  const t2 = t * t;
  const t3 = t2 * t;
  return (2 * t3 - 3 * t2 + 1) * p0 +
         (t3 - 2 * t2 + t) * m0 +
         (-2 * t3 + 3 * t2) * p1 +
         (t3 - t2) * m1;
}

// ─── Event Loader ───────────────────────────────────────

export class EventLoader {
  private events: ReplayEventRecord[] = [];

  /**
   * Load events from a raw array (e.g., from API or file)
   */
  load(rawEvents: Array<{
    type: ReplayEventType;
    timestamp: number;
    data: Record<string, unknown>;
    source?: string;
  }>): ReplayEventRecord[] {
    this.events = rawEvents.map((e, i) => ({
      id: `evt-${i}-${e.timestamp}`,
      type: e.type,
      timestamp: e.timestamp,
      data: e.data,
      source: e.source || 'unknown',
      seq: i,
      hash: simpleHash(JSON.stringify(e.data) + e.timestamp),
    }));

    // Sort by timestamp, then by sequence
    this.events.sort((a, b) => a.timestamp - b.timestamp || a.seq - b.seq);

    return this.events;
  }

  /**
   * Filter events by type
   */
  filterByType(type: ReplayEventType): ReplayEventRecord[] {
    return this.events.filter(e => e.type === type);
  }

  /**
   * Get events within a time range
   */
  getRange(startTime: number, endTime: number): ReplayEventRecord[] {
    return this.events.filter(e => e.timestamp >= startTime && e.timestamp <= endTime);
  }

  /**
   * Get total loaded events
   */
  getCount(): number {
    return this.events.length;
  }

  /**
   * Get all loaded events
   */
  getAll(): ReplayEventRecord[] {
    return [...this.events];
  }

  /**
   * Clear loaded events
   */
  clear(): void {
    this.events = [];
  }
}

// ─── Timeline Reconstructor ─────────────────────────────

export class TimelineReconstructor {
  /**
   * Build a complete timeline from loaded events
   */
  reconstruct(tripId: string, events: ReplayEventRecord[]): ReplayTimeline {
    if (events.length === 0) {
      return {
        tripId,
        startTime: 0,
        endTime: 0,
        duration: 0,
        eventCount: 0,
        events: [],
        timeIndex: [],
      };
    }

    const sorted = [...events].sort((a, b) => a.timestamp - b.timestamp || a.seq - b.seq);
    const startTime = sorted[0].timestamp;
    const endTime = sorted[sorted.length - 1].timestamp;

    return {
      tripId,
      startTime,
      endTime,
      duration: endTime - startTime,
      eventCount: sorted.length,
      events: sorted,
      timeIndex: sorted.map(e => e.timestamp),
    };
  }

  /**
   * Find the event index at or just before a given time using binary search
   */
  findEventAt(timeline: ReplayTimeline, time: number): number {
    const { timeIndex } = timeline;
    if (timeIndex.length === 0) return -1;
    if (time <= timeIndex[0]) return 0;
    if (time >= timeIndex[timeIndex.length - 1]) return timeIndex.length - 1;

    let lo = 0;
    let hi = timeIndex.length - 1;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (timeIndex[mid] <= time) {
        lo = mid;
      } else {
        hi = mid - 1;
      }
    }
    return lo;
  }

  /**
   * Get events in a window around a timestamp
   */
  getWindow(timeline: ReplayTimeline, centerTime: number, windowMs: number): ReplayEventRecord[] {
    const start = centerTime - windowMs / 2;
    const end = centerTime + windowMs / 2;
    return timeline.events.filter(e => e.timestamp >= start && e.timestamp <= end);
  }

  /**
   * Compute statistics about the timeline
   */
  getStats(timeline: ReplayTimeline): {
    totalEvents: number;
    eventsByType: Record<string, number>;
    avgEventInterval: number;
    maxGap: number;
    density: number; // events per second
  } {
    const eventsByType: Record<string, number> = {};
    let maxGap = 0;
    let totalInterval = 0;

    for (let i = 0; i < timeline.events.length; i++) {
      const evt = timeline.events[i];
      eventsByType[evt.type] = (eventsByType[evt.type] || 0) + 1;

      if (i > 0) {
        const gap = evt.timestamp - timeline.events[i - 1].timestamp;
        totalInterval += gap;
        if (gap > maxGap) maxGap = gap;
      }
    }

    const avgEventInterval = timeline.eventCount > 1
      ? totalInterval / (timeline.eventCount - 1)
      : 0;

    return {
      totalEvents: timeline.eventCount,
      eventsByType,
      avgEventInterval,
      maxGap,
      density: timeline.duration > 0 ? (timeline.eventCount / (timeline.duration / 1000)) : 0,
    };
  }
}

// ─── Frame Interpolator ─────────────────────────────────

export class FrameInterpolator {
  private config: ReplayConfig;

  constructor(config: Partial<ReplayConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * Interpolate a frame at a given timestamp from position events
   */
  interpolate(
    timeline: ReplayTimeline,
    time: number,
    positionEvents: ReplayEventRecord[]
  ): InterpolatedFrame | null {
    if (positionEvents.length === 0) return null;

    // Find surrounding position events
    let prevIdx = -1;
    let nextIdx = -1;

    for (let i = 0; i < positionEvents.length; i++) {
      if (positionEvents[i].timestamp <= time) {
        prevIdx = i;
      }
      if (positionEvents[i].timestamp >= time && nextIdx === -1) {
        nextIdx = i;
      }
    }

    if (prevIdx === -1 && nextIdx === -1) return null;
    if (prevIdx === -1) prevIdx = nextIdx;
    if (nextIdx === -1) nextIdx = prevIdx;

    const prev = positionEvents[prevIdx];
    const next = positionEvents[nextIdx];

    // Exact match
    if (prevIdx === nextIdx || prev.timestamp === next.timestamp) {
      return {
        timestamp: time,
        lat: (prev.data.lat as number) || 0,
        lng: (prev.data.lng as number) || 0,
        heading: (prev.data.heading as number) || 0,
        speed: (prev.data.speed as number) || 0,
        altitude: (prev.data.altitude as number) || 0,
        accuracy: (prev.data.accuracy as number) || 10,
        t: 0,
        isInterpolated: false,
      };
    }

    // Calculate interpolation factor
    const t = (time - prev.timestamp) / (next.timestamp - prev.timestamp);

    const prevLat = (prev.data.lat as number) || 0;
    const prevLng = (prev.data.lng as number) || 0;
    const nextLat = (next.data.lat as number) || 0;
    const nextLng = (next.data.lng as number) || 0;

    let lat: number, lng: number;

    if (this.config.interpolation === 'hermite' && prevIdx > 0 && nextIdx < positionEvents.length - 1) {
      // Hermite interpolation with tangents from neighboring points
      const pp = positionEvents[prevIdx - 1];
      const nn = positionEvents[nextIdx + 1];
      const m0Lat = ((nextLat - ((pp.data.lat as number) || 0)) / 2);
      const m1Lat = (((nn.data.lat as number) || 0) - prevLat) / 2;
      const m0Lng = ((nextLng - ((pp.data.lng as number) || 0)) / 2);
      const m1Lng = (((nn.data.lng as number) || 0) - prevLng) / 2;
      lat = hermite(prevLat, m0Lat, nextLat, m1Lat, t);
      lng = hermite(prevLng, m0Lng, nextLng, m1Lng, t);
    } else {
      lat = lerp(prevLat, nextLat, t);
      lng = lerp(prevLng, nextLng, t);
    }

    return {
      timestamp: time,
      lat,
      lng,
      heading: lerpAngle(
        (prev.data.heading as number) || 0,
        (next.data.heading as number) || 0,
        t
      ),
      speed: lerp(
        (prev.data.speed as number) || 0,
        (next.data.speed as number) || 0,
        t
      ),
      altitude: lerp(
        (prev.data.altitude as number) || 0,
        (next.data.altitude as number) || 0,
        t
      ),
      accuracy: lerp(
        (prev.data.accuracy as number) || 10,
        (next.data.accuracy as number) || 10,
        t
      ),
      t,
      isInterpolated: true,
    };
  }

  /**
   * Generate a sequence of frames at the configured frame rate
   */
  generateFrames(
    timeline: ReplayTimeline,
    startTime: number,
    endTime: number,
    positionEvents: ReplayEventRecord[]
  ): InterpolatedFrame[] {
    const frames: InterpolatedFrame[] = [];
    const frameInterval = 1000 / this.config.frameRate;

    for (let t = startTime; t <= endTime; t += frameInterval) {
      const frame = this.interpolate(timeline, t, positionEvents);
      if (frame) frames.push(frame);
    }

    return frames;
  }
}

// ─── Determinism Checker ────────────────────────────────

export class DeterminismChecker {
  /**
   * Compare two replay runs for deterministic output
   */
  compare(
    baseline: ReplayEventRecord[],
    replay: ReplayEventRecord[]
  ): DeterminismResult {
    const mismatches: DeterminismMismatch[] = [];
    const maxLen = Math.max(baseline.length, replay.length);
    let matched = 0;

    for (let i = 0; i < maxLen; i++) {
      if (i >= baseline.length) {
        mismatches.push({
          eventIndex: i,
          field: 'existence',
          expected: 'no event',
          actual: replay[i].type,
          severity: 'critical',
        });
        continue;
      }
      if (i >= replay.length) {
        mismatches.push({
          eventIndex: i,
          field: 'existence',
          expected: baseline[i].type,
          actual: 'no event',
          severity: 'critical',
        });
        continue;
      }

      const b = baseline[i];
      const r = replay[i];

      if (b.hash !== r.hash) {
        // Check which fields differ
        if (b.type !== r.type) {
          mismatches.push({
            eventIndex: i,
            field: 'type',
            expected: b.type,
            actual: r.type,
            severity: 'critical',
          });
        }
        if (b.timestamp !== r.timestamp) {
          const drift = Math.abs(b.timestamp - r.timestamp);
          mismatches.push({
            eventIndex: i,
            field: 'timestamp',
            expected: b.timestamp,
            actual: r.timestamp,
            severity: drift > 100 ? 'critical' : drift > 10 ? 'warning' : 'info',
          });
        }
        // Check data fields
        const bKeys = Object.keys(b.data);
        const rKeys = Object.keys(r.data);
        const allKeys = Array.from(new Set([...bKeys, ...rKeys]));
        for (const key of allKeys) {
          if (JSON.stringify(b.data[key]) !== JSON.stringify(r.data[key])) {
            mismatches.push({
              eventIndex: i,
              field: `data.${key}`,
              expected: b.data[key],
              actual: r.data[key],
              severity: key === 'lat' || key === 'lng' ? 'critical' : 'warning',
            });
          }
        }
      } else {
        matched++;
      }
    }

    return {
      isIdentical: mismatches.length === 0,
      totalEvents: maxLen,
      matchedEvents: matched,
      mismatches,
      confidence: maxLen > 0 ? matched / maxLen : 1,
    };
  }
}

// ─── Playback Controller ────────────────────────────────

export class PlaybackController {
  private state: PlaybackState;
  private timeline: ReplayTimeline | null = null;
  private config: ReplayConfig;
  private animationFrame: number | null = null;
  private lastFrameTime: number = 0;
  private listeners: Array<(frame: InterpolatedFrame, state: PlaybackState) => void> = [];
  private interpolator: FrameInterpolator;
  private positionEvents: ReplayEventRecord[] = [];

  constructor(config: Partial<ReplayConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.interpolator = new FrameInterpolator(this.config);
    this.state = {
      status: 'idle',
      currentTime: 0,
      playbackSpeed: this.config.defaultSpeed,
      currentEventIndex: 0,
      progress: 0,
      looping: this.config.autoLoop,
    };
  }

  /**
   * Load a timeline for playback
   */
  loadTimeline(timeline: ReplayTimeline): void {
    this.timeline = timeline;
    this.positionEvents = timeline.events.filter(e => e.type === 'position_fix');
    this.state = {
      ...this.state,
      status: 'loading',
      currentTime: timeline.startTime,
      currentEventIndex: 0,
      progress: 0,
    };
    this.state.status = 'paused';
  }

  /**
   * Start or resume playback
   */
  play(): void {
    if (!this.timeline || this.state.status === 'playing') return;
    this.state.status = 'playing';
    this.lastFrameTime = performance.now();
    this.tick();
  }

  /**
   * Pause playback
   */
  pause(): void {
    this.state.status = 'paused';
    if (this.animationFrame !== null) {
      cancelAnimationFrame(this.animationFrame);
      this.animationFrame = null;
    }
  }

  /**
   * Seek to a specific time
   */
  seek(time: number): void {
    if (!this.timeline) return;
    this.state.currentTime = Math.max(
      this.timeline.startTime,
      Math.min(time, this.timeline.endTime)
    );
    this.state.progress = this.timeline.duration > 0
      ? (this.state.currentTime - this.timeline.startTime) / this.timeline.duration
      : 0;
    this.emitFrame();
  }

  /**
   * Set playback speed
   */
  setSpeed(speed: number): void {
    this.state.playbackSpeed = Math.max(0.25, Math.min(16, speed));
  }

  /**
   * Toggle looping
   */
  setLooping(loop: boolean): void {
    this.state.looping = loop;
  }

  /**
   * Subscribe to frame updates
   */
  onFrame(listener: (frame: InterpolatedFrame, state: PlaybackState) => void): () => void {
    this.listeners.push(listener);
    return () => {
      this.listeners = this.listeners.filter(l => l !== listener);
    };
  }

  /**
   * Get current playback state
   */
  getState(): PlaybackState {
    return { ...this.state };
  }

  /**
   * Stop and reset
   */
  stop(): void {
    this.pause();
    if (this.timeline) {
      this.state.currentTime = this.timeline.startTime;
      this.state.progress = 0;
      this.state.currentEventIndex = 0;
    }
    this.state.status = 'idle';
  }

  /**
   * Destroy and clean up
   */
  destroy(): void {
    this.stop();
    this.listeners = [];
    this.timeline = null;
    this.positionEvents = [];
  }

  private tick = (): void => {
    if (this.state.status !== 'playing' || !this.timeline) return;

    const now = performance.now();
    const deltaMs = (now - this.lastFrameTime) * this.state.playbackSpeed;
    this.lastFrameTime = now;

    this.state.currentTime += deltaMs;

    if (this.state.currentTime >= this.timeline.endTime) {
      if (this.state.looping) {
        this.state.currentTime = this.timeline.startTime;
      } else {
        this.state.currentTime = this.timeline.endTime;
        this.state.status = 'complete';
        this.emitFrame();
        return;
      }
    }

    this.state.progress = this.timeline.duration > 0
      ? (this.state.currentTime - this.timeline.startTime) / this.timeline.duration
      : 0;

    this.emitFrame();
    this.animationFrame = requestAnimationFrame(this.tick);
  };

  private emitFrame(): void {
    if (!this.timeline) return;
    const frame = this.interpolator.interpolate(
      this.timeline,
      this.state.currentTime,
      this.positionEvents
    );
    if (frame) {
      for (const listener of this.listeners) {
        listener(frame, { ...this.state });
      }
    }
  }
}

// ─── Annotation Manager ─────────────────────────────────

export class AnnotationManager {
  private annotations: ReplayAnnotation[] = [];

  add(annotation: Omit<ReplayAnnotation, 'id' | 'createdAt'>): ReplayAnnotation {
    const entry: ReplayAnnotation = {
      ...annotation,
      id: `ann-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      createdAt: Date.now(),
    };
    this.annotations.push(entry);
    this.annotations.sort((a, b) => a.timestamp - b.timestamp);
    return entry;
  }

  remove(id: string): boolean {
    const idx = this.annotations.findIndex(a => a.id === id);
    if (idx === -1) return false;
    this.annotations.splice(idx, 1);
    return true;
  }

  getAll(): ReplayAnnotation[] {
    return [...this.annotations];
  }

  getByType(type: ReplayAnnotation['type']): ReplayAnnotation[] {
    return this.annotations.filter(a => a.type === type);
  }

  getInRange(startTime: number, endTime: number): ReplayAnnotation[] {
    return this.annotations.filter(a => a.timestamp >= startTime && a.timestamp <= endTime);
  }

  clear(): void {
    this.annotations = [];
  }
}

// ─── Main Replay Engine ─────────────────────────────────

export class ReplayEngine {
  readonly loader: EventLoader;
  readonly reconstructor: TimelineReconstructor;
  readonly interpolator: FrameInterpolator;
  readonly determinism: DeterminismChecker;
  readonly playback: PlaybackController;
  readonly annotations: AnnotationManager;

  private config: ReplayConfig;
  private timeline: ReplayTimeline | null = null;

  constructor(config: Partial<ReplayConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.loader = new EventLoader();
    this.reconstructor = new TimelineReconstructor();
    this.interpolator = new FrameInterpolator(this.config);
    this.determinism = new DeterminismChecker();
    this.playback = new PlaybackController(this.config);
    this.annotations = new AnnotationManager();
  }

  /**
   * Load and prepare a trip for replay
   */
  loadTrip(
    tripId: string,
    events: Array<{
      type: ReplayEventType;
      timestamp: number;
      data: Record<string, unknown>;
      source?: string;
    }>
  ): ReplayTimeline {
    const loaded = this.loader.load(events);
    this.timeline = this.reconstructor.reconstruct(tripId, loaded);
    this.playback.loadTimeline(this.timeline);
    return this.timeline;
  }

  /**
   * Get the current timeline
   */
  getTimeline(): ReplayTimeline | null {
    return this.timeline;
  }

  /**
   * Get timeline statistics
   */
  getStats() {
    if (!this.timeline) return null;
    return this.reconstructor.getStats(this.timeline);
  }

  /**
   * Verify determinism against a baseline
   */
  verifyDeterminism(baseline: ReplayEventRecord[]): DeterminismResult {
    const current = this.loader.getAll();
    return this.determinism.compare(baseline, current);
  }

  /**
   * Destroy and clean up all resources
   */
  destroy(): void {
    this.playback.destroy();
    this.loader.clear();
    this.annotations.clear();
    this.timeline = null;
  }
}

// ─── Singleton ──────────────────────────────────────────

let _instance: ReplayEngine | null = null;

export function getReplayEngine(config?: Partial<ReplayConfig>): ReplayEngine {
  if (!_instance) {
    _instance = new ReplayEngine(config);
  }
  return _instance;
}

export function resetReplayEngine(): void {
  if (_instance) {
    _instance.destroy();
    _instance = null;
  }
}
