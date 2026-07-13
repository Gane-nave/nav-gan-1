/**
 * G.A.N.E — PRODUCTION INFRASTRUCTURE MODEL
 * 
 * Spec Reference: Doc 8 §5-8, Doc 9 §6-8
 * 
 * Production-grade low-level design:
 * 1. Thread/Process model
 * 2. Queue topology
 * 3. Memory budgets
 * 4. CPU/GPU budgets
 * 5. Latency budgets per hop
 * 6. Cache hierarchy
 * 7. Backpressure rules
 * 8. Failure isolation boundaries
 */

// ═══════════════════════════════════════════════════════════
// 1. THREAD/PROCESS MODEL
// ═══════════════════════════════════════════════════════════

export interface ThreadModel {
  name: string;
  type: 'main' | 'worker' | 'io' | 'compute' | 'timer';
  priority: 'realtime' | 'high' | 'normal' | 'low' | 'idle';
  affinity?: number[];
  stack_size_kb: number;
  max_cpu_percent: number;
  watchdog_timeout_ms: number;
}

export const THREAD_MODELS: ThreadModel[] = [
  // Main thread — UI rendering, event dispatch
  { name: 'main', type: 'main', priority: 'high', stack_size_kb: 1024, max_cpu_percent: 30, watchdog_timeout_ms: 5000 },
  // GNSS processing — real-time position fixes
  { name: 'gnss-processor', type: 'compute', priority: 'realtime', stack_size_kb: 512, max_cpu_percent: 15, watchdog_timeout_ms: 2000 },
  // Sensor fusion — ESKF prediction/update
  { name: 'sensor-fusion', type: 'compute', priority: 'realtime', stack_size_kb: 512, max_cpu_percent: 20, watchdog_timeout_ms: 1000 },
  // Route computation — graph algorithms
  { name: 'route-compute', type: 'compute', priority: 'high', stack_size_kb: 2048, max_cpu_percent: 25, watchdog_timeout_ms: 5000 },
  // Network I/O — API calls, WebSocket
  { name: 'network-io', type: 'io', priority: 'normal', stack_size_kb: 256, max_cpu_percent: 10, watchdog_timeout_ms: 30000 },
  // Map tile decoder
  { name: 'tile-decoder', type: 'compute', priority: 'normal', stack_size_kb: 1024, max_cpu_percent: 15, watchdog_timeout_ms: 10000 },
  // ML inference
  { name: 'ml-inference', type: 'compute', priority: 'low', stack_size_kb: 2048, max_cpu_percent: 20, watchdog_timeout_ms: 5000 },
  // Offline sync
  { name: 'offline-sync', type: 'io', priority: 'low', stack_size_kb: 256, max_cpu_percent: 5, watchdog_timeout_ms: 60000 },
  // Telemetry upload
  { name: 'telemetry', type: 'io', priority: 'idle', stack_size_kb: 128, max_cpu_percent: 3, watchdog_timeout_ms: 120000 },
  // Watchdog timer
  { name: 'watchdog', type: 'timer', priority: 'realtime', stack_size_kb: 64, max_cpu_percent: 1, watchdog_timeout_ms: 500 },
];

// ═══════════════════════════════════════════════════════════
// 2. QUEUE TOPOLOGY
// ═══════════════════════════════════════════════════════════

export interface QueueConfig {
  name: string;
  type: 'fifo' | 'priority' | 'ring_buffer' | 'work_stealing';
  capacity: number;
  overflow_policy: 'drop_oldest' | 'drop_newest' | 'block' | 'reject';
  consumer_count: number;
  batch_size: number;
  flush_interval_ms: number;
  dead_letter: boolean;
  retry_max: number;
  retry_backoff_ms: number;
}

export const QUEUE_TOPOLOGY: QueueConfig[] = [
  // Position fixes — ring buffer, drop oldest when full
  { name: 'position-fixes', type: 'ring_buffer', capacity: 1000, overflow_policy: 'drop_oldest', consumer_count: 1, batch_size: 1, flush_interval_ms: 0, dead_letter: false, retry_max: 0, retry_backoff_ms: 0 },
  // Sensor readings — ring buffer, high throughput
  { name: 'sensor-readings', type: 'ring_buffer', capacity: 5000, overflow_policy: 'drop_oldest', consumer_count: 1, batch_size: 10, flush_interval_ms: 10, dead_letter: false, retry_max: 0, retry_backoff_ms: 0 },
  // Route requests — priority queue
  { name: 'route-requests', type: 'priority', capacity: 100, overflow_policy: 'reject', consumer_count: 2, batch_size: 1, flush_interval_ms: 0, dead_letter: true, retry_max: 3, retry_backoff_ms: 1000 },
  // Incident reports — FIFO with retry
  { name: 'incident-reports', type: 'fifo', capacity: 500, overflow_policy: 'block', consumer_count: 2, batch_size: 1, flush_interval_ms: 0, dead_letter: true, retry_max: 5, retry_backoff_ms: 2000 },
  // Alert dispatch — priority queue
  { name: 'alert-dispatch', type: 'priority', capacity: 1000, overflow_policy: 'reject', consumer_count: 4, batch_size: 1, flush_interval_ms: 0, dead_letter: true, retry_max: 3, retry_backoff_ms: 1000 },
  // Telemetry upload — FIFO with batching
  { name: 'telemetry-upload', type: 'fifo', capacity: 10000, overflow_policy: 'drop_oldest', consumer_count: 1, batch_size: 100, flush_interval_ms: 5000, dead_letter: false, retry_max: 2, retry_backoff_ms: 5000 },
  // Crowd contributions — FIFO
  { name: 'crowd-contributions', type: 'fifo', capacity: 2000, overflow_policy: 'drop_oldest', consumer_count: 2, batch_size: 10, flush_interval_ms: 1000, dead_letter: false, retry_max: 1, retry_backoff_ms: 3000 },
  // Map tile requests — work stealing
  { name: 'tile-requests', type: 'work_stealing', capacity: 200, overflow_policy: 'reject', consumer_count: 4, batch_size: 1, flush_interval_ms: 0, dead_letter: false, retry_max: 2, retry_backoff_ms: 1000 },
  // Event bus — priority queue
  { name: 'event-bus', type: 'priority', capacity: 5000, overflow_policy: 'drop_oldest', consumer_count: 4, batch_size: 10, flush_interval_ms: 50, dead_letter: true, retry_max: 3, retry_backoff_ms: 500 },
  // Sync operations — FIFO
  { name: 'sync-operations', type: 'fifo', capacity: 500, overflow_policy: 'block', consumer_count: 1, batch_size: 50, flush_interval_ms: 10000, dead_letter: true, retry_max: 5, retry_backoff_ms: 10000 },
];

/**
 * Priority Queue implementation with backpressure
 */
export class PriorityQueue<T> {
  private heap: { priority: number; item: T; seq: number }[] = [];
  private seq = 0;
  private readonly capacity: number;
  private readonly overflowPolicy: 'drop_oldest' | 'drop_newest' | 'block' | 'reject';

  constructor(capacity: number, overflowPolicy: 'drop_oldest' | 'drop_newest' | 'block' | 'reject' = 'reject') {
    this.capacity = capacity;
    this.overflowPolicy = overflowPolicy;
  }

  get size(): number { return this.heap.length; }
  get isFull(): boolean { return this.heap.length >= this.capacity; }
  get isEmpty(): boolean { return this.heap.length === 0; }
  get pressure(): number { return this.heap.length / this.capacity; }

  enqueue(item: T, priority: number): boolean {
    if (this.isFull) {
      switch (this.overflowPolicy) {
        case 'drop_oldest':
          this.dequeue();
          break;
        case 'drop_newest':
          return false;
        case 'reject':
          return false;
        case 'block':
          return false; // Caller must handle
      }
    }

    this.heap.push({ priority, item, seq: this.seq++ });
    this.bubbleUp(this.heap.length - 1);
    return true;
  }

  dequeue(): T | undefined {
    if (this.isEmpty) return undefined;
    const top = this.heap[0];
    const last = this.heap.pop()!;
    if (this.heap.length > 0) {
      this.heap[0] = last;
      this.sinkDown(0);
    }
    return top.item;
  }

  peek(): T | undefined {
    return this.heap[0]?.item;
  }

  private bubbleUp(i: number): void {
    while (i > 0) {
      const parent = Math.floor((i - 1) / 2);
      if (this.compare(i, parent) < 0) {
        this.swap(i, parent);
        i = parent;
      } else break;
    }
  }

  private sinkDown(i: number): void {
    const n = this.heap.length;
    while (true) {
      let smallest = i;
      const left = 2 * i + 1;
      const right = 2 * i + 2;
      if (left < n && this.compare(left, smallest) < 0) smallest = left;
      if (right < n && this.compare(right, smallest) < 0) smallest = right;
      if (smallest === i) break;
      this.swap(i, smallest);
      i = smallest;
    }
  }

  private compare(a: number, b: number): number {
    const pa = this.heap[a].priority;
    const pb = this.heap[b].priority;
    if (pa !== pb) return pa - pb; // Lower priority number = higher priority
    return this.heap[a].seq - this.heap[b].seq; // FIFO within same priority
  }

  private swap(a: number, b: number): void {
    [this.heap[a], this.heap[b]] = [this.heap[b], this.heap[a]];
  }
}

/**
 * Ring Buffer for high-throughput sensor data
 */
export class RingBuffer<T> {
  private buffer: (T | undefined)[];
  private head = 0;
  private tail = 0;
  private count = 0;
  private readonly capacity: number;

  constructor(capacity: number) {
    this.capacity = capacity;
    this.buffer = new Array(capacity);
  }

  get size(): number { return this.count; }
  get isFull(): boolean { return this.count >= this.capacity; }
  get isEmpty(): boolean { return this.count === 0; }
  get pressure(): number { return this.count / this.capacity; }

  push(item: T): T | undefined {
    let dropped: T | undefined;
    if (this.isFull) {
      dropped = this.buffer[this.tail];
      this.tail = (this.tail + 1) % this.capacity;
      this.count--;
    }
    this.buffer[this.head] = item;
    this.head = (this.head + 1) % this.capacity;
    this.count++;
    return dropped;
  }

  pop(): T | undefined {
    if (this.isEmpty) return undefined;
    const item = this.buffer[this.tail];
    this.buffer[this.tail] = undefined;
    this.tail = (this.tail + 1) % this.capacity;
    this.count--;
    return item;
  }

  peek(): T | undefined {
    if (this.isEmpty) return undefined;
    return this.buffer[this.tail];
  }

  toArray(): T[] {
    const result: T[] = [];
    let idx = this.tail;
    for (let i = 0; i < this.count; i++) {
      result.push(this.buffer[idx] as T);
      idx = (idx + 1) % this.capacity;
    }
    return result;
  }
}

// ═══════════════════════════════════════════════════════════
// 3. MEMORY BUDGETS
// ═══════════════════════════════════════════════════════════

export interface MemoryBudget {
  subsystem: string;
  budget_mb: number;
  warning_threshold: number; // 0..1
  critical_threshold: number;
  eviction_policy: 'lru' | 'lfu' | 'fifo' | 'priority' | 'none';
}

export const MEMORY_BUDGETS: MemoryBudget[] = [
  { subsystem: 'map-tiles', budget_mb: 128, warning_threshold: 0.8, critical_threshold: 0.95, eviction_policy: 'lru' },
  { subsystem: 'route-graph', budget_mb: 64, warning_threshold: 0.85, critical_threshold: 0.95, eviction_policy: 'lru' },
  { subsystem: 'sensor-buffers', budget_mb: 16, warning_threshold: 0.9, critical_threshold: 0.98, eviction_policy: 'fifo' },
  { subsystem: 'position-history', budget_mb: 8, warning_threshold: 0.8, critical_threshold: 0.95, eviction_policy: 'fifo' },
  { subsystem: 'incident-cache', budget_mb: 8, warning_threshold: 0.8, critical_threshold: 0.95, eviction_policy: 'lru' },
  { subsystem: 'ml-models', budget_mb: 32, warning_threshold: 0.9, critical_threshold: 0.98, eviction_policy: 'priority' },
  { subsystem: 'offline-store', budget_mb: 256, warning_threshold: 0.7, critical_threshold: 0.9, eviction_policy: 'lru' },
  { subsystem: 'event-bus', budget_mb: 16, warning_threshold: 0.8, critical_threshold: 0.95, eviction_policy: 'fifo' },
  { subsystem: 'telemetry-buffer', budget_mb: 4, warning_threshold: 0.8, critical_threshold: 0.95, eviction_policy: 'fifo' },
  { subsystem: 'ui-state', budget_mb: 32, warning_threshold: 0.85, critical_threshold: 0.95, eviction_policy: 'none' },
];

export const TOTAL_MEMORY_BUDGET_MB = MEMORY_BUDGETS.reduce((sum, b) => sum + b.budget_mb, 0);

/**
 * LRU Cache with memory budget enforcement
 */
export class LRUCache<K, V> {
  private cache = new Map<K, { value: V; size: number }>();
  private order: K[] = [];
  private currentSize = 0;
  private readonly maxSize: number;

  constructor(maxSizeBytes: number) {
    this.maxSize = maxSizeBytes;
  }

  get(key: K): V | undefined {
    const entry = this.cache.get(key);
    if (!entry) return undefined;
    // Move to front
    const idx = this.order.indexOf(key);
    if (idx > -1) {
      this.order.splice(idx, 1);
      this.order.push(key);
    }
    return entry.value;
  }

  set(key: K, value: V, sizeBytes: number): void {
    // Remove existing
    if (this.cache.has(key)) {
      const existing = this.cache.get(key)!;
      this.currentSize -= existing.size;
      const idx = this.order.indexOf(key);
      if (idx > -1) this.order.splice(idx, 1);
    }

    // Evict until we have space
    while (this.currentSize + sizeBytes > this.maxSize && this.order.length > 0) {
      const evictKey = this.order.shift()!;
      const evicted = this.cache.get(evictKey);
      if (evicted) {
        this.currentSize -= evicted.size;
        this.cache.delete(evictKey);
      }
    }

    this.cache.set(key, { value, size: sizeBytes });
    this.order.push(key);
    this.currentSize += sizeBytes;
  }

  delete(key: K): boolean {
    const entry = this.cache.get(key);
    if (!entry) return false;
    this.currentSize -= entry.size;
    this.cache.delete(key);
    const idx = this.order.indexOf(key);
    if (idx > -1) this.order.splice(idx, 1);
    return true;
  }

  get size(): number { return this.cache.size; }
  get usedBytes(): number { return this.currentSize; }
  get utilization(): number { return this.currentSize / this.maxSize; }

  clear(): void {
    this.cache.clear();
    this.order = [];
    this.currentSize = 0;
  }
}

// ═══════════════════════════════════════════════════════════
// 4. LATENCY BUDGETS PER HOP
// ═══════════════════════════════════════════════════════════

export interface LatencyBudget {
  path: string;
  hops: LatencyHop[];
  total_budget_ms: number;
  slo_p50_ms: number;
  slo_p99_ms: number;
}

export interface LatencyHop {
  name: string;
  budget_ms: number;
  type: 'compute' | 'io' | 'network' | 'queue';
}

export const LATENCY_BUDGETS: LatencyBudget[] = [
  {
    path: 'position_fix',
    total_budget_ms: 1000,
    slo_p50_ms: 200,
    slo_p99_ms: 800,
    hops: [
      { name: 'gnss_decode', budget_ms: 50, type: 'compute' },
      { name: 'sensor_read', budget_ms: 10, type: 'io' },
      { name: 'eskf_predict', budget_ms: 20, type: 'compute' },
      { name: 'eskf_update', budget_ms: 30, type: 'compute' },
      { name: 'map_match', budget_ms: 100, type: 'compute' },
      { name: 'urban_canyon_correction', budget_ms: 50, type: 'compute' },
      { name: 'integrity_check', budget_ms: 20, type: 'compute' },
      { name: 'event_publish', budget_ms: 5, type: 'queue' },
      { name: 'headroom', budget_ms: 715, type: 'compute' },
    ],
  },
  {
    path: 'reroute',
    total_budget_ms: 500,
    slo_p50_ms: 150,
    slo_p99_ms: 400,
    hops: [
      { name: 'deviation_detect', budget_ms: 10, type: 'compute' },
      { name: 'graph_load', budget_ms: 20, type: 'io' },
      { name: 'dijkstra_compute', budget_ms: 100, type: 'compute' },
      { name: 'policy_evaluate', budget_ms: 30, type: 'compute' },
      { name: 'eta_compute', budget_ms: 20, type: 'compute' },
      { name: 'alternatives_rank', budget_ms: 30, type: 'compute' },
      { name: 'event_publish', budget_ms: 5, type: 'queue' },
      { name: 'headroom', budget_ms: 285, type: 'compute' },
    ],
  },
  {
    path: 'alert_dispatch',
    total_budget_ms: 300,
    slo_p50_ms: 50,
    slo_p99_ms: 250,
    hops: [
      { name: 'alert_evaluate', budget_ms: 10, type: 'compute' },
      { name: 'template_render', budget_ms: 20, type: 'compute' },
      { name: 'channel_select', budget_ms: 5, type: 'compute' },
      { name: 'queue_enqueue', budget_ms: 5, type: 'queue' },
      { name: 'api_call', budget_ms: 200, type: 'network' },
      { name: 'headroom', budget_ms: 60, type: 'compute' },
    ],
  },
  {
    path: 'incident_report',
    total_budget_ms: 2000,
    slo_p50_ms: 300,
    slo_p99_ms: 1500,
    hops: [
      { name: 'validate_input', budget_ms: 10, type: 'compute' },
      { name: 'geo_cluster', budget_ms: 50, type: 'compute' },
      { name: 'bayesian_fusion', budget_ms: 100, type: 'compute' },
      { name: 'trust_evaluate', budget_ms: 50, type: 'compute' },
      { name: 'db_write', budget_ms: 100, type: 'io' },
      { name: 'event_publish', budget_ms: 10, type: 'queue' },
      { name: 'affected_routes', budget_ms: 200, type: 'compute' },
      { name: 'headroom', budget_ms: 1480, type: 'compute' },
    ],
  },
  {
    path: 'ml_prediction',
    total_budget_ms: 500,
    slo_p50_ms: 100,
    slo_p99_ms: 400,
    hops: [
      { name: 'feature_extract', budget_ms: 30, type: 'compute' },
      { name: 'model_inference', budget_ms: 200, type: 'compute' },
      { name: 'result_validate', budget_ms: 20, type: 'compute' },
      { name: 'cache_update', budget_ms: 10, type: 'io' },
      { name: 'headroom', budget_ms: 240, type: 'compute' },
    ],
  },
];

// ═══════════════════════════════════════════════════════════
// 5. CACHE HIERARCHY
// ═══════════════════════════════════════════════════════════

export interface CacheLayer {
  name: string;
  level: 'L1' | 'L2' | 'L3';
  type: 'memory' | 'indexeddb' | 'disk' | 'cdn';
  capacity_mb: number;
  ttl_seconds: number;
  eviction: 'lru' | 'lfu' | 'ttl';
  hit_rate_target: number;
}

export const CACHE_HIERARCHY: CacheLayer[] = [
  // L1: In-memory hot cache
  { name: 'position-hot', level: 'L1', type: 'memory', capacity_mb: 2, ttl_seconds: 30, eviction: 'lru', hit_rate_target: 0.95 },
  { name: 'tile-hot', level: 'L1', type: 'memory', capacity_mb: 32, ttl_seconds: 300, eviction: 'lru', hit_rate_target: 0.90 },
  { name: 'route-hot', level: 'L1', type: 'memory', capacity_mb: 8, ttl_seconds: 60, eviction: 'lru', hit_rate_target: 0.85 },
  { name: 'incident-hot', level: 'L1', type: 'memory', capacity_mb: 4, ttl_seconds: 120, eviction: 'lru', hit_rate_target: 0.80 },
  // L2: IndexedDB persistent cache
  { name: 'tile-warm', level: 'L2', type: 'indexeddb', capacity_mb: 256, ttl_seconds: 86400, eviction: 'lru', hit_rate_target: 0.95 },
  { name: 'route-warm', level: 'L2', type: 'indexeddb', capacity_mb: 32, ttl_seconds: 3600, eviction: 'lru', hit_rate_target: 0.80 },
  { name: 'offline-data', level: 'L2', type: 'indexeddb', capacity_mb: 512, ttl_seconds: 604800, eviction: 'lru', hit_rate_target: 0.99 },
  // L3: CDN edge cache
  { name: 'tile-cdn', level: 'L3', type: 'cdn', capacity_mb: 0, ttl_seconds: 86400, eviction: 'ttl', hit_rate_target: 0.98 },
  { name: 'static-cdn', level: 'L3', type: 'cdn', capacity_mb: 0, ttl_seconds: 2592000, eviction: 'ttl', hit_rate_target: 0.99 },
];

// ═══════════════════════════════════════════════════════════
// 6. BACKPRESSURE RULES
// ═══════════════════════════════════════════════════════════

export interface BackpressureRule {
  queue: string;
  /** Pressure threshold (0..1) to trigger */
  threshold: number;
  /** Action to take */
  action: 'throttle' | 'shed_load' | 'degrade' | 'circuit_break' | 'scale_up';
  /** Parameters for the action */
  params: Record<string, number>;
}

export const BACKPRESSURE_RULES: BackpressureRule[] = [
  // Position fixes — degrade to lower frequency
  { queue: 'position-fixes', threshold: 0.8, action: 'throttle', params: { rate_limit_hz: 5 } },
  { queue: 'position-fixes', threshold: 0.95, action: 'degrade', params: { skip_map_match: 1, skip_urban_canyon: 1 } },
  // Sensor readings — drop non-critical
  { queue: 'sensor-readings', threshold: 0.9, action: 'shed_load', params: { keep_ratio: 0.5 } },
  // Route requests — reject low-priority
  { queue: 'route-requests', threshold: 0.7, action: 'throttle', params: { max_concurrent: 2 } },
  { queue: 'route-requests', threshold: 0.9, action: 'shed_load', params: { min_priority: 1 } },
  // Incident reports — throttle
  { queue: 'incident-reports', threshold: 0.8, action: 'throttle', params: { rate_limit_per_user: 2 } },
  // Alert dispatch — never drop, circuit break instead
  { queue: 'alert-dispatch', threshold: 0.9, action: 'circuit_break', params: { cooldown_ms: 5000 } },
  // Telemetry — aggressive shedding
  { queue: 'telemetry-upload', threshold: 0.5, action: 'throttle', params: { batch_size: 200 } },
  { queue: 'telemetry-upload', threshold: 0.8, action: 'shed_load', params: { keep_ratio: 0.1 } },
  // Event bus — degrade
  { queue: 'event-bus', threshold: 0.7, action: 'throttle', params: { max_events_per_sec: 100 } },
  { queue: 'event-bus', threshold: 0.9, action: 'shed_load', params: { min_priority: 1 } },
];

/**
 * Backpressure controller
 */
export class BackpressureController {
  private rules: BackpressureRule[];
  private activeActions = new Map<string, string>();

  constructor(rules: BackpressureRule[] = BACKPRESSURE_RULES) {
    this.rules = rules;
  }

  evaluate(queueName: string, pressure: number): BackpressureRule[] {
    const triggered: BackpressureRule[] = [];
    for (const rule of this.rules) {
      if (rule.queue === queueName && pressure >= rule.threshold) {
        triggered.push(rule);
        this.activeActions.set(`${queueName}:${rule.action}`, rule.action);
      }
    }
    // Clear actions when pressure drops
    if (pressure < 0.5) {
      for (const key of Array.from(this.activeActions.keys())) {
        if (key.startsWith(queueName + ':')) {
          this.activeActions.delete(key);
        }
      }
    }
    return triggered;
  }

  getActiveActions(): Map<string, string> {
    return new Map(this.activeActions);
  }
}

// ═══════════════════════════════════════════════════════════
// 7. FAILURE ISOLATION BOUNDARIES
// ═══════════════════════════════════════════════════════════

export interface FailureDomain {
  name: string;
  components: string[];
  isolation_type: 'process' | 'thread' | 'circuit_breaker' | 'bulkhead';
  blast_radius: string[];
  recovery_strategy: 'restart' | 'failover' | 'degrade' | 'manual';
  recovery_time_target_ms: number;
  health_check_interval_ms: number;
}

export const FAILURE_DOMAINS: FailureDomain[] = [
  {
    name: 'positioning',
    components: ['gnss-processor', 'sensor-fusion', 'map-matcher', 'urban-canyon'],
    isolation_type: 'thread',
    blast_radius: ['routing', 'navigation'],
    recovery_strategy: 'restart',
    recovery_time_target_ms: 2000,
    health_check_interval_ms: 1000,
  },
  {
    name: 'routing',
    components: ['route-compute', 'reroute-engine', 'eta-engine', 'policy-engine'],
    isolation_type: 'thread',
    blast_radius: ['navigation'],
    recovery_strategy: 'restart',
    recovery_time_target_ms: 3000,
    health_check_interval_ms: 2000,
  },
  {
    name: 'network',
    components: ['api-client', 'websocket', 'tile-fetcher', 'telemetry-uploader'],
    isolation_type: 'circuit_breaker',
    blast_radius: ['sync', 'crowd', 'alerts'],
    recovery_strategy: 'failover',
    recovery_time_target_ms: 5000,
    health_check_interval_ms: 5000,
  },
  {
    name: 'storage',
    components: ['indexeddb', 'offline-store', 'cache-manager'],
    isolation_type: 'bulkhead',
    blast_radius: ['offline-sync', 'tile-cache'],
    recovery_strategy: 'degrade',
    recovery_time_target_ms: 10000,
    health_check_interval_ms: 10000,
  },
  {
    name: 'ml-inference',
    components: ['traffic-predictor', 'eta-corrector', 'incident-classifier'],
    isolation_type: 'thread',
    blast_radius: [],
    recovery_strategy: 'degrade',
    recovery_time_target_ms: 5000,
    health_check_interval_ms: 30000,
  },
  {
    name: 'ui-rendering',
    components: ['map-renderer', 'hud', 'panel-system', 'animation-engine'],
    isolation_type: 'process',
    blast_radius: [],
    recovery_strategy: 'restart',
    recovery_time_target_ms: 1000,
    health_check_interval_ms: 5000,
  },
];

// ═══════════════════════════════════════════════════════════
// 8. CIRCUIT BREAKER
// ═══════════════════════════════════════════════════════════

export type CircuitState = 'closed' | 'open' | 'half_open';

export interface CircuitBreakerConfig {
  name: string;
  failure_threshold: number;
  success_threshold: number;
  timeout_ms: number;
  half_open_max_calls: number;
}

export class CircuitBreaker {
  private state: CircuitState = 'closed';
  private failureCount = 0;
  private successCount = 0;
  private lastFailureTime = 0;
  private halfOpenCalls = 0;
  private readonly config: CircuitBreakerConfig;

  constructor(config: CircuitBreakerConfig) {
    this.config = config;
  }

  get currentState(): CircuitState { return this.state; }
  get failures(): number { return this.failureCount; }

  canExecute(): boolean {
    switch (this.state) {
      case 'closed':
        return true;
      case 'open': {
        const elapsed = Date.now() - this.lastFailureTime;
        if (elapsed >= this.config.timeout_ms) {
          this.state = 'half_open';
          this.halfOpenCalls = 0;
          return true;
        }
        return false;
      }
      case 'half_open':
        return this.halfOpenCalls < this.config.half_open_max_calls;
    }
  }

  recordSuccess(): void {
    switch (this.state) {
      case 'closed':
        this.failureCount = 0;
        break;
      case 'half_open':
        this.successCount++;
        if (this.successCount >= this.config.success_threshold) {
          this.state = 'closed';
          this.failureCount = 0;
          this.successCount = 0;
        }
        break;
    }
  }

  recordFailure(): void {
    this.failureCount++;
    this.lastFailureTime = Date.now();

    switch (this.state) {
      case 'closed':
        if (this.failureCount >= this.config.failure_threshold) {
          this.state = 'open';
        }
        break;
      case 'half_open':
        this.state = 'open';
        this.successCount = 0;
        break;
    }
  }

  reset(): void {
    this.state = 'closed';
    this.failureCount = 0;
    this.successCount = 0;
    this.halfOpenCalls = 0;
  }

  getMetrics(): { state: CircuitState; failures: number; successes: number; last_failure: number } {
    return {
      state: this.state,
      failures: this.failureCount,
      successes: this.successCount,
      last_failure: this.lastFailureTime,
    };
  }
}
