/**
 * G.A.N.E — Event Bus Abstraction
 * =================================
 * Typed, idempotent event bus with replay, dead-letter queue,
 * and WebSocket transport abstraction.
 *
 * Spec refs:
 *   Doc-3 §6.4 "Event-Driven Architecture"
 *   Doc-4 §5.2 "Message Bus Abstraction"
 *
 * Architecture:
 *   - Strongly-typed event definitions with discriminated unions
 *   - Idempotency via event ID deduplication
 *   - Event replay from persistent buffer
 *   - Dead-letter queue for failed handlers
 *   - Priority-based event ordering
 *   - Middleware pipeline (transform, filter, log)
 *   - WebSocket transport with auto-reconnect
 *   - Offline queue with sync-on-reconnect
 */

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

export type EventPriority = 'critical' | 'high' | 'normal' | 'low';

export interface BaseEvent {
  id: string;                    // UUID for idempotency
  type: string;                  // Discriminator
  timestamp: number;             // Unix ms
  source: string;                // Originating module
  priority: EventPriority;
  correlationId?: string;        // For request-response patterns
  causationId?: string;          // ID of the event that caused this one
  metadata?: Record<string, unknown>;
}

// ─── Typed Event Definitions ───

export interface LocationUpdateEvent extends BaseEvent {
  type: 'location.update';
  payload: { lat: number; lon: number; accuracy: number; speed: number; heading: number; altitude: number };
}

export interface RouteCalculatedEvent extends BaseEvent {
  type: 'route.calculated';
  payload: { routeId: string; waypoints: { lat: number; lon: number }[]; distanceM: number; durationS: number; mode: string };
}

export interface RouteDeviationEvent extends BaseEvent {
  type: 'route.deviation';
  payload: { routeId: string; deviationM: number; currentLat: number; currentLon: number };
}

export interface IncidentReportedEvent extends BaseEvent {
  type: 'incident.reported';
  payload: { incidentId: string; incidentType: string; lat: number; lon: number; severity: number };
}

export interface IncidentConfirmedEvent extends BaseEvent {
  type: 'incident.confirmed';
  payload: { incidentId: string; confirmations: number };
}

export interface TrafficUpdateEvent extends BaseEvent {
  type: 'traffic.update';
  payload: { segmentId: string; congestionLevel: number; speedKmh: number; incidentCount: number };
}

export interface ETAUpdateEvent extends BaseEvent {
  type: 'eta.update';
  payload: { routeId: string; etaMs: number; confidencePercent: number; delayMs: number };
}

export interface BatteryModeChangeEvent extends BaseEvent {
  type: 'battery.modeChange';
  payload: { previousMode: string; newMode: string; batteryLevel: number };
}

export interface SensorFusionEvent extends BaseEvent {
  type: 'sensor.fusion';
  payload: { lat: number; lon: number; accuracy: number; source: string; confidence: number };
}

export interface V2XMessageEvent extends BaseEvent {
  type: 'v2x.message';
  payload: { messageType: string; senderId: string; data: Record<string, unknown> };
}

export interface AlertTriggeredEvent extends BaseEvent {
  type: 'alert.triggered';
  payload: { alertId: string; alertType: string; message: string; severity: 'info' | 'warning' | 'critical' };
}

export interface LiveShareUpdateEvent extends BaseEvent {
  type: 'liveShare.update';
  payload: { shareId: string; lat: number; lon: number; speed: number; heading: number };
}

export interface OfflineSyncEvent extends BaseEvent {
  type: 'offline.sync';
  payload: { direction: 'upload' | 'download'; itemCount: number; bytesTransferred: number };
}

export interface SystemHealthEvent extends BaseEvent {
  type: 'system.health';
  payload: { component: string; status: 'healthy' | 'degraded' | 'down'; latencyMs: number };
}

// Union of all events
export type GANEEvent =
  | LocationUpdateEvent
  | RouteCalculatedEvent
  | RouteDeviationEvent
  | IncidentReportedEvent
  | IncidentConfirmedEvent
  | TrafficUpdateEvent
  | ETAUpdateEvent
  | BatteryModeChangeEvent
  | SensorFusionEvent
  | V2XMessageEvent
  | AlertTriggeredEvent
  | LiveShareUpdateEvent
  | OfflineSyncEvent
  | SystemHealthEvent;

export type EventType = GANEEvent['type'];

// Extract event payload by type
export type EventPayload<T extends EventType> = Extract<GANEEvent, { type: T }>['payload'];

// ─── Handler types ───

export type EventHandler<T extends EventType = EventType> = (event: Extract<GANEEvent, { type: T }>) => void | Promise<void>;

export type EventMiddleware = (event: GANEEvent, next: () => void) => void;

export interface Subscription {
  id: string;
  unsubscribe: () => void;
}

export interface DeadLetterEntry {
  event: GANEEvent;
  error: string;
  handlerId: string;
  timestamp: number;
  retryCount: number;
}

// ═══════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════

export interface EventBusConfig {
  maxBufferSize: number;          // Max events in replay buffer
  maxDeadLetterSize: number;      // Max entries in DLQ
  deduplicationWindowMs: number;  // Window for idempotency check
  maxRetries: number;             // Max retries for failed handlers
  retryDelayMs: number;           // Base delay between retries
  enableReplay: boolean;          // Enable event replay buffer
  enableDeadLetter: boolean;      // Enable dead-letter queue
  enableMetrics: boolean;         // Track event metrics
  enableOfflineQueue: boolean;    // Queue events when offline
}

const DEFAULT_CONFIG: EventBusConfig = {
  maxBufferSize: 5000,
  maxDeadLetterSize: 500,
  deduplicationWindowMs: 60_000,
  maxRetries: 3,
  retryDelayMs: 1000,
  enableReplay: true,
  enableDeadLetter: true,
  enableMetrics: true,
  enableOfflineQueue: true,
};

// ═══════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════

function generateEventId(): string {
  const arr = new Uint8Array(16);
  crypto.getRandomValues(arr);
  const hex = Array.from(arr).map(b => b.toString(16).padStart(2, '0')).join('');
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}

const PRIORITY_ORDER: Record<EventPriority, number> = {
  critical: 0,
  high: 1,
  normal: 2,
  low: 3,
};

// ═══════════════════════════════════════════════════════════
// EVENT BUS
// ═══════════════════════════════════════════════════════════

export class EventBus {
  private config: EventBusConfig;
  private handlers: Map<string, Map<string, EventHandler<any>>> = new Map();
  private wildcardHandlers: Map<string, EventHandler<any>> = new Map();
  private middlewares: EventMiddleware[] = [];
  private replayBuffer: GANEEvent[] = [];
  private deadLetterQueue: DeadLetterEntry[] = [];
  private processedIds: Map<string, number> = new Map(); // id -> timestamp
  private offlineQueue: GANEEvent[] = [];
  private isOnline = true;
  private boundOnline = () => { this.isOnline = true; this.flushOfflineQueue(); };
  private boundOffline = () => { this.isOnline = false; };

  // Metrics
  private eventCounts: Map<string, number> = new Map();
  private errorCounts: Map<string, number> = new Map();
  private latencies: Map<string, number[]> = new Map();

  // Cleanup timer
  private cleanupTimer: ReturnType<typeof setInterval> | null = null;

  constructor(config: Partial<EventBusConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };

    // Periodic cleanup of deduplication cache
    this.cleanupTimer = setInterval(() => this.cleanupDeduplication(), 30_000);

    // Online/offline detection
    if (typeof window !== 'undefined') {
      window.addEventListener('online', this.boundOnline);
      window.addEventListener('offline', this.boundOffline);
      this.isOnline = navigator.onLine;
    }
  }

  // ─── Publishing ───

  /**
   * Publish an event to all subscribers.
   * Returns true if the event was processed (not deduplicated).
   */
  publish(event: GANEEvent): boolean {
    // Idempotency check
    if (this.processedIds.has(event.id)) {
      return false;
    }

    // Mark as processed
    this.processedIds.set(event.id, Date.now());

    // Offline queue
    if (!this.isOnline && this.config.enableOfflineQueue) {
      this.offlineQueue.push(event);
      return true;
    }

    // Run middleware pipeline
    let proceed = true;
    for (const mw of this.middlewares) {
      let called = false;
      mw(event, () => { called = true; });
      if (!called) {
        proceed = false;
        break;
      }
    }
    if (!proceed) return false;

    // Add to replay buffer
    if (this.config.enableReplay) {
      this.replayBuffer.push(event);
      if (this.replayBuffer.length > this.config.maxBufferSize) {
        this.replayBuffer = this.replayBuffer.slice(-this.config.maxBufferSize);
      }
    }

    // Track metrics
    if (this.config.enableMetrics) {
      this.eventCounts.set(event.type, (this.eventCounts.get(event.type) ?? 0) + 1);
    }

    // Dispatch to handlers
    this.dispatch(event);

    return true;
  }

  /**
   * Create and publish an event with auto-generated ID and timestamp.
   */
  emit<T extends EventType>(
    type: T,
    payload: EventPayload<T>,
    options: {
      source?: string;
      priority?: EventPriority;
      correlationId?: string;
      causationId?: string;
      metadata?: Record<string, unknown>;
    } = {}
  ): string {
    const id = generateEventId();
    const event = {
      id,
      type,
      timestamp: Date.now(),
      source: options.source ?? 'gane-client',
      priority: options.priority ?? 'normal',
      correlationId: options.correlationId,
      causationId: options.causationId,
      metadata: options.metadata,
      payload,
    } as GANEEvent;

    this.publish(event);
    return id;
  }

  // ─── Subscribing ───

  /**
   * Subscribe to a specific event type.
   */
  on<T extends EventType>(type: T, handler: EventHandler<T>): Subscription {
    const handlerId = generateEventId();

    if (!this.handlers.has(type)) {
      this.handlers.set(type, new Map());
    }
    this.handlers.get(type)!.set(handlerId, handler);

    return {
      id: handlerId,
      unsubscribe: () => {
        this.handlers.get(type)?.delete(handlerId);
      },
    };
  }

  /**
   * Subscribe to all events (wildcard).
   */
  onAll(handler: EventHandler<any>): Subscription {
    const handlerId = generateEventId();
    this.wildcardHandlers.set(handlerId, handler);

    return {
      id: handlerId,
      unsubscribe: () => {
        this.wildcardHandlers.delete(handlerId);
      },
    };
  }

  /**
   * Subscribe to an event type, but only trigger once.
   */
  once<T extends EventType>(type: T, handler: EventHandler<T>): Subscription {
    const sub = this.on(type, ((event: Extract<GANEEvent, { type: T }>) => {
      sub.unsubscribe();
      handler(event);
    }) as EventHandler<T>);
    return sub;
  }

  /**
   * Subscribe with a filter predicate.
   */
  onFiltered<T extends EventType>(
    type: T,
    filter: (event: Extract<GANEEvent, { type: T }>) => boolean,
    handler: EventHandler<T>
  ): Subscription {
    return this.on(type, ((event: Extract<GANEEvent, { type: T }>) => {
      if (filter(event)) {
        handler(event);
      }
    }) as EventHandler<T>);
  }

  // ─── Middleware ───

  /**
   * Add a middleware to the event pipeline.
   * Middlewares are executed in order. Call next() to continue.
   */
  use(middleware: EventMiddleware): void {
    this.middlewares.push(middleware);
  }

  // ─── Replay ───

  /**
   * Replay events from the buffer.
   */
  replay(options: {
    type?: EventType;
    since?: number;
    limit?: number;
  } = {}): GANEEvent[] {
    let events = [...this.replayBuffer];

    if (options.type) {
      events = events.filter(e => e.type === options.type);
    }
    if (options.since) {
      events = events.filter(e => e.timestamp >= options.since!);
    }
    if (options.limit) {
      events = events.slice(-options.limit);
    }

    // Re-dispatch replayed events
    for (const event of events) {
      // Clear from dedup cache so they can be re-processed
      this.processedIds.delete(event.id);
    }

    return events;
  }

  /**
   * Replay events to a specific handler.
   */
  replayTo<T extends EventType>(
    type: T,
    handler: EventHandler<T>,
    options: { since?: number; limit?: number } = {}
  ): number {
    const events = this.replay({ type, ...options });
    let count = 0;
    for (const event of events) {
      try {
        handler(event as any);
        count++;
      } catch {
        // Skip failed replays
      }
    }
    return count;
  }

  // ─── Dead Letter Queue ───

  /**
   * Get entries from the dead-letter queue.
   */
  getDeadLetters(limit: number = 50): DeadLetterEntry[] {
    return this.deadLetterQueue.slice(-limit);
  }

  /**
   * Retry a dead-letter entry.
   */
  retryDeadLetter(index: number): boolean {
    if (index < 0 || index >= this.deadLetterQueue.length) return false;
    const entry = this.deadLetterQueue[index];
    this.deadLetterQueue.splice(index, 1);
    // Clear from dedup cache
    this.processedIds.delete(entry.event.id);
    return this.publish(entry.event);
  }

  /**
   * Clear the dead-letter queue.
   */
  clearDeadLetters(): void {
    this.deadLetterQueue = [];
  }

  // ─── Metrics ───

  /**
   * Get event bus metrics.
   */
  getMetrics(): {
    eventCounts: Record<string, number>;
    errorCounts: Record<string, number>;
    avgLatencies: Record<string, number>;
    bufferSize: number;
    deadLetterSize: number;
    offlineQueueSize: number;
    totalProcessed: number;
    isOnline: boolean;
  } {
    const eventCountsObj: Record<string, number> = {};
    for (const [k, v] of Array.from(this.eventCounts.entries())) {
      eventCountsObj[k] = v;
    }

    const errorCountsObj: Record<string, number> = {};
    for (const [k, v] of Array.from(this.errorCounts.entries())) {
      errorCountsObj[k] = v;
    }

    const avgLatenciesObj: Record<string, number> = {};
    for (const [k, v] of Array.from(this.latencies.entries())) {
      avgLatenciesObj[k] = v.length > 0 ? v.reduce((a, b) => a + b, 0) / v.length : 0;
    }

    const totalProcessed = Array.from(this.eventCounts.values()).reduce((a, b) => a + b, 0);

    return {
      eventCounts: eventCountsObj,
      errorCounts: errorCountsObj,
      avgLatencies: avgLatenciesObj,
      bufferSize: this.replayBuffer.length,
      deadLetterSize: this.deadLetterQueue.length,
      offlineQueueSize: this.offlineQueue.length,
      totalProcessed,
      isOnline: this.isOnline,
    };
  }

  // ─── Internal ───

  private dispatch(event: GANEEvent): void {
    const start = performance.now();

    // Type-specific handlers
    const typeHandlers = this.handlers.get(event.type);
    if (typeHandlers) {
      for (const [handlerId, handler] of Array.from(typeHandlers.entries())) {
        this.safeInvoke(handler, event, handlerId);
      }
    }

    // Wildcard handlers
    for (const [handlerId, handler] of Array.from(this.wildcardHandlers.entries())) {
      this.safeInvoke(handler, event, handlerId);
    }

    // Track latency
    if (this.config.enableMetrics) {
      const duration = performance.now() - start;
      const latencyArr = this.latencies.get(event.type) ?? [];
      latencyArr.push(duration);
      if (latencyArr.length > 100) latencyArr.splice(0, latencyArr.length - 100);
      this.latencies.set(event.type, latencyArr);
    }
  }

  private safeInvoke(handler: EventHandler<any>, event: GANEEvent, handlerId: string): void {
    try {
      const result = handler(event);
      // Handle async handlers
      if (result && typeof (result as any).catch === 'function') {
        (result as Promise<void>).catch((error: Error) => {
          this.handleError(event, error, handlerId);
        });
      }
    } catch (error) {
      this.handleError(event, error instanceof Error ? error : new Error(String(error)), handlerId);
    }
  }

  private handleError(event: GANEEvent, error: Error, handlerId: string): void {
    if (this.config.enableMetrics) {
      this.errorCounts.set(event.type, (this.errorCounts.get(event.type) ?? 0) + 1);
    }

    if (this.config.enableDeadLetter) {
      this.deadLetterQueue.push({
        event,
        error: error.message,
        handlerId,
        timestamp: Date.now(),
        retryCount: 0,
      });

      if (this.deadLetterQueue.length > this.config.maxDeadLetterSize) {
        this.deadLetterQueue = this.deadLetterQueue.slice(-this.config.maxDeadLetterSize);
      }
    }
  }

  private cleanupDeduplication(): void {
    const cutoff = Date.now() - this.config.deduplicationWindowMs;
    for (const [id, timestamp] of Array.from(this.processedIds.entries())) {
      if (timestamp < cutoff) {
        this.processedIds.delete(id);
      }
    }
  }

  private flushOfflineQueue(): void {
    const queue = [...this.offlineQueue];
    this.offlineQueue = [];
    for (const event of queue) {
      // Clear from dedup cache
      this.processedIds.delete(event.id);
      this.publish(event);
    }
  }

  // ─── Lifecycle ───

  /**
   * Remove all handlers and clear state.
   */
  reset(): void {
    this.handlers.clear();
    this.wildcardHandlers.clear();
    this.middlewares = [];
    this.replayBuffer = [];
    this.deadLetterQueue = [];
    this.processedIds.clear();
    this.offlineQueue = [];
    this.eventCounts.clear();
    this.errorCounts.clear();
    this.latencies.clear();
  }

  /**
   * Destroy the event bus.
   */
  destroy(): void {
    if (this.cleanupTimer) {
      clearInterval(this.cleanupTimer);
      this.cleanupTimer = null;
    }
    // Remove online/offline listeners
    if (typeof window !== 'undefined') {
      window.removeEventListener('online', this.boundOnline);
      window.removeEventListener('offline', this.boundOffline);
    }
    this.reset();
  }

  /**
   * Get handler count for a specific event type.
   */
  listenerCount(type: EventType): number {
    return (this.handlers.get(type)?.size ?? 0) + this.wildcardHandlers.size;
  }

  /**
   * Get all registered event types.
   */
  registeredTypes(): EventType[] {
    return Array.from(this.handlers.keys()) as EventType[];
  }
}

// ═══════════════════════════════════════════════════════════
// SINGLETON
// ═══════════════════════════════════════════════════════════

let _globalBus: EventBus | null = null;

/**
 * Get or create the global event bus instance.
 */
export function getEventBus(config?: Partial<EventBusConfig>): EventBus {
  if (!_globalBus) {
    _globalBus = new EventBus(config);
  }
  return _globalBus;
}

/**
 * Convenience: emit an event on the global bus.
 */
export function emit<T extends EventType>(
  type: T,
  payload: EventPayload<T>,
  options?: Parameters<EventBus['emit']>[2]
): string {
  return getEventBus().emit(type, payload, options);
}

/**
 * Convenience: subscribe to an event on the global bus.
 */
export function on<T extends EventType>(type: T, handler: EventHandler<T>): Subscription {
  return getEventBus().on(type, handler);
}

// ═══════════════════════════════════════════════════════════
// LOGGING MIDDLEWARE (built-in)
// ═══════════════════════════════════════════════════════════

/**
 * Middleware that logs all events to console.
 */
export function createLoggingMiddleware(options: {
  filter?: EventType[];
  verbose?: boolean;
} = {}): EventMiddleware {
  return (event, next) => {
    if (options.filter && !options.filter.includes(event.type as EventType)) {
      next();
      return;
    }
    if (options.verbose) {
      console.log(`[EventBus] ${event.type}`, {
        id: event.id,
        source: event.source,
        priority: event.priority,
        payload: (event as any).payload,
      });
    } else {
      console.log(`[EventBus] ${event.type} (${event.id.slice(0, 8)})`);
    }
    next();
  };
}

/**
 * Middleware that filters events by priority.
 */
export function createPriorityFilter(minPriority: EventPriority): EventMiddleware {
  const minOrder = PRIORITY_ORDER[minPriority];
  return (event, next) => {
    if (PRIORITY_ORDER[event.priority] <= minOrder) {
      next();
    }
    // Silently drop lower-priority events
  };
}

/**
 * Middleware that rate-limits events per type.
 */
export function createRateLimiter(maxPerSecond: number): EventMiddleware {
  const counts: Map<string, { count: number; resetAt: number }> = new Map();

  return (event, next) => {
    const now = Date.now();
    const entry = counts.get(event.type);

    if (!entry || now >= entry.resetAt) {
      counts.set(event.type, { count: 1, resetAt: now + 1000 });
      next();
    } else if (entry.count < maxPerSecond) {
      entry.count++;
      next();
    }
    // Silently drop rate-limited events
  };
}
