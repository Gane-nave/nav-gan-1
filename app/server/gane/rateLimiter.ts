/**
 * G.A.N.E — Rate Limiter & Scalability Middleware
 * =================================================
 * Token-bucket rate limiting with per-user and per-endpoint limits.
 * Designed for planetary-scale deployment:
 * - In-memory token buckets with automatic cleanup
 * - Configurable per-procedure limits
 * - Cursor update throttling (server-side deduplication)
 * - SSE connection management (per-user and per-session caps)
 * - Graceful degradation with informative error responses
 */

import { TRPCError } from "@trpc/server";

// ─── Token Bucket Implementation ───
interface TokenBucket {
  tokens: number;
  lastRefill: number;
  maxTokens: number;
  refillRate: number; // tokens per second
}

// ─── Rate Limit Configuration ───
export interface RateLimitConfig {
  maxTokens: number;       // Maximum burst capacity
  refillRate: number;      // Tokens restored per second
  tokensPerRequest: number; // Tokens consumed per request
}

// ─── Default Limits by Procedure Category ───
export const RATE_LIMITS = {
  // High-frequency operations (cursor moves, heartbeats)
  realtime: { maxTokens: 30, refillRate: 10, tokensPerRequest: 1 } as RateLimitConfig,
  // Standard mutations (add marker, annotation, join session)
  mutation: { maxTokens: 20, refillRate: 5, tokensPerRequest: 1 } as RateLimitConfig,
  // Read queries (list sessions, get markers)
  query: { maxTokens: 40, refillRate: 15, tokensPerRequest: 1 } as RateLimitConfig,
  // Session creation (expensive operation)
  create: { maxTokens: 5, refillRate: 1, tokensPerRequest: 1 } as RateLimitConfig,
  // Invite generation
  invite: { maxTokens: 10, refillRate: 2, tokensPerRequest: 1 } as RateLimitConfig,
} as const;

// ─── In-Memory Bucket Store ───
const buckets = new Map<string, TokenBucket>();

// ─── Cleanup interval (every 5 minutes, remove stale buckets) ───
const BUCKET_CLEANUP_INTERVAL_MS = 5 * 60 * 1000;
const BUCKET_STALE_THRESHOLD_MS = 10 * 60 * 1000; // 10 minutes without activity

let cleanupTimer: ReturnType<typeof setInterval> | null = null;

function startBucketCleanup() {
  if (cleanupTimer) return;
  cleanupTimer = setInterval(() => {
    const now = Date.now();
    for (const [key, bucket] of Array.from(buckets.entries())) {
      if (now - bucket.lastRefill > BUCKET_STALE_THRESHOLD_MS) {
        buckets.delete(key);
      }
    }
  }, BUCKET_CLEANUP_INTERVAL_MS);
  // Don't block process exit
  if (cleanupTimer && typeof cleanupTimer === "object" && "unref" in cleanupTimer) {
    cleanupTimer.unref();
  }
}

startBucketCleanup();

// ─── Test Utility: Clear all rate limit buckets ───
export function clearAllBuckets(): void {
  buckets.clear();
  lastCursorUpdate.clear();
}

// ─── Core Rate Limit Check ───
export function checkRateLimit(
  userId: number | string,
  endpoint: string,
  config: RateLimitConfig
): { allowed: boolean; retryAfterMs: number; remaining: number } {
  const key = `${userId}:${endpoint}`;
  const now = Date.now();

  let bucket = buckets.get(key);
  if (!bucket) {
    bucket = {
      tokens: config.maxTokens,
      lastRefill: now,
      maxTokens: config.maxTokens,
      refillRate: config.refillRate,
    };
    buckets.set(key, bucket);
  }

  // Refill tokens based on elapsed time
  const elapsed = (now - bucket.lastRefill) / 1000;
  bucket.tokens = Math.min(
    bucket.maxTokens,
    bucket.tokens + elapsed * bucket.refillRate
  );
  bucket.lastRefill = now;

  if (bucket.tokens >= config.tokensPerRequest) {
    bucket.tokens -= config.tokensPerRequest;
    return { allowed: true, retryAfterMs: 0, remaining: Math.floor(bucket.tokens) };
  }

  // Calculate when enough tokens will be available
  const deficit = config.tokensPerRequest - bucket.tokens;
  const retryAfterMs = Math.ceil((deficit / config.refillRate) * 1000);

  return { allowed: false, retryAfterMs, remaining: 0 };
}

// ─── tRPC-compatible rate limit enforcer ───
export function enforceRateLimit(
  userId: number | string,
  endpoint: string,
  config: RateLimitConfig
): void {
  const result = checkRateLimit(userId, endpoint, config);
  if (!result.allowed) {
    throw new TRPCError({
      code: "TOO_MANY_REQUESTS",
      message: `Rate limit exceeded for ${endpoint}. Retry after ${result.retryAfterMs}ms.`,
    });
  }
}

// ─── Cursor Update Throttling ───
// Server-side deduplication: max 5 cursor updates per second per user per session
const lastCursorUpdate = new Map<string, number>();
const CURSOR_MIN_INTERVAL_MS = 200; // 5 Hz max

export function shouldThrottleCursor(userId: number, sessionId: string): boolean {
  const key = `cursor:${userId}:${sessionId}`;
  const now = Date.now();
  const last = lastCursorUpdate.get(key) || 0;

  if (now - last < CURSOR_MIN_INTERVAL_MS) {
    return true; // Throttle — too soon
  }

  lastCursorUpdate.set(key, now);
  return false;
}

// Cleanup stale cursor entries periodically
setInterval(() => {
  const now = Date.now();
  for (const [key, ts] of Array.from(lastCursorUpdate.entries())) {
    if (now - ts > 60_000) {
      lastCursorUpdate.delete(key);
    }
  }
}, 60_000).unref?.();

// ─── SSE Connection Manager ───
// Tracks active SSE connections per user and per session
interface SSEConnectionInfo {
  userId: string;
  sessionId: string;
  connectedAt: number;
  lastActivity: number;
}

const activeSSEConnections = new Map<string, SSEConnectionInfo>(); // connectionId → info
const MAX_SSE_PER_USER = 5;       // Max concurrent SSE connections per user
const MAX_SSE_PER_SESSION = 100;   // Max concurrent SSE connections per session (room cap)
const SSE_IDLE_TIMEOUT_MS = 5 * 60 * 1000; // 5 minutes idle → disconnect

export function canOpenSSEConnection(userId: string, sessionId: string): {
  allowed: boolean;
  reason?: string;
  userConnections: number;
  sessionConnections: number;
} {
  let userCount = 0;
  let sessionCount = 0;

  for (const info of Array.from(activeSSEConnections.values())) {
    if (info.userId === userId) userCount++;
    if (info.sessionId === sessionId) sessionCount++;
  }

  if (userCount >= MAX_SSE_PER_USER) {
    return {
      allowed: false,
      reason: `Maximum ${MAX_SSE_PER_USER} concurrent connections per user`,
      userConnections: userCount,
      sessionConnections: sessionCount,
    };
  }

  if (sessionCount >= MAX_SSE_PER_SESSION) {
    return {
      allowed: false,
      reason: `Session at capacity (${MAX_SSE_PER_SESSION} connections)`,
      userConnections: userCount,
      sessionConnections: sessionCount,
    };
  }

  return { allowed: true, userConnections: userCount, sessionConnections: sessionCount };
}

export function registerSSEConnection(
  connectionId: string,
  userId: string,
  sessionId: string
): void {
  activeSSEConnections.set(connectionId, {
    userId,
    sessionId,
    connectedAt: Date.now(),
    lastActivity: Date.now(),
  });
}

export function unregisterSSEConnection(connectionId: string): void {
  activeSSEConnections.delete(connectionId);
}

export function updateSSEActivity(connectionId: string): void {
  const info = activeSSEConnections.get(connectionId);
  if (info) {
    info.lastActivity = Date.now();
  }
}

export function getSSEStats(): {
  totalConnections: number;
  uniqueUsers: number;
  uniqueSessions: number;
} {
  const users = new Set<string>();
  const sessions = new Set<string>();
  for (const info of Array.from(activeSSEConnections.values())) {
    users.add(info.userId);
    sessions.add(info.sessionId);
  }
  return {
    totalConnections: activeSSEConnections.size,
    uniqueUsers: users.size,
    uniqueSessions: sessions.size,
  };
}

// Cleanup idle SSE connections
setInterval(() => {
  const now = Date.now();
  for (const [connId, info] of Array.from(activeSSEConnections.entries())) {
    if (now - info.lastActivity > SSE_IDLE_TIMEOUT_MS) {
      activeSSEConnections.delete(connId);
    }
  }
}, 60_000).unref?.();

// ─── Session Capacity Check ───
export const MAX_PARTICIPANTS_DEFAULT = 50;
export const MAX_PARTICIPANTS_HARD_LIMIT = 200;

// ─── Pagination Defaults ───
export const PAGINATION = {
  DEFAULT_PAGE_SIZE: 20,
  MAX_PAGE_SIZE: 100,
  SESSIONS_DEFAULT: 20,
  MARKERS_DEFAULT: 50,
  ANNOTATIONS_DEFAULT: 50,
  EVENTS_DEFAULT: 50,
  PARTICIPANTS_DEFAULT: 50,
} as const;

// ─── Export stats for monitoring ───
export function getRateLimiterStats() {
  return {
    activeBuckets: buckets.size,
    activeCursorTrackers: lastCursorUpdate.size,
    sseConnections: getSSEStats(),
  };
}
