/**
 * G.A.N.E — OpenTelemetry Distributed Tracing
 * ==============================================
 * Initializes OpenTelemetry SDK with auto-instrumentation for HTTP, Express,
 * and custom spans for business-critical operations.
 *
 * Exports to Jaeger/Zipkin via OTLP HTTP protocol.
 * Falls back gracefully when no collector is available.
 *
 * Environment variables:
 *   OTEL_EXPORTER_OTLP_ENDPOINT — Collector endpoint (default: http://localhost:4318)
 *   OTEL_SERVICE_NAME — Service name (default: gane-app)
 *   OTEL_ENABLED — Enable/disable tracing (default: true)
 */
import { NodeSDK } from "@opentelemetry/sdk-node";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-http";
import { HttpInstrumentation } from "@opentelemetry/instrumentation-http";
import { ExpressInstrumentation } from "@opentelemetry/instrumentation-express";
import {
  BatchSpanProcessor,
  ConsoleSpanExporter,
  SimpleSpanProcessor,
} from "@opentelemetry/sdk-trace-base";
import { resourceFromAttributes } from "@opentelemetry/resources";
import {
  ATTR_SERVICE_NAME,
  ATTR_SERVICE_VERSION,
} from "@opentelemetry/semantic-conventions";
import { trace, SpanStatusCode, context, type Span, type Tracer } from "@opentelemetry/api";

// ─── Configuration ───
const OTEL_ENABLED = process.env.OTEL_ENABLED !== "false";
const OTEL_ENDPOINT =
  process.env.OTEL_EXPORTER_OTLP_ENDPOINT || "http://localhost:4318";
const SERVICE_NAME = process.env.OTEL_SERVICE_NAME || "gane-app";
const SERVICE_VERSION = process.env.npm_package_version || "1.0.0";
const ENVIRONMENT = process.env.NODE_ENV || "development";

let sdk: NodeSDK | null = null;

// ─── Initialize SDK ───
export function initTracing(): void {
  if (!OTEL_ENABLED) {
    console.log("[Tracing] OpenTelemetry disabled via OTEL_ENABLED=false");
    return;
  }

  try {
    const resource = resourceFromAttributes({
      [ATTR_SERVICE_NAME]: SERVICE_NAME,
      [ATTR_SERVICE_VERSION]: SERVICE_VERSION,
      "deployment.environment": ENVIRONMENT,
      "service.namespace": "gane",
      "service.instance.id": `${SERVICE_NAME}-${process.pid}`,
    });

    // OTLP exporter sends traces to Jaeger/Zipkin/any OTLP-compatible collector
    const otlpExporter = new OTLPTraceExporter({
      url: `${OTEL_ENDPOINT}/v1/traces`,
      headers: {},
      timeoutMillis: 5000,
    });

    // Use BatchSpanProcessor for production (buffers and sends in batches)
    const spanProcessor = new BatchSpanProcessor(otlpExporter, {
      maxQueueSize: 2048,
      maxExportBatchSize: 512,
      scheduledDelayMillis: 5000,
      exportTimeoutMillis: 30000,
    });

    sdk = new NodeSDK({
      resource,
      spanProcessors: [spanProcessor],
      instrumentations: [
        new HttpInstrumentation({
          // Ignore health checks and metrics to reduce noise
          ignoreIncomingRequestHook: (req) => {
            const url = req.url || "";
            return (
              url === "/metrics" ||
              url === "/api/health" ||
              url.startsWith("/api/collab/stream")
            );
          },
          // Add custom attributes to HTTP spans
          requestHook: (span, request) => {
            if ("headers" in request && request.headers) {
              const userAgent = request.headers["user-agent"];
              if (userAgent) {
                span.setAttribute("http.user_agent", String(userAgent));
              }
            }
          },
        }),
        new ExpressInstrumentation({
          // Capture route parameters
          requestHook: (span, info) => {
            if (info.request.route) {
              span.setAttribute("express.route", info.request.route.path);
            }
          },
        }),
      ],
    });

    sdk.start();
    console.log(
      `[Tracing] OpenTelemetry initialized — service: ${SERVICE_NAME}, exporter: ${OTEL_ENDPOINT}`
    );
  } catch (err) {
    console.warn(
      "[Tracing] Failed to initialize OpenTelemetry:",
      (err as Error).message
    );
    console.warn("[Tracing] Tracing will be disabled for this session.");
  }
}

// ─── Shutdown SDK ───
export async function shutdownTracing(): Promise<void> {
  if (sdk) {
    try {
      await sdk.shutdown();
      console.log("[Tracing] OpenTelemetry shut down gracefully");
    } catch (err) {
      console.warn(
        "[Tracing] Error during shutdown:",
        (err as Error).message
      );
    }
  }
}

// ═══════════════════════════════════════════════════════════
// Custom Span Helpers — Business-Critical Operations
// ═══════════════════════════════════════════════════════════

/**
 * Get a tracer for a specific component.
 */
export function getTracer(name: string): Tracer {
  return trace.getTracer(name, SERVICE_VERSION);
}

/**
 * Wrap an async function in a traced span.
 * Automatically records errors and sets span status.
 */
export async function withSpan<T>(
  tracerName: string,
  spanName: string,
  attributes: Record<string, string | number | boolean>,
  fn: (span: Span) => Promise<T>
): Promise<T> {
  const tracer = getTracer(tracerName);
  return tracer.startActiveSpan(spanName, async (span) => {
    try {
      // Set initial attributes
      for (const [key, value] of Object.entries(attributes)) {
        span.setAttribute(key, value);
      }

      const result = await fn(span);
      span.setStatus({ code: SpanStatusCode.OK });
      return result;
    } catch (err) {
      span.setStatus({
        code: SpanStatusCode.ERROR,
        message: (err as Error).message,
      });
      span.recordException(err as Error);
      throw err;
    } finally {
      span.end();
    }
  });
}

// ─── Pre-built Span Helpers for Common Operations ───

/**
 * Trace a tRPC procedure call.
 */
export async function traceRPC<T>(
  procedureName: string,
  userId: string | null,
  fn: (span: Span) => Promise<T>
): Promise<T> {
  return withSpan("gane.trpc", `trpc.${procedureName}`, {
    "rpc.method": procedureName,
    "rpc.system": "trpc",
    ...(userId ? { "enduser.id": userId } : {}),
  }, fn);
}

/**
 * Trace a database query.
 */
export async function traceDB<T>(
  operation: string,
  table: string,
  fn: (span: Span) => Promise<T>
): Promise<T> {
  return withSpan("gane.db", `db.${operation}.${table}`, {
    "db.system": "mysql",
    "db.operation": operation,
    "db.sql.table": table,
  }, fn);
}

/**
 * Trace a collaboration event.
 */
export async function traceCollabEvent<T>(
  eventType: string,
  sessionId: string,
  userId: string,
  fn: (span: Span) => Promise<T>
): Promise<T> {
  return withSpan("gane.collab", `collab.${eventType}`, {
    "collab.event_type": eventType,
    "collab.session_id": sessionId,
    "enduser.id": userId,
  }, fn);
}

/**
 * Trace a WebSocket message.
 */
export async function traceWSMessage<T>(
  messageType: string,
  connectionId: string,
  fn: (span: Span) => Promise<T>
): Promise<T> {
  return withSpan("gane.websocket", `ws.${messageType}`, {
    "ws.message_type": messageType,
    "ws.connection_id": connectionId,
    "messaging.system": "websocket",
  }, fn);
}

/**
 * Trace a Redis Pub/Sub operation.
 */
export async function traceRedis<T>(
  operation: string,
  channel: string,
  fn: (span: Span) => Promise<T>
): Promise<T> {
  return withSpan("gane.redis", `redis.${operation}`, {
    "db.system": "redis",
    "db.operation": operation,
    "messaging.destination": channel,
  }, fn);
}

/**
 * Add an event to the current active span (if any).
 */
export function addSpanEvent(
  name: string,
  attributes?: Record<string, string | number | boolean>
): void {
  const activeSpan = trace.getActiveSpan();
  if (activeSpan) {
    activeSpan.addEvent(name, attributes);
  }
}
