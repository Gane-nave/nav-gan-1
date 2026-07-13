/**
 * G.A.N.E — Prometheus Metrics Collector
 * ========================================
 * Exposes system and collaboration metrics for Prometheus scraping.
 *
 * Metrics exposed:
 * - HTTP request duration & count (by method, route, status)
 * - WebSocket connections (active, total, by type)
 * - Collaboration sessions (active, participants, events/sec)
 * - Rate limiter stats (allowed, blocked, by endpoint)
 * - Redis Pub/Sub health (mode, channels, handlers, ping latency)
 * - Node.js runtime (memory, event loop lag, CPU)
 *
 * Endpoint: GET /metrics (Prometheus text format)
 */
import {
  Registry,
  Counter,
  Histogram,
  Gauge,
  Summary,
  collectDefaultMetrics,
} from "prom-client";
import type { Request, Response, NextFunction } from "express";
import { pubsub } from "./redisPubSub";
import { getCollabWSStats } from "./collabWsHandler";

// ─── Registry ───
const register = new Registry();

// Collect default Node.js metrics (memory, CPU, event loop, GC)
collectDefaultMetrics({
  register,
  prefix: "gane_",
  labels: { app: "gane" },
});

// ═══════════════════════════════════════════════════════════
// HTTP Metrics
// ═══════════════════════════════════════════════════════════

export const httpRequestDuration = new Histogram({
  name: "gane_http_request_duration_seconds",
  help: "Duration of HTTP requests in seconds",
  labelNames: ["method", "route", "status_code"] as const,
  buckets: [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10],
  registers: [register],
});

export const httpRequestTotal = new Counter({
  name: "gane_http_requests_total",
  help: "Total number of HTTP requests",
  labelNames: ["method", "route", "status_code"] as const,
  registers: [register],
});

export const httpActiveRequests = new Gauge({
  name: "gane_http_active_requests",
  help: "Number of currently active HTTP requests",
  registers: [register],
});

// ═══════════════════════════════════════════════════════════
// WebSocket Metrics
// ═══════════════════════════════════════════════════════════

export const wsConnectionsActive = new Gauge({
  name: "gane_ws_connections_active",
  help: "Number of active WebSocket connections",
  labelNames: ["type"] as const,
  registers: [register],
});

export const wsConnectionsTotal = new Counter({
  name: "gane_ws_connections_total",
  help: "Total WebSocket connections opened",
  labelNames: ["type"] as const,
  registers: [register],
});

export const wsDisconnectsTotal = new Counter({
  name: "gane_ws_disconnects_total",
  help: "Total WebSocket disconnections",
  labelNames: ["type", "reason"] as const,
  registers: [register],
});

export const wsMessagesReceived = new Counter({
  name: "gane_ws_messages_received_total",
  help: "Total WebSocket messages received",
  labelNames: ["type", "message_type"] as const,
  registers: [register],
});

export const wsMessagesSent = new Counter({
  name: "gane_ws_messages_sent_total",
  help: "Total WebSocket messages sent",
  labelNames: ["type"] as const,
  registers: [register],
});

// ═══════════════════════════════════════════════════════════
// Collaboration Metrics
// ═══════════════════════════════════════════════════════════

export const collabSessionsActive = new Gauge({
  name: "gane_collab_sessions_active",
  help: "Number of active collaboration sessions",
  registers: [register],
});

export const collabParticipantsActive = new Gauge({
  name: "gane_collab_participants_active",
  help: "Number of active collaboration participants",
  registers: [register],
});

export const collabEventsTotal = new Counter({
  name: "gane_collab_events_total",
  help: "Total collaboration events emitted",
  labelNames: ["event_type"] as const,
  registers: [register],
});

export const collabCursorUpdatesTotal = new Counter({
  name: "gane_collab_cursor_updates_total",
  help: "Total cursor position updates",
  registers: [register],
});

export const collabMarkersTotal = new Counter({
  name: "gane_collab_markers_total",
  help: "Total markers created",
  registers: [register],
});

export const collabAnnotationsTotal = new Counter({
  name: "gane_collab_annotations_total",
  help: "Total annotations created",
  registers: [register],
});

// ═══════════════════════════════════════════════════════════
// Rate Limiter Metrics
// ═══════════════════════════════════════════════════════════

export const rateLimitAllowed = new Counter({
  name: "gane_rate_limit_allowed_total",
  help: "Total requests allowed by rate limiter",
  labelNames: ["endpoint"] as const,
  registers: [register],
});

export const rateLimitBlocked = new Counter({
  name: "gane_rate_limit_blocked_total",
  help: "Total requests blocked by rate limiter",
  labelNames: ["endpoint"] as const,
  registers: [register],
});

// ═══════════════════════════════════════════════════════════
// Redis Pub/Sub Metrics
// ═══════════════════════════════════════════════════════════

export const redisPubSubMode = new Gauge({
  name: "gane_redis_pubsub_mode",
  help: "Redis Pub/Sub mode (1=redis, 0=memory fallback)",
  registers: [register],
});

export const redisPubSubChannels = new Gauge({
  name: "gane_redis_pubsub_channels",
  help: "Number of subscribed Redis Pub/Sub channels",
  registers: [register],
});

export const redisPubSubHandlers = new Gauge({
  name: "gane_redis_pubsub_handlers",
  help: "Total number of registered Pub/Sub handlers",
  registers: [register],
});

export const redisPingLatency = new Gauge({
  name: "gane_redis_ping_latency_ms",
  help: "Redis ping latency in milliseconds",
  registers: [register],
});

// ═══════════════════════════════════════════════════════════
// Middleware: HTTP Request Instrumentation
// ═══════════════════════════════════════════════════════════

export function metricsMiddleware(req: Request, res: Response, next: NextFunction): void {
  // Skip metrics endpoint itself to avoid recursion
  if (req.path === "/metrics") {
    next();
    return;
  }

  httpActiveRequests.inc();
  const end = httpRequestDuration.startTimer();

  res.on("finish", () => {
    httpActiveRequests.dec();

    // Normalize route to avoid high cardinality
    const route = normalizeRoute(req.path);
    const labels = {
      method: req.method,
      route,
      status_code: String(res.statusCode),
    };

    end(labels);
    httpRequestTotal.inc(labels);
  });

  next();
}

/**
 * Normalize route paths to prevent high-cardinality metric labels.
 * Replaces UUIDs, numeric IDs, and session IDs with placeholders.
 */
function normalizeRoute(path: string): string {
  return path
    .replace(/\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/gi, "/:uuid")
    .replace(/\/\d+/g, "/:id")
    .replace(/\/sess_[a-z0-9]+/gi, "/:sessionId");
}

// ═══════════════════════════════════════════════════════════
// Periodic Gauge Updates (Redis, WS stats)
// ═══════════════════════════════════════════════════════════

let gaugeUpdateInterval: ReturnType<typeof setInterval> | null = null;

export function startMetricsCollection(): void {
  // Update gauges every 15 seconds
  gaugeUpdateInterval = setInterval(() => {
    // Redis Pub/Sub stats
    const pubsubStats = pubsub.getStats();
    redisPubSubMode.set(pubsubStats.isRedisAvailable ? 1 : 0);
    redisPubSubChannels.set(pubsubStats.subscribedChannels);
    redisPubSubHandlers.set(pubsubStats.totalHandlers);
    if (pubsubStats.lastPingMs !== null) {
      redisPingLatency.set(pubsubStats.lastPingMs);
    }

    // WebSocket stats
    const wsStats = getCollabWSStats();
    wsConnectionsActive.set({ type: "collab" }, wsStats.totalConnections);
  }, 15_000);

  if (gaugeUpdateInterval && typeof gaugeUpdateInterval === "object" && "unref" in gaugeUpdateInterval) {
    (gaugeUpdateInterval as NodeJS.Timeout).unref();
  }
}

export function stopMetricsCollection(): void {
  if (gaugeUpdateInterval) {
    clearInterval(gaugeUpdateInterval);
    gaugeUpdateInterval = null;
  }
}

// ═══════════════════════════════════════════════════════════
// Metrics Endpoint Handler
// ═══════════════════════════════════════════════════════════

export async function metricsHandler(_req: Request, res: Response): Promise<void> {
  try {
    res.set("Content-Type", register.contentType);
    const metrics = await register.metrics();
    res.end(metrics);
  } catch (err) {
    res.status(500).end("Error collecting metrics");
  }
}

// Export registry for testing
export { register };
