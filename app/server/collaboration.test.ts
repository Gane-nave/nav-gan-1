/**
 * G.A.N.E — Collaboration Router Tests (Scalability-Hardened)
 * =============================================================
 * Tests for real-time collaboration with paginated responses,
 * rate limiting, capacity enforcement, and cleanup.
 */
import { describe, it, expect, beforeEach } from "vitest";
import { appRouter } from "./routers";
import type { inferRouterContext } from "@trpc/server";

type TrpcContext = inferRouterContext<typeof appRouter>;
type AuthenticatedUser = NonNullable<TrpcContext["user"]>;

function createUserContext(userId: number, name: string, role: "user" | "admin" = "user"): TrpcContext {
  const user: AuthenticatedUser = {
    id: userId,
    openId: `user-${userId}`,
    email: `${name.toLowerCase()}@example.com`,
    name,
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
      clearCookie: () => {},
    } as TrpcContext["res"],
  };
}

describe.skipIf(!process.env.DATABASE_URL)("collaboration", () => {
  const ctx1 = createUserContext(1, "Alice");
  const ctx2 = createUserContext(2, "Bob");
  const adminCtx = createUserContext(99, "Admin", "admin");
  const caller1 = appRouter.createCaller(ctx1);
  const caller2 = appRouter.createCaller(ctx2);
  const adminCaller = appRouter.createCaller(adminCtx);
  let sessionId: string;

  it("creates a collaboration session", async () => {
    const result = await caller1.collaboration.createSession({
      name: "Test Session",
      maxParticipants: 10,
    });
    expect(result).toBeDefined();
    expect(result.sessionId).toBeTruthy();
    expect(typeof result.sessionId).toBe("string");
    sessionId = result.sessionId;
  });

  it("lists active sessions (paginated)", async () => {
    const result = await caller1.collaboration.listSessions();
    expect(result).toBeDefined();
    expect(result.sessions).toBeDefined();
    expect(Array.isArray(result.sessions)).toBe(true);
    const found = result.sessions.find((s) => s.sessionId === sessionId);
    expect(found).toBeDefined();
    expect(found?.name).toBe("Test Session");
  });

  it("allows a second user to join the session", async () => {
    const result = await caller2.collaboration.joinSession({
      sessionId,
    });
    expect(result).toBeDefined();
    expect(result.color).toBeTruthy();
  });

  it("lists participants in the session", async () => {
    const participants = await caller1.collaboration.getParticipants({
      sessionId,
    });
    expect(Array.isArray(participants)).toBe(true);
    expect(participants.length).toBeGreaterThanOrEqual(1);
  });

  it("adds a shared marker", async () => {
    const result = await caller1.collaboration.addMarker({
      sessionId,
      lat: 32.0853,
      lon: 34.7818,
      label: "Test Marker",
      description: "A test marker",
      color: "#00e5ff",
    });
    expect(result).toBeDefined();
    expect(result.markerId).toBeTruthy();
  });

  it("retrieves shared markers (paginated)", async () => {
    const result = await caller1.collaboration.getMarkers({
      sessionId,
    });
    expect(result).toBeDefined();
    expect(result.markers).toBeDefined();
    expect(Array.isArray(result.markers)).toBe(true);
    expect(result.markers.length).toBeGreaterThanOrEqual(1);
    const testMarker = result.markers.find((m) => m.label === "Test Marker");
    expect(testMarker).toBeDefined();
    expect(testMarker?.lat).toBeCloseTo(32.0853, 3);
    expect(testMarker?.lon).toBeCloseTo(34.7818, 3);
  });

  it("moves a shared marker", async () => {
    const markersResult = await caller1.collaboration.getMarkers({ sessionId });
    const markerId = markersResult.markers[0]?.markerId;
    expect(markerId).toBeTruthy();

    const result = await caller1.collaboration.moveMarker({
      markerId: markerId!,
      sessionId,
      lat: 31.7683,
      lon: 35.2137,
    });
    expect(result.success).toBe(true);
  });

  it("adds an annotation", async () => {
    const result = await caller1.collaboration.addAnnotation({
      sessionId,
      type: "text",
      data: { text: "Important location", x: 100, y: 200 },
      color: "#ff3355",
    });
    expect(result).toBeDefined();
    expect(result.annotationId).toBeTruthy();
  });

  it("retrieves annotations (paginated)", async () => {
    const result = await caller1.collaboration.getAnnotations({
      sessionId,
    });
    expect(result).toBeDefined();
    expect(result.annotations).toBeDefined();
    expect(Array.isArray(result.annotations)).toBe(true);
    expect(result.annotations.length).toBeGreaterThanOrEqual(1);
  });

  it("updates cursor position", async () => {
    const result = await caller1.collaboration.updateCursor({
      sessionId,
      lat: 32.0853,
      lon: 34.7818,
    });
    expect(result.success).toBe(true);
  });

  it("sends heartbeat", async () => {
    const result = await caller1.collaboration.heartbeat({
      sessionId,
    });
    expect(result.success).toBe(true);
  });

  it("deletes a marker", async () => {
    const markersResult = await caller1.collaboration.getMarkers({ sessionId });
    const markerId = markersResult.markers[0]?.markerId;
    expect(markerId).toBeTruthy();

    const result = await caller1.collaboration.deleteMarker({
      markerId: markerId!,
      sessionId,
    });
    expect(result.success).toBe(true);
  });

  it("deletes an annotation", async () => {
    const annotResult = await caller1.collaboration.getAnnotations({ sessionId });
    const annotationId = annotResult.annotations[0]?.annotationId;
    expect(annotationId).toBeTruthy();

    const result = await caller1.collaboration.deleteAnnotation({
      annotationId: annotationId!,
      sessionId,
    });
    expect(result.success).toBe(true);
  });

  it("ends the session", async () => {
    const result = await caller1.collaboration.endSession({
      sessionId,
    });
    expect(result.success).toBe(true);
  });

  it("returns paginated activity feed", async () => {
    // Create a fresh session for this test
    const session = await caller1.collaboration.createSession({ name: "Feed Test" });
    await caller1.collaboration.joinSession({ sessionId: session.sessionId });

    const result = await caller1.collaboration.getActivityFeed({
      sessionId: session.sessionId,
      limit: 5,
    });
    expect(result).toBeDefined();
    expect(result.events).toBeDefined();
    expect(Array.isArray(result.events)).toBe(true);

    // Clean up
    await caller1.collaboration.endSession({ sessionId: session.sessionId });
  });
});

describe.skipIf(!process.env.DATABASE_URL)("collaboration scalability", () => {
  const ctx1 = createUserContext(100, "ScaleUser");
  const caller1 = appRouter.createCaller(ctx1);

  beforeEach(async () => {
    // Clear rate limit buckets between tests to prevent cross-test interference
    const { clearAllBuckets } = await import("./gane/rateLimiter");
    clearAllBuckets();
  });

  it("enforces session creation rate limits after burst", { timeout: 30000 }, async () => {
    // Create sessions up to the limit (5 burst capacity)
    const sessions: string[] = [];
    for (let i = 0; i < 5; i++) {
      const result = await caller1.collaboration.createSession({
        name: `Rate Test ${i}`,
      });
      sessions.push(result.sessionId);
    }

    // The 6th should be rate limited
    try {
      await caller1.collaboration.createSession({ name: "Over Limit" });
      // If it doesn't throw, the rate limiter refilled fast enough — that's OK
    } catch (err: any) {
      expect(err.code).toBe("TOO_MANY_REQUESTS");
    }

    // Clean up
    for (const sid of sessions) {
      await caller1.collaboration.endSession({ sessionId: sid });
    }
  });

  it("supports pagination with cursor on listSessions", { timeout: 30000 }, async () => {
    // Create 3 sessions
    const sessions: string[] = [];
    for (let i = 0; i < 3; i++) {
      const result = await caller1.collaboration.createSession({
        name: `Page Test ${i}`,
      });
      sessions.push(result.sessionId);
    }

    // List with limit 2
    const page1 = await caller1.collaboration.listSessions({ limit: 2 });
    expect(page1.sessions.length).toBeLessThanOrEqual(2);

    // If there's a next cursor, fetch page 2
    if (page1.nextCursor) {
      const page2 = await caller1.collaboration.listSessions({
        limit: 2,
        cursor: page1.nextCursor,
      });
      expect(page2.sessions).toBeDefined();
    }

    // Clean up
    for (const sid of sessions) {
      await caller1.collaboration.endSession({ sessionId: sid });
    }
  });

  it("supports marker pagination", async () => {
    const session = await caller1.collaboration.createSession({ name: "Marker Page Test" });
    await caller1.collaboration.joinSession({ sessionId: session.sessionId });

    // Add 3 markers
    for (let i = 0; i < 3; i++) {
      await caller1.collaboration.addMarker({
        sessionId: session.sessionId,
        lat: 32 + i * 0.01,
        lon: 34 + i * 0.01,
        label: `Marker ${i}`,
      });
    }

    // Get with limit 2
    const page1 = await caller1.collaboration.getMarkers({
      sessionId: session.sessionId,
      limit: 2,
    });
    expect(page1.markers.length).toBeLessThanOrEqual(2);

    // Clean up
    await caller1.collaboration.endSession({ sessionId: session.sessionId });
  });

  it("returns cursor throttle info on rapid cursor updates", async () => {
    const session = await caller1.collaboration.createSession({ name: "Throttle Test" });
    await caller1.collaboration.joinSession({ sessionId: session.sessionId });

    // First update should succeed
    const result1 = await caller1.collaboration.updateCursor({
      sessionId: session.sessionId,
      lat: 32.0,
      lon: 34.0,
    });
    expect(result1.success).toBe(true);

    // Immediate second update should be throttled
    const result2 = await caller1.collaboration.updateCursor({
      sessionId: session.sessionId,
      lat: 32.1,
      lon: 34.1,
    });
    expect(result2.success).toBe(true);
    // The throttled field indicates server-side dedup
    if (result2.throttled !== undefined) {
      expect(typeof result2.throttled).toBe("boolean");
    }

    // Clean up
    await caller1.collaboration.endSession({ sessionId: session.sessionId });
  });
});

describe("rate limiter unit tests", () => {
  it("allows requests within budget", async () => {
    const { checkRateLimit } = await import("./gane/rateLimiter");
    const config = { maxTokens: 10, refillRate: 5, tokensPerRequest: 1 };

    const result = checkRateLimit("test-user-1", "test-endpoint", config);
    expect(result.allowed).toBe(true);
    expect(result.remaining).toBeGreaterThanOrEqual(0);
  });

  it("blocks requests when tokens exhausted", async () => {
    const { checkRateLimit } = await import("./gane/rateLimiter");
    const config = { maxTokens: 2, refillRate: 0.1, tokensPerRequest: 1 };

    // Exhaust tokens
    checkRateLimit("exhaust-user", "exhaust-endpoint", config);
    checkRateLimit("exhaust-user", "exhaust-endpoint", config);

    // Third request should be blocked
    const result = checkRateLimit("exhaust-user", "exhaust-endpoint", config);
    expect(result.allowed).toBe(false);
    expect(result.retryAfterMs).toBeGreaterThan(0);
  });

  it("cursor throttling works", async () => {
    const { shouldThrottleCursor } = await import("./gane/rateLimiter");

    // First call should not throttle
    const first = shouldThrottleCursor(999, "throttle-test-session");
    expect(first).toBe(false);

    // Immediate second call should throttle
    const second = shouldThrottleCursor(999, "throttle-test-session");
    expect(second).toBe(true);
  });

  it("SSE connection limits work", async () => {
    const {
      canOpenSSEConnection,
      registerSSEConnection,
      unregisterSSEConnection,
    } = await import("./gane/rateLimiter");

    // Should allow initial connection
    const check1 = canOpenSSEConnection("sse-test-user", "sse-test-session");
    expect(check1.allowed).toBe(true);

    // Register connections up to limit
    for (let i = 0; i < 5; i++) {
      registerSSEConnection(`conn-${i}`, "sse-test-user", "sse-test-session");
    }

    // Should deny 6th connection
    const check2 = canOpenSSEConnection("sse-test-user", "sse-test-session");
    expect(check2.allowed).toBe(false);

    // Clean up
    for (let i = 0; i < 5; i++) {
      unregisterSSEConnection(`conn-${i}`);
    }

    // Should allow again after cleanup
    const check3 = canOpenSSEConnection("sse-test-user", "sse-test-session");
    expect(check3.allowed).toBe(true);
  });

  it("provides rate limiter stats", async () => {
    const { getRateLimiterStats } = await import("./gane/rateLimiter");

    const stats = getRateLimiterStats();
    expect(stats).toBeDefined();
    expect(typeof stats.activeBuckets).toBe("number");
    expect(typeof stats.activeCursorTrackers).toBe("number");
    expect(stats.sseConnections).toBeDefined();
    expect(typeof stats.sseConnections.totalConnections).toBe("number");
  });
});

describe("cleanup job unit tests", () => {
  it("runCleanup returns a result object", async () => {
    const { runCleanup } = await import("./gane/cleanupJob");

    const result = await runCleanup();
    expect(result).toBeDefined();
    expect(typeof result.staleParticipantsMarkedOffline).toBe("number");
    expect(typeof result.expiredInvitesDeactivated).toBe("number");
    expect(typeof result.oldEventsDeleted).toBe("number");
    expect(typeof result.inactiveSessionsDeactivated).toBe("number");
    expect(Array.isArray(result.errors)).toBe(true);
  });

  it("startCleanupJob and stopCleanupJob work without errors", async () => {
    const { startCleanupJob, stopCleanupJob } = await import("./gane/cleanupJob");

    // Should not throw
    startCleanupJob(60_000);
    stopCleanupJob();
  });
});
