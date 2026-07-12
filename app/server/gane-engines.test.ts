/**
 * G.A.N.E Engine Tests
 * =====================
 * Tests for the new engine modules added in the gap-closure sprint.
 */
import { describe, expect, it } from "vitest";
import { appRouter } from "./routers";
import type { TrpcContext } from "./_core/context";

// ─── Test Helpers ───────────────────────────────────────

function createPublicContext(): TrpcContext {
  return {
    user: null,
    req: { protocol: "https", headers: {} } as TrpcContext["req"],
    res: {
      clearCookie: () => {},
    } as unknown as TrpcContext["res"],
  };
}

function createAuthContext(): TrpcContext {
  return {
    user: {
      id: 1,
      openId: "test-user-001",
      email: "test@gane.dev",
      name: "Test User",
      loginMethod: "manus",
      role: "user",
      createdAt: new Date(),
      updatedAt: new Date(),
      lastSignedIn: new Date(),
    },
    req: { protocol: "https", headers: {} } as TrpcContext["req"],
    res: {
      clearCookie: () => {},
    } as unknown as TrpcContext["res"],
  };
}

// ═══════════════════════════════════════════════════════
// 1. INCIDENT ROUTER TESTS
// ═══════════════════════════════════════════════════════

describe("incident router", () => {
  it("reports an incident and returns it in nearby", async () => {
    const caller = appRouter.createCaller(createPublicContext());

    const result = await caller.incident.report({
      type: "accident",
      lat: 32.0853,
      lon: 34.7818,
      deviceId: "device-test-001",
      description: "Two-car collision on Ayalon Highway",
    });

    expect(result).toBeDefined();
    expect(result.incidentId).toBeTruthy();
    expect(result.isNew).toBe(true);

    // Verify it appears in nearby query
    const nearby = await caller.incident.nearby({
      lat: 32.0853,
      lon: 34.7818,
      radiusM: 1000,
    });

    expect(nearby.length).toBeGreaterThanOrEqual(1);
  });

  it("confirms an incident", async () => {
    const caller = appRouter.createCaller(createPublicContext());

    const reported = await caller.incident.report({
      type: "hazard",
      lat: 31.7683,
      lon: 35.2137,
      deviceId: "device-test-002",
      description: "Debris on road",
    });

    const confirmed = await caller.incident.confirm({
      incidentId: reported.incidentId,
      deviceId: "device-test-003",
      action: "confirm",
    });

    expect(confirmed).toBeDefined();
    expect(confirmed.success).toBe(true);
  });

  it("gets active incidents", async () => {
    const caller = appRouter.createCaller(createPublicContext());
    const active = await caller.incident.active();
    expect(Array.isArray(active)).toBe(true);
  });

  it("gets incident stats", async () => {
    const caller = appRouter.createCaller(createPublicContext());
    const stats = await caller.incident.stats();
    expect(stats).toBeDefined();
    expect(typeof stats.total).toBe("number");
  });
});

// ═══════════════════════════════════════════════════════
// 2. LIVE SHARING ROUTER TESTS
// ═══════════════════════════════════════════════════════

describe("liveSharing router", () => {
  it("creates a sharing session", async () => {
    const caller = appRouter.createCaller(createAuthContext());

    const session = await caller.liveSharing.create({
      deviceId: "device-test-share-001",
      durationMinutes: 30,
      permission: "view",
    });

    expect(session).toBeDefined();
    expect(session.token).toBeTruthy();
    expect(session.expiresAt).toBeGreaterThan(Date.now());
  });

  it("tracks a location via token", async () => {
    const caller = appRouter.createCaller(createAuthContext());

    // Create session first
    const session = await caller.liveSharing.create({
      deviceId: "device-test-share-002",
      durationMinutes: 30,
      permission: "view",
    });

    // Track location using the token
    const publicCaller = appRouter.createCaller(createPublicContext());
    const tracked = await publicCaller.liveSharing.track({
      token: session.token,
    });

    expect(tracked).toBeDefined();
  });

  it("lists user's sharing sessions", async () => {
    const caller = appRouter.createCaller(createAuthContext());
    const shares = await caller.liveSharing.myShares();
    expect(Array.isArray(shares)).toBe(true);
  });
});

// ═══════════════════════════════════════════════════════
// 3. CROWD INTELLIGENCE ROUTER TESTS
// ═══════════════════════════════════════════════════════

describe("crowd router", () => {
  it("ingests speed samples as batch", async () => {
    const caller = appRouter.createCaller(createPublicContext());

    const result = await caller.crowd.ingest({
      samples: [
        {
          deviceId: "device-test-004",
          lat: 32.0853,
          lon: 34.7818,
          speedKmh: 45,
          heading: 90,
          timestamp: Date.now(),
        },
        {
          deviceId: "device-test-005",
          lat: 32.0854,
          lon: 34.7819,
          speedKmh: 50,
          heading: 92,
          timestamp: Date.now(),
        },
      ],
    });

    expect(result.accepted).toBe(2);
  });

  it("gets heatmap data", async () => {
    const caller = appRouter.createCaller(createPublicContext());

    const heatmap = await caller.crowd.heatmap({
      minLat: 32.08,
      maxLat: 32.09,
      minLon: 34.77,
      maxLon: 34.79,
    });

    expect(heatmap).toBeDefined();
    expect(Array.isArray(heatmap)).toBe(true);
  });

  it("gets crowd metrics", async () => {
    const caller = appRouter.createCaller(createPublicContext());
    const metrics = await caller.crowd.metrics();
    expect(metrics).toBeDefined();
    expect(typeof metrics.totalSamples).toBe("number");
  });
});

// ═══════════════════════════════════════════════════════
// 4. ANALYTICS ROUTER TESTS
// ═══════════════════════════════════════════════════════

describe("analytics router", () => {
  it("tracks an event", async () => {
    const caller = appRouter.createCaller(createPublicContext());

    const result = await caller.analytics.track({
      type: "route_started",
      deviceId: "device-test-006",
      data: { origin: "Tel Aviv", destination: "Jerusalem" },
    });

    expect(result.accepted).toBe(true);
  });

  it("gets latest analytics snapshot", async () => {
    const caller = appRouter.createCaller(createPublicContext());
    const latest = await caller.analytics.latest();
    // May be null if no snapshot generated yet
    expect(latest === null || typeof latest === "object").toBe(true);
  });

  it("gets daily summary", async () => {
    const caller = appRouter.createCaller(createPublicContext());
    const daily = await caller.analytics.daily();
    expect(daily).toBeDefined();
    expect(typeof daily.totalTrips).toBe("number");
    expect(typeof daily.totalDevices).toBe("number");
  });
});

// ═══════════════════════════════════════════════════════
// 5. EXISTING ROUTERS STILL WORK
// ═══════════════════════════════════════════════════════

describe("existing routers", () => {
  it("auth.me returns null for unauthenticated user", async () => {
    const caller = appRouter.createCaller(createPublicContext());
    const result = await caller.auth.me();
    expect(result).toBeNull();
  });

  it("auth.me returns user for authenticated user", async () => {
    const caller = appRouter.createCaller(createAuthContext());
    const result = await caller.auth.me();
    expect(result).toBeDefined();
    expect(result!.openId).toBe("test-user-001");
  });
});
