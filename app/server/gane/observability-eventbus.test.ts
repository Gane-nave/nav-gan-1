/**
 * Tests for Observability Stack and Event Bus
 */
import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { Tracer, extractTraceContext, createTraceparent } from "../../client/src/engine/observability";
import { EventBus, createLoggingMiddleware, createPriorityFilter, createRateLimiter } from "../../client/src/engine/eventBus";
import type { GANEEvent, LocationUpdateEvent } from "../../client/src/engine/eventBus";

// ═══════════════════════════════════════════════════════════
// OBSERVABILITY TRACER TESTS
// ═══════════════════════════════════════════════════════════

describe("Observability — Tracer", () => {
  let tracer: Tracer;

  beforeEach(() => {
    tracer = new Tracer({
      samplingRate: 1.0,
      enableConsoleExport: false,
      exportIntervalMs: 0, // disable auto-export
    });
  });

  afterEach(() => {
    tracer.destroy();
  });

  it("creates spans with valid trace context", () => {
    const span = tracer.startSpan("test-span");
    expect(span.context.traceId).toHaveLength(32);
    expect(span.context.spanId).toHaveLength(16);
    expect(span.name).toBe("test-span");
    span.end();
  });

  it("propagates trace ID to child spans", () => {
    const parent = tracer.startSpan("parent");
    const parentTraceId = parent.context.traceId;
    const parentSpanId = parent.context.spanId;

    const child = tracer.startSpan("child");
    expect(child.context.traceId).toBe(parentTraceId);
    expect(child.parentSpanId).toBe(parentSpanId);

    child.end();
    parent.end();
  });

  it("traces async functions and captures errors", async () => {
    await expect(
      tracer.trace("failing-op", async (span) => {
        span.setAttribute("key", "value");
        throw new Error("test error");
      })
    ).rejects.toThrow("test error");

    const summary = tracer.getTraceSummary();
    expect(summary.completedSpans).toBe(1);
  });

  it("traces sync functions", () => {
    const result = tracer.traceSync("sync-op", (span) => {
      span.setAttribute("count", 42);
      return "hello";
    });
    expect(result).toBe("hello");
  });

  it("generates W3C traceparent header", () => {
    const span = tracer.startSpan("test");
    const traceparent = tracer.getTraceparent();
    expect(traceparent).toMatch(/^00-[0-9a-f]{32}-[0-9a-f]{16}-0[01]$/);
    span.end();
  });

  it("parses traceparent header", () => {
    const ctx = tracer.parseTraceparent("00-abcdef1234567890abcdef1234567890-1234567890abcdef-01");
    expect(ctx).not.toBeNull();
    expect(ctx!.traceId).toBe("abcdef1234567890abcdef1234567890");
    expect(ctx!.spanId).toBe("1234567890abcdef");
    expect(ctx!.traceFlags).toBe(1);
  });

  it("increments counters", () => {
    tracer.incrementCounter("requests", 1, { method: "GET" });
    tracer.incrementCounter("requests", 1, { method: "GET" });
    tracer.incrementCounter("requests", 1, { method: "POST" });

    const metrics = tracer.getMetrics();
    const getCounter = metrics.find(m => m.name === "requests" && m.labels.method === "GET");
    expect(getCounter?.value).toBe(2);
  });

  it("records histogram values", () => {
    for (let i = 0; i < 10; i++) {
      tracer.recordHistogram("latency", i * 10, { endpoint: "/api" });
    }

    const metrics = tracer.getMetrics();
    const p50 = metrics.find(m => m.name === "latency.p50");
    expect(p50).toBeDefined();
    expect(p50!.value).toBeGreaterThanOrEqual(0);
  });

  it("sets gauge values", () => {
    tracer.setGauge("connections", 42);
    tracer.setGauge("connections", 50);

    const metrics = tracer.getMetrics();
    const gauge = metrics.find(m => m.name === "connections");
    expect(gauge?.value).toBe(50);
  });

  it("logs with trace correlation", () => {
    const span = tracer.startSpan("logged-op");
    tracer.info("test message", { key: "value" });
    span.end();

    const logs = tracer.getLogs();
    expect(logs.length).toBeGreaterThan(0);
    expect(logs[0].traceId).toBe(span.context.traceId);
  });

  it("injects headers for context propagation", () => {
    const span = tracer.startSpan("test");
    const headers = tracer.injectHeaders({});
    expect(headers.traceparent).toBeDefined();
    span.end();
  });

  it("resets all state", () => {
    tracer.startSpan("test").end();
    tracer.incrementCounter("test");
    tracer.info("test");
    tracer.reset();

    const summary = tracer.getTraceSummary();
    expect(summary.completedSpans).toBe(0);
    expect(summary.totalMetrics).toBe(0);
    expect(summary.logEntries).toBe(0);
  });
});

describe("Observability — extractTraceContext", () => {
  it("extracts context from valid traceparent header", () => {
    const ctx = extractTraceContext({
      traceparent: "00-abcdef1234567890abcdef1234567890-1234567890abcdef-01",
    });
    expect(ctx).not.toBeNull();
    expect(ctx!.traceId).toBe("abcdef1234567890abcdef1234567890");
  });

  it("returns null for missing traceparent", () => {
    const ctx = extractTraceContext({});
    expect(ctx).toBeNull();
  });
});

describe("Observability — createTraceparent", () => {
  it("creates valid traceparent string", () => {
    const tp = createTraceparent({
      traceId: "abcdef1234567890abcdef1234567890",
      spanId: "1234567890abcdef",
      traceFlags: 1,
      traceState: "",
    });
    expect(tp).toBe("00-abcdef1234567890abcdef1234567890-1234567890abcdef-01");
  });
});

// ═══════════════════════════════════════════════════════════
// EVENT BUS TESTS
// ═══════════════════════════════════════════════════════════

describe("EventBus", () => {
  let bus: EventBus;

  beforeEach(() => {
    bus = new EventBus({
      enableReplay: true,
      enableDeadLetter: true,
      enableMetrics: true,
      enableOfflineQueue: false,
      deduplicationWindowMs: 60_000,
    });
  });

  afterEach(() => {
    bus.destroy();
  });

  it("emits and receives events", () => {
    let received: any = null;
    bus.on("location.update", (event) => {
      received = event;
    });

    bus.emit("location.update", {
      lat: 32.0, lon: 34.0, accuracy: 5, speed: 10, heading: 90, altitude: 50,
    });

    expect(received).not.toBeNull();
    expect(received.payload.lat).toBe(32.0);
  });

  it("deduplicates events by ID", () => {
    let count = 0;
    bus.on("location.update", () => { count++; });

    const event: GANEEvent = {
      id: "test-dedup-id",
      type: "location.update",
      timestamp: Date.now(),
      source: "test",
      priority: "normal",
      payload: { lat: 32, lon: 34, accuracy: 5, speed: 0, heading: 0, altitude: 0 },
    } as LocationUpdateEvent;

    bus.publish(event);
    bus.publish(event); // duplicate

    expect(count).toBe(1);
  });

  it("supports wildcard handlers", () => {
    let count = 0;
    bus.onAll(() => { count++; });

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });
    bus.emit("eta.update", { routeId: "r1", etaMs: 1000, confidencePercent: 90, delayMs: 0 });

    expect(count).toBe(2);
  });

  it("supports once handlers", () => {
    let count = 0;
    bus.once("location.update", () => { count++; });

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });
    bus.emit("location.update", { lat: 1, lon: 1, accuracy: 0, speed: 0, heading: 0, altitude: 0 });

    expect(count).toBe(1);
  });

  it("supports filtered handlers", () => {
    let received: any = null;
    bus.onFiltered(
      "location.update",
      (event) => event.payload.speed > 50,
      (event) => { received = event; }
    );

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 10, heading: 0, altitude: 0 });
    expect(received).toBeNull();

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 60, heading: 0, altitude: 0 });
    expect(received).not.toBeNull();
    expect(received.payload.speed).toBe(60);
  });

  it("unsubscribes correctly", () => {
    let count = 0;
    const sub = bus.on("location.update", () => { count++; });

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });
    sub.unsubscribe();
    bus.emit("location.update", { lat: 1, lon: 1, accuracy: 0, speed: 0, heading: 0, altitude: 0 });

    expect(count).toBe(1);
  });

  it("replays events from buffer", () => {
    bus.emit("location.update", { lat: 1, lon: 1, accuracy: 0, speed: 0, heading: 0, altitude: 0 });
    bus.emit("location.update", { lat: 2, lon: 2, accuracy: 0, speed: 0, heading: 0, altitude: 0 });
    bus.emit("eta.update", { routeId: "r1", etaMs: 1000, confidencePercent: 90, delayMs: 0 });

    const locationEvents = bus.replay({ type: "location.update" });
    expect(locationEvents.length).toBe(2);

    const allEvents = bus.replay();
    expect(allEvents.length).toBe(3);
  });

  it("captures dead letters from failing handlers", () => {
    bus.on("location.update", () => {
      throw new Error("handler failed");
    });

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });

    const deadLetters = bus.getDeadLetters();
    expect(deadLetters.length).toBe(1);
    expect(deadLetters[0].error).toBe("handler failed");
  });

  it("tracks metrics", () => {
    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });
    bus.emit("location.update", { lat: 1, lon: 1, accuracy: 0, speed: 0, heading: 0, altitude: 0 });

    const metrics = bus.getMetrics();
    expect(metrics.eventCounts["location.update"]).toBe(2);
    expect(metrics.totalProcessed).toBe(2);
  });

  it("reports listener count", () => {
    bus.on("location.update", () => {});
    bus.on("location.update", () => {});
    bus.onAll(() => {});

    expect(bus.listenerCount("location.update")).toBe(3); // 2 specific + 1 wildcard
  });

  it("lists registered types", () => {
    bus.on("location.update", () => {});
    bus.on("eta.update", () => {});

    const types = bus.registeredTypes();
    expect(types).toContain("location.update");
    expect(types).toContain("eta.update");
  });

  it("resets all state", () => {
    bus.on("location.update", () => {});
    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });
    bus.reset();

    expect(bus.listenerCount("location.update")).toBe(0);
    expect(bus.getMetrics().totalProcessed).toBe(0);
  });
});

describe("EventBus — Middleware", () => {
  let bus: EventBus;

  beforeEach(() => {
    bus = new EventBus({ enableOfflineQueue: false });
  });

  afterEach(() => {
    bus.destroy();
  });

  it("runs middleware pipeline", () => {
    const order: string[] = [];

    bus.use((event, next) => {
      order.push("mw1");
      next();
    });

    bus.use((event, next) => {
      order.push("mw2");
      next();
    });

    bus.on("location.update", () => { order.push("handler"); });

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });

    expect(order).toEqual(["mw1", "mw2", "handler"]);
  });

  it("middleware can block events", () => {
    let received = false;

    bus.use((_event, _next) => {
      // Don't call next() — blocks the event
    });

    bus.on("location.update", () => { received = true; });

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 });

    expect(received).toBe(false);
  });

  it("priority filter middleware works", () => {
    let count = 0;
    bus.use(createPriorityFilter("high"));
    bus.on("location.update", () => { count++; });

    bus.emit("location.update", { lat: 0, lon: 0, accuracy: 0, speed: 0, heading: 0, altitude: 0 }, { priority: "low" });
    bus.emit("location.update", { lat: 1, lon: 1, accuracy: 0, speed: 0, heading: 0, altitude: 0 }, { priority: "critical" });
    bus.emit("location.update", { lat: 2, lon: 2, accuracy: 0, speed: 0, heading: 0, altitude: 0 }, { priority: "high" });

    expect(count).toBe(2); // critical + high pass, low blocked
  });
});

describe("Observability Router", () => {
  it("observability router is registered in appRouter", async () => {
    const { appRouter } = await import("../routers");
    expect(appRouter._def.procedures).toHaveProperty("observability.metrics");
    expect(appRouter._def.procedures).toHaveProperty("observability.recentSpans");
    expect(appRouter._def.procedures).toHaveProperty("observability.ingestSpans");
  });
});
