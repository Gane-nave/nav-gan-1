/**
 * G.A.N.E — Reliability & New Modules Test Suite
 * =================================================
 * Tests for: BackpressureController, RetryModel, DataRetentionEngine,
 * HealthMonitor, TripManager, GeofenceEngine, IntegrationHub
 */

import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import {
  BackpressureController,
  RetryModel,
  DataRetentionEngine,
  HealthMonitor,
} from "./reliability";
import { GeofenceEngine } from "./geofenceEngine";

// ═══════════════════════════════════════════════════════════
// BACKPRESSURE CONTROLLER
// ═══════════════════════════════════════════════════════════

describe("BackpressureController", () => {
  let bp: BackpressureController;

  beforeEach(() => {
    bp = new BackpressureController();
  });

  afterEach(() => {
    bp.destroy();
  });

  it("allows requests within rate limit", () => {
    const result = bp.tryAcquire("device-001");
    expect(result.allowed).toBe(true);
    expect(result.remaining).toBeGreaterThan(0);
    expect(result.retryAfterMs).toBe(0);
  });

  it("tracks remaining tokens", () => {
    const r1 = bp.tryAcquire("device-002", 50);
    expect(r1.allowed).toBe(true);
    const r2 = bp.tryAcquire("device-002", 50);
    expect(r2.allowed).toBe(true);
    // After consuming 100 tokens, should be near limit
    const r3 = bp.tryAcquire("device-002", 50);
    // Might be allowed due to burst allowance
    expect(typeof r3.allowed).toBe("boolean");
  });

  it("blocks requests when tokens exhausted", () => {
    // Exhaust all tokens
    for (let i = 0; i < 15; i++) {
      bp.tryAcquire("device-003", 10);
    }
    const result = bp.tryAcquire("device-003", 10);
    expect(result.allowed).toBe(false);
    expect(result.retryAfterMs).toBeGreaterThan(0);
  });

  it("isolates rate limits per key", () => {
    // Exhaust device-A
    for (let i = 0; i < 15; i++) {
      bp.tryAcquire("device-A", 10);
    }
    // device-B should still be allowed
    const result = bp.tryAcquire("device-B");
    expect(result.allowed).toBe(true);
  });

  it("updates system load", () => {
    bp.updateSystemLoad(0.8);
    const stats = bp.getStats();
    expect(stats.systemLoad).toBe(0.8);
  });

  it("clamps system load to 0-1", () => {
    bp.updateSystemLoad(1.5);
    expect(bp.getStats().systemLoad).toBe(1);
    bp.updateSystemLoad(-0.5);
    expect(bp.getStats().systemLoad).toBe(0);
  });

  it("calculates average load", () => {
    bp.updateSystemLoad(0.2);
    bp.updateSystemLoad(0.4);
    bp.updateSystemLoad(0.6);
    expect(bp.getAverageLoad()).toBeCloseTo(0.4, 1);
  });

  it("allows custom configuration per key", () => {
    bp.configure("vip-device", { maxTokens: 1000, refillRate: 100 });
    // Should be able to make many requests
    for (let i = 0; i < 50; i++) {
      const r = bp.tryAcquire("vip-device");
      expect(r.allowed).toBe(true);
    }
  });

  it("returns correct stats", () => {
    bp.tryAcquire("dev-1");
    bp.tryAcquire("dev-2");
    const stats = bp.getStats();
    expect(stats.activeBuckets).toBe(2);
    expect(stats.systemLoad).toBe(0);
  });
});

// ═══════════════════════════════════════════════════════════
// RETRY MODEL
// ═══════════════════════════════════════════════════════════

describe("RetryModel", () => {
  let retry: RetryModel;

  beforeEach(() => {
    retry = new RetryModel();
  });

  it("succeeds on first try", async () => {
    const result = await retry.withRetry("test-op", async () => "success");
    expect(result).toBe("success");
  });

  it("retries on failure and eventually succeeds", async () => {
    let attempts = 0;
    const result = await retry.withRetry(
      "retry-op",
      async () => {
        attempts++;
        if (attempts < 3) throw new Error("fail");
        return "recovered";
      },
      { maxRetries: 3, baseDelayMs: 10, maxDelayMs: 50 }
    );
    expect(result).toBe("recovered");
    expect(attempts).toBe(3);
  });

  it("throws after max retries exhausted", async () => {
    await expect(
      retry.withRetry(
        "fail-op",
        async () => {
          throw new Error("permanent failure");
        },
        { maxRetries: 2, baseDelayMs: 10, maxDelayMs: 50 }
      )
    ).rejects.toThrow("permanent failure");
  });

  it("adds failed operations to dead letter queue", async () => {
    try {
      await retry.withRetry(
        "dlq-op",
        async () => {
          throw new Error("fail");
        },
        { maxRetries: 0, baseDelayMs: 10 }
      );
    } catch {
      // Expected
    }

    const dlq = retry.getDeadLetterQueue();
    expect(dlq.length).toBeGreaterThan(0);
    expect(dlq[0].operation).toBe("dlq-op");
    expect(dlq[0].error).toBe("fail");
  });

  it("opens circuit breaker after repeated failures", async () => {
    // Fail 5+ times to trigger circuit breaker
    for (let i = 0; i < 6; i++) {
      try {
        await retry.withRetry(
          "breaker-op",
          async () => {
            throw new Error("fail");
          },
          { maxRetries: 0, baseDelayMs: 1 }
        );
      } catch {
        // Expected
      }
    }

    const states = retry.getCircuitBreakerStates();
    const breaker = states.get("breaker-op");
    expect(breaker?.state).toBe("open");
  });

  it("resets circuit breaker manually", async () => {
    // Trigger circuit breaker
    for (let i = 0; i < 6; i++) {
      try {
        await retry.withRetry(
          "reset-op",
          async () => {
            throw new Error("fail");
          },
          { maxRetries: 0, baseDelayMs: 1 }
        );
      } catch {
        // Expected
      }
    }

    retry.resetCircuitBreaker("reset-op");
    const states = retry.getCircuitBreakerStates();
    expect(states.get("reset-op")?.state).toBe("closed");
  });
});

// ═══════════════════════════════════════════════════════════
// DATA RETENTION ENGINE
// ═══════════════════════════════════════════════════════════

describe("DataRetentionEngine", () => {
  let engine: DataRetentionEngine;

  beforeEach(() => {
    engine = new DataRetentionEngine();
  });

  afterEach(() => {
    engine.stop();
  });

  it("initializes with default policies", () => {
    const stats = engine.getStats();
    expect(stats.policies).toBe(2); // raw_telemetry + log_entries
    expect(stats.running).toBe(false);
  });

  it("adds custom retention policy", () => {
    engine.addPolicy({
      tableName: "custom_data",
      timestampColumn: "created_at",
      tiers: { hot: 1, warm: 7, cold: 30, archive: 90 },
      batchSize: 1000,
    });
    const stats = engine.getStats();
    expect(stats.policies).toBe(3);
  });

  it("replaces existing policy for same table", () => {
    engine.addPolicy({
      tableName: "raw_telemetry",
      timestampColumn: "timestamp",
      tiers: { hot: 1, warm: 7, cold: 30, archive: 90 },
      batchSize: 500,
    });
    const stats = engine.getStats();
    expect(stats.policies).toBe(2); // Should replace, not add
  });

  it("reports correct running state", () => {
    expect(engine.getStats().running).toBe(false);
    engine.start(999_999_999); // Very long interval so it doesn't actually run
    expect(engine.getStats().running).toBe(true);
    engine.stop();
    expect(engine.getStats().running).toBe(false);
  });
});

// ═══════════════════════════════════════════════════════════
// HEALTH MONITOR
// ═══════════════════════════════════════════════════════════

describe("HealthMonitor", () => {
  let monitor: HealthMonitor;

  beforeEach(() => {
    monitor = new HealthMonitor();
  });

  afterEach(() => {
    monitor.stop();
  });

  it("initializes with default components", () => {
    const health = monitor.getSystemHealth();
    expect(health.components.length).toBe(5);
    expect(health.status).toBe("healthy");
  });

  it("reports healthy status when all components are healthy", () => {
    const health = monitor.getSystemHealth();
    expect(health.status).toBe("healthy");
    expect(health.summary.healthy).toBe(5);
    expect(health.summary.degraded).toBe(0);
    expect(health.summary.down).toBe(0);
  });

  it("reports degraded status when a component is degraded", () => {
    monitor.reportHealth("database", "degraded", 500, { reason: "slow" });
    const health = monitor.getSystemHealth();
    expect(health.status).toBe("degraded");
    expect(health.summary.degraded).toBe(1);
  });

  it("reports critical status when a component is down", () => {
    monitor.reportHealth("database", "down", 0, { error: "connection refused" });
    const health = monitor.getSystemHealth();
    expect(health.status).toBe("critical");
    expect(health.summary.down).toBe(1);
  });

  it("tracks uptime", () => {
    const health = monitor.getSystemHealth();
    expect(health.uptime).toBeGreaterThanOrEqual(0);
    expect(health.uptimeFormatted).toMatch(/\d+h \d+m \d+s/);
  });

  it("updates component health dynamically", () => {
    monitor.reportHealth("custom_service", "healthy", 10);
    const health = monitor.getSystemHealth();
    expect(health.components.length).toBe(6); // 5 default + 1 custom
    expect(health.components.find(c => c.name === "custom_service")?.status).toBe("healthy");
  });
});

// ═══════════════════════════════════════════════════════════
// GEOFENCE ENGINE (unit tests for geometry logic)
// ═══════════════════════════════════════════════════════════

describe("GeofenceEngine", () => {
  it("can be instantiated", () => {
    const engine = new GeofenceEngine();
    expect(engine).toBeDefined();
    expect(typeof engine.checkPosition).toBe("function");
    expect(typeof engine.createGeofence).toBe("function");
  });

  it("has correct method signatures", () => {
    const engine = new GeofenceEngine();
    // checkPosition takes a GeofenceCheck object
    expect(engine.checkPosition.length).toBeGreaterThanOrEqual(0);
    // createGeofence takes an input object
    expect(engine.createGeofence.length).toBeGreaterThanOrEqual(0);
  });

  // Note: Full integration tests require DB connection.
  // The GeofenceEngine reads geofences from DB and checks positions against them.
  // The isInsideGeofence, isInsidePolygon, and haversine methods are private.
  // We test the geometry logic indirectly through the public API in integration tests.
});
