/**
 * G.A.N.E — Server-Side Observability Middleware
 * ================================================
 * OpenTelemetry-compatible tracing middleware for tRPC procedures.
 * Provides distributed trace context propagation and metrics collection.
 *
 * Spec refs:
 *   Doc-3 §7.2 "Observability Stack"
 *   Doc-4 §8.1 "Distributed Tracing"
 */

import { publicProcedure } from "../_core/trpc";
import { z } from "zod";

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

interface SpanContext {
  traceId: string;
  spanId: string;
  traceFlags: number;
  traceState: string;
}

interface ServerSpan {
  name: string;
  traceId: string;
  spanId: string;
  parentSpanId: string | null;
  startTime: number;
  endTime: number;
  status: 'ok' | 'error' | 'unset';
  statusMessage?: string;
  attributes: Record<string, string | number | boolean>;
  events: { name: string; timestamp: number; attributes?: Record<string, string | number | boolean> }[];
}

// ═══════════════════════════════════════════════════════════
// SERVER TRACER
// ═══════════════════════════════════════════════════════════

function generateHexId(bytes: number): string {
  const chars = '0123456789abcdef';
  let result = '';
  for (let i = 0; i < bytes * 2; i++) {
    result += chars[Math.floor(Math.random() * 16)];
  }
  return result;
}

class ServerTracer {
  private spans: ServerSpan[] = [];
  private metrics: Map<string, { type: 'counter' | 'gauge' | 'histogram'; value: number; labels: Record<string, string> }> = new Map();
  private maxSpans = 10_000;

  /**
   * Extract trace context from incoming request headers.
   */
  extractContext(traceparent?: string): SpanContext | null {
    if (!traceparent) return null;
    const parts = traceparent.split('-');
    if (parts.length < 4) return null;
    return {
      traceId: parts[1],
      spanId: parts[2],
      traceFlags: parseInt(parts[3], 16),
      traceState: '',
    };
  }

  /**
   * Start a server span.
   */
  startSpan(name: string, parentContext?: SpanContext | null): {
    traceId: string;
    spanId: string;
    end: (status?: 'ok' | 'error', message?: string) => void;
    setAttribute: (key: string, value: string | number | boolean) => void;
    addEvent: (eventName: string, attributes?: Record<string, string | number | boolean>) => void;
  } {
    const traceId = parentContext?.traceId ?? generateHexId(16);
    const spanId = generateHexId(8);
    const startTime = Date.now();
    const attributes: Record<string, string | number | boolean> = {};
    const events: { name: string; timestamp: number; attributes?: Record<string, string | number | boolean> }[] = [];

    return {
      traceId,
      spanId,
      setAttribute: (key: string, value: string | number | boolean) => {
        attributes[key] = value;
      },
      addEvent: (eventName: string, attrs?: Record<string, string | number | boolean>) => {
        events.push({ name: eventName, timestamp: Date.now(), attributes: attrs });
      },
      end: (status: 'ok' | 'error' = 'ok', message?: string) => {
        const span: ServerSpan = {
          name,
          traceId,
          spanId,
          parentSpanId: parentContext?.spanId ?? null,
          startTime,
          endTime: Date.now(),
          status,
          statusMessage: message,
          attributes,
          events,
        };
        this.spans.push(span);
        // Trim old spans
        if (this.spans.length > this.maxSpans) {
          this.spans = this.spans.slice(-this.maxSpans);
        }
      },
    };
  }

  /**
   * Increment a counter.
   */
  incrementCounter(name: string, value: number = 1, labels: Record<string, string> = {}) {
    const key = `counter:${name}:${JSON.stringify(labels)}`;
    const existing = this.metrics.get(key);
    if (existing) {
      existing.value += value;
    } else {
      this.metrics.set(key, { type: 'counter', value, labels });
    }
  }

  /**
   * Set a gauge.
   */
  setGauge(name: string, value: number, labels: Record<string, string> = {}) {
    const key = `gauge:${name}:${JSON.stringify(labels)}`;
    this.metrics.set(key, { type: 'gauge', value, labels });
  }

  /**
   * Get recent spans.
   */
  getRecentSpans(limit: number = 100): ServerSpan[] {
    return this.spans.slice(-limit);
  }

  /**
   * Get spans for a specific trace.
   */
  getTraceSpans(traceId: string): ServerSpan[] {
    return this.spans.filter(s => s.traceId === traceId);
  }

  /**
   * Get all metrics.
   */
  getMetrics(): { name: string; type: string; value: number; labels: Record<string, string> }[] {
    const result: { name: string; type: string; value: number; labels: Record<string, string> }[] = [];
    for (const [key, data] of Array.from(this.metrics.entries())) {
      const parts = key.split(':');
      result.push({
        name: parts[1],
        type: data.type,
        value: data.value,
        labels: data.labels,
      });
    }
    return result;
  }

  /**
   * Get summary stats.
   */
  getSummary(): {
    totalSpans: number;
    errorSpans: number;
    avgDurationMs: number;
    totalMetrics: number;
    traceIds: number;
  } {
    const errorSpans = this.spans.filter(s => s.status === 'error').length;
    const durations = this.spans.map(s => s.endTime - s.startTime);
    const avgDuration = durations.length > 0 ? durations.reduce((a, b) => a + b, 0) / durations.length : 0;
    const uniqueTraces = new Set(this.spans.map(s => s.traceId));

    return {
      totalSpans: this.spans.length,
      errorSpans,
      avgDurationMs: Math.round(avgDuration * 100) / 100,
      totalMetrics: this.metrics.size,
      traceIds: uniqueTraces.size,
    };
  }

  /**
   * Reset all data.
   */
  reset() {
    this.spans = [];
    this.metrics.clear();
  }
}

// Singleton
const serverTracer = new ServerTracer();

export { serverTracer, ServerTracer };

// ═══════════════════════════════════════════════════════════
// tRPC ROUTER
// ═══════════════════════════════════════════════════════════

import { router } from "../_core/trpc";

export const observabilityRouter = router({
  /**
   * Ingest spans from the client-side tracer.
   */
  ingestSpans: publicProcedure
    .input(z.object({
      resourceSpans: z.array(z.object({
        resource: z.object({
          attributes: z.record(z.string(), z.unknown()),
        }),
        scopeSpans: z.array(z.object({
          scope: z.object({ name: z.string(), version: z.string() }),
          spans: z.array(z.object({
            name: z.string(),
            context: z.object({
              traceId: z.string(),
              spanId: z.string(),
              traceFlags: z.number(),
              traceState: z.string(),
            }),
            parentSpanId: z.string().nullable(),
            kind: z.string(),
            status: z.string(),
            statusMessage: z.string().optional(),
            startTime: z.number(),
            endTime: z.number(),
            attributes: z.record(z.string(), z.unknown()),
            events: z.array(z.object({
              name: z.string(),
              timestamp: z.number(),
              attributes: z.record(z.string(), z.unknown()).optional(),
            })),
            links: z.array(z.unknown()),
          })),
        })),
      })),
    }))
    .mutation(async ({ input }) => {
      let ingested = 0;
      for (const rs of input.resourceSpans) {
        for (const ss of rs.scopeSpans) {
          for (const span of ss.spans) {
            const serverSpan: ServerSpan = {
              name: span.name,
              traceId: span.context.traceId,
              spanId: span.context.spanId,
              parentSpanId: span.parentSpanId,
              startTime: span.startTime,
              endTime: span.endTime,
              status: span.status as 'ok' | 'error' | 'unset',
              statusMessage: span.statusMessage,
              attributes: span.attributes as Record<string, string | number | boolean>,
              events: span.events.map(e => ({
                name: e.name,
                timestamp: e.timestamp,
                attributes: e.attributes as Record<string, string | number | boolean> | undefined,
              })),
            };
            // Store via tracer
            const s = serverTracer.startSpan(serverSpan.name, {
              traceId: serverSpan.traceId,
              spanId: serverSpan.parentSpanId ?? serverSpan.spanId,
              traceFlags: 1,
              traceState: '',
            });
            for (const [k, v] of Object.entries(serverSpan.attributes)) {
              if (typeof v === 'string' || typeof v === 'number' || typeof v === 'boolean') {
                s.setAttribute(k, v);
              }
            }
            s.end(serverSpan.status === 'unset' ? 'ok' : serverSpan.status, serverSpan.statusMessage);
            ingested++;
          }
        }
      }
      serverTracer.incrementCounter('spans.ingested', ingested);
      return { ingested };
    }),

  /**
   * Get recent spans for the observability dashboard.
   */
  recentSpans: publicProcedure
    .input(z.object({
      limit: z.number().min(1).max(500).default(100),
      traceId: z.string().optional(),
    }))
    .query(({ input }) => {
      if (input.traceId) {
        return serverTracer.getTraceSpans(input.traceId);
      }
      return serverTracer.getRecentSpans(input.limit);
    }),

  /**
   * Get metrics summary.
   */
  metrics: publicProcedure.query(() => {
    return {
      summary: serverTracer.getSummary(),
      metrics: serverTracer.getMetrics(),
    };
  }),
});
