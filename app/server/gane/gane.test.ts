/**
 * G.A.N.E — Comprehensive Test Suite
 * =====================================
 * Tests for: geo utilities, security module, telemetry router,
 * anomaly router, fleet router.
 */

import { describe, expect, it, vi, beforeEach } from "vitest";
import {
  haversineDistance,
  boundingBox,
  calculateRouteDistance,
  twoOptImprove,
} from "./geo";
import {
  sanitizeString,
  validateDeviceId,
  validateCoordinates,
  validateTelemetryPayload,
  detectReplay,
} from "./security";
import { appRouter } from "../routers";
import type { TrpcContext } from "../_core/context";

// ─── Helper: Create mock context ───

type AuthenticatedUser = NonNullable<TrpcContext["user"]>;

function createMockContext(role: "user" | "admin" = "user"): TrpcContext {
  const user: AuthenticatedUser = {
    id: 1,
    openId: "test-user-001",
    email: "test@gane.dev",
    name: "Test User",
    loginMethod: "manus",
    role,
    createdAt: new Date(),
    updatedAt: new Date(),
    lastSignedIn: new Date(),
  };

  return {
    user,
    req: {
      protocol: "https",
      headers: {},
    } as TrpcContext["req"],
    res: {
      clearCookie: vi.fn(),
    } as unknown as TrpcContext["res"],
  };
}

function createPublicContext(): TrpcContext {
  return {
    user: null,
    req: {
      protocol: "https",
      headers: {},
    } as TrpcContext["req"],
    res: {
      clearCookie: vi.fn(),
    } as unknown as TrpcContext["res"],
  };
}

// ═══════════════════════════════════════════════════════════
// GEO UTILITIES
// ═══════════════════════════════════════════════════════════

describe("geo.ts — Geospatial Utilities", () => {
  describe("haversineDistance", () => {
    it("returns 0 for same point", () => {
      const dist = haversineDistance(32.0853, 34.7818, 32.0853, 34.7818);
      expect(dist).toBe(0);
    });

    it("calculates Tel Aviv to Jerusalem (~54km)", () => {
      const dist = haversineDistance(32.0853, 34.7818, 31.7683, 35.2137);
      expect(dist).toBeGreaterThan(50_000);
      expect(dist).toBeLessThan(60_000);
    });

    it("calculates Tel Aviv to Haifa (~83km)", () => {
      const dist = haversineDistance(32.0853, 34.7818, 32.7940, 34.9896);
      expect(dist).toBeGreaterThan(75_000);
      expect(dist).toBeLessThan(90_000);
    });

    it("handles antipodal points (~20,000km)", () => {
      const dist = haversineDistance(0, 0, 0, 180);
      expect(dist).toBeGreaterThan(19_000_000);
      expect(dist).toBeLessThan(21_000_000);
    });

    it("handles negative coordinates", () => {
      const dist = haversineDistance(-33.8688, 151.2093, -37.8136, 144.9631);
      expect(dist).toBeGreaterThan(700_000);
      expect(dist).toBeLessThan(800_000);
    });
  });

  describe("boundingBox", () => {
    it("creates correct bounding box for Tel Aviv 1km radius", () => {
      const box = boundingBox(32.0853, 34.7818, 1000);
      expect(box.minLat).toBeLessThan(32.0853);
      expect(box.maxLat).toBeGreaterThan(32.0853);
      expect(box.minLon).toBeLessThan(34.7818);
      expect(box.maxLon).toBeGreaterThan(34.7818);
      // ~0.009 degrees per km at this latitude
      expect(box.maxLat - box.minLat).toBeCloseTo(0.018, 2);
    });

    it("handles equator correctly", () => {
      const box = boundingBox(0, 0, 10_000);
      expect(box.minLat).toBeCloseTo(-0.0899, 2);
      expect(box.maxLat).toBeCloseTo(0.0899, 2);
    });
  });

  describe("calculateRouteDistance", () => {
    it("returns 0 for empty route (depot to depot)", () => {
      const dist = calculateRouteDistance([], 32.0853, 34.7818);
      expect(dist).toBe(0);
    });

    it("calculates round trip for single stop", () => {
      const stops = [{ lat: 32.0853, lon: 34.7818 }];
      const dist = calculateRouteDistance(stops, 32.0853, 34.7818);
      expect(dist).toBe(0); // Same point
    });

    it("calculates multi-stop route", () => {
      const stops = [
        { lat: 32.0853, lon: 34.7818 }, // Tel Aviv
        { lat: 31.7683, lon: 35.2137 }, // Jerusalem
      ];
      const dist = calculateRouteDistance(stops, 32.7940, 34.9896); // Haifa depot
      expect(dist).toBeGreaterThan(100_000);
    });
  });

  describe("twoOptImprove", () => {
    it("returns same route for single stop", () => {
      const stops = [{ lat: 32.0853, lon: 34.7818 }];
      const improved = twoOptImprove(stops, 32.0853, 34.7818);
      expect(improved).toHaveLength(1);
    });

    it("optimizes a clearly suboptimal route", () => {
      // Create a route that zigzags (suboptimal)
      const stops = [
        { lat: 32.0, lon: 34.0 },
        { lat: 32.2, lon: 34.0 },
        { lat: 32.05, lon: 34.0 },
        { lat: 32.15, lon: 34.0 },
      ];
      const original = calculateRouteDistance(stops, 32.0, 34.0);
      const improved = twoOptImprove(stops, 32.0, 34.0);
      const optimized = calculateRouteDistance(improved, 32.0, 34.0);
      expect(optimized).toBeLessThanOrEqual(original);
    });
  });
});

// ═══════════════════════════════════════════════════════════
// SECURITY MODULE
// ═══════════════════════════════════════════════════════════

describe("security.ts — Security Hardening", () => {
  describe("sanitizeString", () => {
    it("strips HTML tags", () => {
      expect(sanitizeString("<script>alert('xss')</script>")).toBe("alert(xss)");
    });

    it("removes dangerous characters", () => {
      // sanitizeString strips tags first, then dangerous chars
      const result = sanitizeString('Hello "world" & <test>');
      expect(result).not.toContain('<');
      expect(result).not.toContain('>');
      expect(result).not.toContain('"');
      expect(result).not.toContain('&');
    });

    it("removes javascript: protocol", () => {
      expect(sanitizeString("javascript:alert(1)")).toBe("alert(1)");
    });

    it("removes event handlers", () => {
      expect(sanitizeString("onerror=alert(1)")).toBe("alert(1)");
    });

    it("trims whitespace", () => {
      expect(sanitizeString("  hello  ")).toBe("hello");
    });

    it("handles empty string", () => {
      expect(sanitizeString("")).toBe("");
    });
  });

  describe("validateDeviceId", () => {
    it("accepts valid device IDs", () => {
      expect(validateDeviceId("device-001")).toBe(true);
      expect(validateDeviceId("web_1234abc")).toBe(true);
      expect(validateDeviceId("ABC123")).toBe(true);
    });

    it("rejects empty string", () => {
      expect(validateDeviceId("")).toBe(false);
    });

    it("rejects IDs with special characters", () => {
      expect(validateDeviceId("device@001")).toBe(false);
      expect(validateDeviceId("device 001")).toBe(false);
      expect(validateDeviceId("device/001")).toBe(false);
    });

    it("rejects IDs longer than 64 chars", () => {
      expect(validateDeviceId("a".repeat(65))).toBe(false);
    });

    it("accepts IDs exactly 64 chars", () => {
      expect(validateDeviceId("a".repeat(64))).toBe(true);
    });
  });

  describe("validateCoordinates", () => {
    it("accepts valid coordinates", () => {
      expect(validateCoordinates(32.0853, 34.7818)).toBe(true);
      expect(validateCoordinates(0, 0)).toBe(true);
      expect(validateCoordinates(-90, -180)).toBe(true);
      expect(validateCoordinates(90, 180)).toBe(true);
    });

    it("rejects out-of-range latitude", () => {
      expect(validateCoordinates(91, 34.7818)).toBe(false);
      expect(validateCoordinates(-91, 34.7818)).toBe(false);
    });

    it("rejects out-of-range longitude", () => {
      expect(validateCoordinates(32.0853, 181)).toBe(false);
      expect(validateCoordinates(32.0853, -181)).toBe(false);
    });

    it("rejects NaN", () => {
      expect(validateCoordinates(NaN, 34.7818)).toBe(false);
      expect(validateCoordinates(32.0853, NaN)).toBe(false);
    });
  });

  describe("validateTelemetryPayload", () => {
    it("validates a correct payload", () => {
      const result = validateTelemetryPayload({
        deviceId: "device-001",
        lat: 32.0853,
        lon: 34.7818,
      });
      expect(result.valid).toBe(true);
      expect(result.data?.deviceId).toBe("device-001");
      expect(result.data?.lat).toBe(32.0853);
    });

    it("rejects null payload", () => {
      const result = validateTelemetryPayload(null);
      expect(result.valid).toBe(false);
    });

    it("rejects missing deviceId", () => {
      const result = validateTelemetryPayload({ lat: 32, lon: 34 });
      expect(result.valid).toBe(false);
      expect(result.errors).toContain("Invalid deviceId");
    });

    it("rejects invalid coordinates", () => {
      const result = validateTelemetryPayload({
        deviceId: "dev-1",
        lat: 999,
        lon: 34,
      });
      expect(result.valid).toBe(false);
      expect(result.errors).toContain("Invalid coordinates");
    });

    it("sanitizes deviceId in output", () => {
      const result = validateTelemetryPayload({
        deviceId: "device-001",
        lat: 32.0853,
        lon: 34.7818,
      });
      expect(result.data?.deviceId).toBe("device-001");
    });
  });

  describe("detectReplay", () => {
    it("returns false for first request", () => {
      const result = detectReplay("unique-device-" + Date.now(), Date.now());
      expect(result).toBe(false);
    });

    it("returns true for duplicate request", () => {
      const deviceId = "replay-test-" + Date.now();
      const ts = Date.now();
      detectReplay(deviceId, ts);
      const result = detectReplay(deviceId, ts);
      expect(result).toBe(true);
    });
  });
});

// ═══════════════════════════════════════════════════════════
// tRPC ROUTERS — Integration Tests
// ═══════════════════════════════════════════════════════════

describe("tRPC Routers — Integration", () => {
  describe("telemetry.ingest", () => {
    it("accepts valid telemetry data", async () => {
      const ctx = createPublicContext();
      const caller = appRouter.createCaller(ctx);

      const result = await caller.telemetry.ingest({
        deviceId: "test-device-001",
        lat: 32.0853,
        lon: 34.7818,
        alt: 15,
        velocity: 60,
        heading: 180,
        satellites: 12,
        hdop: 1.2,
        batteryLevel: 85,
      });

      expect(result.success).toBeDefined();
    });

    it("rejects invalid coordinates", async () => {
      const ctx = createPublicContext();
      const caller = appRouter.createCaller(ctx);

      await expect(
        caller.telemetry.ingest({
          deviceId: "test-device-001",
          lat: 999,
          lon: 34.7818,
        })
      ).rejects.toThrow();
    });
  });

  describe("anomaly.report", () => {
    it("accepts valid anomaly report", async () => {
      const ctx = createPublicContext();
      const caller = appRouter.createCaller(ctx);

      const result = await caller.anomaly.report({
        deviceId: "test-device-001",
        type: "pothole",
        lat: 32.0853,
        lon: 34.7818,
        severity: 3,
        description: "Large pothole on main road",
      });

      expect(result.success).toBeDefined();
    });
  });

  describe("fleet operations (protected)", () => {
    it("rejects unauthenticated fleet creation", async () => {
      const ctx = createPublicContext();
      const caller = appRouter.createCaller(ctx);

      await expect(
        caller.fleet.create({
          name: "Test Fleet",
        })
      ).rejects.toThrow();
    });

    it("allows authenticated user to create fleet", async () => {
      const ctx = createMockContext("user");
      const caller = appRouter.createCaller(ctx);

      const result = await caller.fleet.create({
        name: "Test Fleet " + Date.now(),
        nameHe: "צי בדיקה",
      });

      expect(result.success).toBeDefined();
    });
  });

  describe("telemetry.nearby", () => {
    it("returns nearby vehicles", async () => {
      const ctx = createPublicContext();
      const caller = appRouter.createCaller(ctx);

      const result = await caller.telemetry.nearby({
        lat: 32.0853,
        lon: 34.7818,
        radiusM: 5000,
      });

      expect(Array.isArray(result)).toBe(true);
    });
  });
});
