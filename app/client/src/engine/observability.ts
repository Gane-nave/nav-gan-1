/**
 * G.A.N.E — Observability Stack
 * ===============================
 * OpenTelemetry-compatible tracing, metrics, and logging infrastructure.
 * Provides distributed trace context propagation across all GANE engines.
 *
 * Spec refs:
 *   Doc-3 §7.2 "Observability Stack"
 *   Doc-4 §8.1 "Distributed Tracing"
 *
 * Architecture:
 *   - Span-based tracing with W3C TraceContext propagation
 *   - Metric collectors (counters, histograms, gauges)
 *   - Structured logging with trace correlation
 *   - Automatic context injection into tRPC/fetch calls
 *   - Batched export to backend collector
 */

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

export type SpanKind = 'internal' | 'client' | 'server' | 'producer' | 'consumer';
export type SpanStatus = 'ok' | 'error' | 'unset';

export interface SpanContext {
  traceId: string;     // 32-char hex
  spanId: string;      // 16-char hex
  traceFlags: number;  // 0 = not sampled, 1 = sampled
  traceState: string;  // W3C tracestate header value
}

export interface SpanAttributes {
  [key: string]: string | number | boolean | undefined;
}

export interface SpanEvent {
  name: string;
  timestamp: number;
  attributes?: SpanAttributes;
}

export interface Span {
  name: string;
  context: SpanContext;
  parentSpanId: string | null;
  kind: SpanKind;
  status: SpanStatus;
  statusMessage?: string;
  startTime: number;
  endTime: number;
  attributes: SpanAttributes;
  events: SpanEvent[];
  links: SpanContext[];
}

export interface ActiveSpan {
  name: string;
  context: SpanContext;
  parentSpanId: string | null;
  kind: SpanKind;
  startTime: number;
  attributes: SpanAttributes;
  events: SpanEvent[];
  links: SpanContext[];

  // Mutation methods
  setAttribute(key: string, value: string | number | boolean): void;
  addEvent(name: string, attributes?: SpanAttributes): void;
  setStatus(status: SpanStatus, message?: string): void;
  end(): void;
}

export type MetricType = 'counter' | 'histogram' | 'gauge';

export interface MetricPoint {
  name: string;
  type: MetricType;
  value: number;
  labels: Record<string, string>;
  timestamp: number;
}

export interface LogEntry {
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  traceId?: string;
  spanId?: string;
  timestamp: number;
  attributes?: SpanAttributes;
}

export interface ObservabilityConfig {
  serviceName: string;
  serviceVersion: string;
  environment: string;
  samplingRate: number;          // 0-1, fraction of traces to sample
  maxSpansPerTrace: number;      // prevent unbounded growth
  maxEventsPerSpan: number;
  exportIntervalMs: number;      // batch export interval
  exportEndpoint: string | null; // null = console only
  enableConsoleExport: boolean;
  enableMetrics: boolean;
  enableLogging: boolean;
  propagateContext: boolean;     // inject traceparent into fetch headers
}

// ═══════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════

function generateId(bytes: number): string {
  const arr = new Uint8Array(bytes);
  crypto.getRandomValues(arr);
  return Array.from(arr).map(b => b.toString(16).padStart(2, '0')).join('');
}

function generateTraceId(): string { return generateId(16); }
function generateSpanId(): string { return generateId(8); }

// ═══════════════════════════════════════════════════════════
// TRACER
// ═══════════════════════════════════════════════════════════

const DEFAULT_CONFIG: ObservabilityConfig = {
  serviceName: 'gane-client',
  serviceVersion: '3.0.0',
  environment: 'production',
  samplingRate: 0.1,
  maxSpansPerTrace: 128,
  maxEventsPerSpan: 32,
  exportIntervalMs: 30_000,
  exportEndpoint: null,
  enableConsoleExport: false,
  enableMetrics: true,
  enableLogging: true,
  propagateContext: true,
};

export class Tracer {
  private config: ObservabilityConfig;
  private completedSpans: Span[] = [];
  private activeSpans: Map<string, ActiveSpan> = new Map();
  private currentContext: SpanContext | null = null;
  private exportTimer: ReturnType<typeof setInterval> | null = null;

  // Metrics storage
  private counters: Map<string, { value: number; labels: Record<string, string> }> = new Map();
  private histograms: Map<string, { values: number[]; labels: Record<string, string> }> = new Map();
  private gauges: Map<string, { value: number; labels: Record<string, string> }> = new Map();

  // Log buffer
  private logBuffer: LogEntry[] = [];

  constructor(config: Partial<ObservabilityConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };

    // Start export timer
    if (this.config.exportIntervalMs > 0) {
      this.exportTimer = setInterval(() => this.flush(), this.config.exportIntervalMs);
    }
  }

  // ─── Span Management ───

  /**
   * Start a new span. If there's a current context, it becomes the parent.
   */
  startSpan(name: string, options: {
    kind?: SpanKind;
    attributes?: SpanAttributes;
    links?: SpanContext[];
    parentContext?: SpanContext | null;
  } = {}): ActiveSpan {
    const parentCtx = options.parentContext !== undefined ? options.parentContext : this.currentContext;
    const traceId = parentCtx?.traceId ?? generateTraceId();
    const spanId = generateSpanId();
    const sampled = Math.random() < this.config.samplingRate;

    const context: SpanContext = {
      traceId,
      spanId,
      traceFlags: sampled ? 1 : 0,
      traceState: '',
    };

    const span: ActiveSpan = {
      name,
      context,
      parentSpanId: parentCtx?.spanId ?? null,
      kind: options.kind ?? 'internal',
      startTime: performance.now(),
      attributes: { ...options.attributes },
      events: [],
      links: options.links ?? [],

      setAttribute: (key: string, value: string | number | boolean) => {
        span.attributes[key] = value;
      },

      addEvent: (eventName: string, attrs?: SpanAttributes) => {
        if (span.events.length < this.config.maxEventsPerSpan) {
          span.events.push({
            name: eventName,
            timestamp: performance.now(),
            attributes: attrs,
          });
        }
      },

      setStatus: (status: SpanStatus, message?: string) => {
        (span as any)._status = status;
        (span as any)._statusMessage = message;
      },

      end: () => {
        const completed: Span = {
          name: span.name,
          context: span.context,
          parentSpanId: span.parentSpanId,
          kind: span.kind,
          status: (span as any)._status ?? 'unset',
          statusMessage: (span as any)._statusMessage,
          startTime: span.startTime,
          endTime: performance.now(),
          attributes: span.attributes,
          events: span.events,
          links: span.links,
        };

        this.activeSpans.delete(spanId);
        this.completedSpans.push(completed);

        // Restore parent context
        if (parentCtx) {
          this.currentContext = parentCtx;
        } else {
          this.currentContext = null;
        }

        // Auto-export if buffer is large
        if (this.completedSpans.length >= this.config.maxSpansPerTrace) {
          this.flush();
        }
      },
    };

    this.activeSpans.set(spanId, span);
    this.currentContext = context;

    return span;
  }

  /**
   * Execute a function within a span context.
   * Automatically ends the span when the function completes.
   */
  async trace<T>(name: string, fn: (span: ActiveSpan) => Promise<T>, options?: {
    kind?: SpanKind;
    attributes?: SpanAttributes;
  }): Promise<T> {
    const span = this.startSpan(name, options);
    try {
      const result = await fn(span);
      span.setStatus('ok');
      return result;
    } catch (error) {
      span.setStatus('error', error instanceof Error ? error.message : String(error));
      span.addEvent('exception', {
        'exception.type': error instanceof Error ? error.constructor.name : 'Error',
        'exception.message': error instanceof Error ? error.message : String(error),
      });
      throw error;
    } finally {
      span.end();
    }
  }

  /**
   * Synchronous version of trace.
   */
  traceSync<T>(name: string, fn: (span: ActiveSpan) => T, options?: {
    kind?: SpanKind;
    attributes?: SpanAttributes;
  }): T {
    const span = this.startSpan(name, options);
    try {
      const result = fn(span);
      span.setStatus('ok');
      return result;
    } catch (error) {
      span.setStatus('error', error instanceof Error ? error.message : String(error));
      throw error;
    } finally {
      span.end();
    }
  }

  /**
   * Get current trace context for propagation.
   */
  getCurrentContext(): SpanContext | null {
    return this.currentContext;
  }

  /**
   * Generate W3C traceparent header value.
   */
  getTraceparent(): string | null {
    if (!this.currentContext) return null;
    const { traceId, spanId, traceFlags } = this.currentContext;
    return `00-${traceId}-${spanId}-${traceFlags.toString(16).padStart(2, '0')}`;
  }

  /**
   * Parse W3C traceparent header into SpanContext.
   */
  parseTraceparent(header: string): SpanContext | null {
    const parts = header.split('-');
    if (parts.length < 4) return null;
    return {
      traceId: parts[1],
      spanId: parts[2],
      traceFlags: parseInt(parts[3], 16),
      traceState: '',
    };
  }

  /**
   * Inject trace context into fetch headers.
   */
  injectHeaders(headers: Record<string, string> = {}): Record<string, string> {
    if (!this.config.propagateContext || !this.currentContext) return headers;
    const traceparent = this.getTraceparent();
    if (traceparent) {
      headers['traceparent'] = traceparent;
    }
    return headers;
  }

  // ─── Metrics ───

  /**
   * Increment a counter metric.
   */
  incrementCounter(name: string, value: number = 1, labels: Record<string, string> = {}) {
    if (!this.config.enableMetrics) return;
    const key = `${name}:${JSON.stringify(labels)}`;
    const existing = this.counters.get(key);
    if (existing) {
      existing.value += value;
    } else {
      this.counters.set(key, { value, labels });
    }
  }

  /**
   * Record a histogram value.
   */
  recordHistogram(name: string, value: number, labels: Record<string, string> = {}) {
    if (!this.config.enableMetrics) return;
    const key = `${name}:${JSON.stringify(labels)}`;
    const existing = this.histograms.get(key);
    if (existing) {
      existing.values.push(value);
      // Keep last 1000 values
      if (existing.values.length > 1000) {
        existing.values = existing.values.slice(-1000);
      }
    } else {
      this.histograms.set(key, { values: [value], labels });
    }
  }

  /**
   * Set a gauge value.
   */
  setGauge(name: string, value: number, labels: Record<string, string> = {}) {
    if (!this.config.enableMetrics) return;
    const key = `${name}:${JSON.stringify(labels)}`;
    this.gauges.set(key, { value, labels });
  }

  /**
   * Get all current metric points.
   */
  getMetrics(): MetricPoint[] {
    const now = Date.now();
    const points: MetricPoint[] = [];

    for (const [key, data] of Array.from(this.counters.entries())) {
      const name = key.split(':')[0];
      points.push({ name, type: 'counter', value: data.value, labels: data.labels, timestamp: now });
    }

    for (const [key, data] of Array.from(this.histograms.entries())) {
      const name = key.split(':')[0];
      const sorted = [...data.values].sort((a, b) => a - b);
      const p50 = sorted[Math.floor(sorted.length * 0.5)] ?? 0;
      const p95 = sorted[Math.floor(sorted.length * 0.95)] ?? 0;
      const p99 = sorted[Math.floor(sorted.length * 0.99)] ?? 0;
      const avg = sorted.length > 0 ? sorted.reduce((s, v) => s + v, 0) / sorted.length : 0;
      points.push({ name: `${name}.p50`, type: 'histogram', value: p50, labels: data.labels, timestamp: now });
      points.push({ name: `${name}.p95`, type: 'histogram', value: p95, labels: data.labels, timestamp: now });
      points.push({ name: `${name}.p99`, type: 'histogram', value: p99, labels: data.labels, timestamp: now });
      points.push({ name: `${name}.avg`, type: 'histogram', value: avg, labels: data.labels, timestamp: now });
      points.push({ name: `${name}.count`, type: 'histogram', value: sorted.length, labels: data.labels, timestamp: now });
    }

    for (const [key, data] of Array.from(this.gauges.entries())) {
      const name = key.split(':')[0];
      points.push({ name, type: 'gauge', value: data.value, labels: data.labels, timestamp: now });
    }

    return points;
  }

  // ─── Logging ───

  /**
   * Structured log with automatic trace correlation.
   */
  log(level: LogEntry['level'], message: string, attributes?: SpanAttributes) {
    if (!this.config.enableLogging) return;

    const entry: LogEntry = {
      level,
      message,
      traceId: this.currentContext?.traceId,
      spanId: this.currentContext?.spanId,
      timestamp: Date.now(),
      attributes,
    };

    this.logBuffer.push(entry);

    // Keep last 500 log entries
    if (this.logBuffer.length > 500) {
      this.logBuffer = this.logBuffer.slice(-500);
    }

    if (this.config.enableConsoleExport) {
      const prefix = entry.traceId ? `[${entry.traceId.slice(0, 8)}:${entry.spanId?.slice(0, 4)}]` : '';
      const logFn = level === 'error' ? console.error : level === 'warn' ? console.warn : level === 'debug' ? console.debug : console.log;
      logFn(`[GANE:${level.toUpperCase()}] ${prefix} ${message}`, attributes ?? '');
    }
  }

  debug(message: string, attributes?: SpanAttributes) { this.log('debug', message, attributes); }
  info(message: string, attributes?: SpanAttributes) { this.log('info', message, attributes); }
  warn(message: string, attributes?: SpanAttributes) { this.log('warn', message, attributes); }
  error(message: string, attributes?: SpanAttributes) { this.log('error', message, attributes); }

  /**
   * Get recent log entries.
   */
  getLogs(limit: number = 100): LogEntry[] {
    return this.logBuffer.slice(-limit);
  }

  // ─── Export ───

  /**
   * Flush completed spans and metrics to the export endpoint.
   */
  async flush(): Promise<void> {
    if (this.completedSpans.length === 0) return;

    const spans = [...this.completedSpans];
    this.completedSpans = [];

    if (this.config.exportEndpoint) {
      try {
        await fetch(this.config.exportEndpoint, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...this.injectHeaders(),
          },
          body: JSON.stringify({
            resourceSpans: [{
              resource: {
                attributes: {
                  'service.name': this.config.serviceName,
                  'service.version': this.config.serviceVersion,
                  'deployment.environment': this.config.environment,
                },
              },
              scopeSpans: [{
                scope: { name: 'gane-tracer', version: '1.0.0' },
                spans,
              }],
            }],
          }),
        });
      } catch {
        // Re-add spans if export fails
        this.completedSpans.unshift(...spans);
      }
    }
  }

  /**
   * Get trace summary for debugging.
   */
  getTraceSummary(): {
    activeSpans: number;
    completedSpans: number;
    totalMetrics: number;
    logEntries: number;
    currentTraceId: string | null;
  } {
    return {
      activeSpans: this.activeSpans.size,
      completedSpans: this.completedSpans.length,
      totalMetrics: this.counters.size + this.histograms.size + this.gauges.size,
      logEntries: this.logBuffer.length,
      currentTraceId: this.currentContext?.traceId ?? null,
    };
  }

  /**
   * Reset all state.
   */
  reset() {
    this.completedSpans = [];
    this.activeSpans.clear();
    this.currentContext = null;
    this.counters.clear();
    this.histograms.clear();
    this.gauges.clear();
    this.logBuffer = [];
  }

  /**
   * Destroy the tracer and clean up.
   */
  destroy() {
    if (this.exportTimer) {
      clearInterval(this.exportTimer);
      this.exportTimer = null;
    }
    this.flush();
    this.reset();
  }
}

// ═══════════════════════════════════════════════════════════
// SERVER-SIDE TRACING MIDDLEWARE
// ═══════════════════════════════════════════════════════════

/**
 * Extract trace context from incoming request headers.
 */
export function extractTraceContext(headers: Record<string, string | string[] | undefined>): SpanContext | null {
  const traceparent = typeof headers['traceparent'] === 'string' ? headers['traceparent'] : null;
  if (!traceparent) return null;

  const parts = traceparent.split('-');
  if (parts.length < 4) return null;

  return {
    traceId: parts[1],
    spanId: parts[2],
    traceFlags: parseInt(parts[3], 16),
    traceState: typeof headers['tracestate'] === 'string' ? headers['tracestate'] : '',
  };
}

/**
 * Create traceparent header from context.
 */
export function createTraceparent(ctx: SpanContext): string {
  return `00-${ctx.traceId}-${ctx.spanId}-${ctx.traceFlags.toString(16).padStart(2, '0')}`;
}

// ═══════════════════════════════════════════════════════════
// SINGLETON INSTANCE
// ═══════════════════════════════════════════════════════════

let _globalTracer: Tracer | null = null;

/**
 * Get or create the global tracer instance.
 */
export function getTracer(config?: Partial<ObservabilityConfig>): Tracer {
  if (!_globalTracer) {
    _globalTracer = new Tracer(config);
  }
  return _globalTracer;
}

/**
 * Convenience: start a span on the global tracer.
 */
export function startSpan(name: string, options?: Parameters<Tracer['startSpan']>[1]): ActiveSpan {
  return getTracer().startSpan(name, options);
}

/**
 * Convenience: trace an async function on the global tracer.
 */
export function trace<T>(name: string, fn: (span: ActiveSpan) => Promise<T>, options?: {
  kind?: SpanKind;
  attributes?: SpanAttributes;
}): Promise<T> {
  return getTracer().trace(name, fn, options);
}

/**
 * Convenience: increment a counter on the global tracer.
 */
export function incrementCounter(name: string, value?: number, labels?: Record<string, string>) {
  getTracer().incrementCounter(name, value, labels);
}

/**
 * Convenience: record a histogram value on the global tracer.
 */
export function recordHistogram(name: string, value: number, labels?: Record<string, string>) {
  getTracer().recordHistogram(name, value, labels);
}
