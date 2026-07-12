/**
 * G.A.N.E — Redis Pub/Sub Adapter
 * =================================
 * Multi-server event broadcasting via Redis Pub/Sub.
 * Gracefully falls back to in-memory EventEmitter when Redis is unavailable.
 *
 * Architecture:
 * - Publisher client: sends events to Redis channels
 * - Subscriber client: receives events from Redis channels (dedicated connection)
 * - Local handler registry: routes incoming messages to registered callbacks
 * - Health monitor: pings Redis every 30s, switches to fallback after 3 failures
 * - Auto-reconnection: resumes Redis when connection recovers
 *
 * Channel naming:
 * - collab:session:{sessionId}  — Session events (join, leave, markers, annotations)
 * - collab:cursor:{sessionId}   — Cursor position updates (high frequency)
 * - collab:system               — System-wide events (maintenance, shutdown)
 */
import Redis from "ioredis";
import { EventEmitter } from "events";

// ─── Types ───
export type PubSubHandler = (data: unknown) => void;

export interface PubSubStats {
  mode: "redis" | "memory";
  isRedisAvailable: boolean;
  subscribedChannels: number;
  totalHandlers: number;
  consecutiveFailures: number;
  lastPingMs: number | null;
}

// ─── Configuration ───
const HEALTH_CHECK_INTERVAL_MS = 30_000;
const MAX_CONSECUTIVE_FAILURES = 3;
const RECONNECT_DELAY_MS = 5_000;

class RedisPubSubAdapter {
  private pub: Redis | null = null;
  private sub: Redis | null = null;
  private fallback = new EventEmitter();
  private isRedisAvailable = false;
  private redisUrl: string | null = null;
  private consecutiveFailures = 0;
  private lastPingMs: number | null = null;

  // Local handler registry: channel → Set of handlers
  private localHandlers = new Map<string, Set<PubSubHandler>>();
  private healthCheckTimer: ReturnType<typeof setInterval> | null = null;
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private isConnecting = false;
  private isShuttingDown = false;

  constructor() {
    this.fallback.setMaxListeners(500);
  }

  /**
   * Connect to Redis. If redisUrl is not provided or connection fails,
   * falls back to in-memory EventEmitter.
   */
  async connect(redisUrl?: string): Promise<void> {
    if (!redisUrl) {
      console.log("[PubSub] No REDIS_URL provided — using in-memory EventEmitter");
      this.isRedisAvailable = false;
      return;
    }

    this.redisUrl = redisUrl;
    await this.connectToRedis();
  }

  private async connectToRedis(): Promise<void> {
    if (this.isConnecting || this.isShuttingDown || !this.redisUrl) return;
    this.isConnecting = true;

    try {
      // Create publisher client
      this.pub = new Redis(this.redisUrl, {
        maxRetriesPerRequest: 3,
        lazyConnect: true,
        retryStrategy: (times) => {
          if (times > 5) return null; // Stop retrying
          return Math.min(times * 1000, 5000);
        },
        enableReadyCheck: true,
        connectTimeout: 10_000,
      });

      // Create subscriber client (must be separate — Redis requirement)
      this.sub = new Redis(this.redisUrl, {
        maxRetriesPerRequest: 3,
        lazyConnect: true,
        retryStrategy: (times) => {
          if (times > 5) return null;
          return Math.min(times * 1000, 5000);
        },
        enableReadyCheck: true,
        connectTimeout: 10_000,
      });

      // Wire up subscriber message handler
      this.sub.on("message", (channel: string, message: string) => {
        const handlers = this.localHandlers.get(channel);
        if (!handlers || handlers.size === 0) return;

        try {
          const parsed = JSON.parse(message);
          for (const handler of Array.from(handlers)) {
            try {
              handler(parsed);
            } catch (err) {
              console.error(`[PubSub] Handler error on channel ${channel}:`, err);
            }
          }
        } catch (err) {
          console.error(`[PubSub] Failed to parse message on ${channel}:`, err);
        }
      });

      // Handle disconnection events
      this.pub.on("error", (err) => {
        console.error("[PubSub] Publisher error:", err.message);
        this.handleRedisFailure();
      });

      this.sub.on("error", (err) => {
        console.error("[PubSub] Subscriber error:", err.message);
        this.handleRedisFailure();
      });

      this.pub.on("close", () => {
        if (!this.isShuttingDown) this.handleRedisFailure();
      });

      this.sub.on("close", () => {
        if (!this.isShuttingDown) this.handleRedisFailure();
      });

      // Connect both clients
      await Promise.all([this.pub.connect(), this.sub.connect()]);

      this.isRedisAvailable = true;
      this.consecutiveFailures = 0;
      console.log("[PubSub] Connected to Redis — multi-server mode active");

      // Re-subscribe to any channels that were active before reconnection
      for (const channel of Array.from(this.localHandlers.keys())) {
        await this.sub.subscribe(channel);
      }

      // Start health check
      this.startHealthCheck();
    } catch (err) {
      console.warn("[PubSub] Redis connection failed — falling back to EventEmitter:", (err as Error).message);
      this.isRedisAvailable = false;
      this.cleanupRedisClients();
      this.scheduleReconnect();
    } finally {
      this.isConnecting = false;
    }
  }

  /**
   * Publish an event to a channel.
   * Uses Redis when available, falls back to local EventEmitter.
   */
  async publish(channel: string, data: unknown): Promise<void> {
    if (this.isRedisAvailable && this.pub) {
      try {
        await this.pub.publish(channel, JSON.stringify(data));
      } catch (err) {
        console.error("[PubSub] Publish failed, falling back:", (err as Error).message);
        this.handleRedisFailure();
        // Fallback: emit locally
        this.fallback.emit(channel, data);
      }
    } else {
      this.fallback.emit(channel, data);
    }
  }

  /**
   * Subscribe to a channel with a handler.
   * Manages Redis subscriptions and local handler registry.
   */
  subscribe(channel: string, handler: PubSubHandler): void {
    // Register handler locally
    if (!this.localHandlers.has(channel)) {
      this.localHandlers.set(channel, new Set());
    }
    this.localHandlers.get(channel)!.add(handler);

    if (this.isRedisAvailable && this.sub) {
      // Subscribe to Redis channel (idempotent — Redis handles duplicate subscribes)
      this.sub.subscribe(channel).catch((err) => {
        console.error(`[PubSub] Redis subscribe failed for ${channel}:`, err.message);
      });
    } else {
      // Fallback: use local EventEmitter
      this.fallback.on(channel, handler);
    }
  }

  /**
   * Unsubscribe a handler from a channel.
   * Cleans up Redis subscription when no handlers remain.
   */
  unsubscribe(channel: string, handler: PubSubHandler): void {
    const handlers = this.localHandlers.get(channel);
    if (handlers) {
      handlers.delete(handler);

      if (handlers.size === 0) {
        this.localHandlers.delete(channel);

        if (this.isRedisAvailable && this.sub) {
          this.sub.unsubscribe(channel).catch(() => {});
        }
      }
    }

    // Always clean up fallback listener
    this.fallback.off(channel, handler);
  }

  /**
   * Get current stats for monitoring.
   */
  getStats(): PubSubStats {
    let totalHandlers = 0;
    for (const handlers of Array.from(this.localHandlers.values())) {
      totalHandlers += handlers.size;
    }

    return {
      mode: this.isRedisAvailable ? "redis" : "memory",
      isRedisAvailable: this.isRedisAvailable,
      subscribedChannels: this.localHandlers.size,
      totalHandlers,
      consecutiveFailures: this.consecutiveFailures,
      lastPingMs: this.lastPingMs,
    };
  }

  /**
   * Gracefully disconnect from Redis.
   */
  async disconnect(): Promise<void> {
    this.isShuttingDown = true;

    if (this.healthCheckTimer) {
      clearInterval(this.healthCheckTimer);
      this.healthCheckTimer = null;
    }

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    this.cleanupRedisClients();
    this.fallback.removeAllListeners();
    this.localHandlers.clear();

    console.log("[PubSub] Disconnected");
  }

  // ─── Internal Methods ───

  private handleRedisFailure(): void {
    this.consecutiveFailures++;

    if (this.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES && this.isRedisAvailable) {
      console.warn(
        `[PubSub] ${this.consecutiveFailures} consecutive failures — switching to EventEmitter fallback`
      );
      this.isRedisAvailable = false;

      // Migrate all handlers to fallback
      for (const [channel, handlers] of Array.from(this.localHandlers.entries())) {
        for (const handler of Array.from(handlers)) {
          this.fallback.on(channel, handler);
        }
      }

      this.cleanupRedisClients();
      this.scheduleReconnect();
    }
  }

  private cleanupRedisClients(): void {
    try {
      this.pub?.disconnect();
    } catch {}
    try {
      this.sub?.disconnect();
    } catch {}
    this.pub = null;
    this.sub = null;
  }

  private scheduleReconnect(): void {
    if (this.isShuttingDown || this.reconnectTimer) return;

    this.reconnectTimer = setTimeout(async () => {
      this.reconnectTimer = null;
      console.log("[PubSub] Attempting Redis reconnection...");

      // Remove handlers from fallback before reconnecting
      for (const [channel, handlers] of Array.from(this.localHandlers.entries())) {
        for (const handler of Array.from(handlers)) {
          this.fallback.off(channel, handler);
        }
      }

      await this.connectToRedis();

      // If reconnection failed, re-add handlers to fallback
      if (!this.isRedisAvailable) {
        for (const [channel, handlers] of Array.from(this.localHandlers.entries())) {
          for (const handler of Array.from(handlers)) {
            this.fallback.on(channel, handler);
          }
        }
      }
    }, RECONNECT_DELAY_MS);

    if (this.reconnectTimer && typeof this.reconnectTimer === "object" && "unref" in this.reconnectTimer) {
      (this.reconnectTimer as NodeJS.Timeout).unref();
    }
  }

  private startHealthCheck(): void {
    if (this.healthCheckTimer) return;

    this.healthCheckTimer = setInterval(async () => {
      if (!this.isRedisAvailable || !this.pub) return;

      try {
        const start = Date.now();
        await this.pub.ping();
        this.lastPingMs = Date.now() - start;
        this.consecutiveFailures = 0;
      } catch {
        this.handleRedisFailure();
      }
    }, HEALTH_CHECK_INTERVAL_MS);

    if (this.healthCheckTimer && typeof this.healthCheckTimer === "object" && "unref" in this.healthCheckTimer) {
      (this.healthCheckTimer as NodeJS.Timeout).unref();
    }
  }
}

// ─── Singleton Instance ───
export const pubsub = new RedisPubSubAdapter();

// ─── Channel Helpers ───
export function sessionChannel(sessionId: string): string {
  return `collab:session:${sessionId}`;
}

export function cursorChannel(sessionId: string): string {
  return `collab:cursor:${sessionId}`;
}

export const SYSTEM_CHANNEL = "collab:system";
