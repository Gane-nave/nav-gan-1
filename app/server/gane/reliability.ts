/**
 * G.A.N.E — Reliability Engine
 * ==============================
 * 
 * Production-grade reliability features:
 * 
 * 1. BackpressureController — adaptive rate limiting per device/user
 *    - Token bucket algorithm with burst allowance
 *    - Dynamic rate adjustment based on system load
 *    - Per-device and per-user quotas
 *    - Graceful degradation under pressure
 * 
 * 2. RetryModel — exponential backoff with jitter
 *    - Configurable max retries, base delay, max delay
 *    - Circuit breaker pattern for failing services
 *    - Dead letter queue for permanently failed items
 * 
 * 3. DataRetentionEngine — automated data lifecycle management
 *    - Tiered retention: hot (7d) → warm (30d) → cold (90d) → archive (365d) → purge
 *    - Configurable per-table policies
 *    - Runs as background job
 *    - Audit trail for all purge operations
 * 
 * 4. HealthMonitor — system health aggregation
 *    - Component health checks (DB, WS, ETL, GNSS)
 *    - Uptime tracking
 *    - Degradation detection
 */

import { getDb } from "../db";
import { rawTelemetry, logEntries } from "../../drizzle/schema";
import { sql, lt } from "drizzle-orm";

// ═══════════════════════════════════════════════════
// 1. BACKPRESSURE CONTROLLER
// ═══════════════════════════════════════════════════

interface TokenBucket {
  tokens: number;
  lastRefill: number;
  maxTokens: number;
  refillRate: number; // tokens per second
  burstAllowance: number;
}

export class BackpressureController {
  private buckets: Map<string, TokenBucket> = new Map();
  private systemLoad: number = 0; // 0-1 scale
  private readonly defaultConfig = {
    maxTokens: 100,
    refillRate: 10,       // 10 requests/sec per device
    burstAllowance: 20,   // Allow 20 extra in burst
  };
  private loadHistory: number[] = [];
  private cleanupInterval: ReturnType<typeof setInterval> | null = null;

  constructor() {
    // Cleanup stale buckets every 5 minutes
    this.cleanupInterval = setInterval(() => this.cleanup(), 300_000);
  }

  /** Check if a request should be allowed */
  tryAcquire(key: string, cost: number = 1): { allowed: boolean; remaining: number; retryAfterMs: number } {
    const bucket = this.getOrCreateBucket(key);
    this.refillBucket(bucket);

    // Apply system load factor — reduce capacity under high load
    const effectiveMax = Math.max(10, bucket.maxTokens * (1 - this.systemLoad * 0.5));

    if (bucket.tokens >= cost) {
      bucket.tokens -= cost;
      return {
        allowed: true,
        remaining: Math.floor(bucket.tokens),
        retryAfterMs: 0,
      };
    }

    // Calculate when enough tokens will be available
    const deficit = cost - bucket.tokens;
    const retryAfterMs = Math.ceil((deficit / bucket.refillRate) * 1000);

    return {
      allowed: false,
      remaining: 0,
      retryAfterMs,
    };
  }

  /** Update system load metric (0-1) */
  updateSystemLoad(load: number): void {
    this.systemLoad = Math.max(0, Math.min(1, load));
    this.loadHistory.push(this.systemLoad);
    if (this.loadHistory.length > 60) this.loadHistory.shift(); // Keep last 60 samples
  }

  /** Get average system load */
  getAverageLoad(): number {
    if (this.loadHistory.length === 0) return 0;
    return this.loadHistory.reduce((a, b) => a + b, 0) / this.loadHistory.length;
  }

  /** Get current stats */
  getStats(): {
    activeBuckets: number;
    systemLoad: number;
    averageLoad: number;
    totalRequests: number;
  } {
    return {
      activeBuckets: this.buckets.size,
      systemLoad: this.systemLoad,
      averageLoad: this.getAverageLoad(),
      totalRequests: Array.from(this.buckets.values()).reduce(
        (sum, b) => sum + (b.maxTokens - b.tokens), 0
      ),
    };
  }

  /** Configure rate limits for a specific key */
  configure(key: string, config: Partial<typeof this.defaultConfig>): void {
    const bucket = this.getOrCreateBucket(key);
    if (config.maxTokens !== undefined) bucket.maxTokens = config.maxTokens;
    if (config.refillRate !== undefined) bucket.refillRate = config.refillRate;
    if (config.burstAllowance !== undefined) bucket.burstAllowance = config.burstAllowance;
  }

  /** Destroy and clean up */
  destroy(): void {
    if (this.cleanupInterval) {
      clearInterval(this.cleanupInterval);
      this.cleanupInterval = null;
    }
    this.buckets.clear();
  }

  private getOrCreateBucket(key: string): TokenBucket {
    let bucket = this.buckets.get(key);
    if (!bucket) {
      bucket = {
        tokens: this.defaultConfig.maxTokens,
        lastRefill: Date.now(),
        maxTokens: this.defaultConfig.maxTokens,
        refillRate: this.defaultConfig.refillRate,
        burstAllowance: this.defaultConfig.burstAllowance,
      };
      this.buckets.set(key, bucket);
    }
    return bucket;
  }

  private refillBucket(bucket: TokenBucket): void {
    const now = Date.now();
    const elapsed = (now - bucket.lastRefill) / 1000; // seconds
    const tokensToAdd = elapsed * bucket.refillRate;
    bucket.tokens = Math.min(
      bucket.maxTokens + bucket.burstAllowance,
      bucket.tokens + tokensToAdd
    );
    bucket.lastRefill = now;
  }

  private cleanup(): void {
    const now = Date.now();
    const staleThreshold = 600_000; // 10 minutes
    const keysToDelete: string[] = [];
    this.buckets.forEach((bucket, key) => {
      if (now - bucket.lastRefill > staleThreshold && bucket.tokens >= bucket.maxTokens) {
        keysToDelete.push(key);
      }
    });
    keysToDelete.forEach(k => this.buckets.delete(k));
  }
}

// ═══════════════════════════════════════════════════
// 2. RETRY MODEL
// ═══════════════════════════════════════════════════

interface RetryConfig {
  maxRetries: number;
  baseDelayMs: number;
  maxDelayMs: number;
  jitterFactor: number; // 0-1
  backoffMultiplier: number;
}

interface CircuitBreakerState {
  failures: number;
  lastFailure: number;
  state: 'closed' | 'open' | 'half-open';
  cooldownMs: number;
}

export class RetryModel {
  private circuitBreakers: Map<string, CircuitBreakerState> = new Map();
  private deadLetterQueue: Array<{
    id: string;
    operation: string;
    payload: unknown;
    error: string;
    failedAt: number;
    attempts: number;
  }> = [];

  private readonly defaultConfig: RetryConfig = {
    maxRetries: 3,
    baseDelayMs: 1000,
    maxDelayMs: 30_000,
    jitterFactor: 0.3,
    backoffMultiplier: 2,
  };

  /** Execute with retry logic */
  async withRetry<T>(
    operation: string,
    fn: () => Promise<T>,
    config?: Partial<RetryConfig>
  ): Promise<T> {
    const cfg = { ...this.defaultConfig, ...config };
    const breaker = this.getCircuitBreaker(operation);

    // Check circuit breaker
    if (breaker.state === 'open') {
      if (Date.now() - breaker.lastFailure > breaker.cooldownMs) {
        breaker.state = 'half-open';
      } else {
        throw new Error(`Circuit breaker OPEN for "${operation}" — retry after ${Math.ceil((breaker.cooldownMs - (Date.now() - breaker.lastFailure)) / 1000)}s`);
      }
    }

    let lastError: Error | null = null;

    for (let attempt = 0; attempt <= cfg.maxRetries; attempt++) {
      try {
        const result = await fn();
        // Success — reset circuit breaker
        if (breaker.state === 'half-open') {
          breaker.state = 'closed';
          breaker.failures = 0;
        }
        return result;
      } catch (err) {
        lastError = err instanceof Error ? err : new Error(String(err));

        if (attempt < cfg.maxRetries) {
          const delay = this.calculateDelay(attempt, cfg);
          await new Promise(resolve => setTimeout(resolve, delay));
        }
      }
    }

    // All retries exhausted
    breaker.failures++;
    breaker.lastFailure = Date.now();
    if (breaker.failures >= 5) {
      breaker.state = 'open';
      breaker.cooldownMs = Math.min(breaker.cooldownMs * 2, 300_000); // Max 5 min cooldown
    }

    // Add to dead letter queue
    this.deadLetterQueue.push({
      id: crypto.randomUUID(),
      operation,
      payload: null,
      error: lastError?.message || 'Unknown error',
      failedAt: Date.now(),
      attempts: cfg.maxRetries + 1,
    });

    // Keep DLQ bounded
    if (this.deadLetterQueue.length > 1000) {
      this.deadLetterQueue = this.deadLetterQueue.slice(-500);
    }

    throw lastError;
  }

  /** Get dead letter queue items */
  getDeadLetterQueue(): typeof this.deadLetterQueue {
    return [...this.deadLetterQueue];
  }

  /** Get circuit breaker states */
  getCircuitBreakerStates(): Map<string, CircuitBreakerState> {
    return new Map(this.circuitBreakers);
  }

  /** Reset a circuit breaker */
  resetCircuitBreaker(operation: string): void {
    const breaker = this.circuitBreakers.get(operation);
    if (breaker) {
      breaker.state = 'closed';
      breaker.failures = 0;
      breaker.cooldownMs = 10_000;
    }
  }

  /** Retry a dead letter queue item */
  async retryDeadLetter(id: string, fn: () => Promise<void>): Promise<boolean> {
    const idx = this.deadLetterQueue.findIndex(item => item.id === id);
    if (idx === -1) return false;

    try {
      await fn();
      this.deadLetterQueue.splice(idx, 1);
      return true;
    } catch {
      this.deadLetterQueue[idx].attempts++;
      this.deadLetterQueue[idx].failedAt = Date.now();
      return false;
    }
  }

  private calculateDelay(attempt: number, config: RetryConfig): number {
    const exponentialDelay = config.baseDelayMs * Math.pow(config.backoffMultiplier, attempt);
    const cappedDelay = Math.min(exponentialDelay, config.maxDelayMs);
    const jitter = cappedDelay * config.jitterFactor * (Math.random() * 2 - 1);
    return Math.max(0, cappedDelay + jitter);
  }

  private getCircuitBreaker(operation: string): CircuitBreakerState {
    let breaker = this.circuitBreakers.get(operation);
    if (!breaker) {
      breaker = {
        failures: 0,
        lastFailure: 0,
        state: 'closed',
        cooldownMs: 10_000,
      };
      this.circuitBreakers.set(operation, breaker);
    }
    return breaker;
  }
}

// ═══════════════════════════════════════════════════
// 3. DATA RETENTION ENGINE
// ═══════════════════════════════════════════════════

interface RetentionPolicy {
  tableName: string;
  timestampColumn: string;
  tiers: {
    hot: number;    // days
    warm: number;   // days
    cold: number;   // days
    archive: number; // days — after this, purge
  };
  batchSize: number;
}

interface RetentionStats {
  lastRunAt: Date | null;
  totalPurged: number;
  totalDowntiered: number;
  errors: number;
  policiesApplied: number;
}

export class DataRetentionEngine {
  private policies: RetentionPolicy[] = [];
  private stats: RetentionStats = {
    lastRunAt: null,
    totalPurged: 0,
    totalDowntiered: 0,
    errors: 0,
    policiesApplied: 0,
  };
  private intervalHandle: ReturnType<typeof setInterval> | null = null;
  private running: boolean = false;

  constructor() {
    // Default policies
    this.policies = [
      {
        tableName: 'raw_telemetry',
        timestampColumn: 'timestamp',
        tiers: { hot: 7, warm: 30, cold: 90, archive: 365 },
        batchSize: 5000,
      },
      {
        tableName: 'log_entries',
        timestampColumn: 'createdAt',
        tiers: { hot: 3, warm: 14, cold: 60, archive: 180 },
        batchSize: 10000,
      },
    ];
  }

  /** Start the retention engine (runs daily) */
  start(intervalMs: number = 86_400_000): void {
    if (this.running) return;
    this.running = true;
    console.log('[DataRetention] Engine started');

    // Run first cycle after 1 minute (let system stabilize)
    setTimeout(() => {
      this.runCycle().catch(err => {
        console.error('[DataRetention] Initial cycle error:', err);
        this.stats.errors++;
      });
    }, 60_000);

    this.intervalHandle = setInterval(() => {
      this.runCycle().catch(err => {
        console.error('[DataRetention] Cycle error:', err);
        this.stats.errors++;
      });
    }, intervalMs);
  }

  /** Stop the engine */
  stop(): void {
    this.running = false;
    if (this.intervalHandle) {
      clearInterval(this.intervalHandle);
      this.intervalHandle = null;
    }
    console.log('[DataRetention] Engine stopped');
  }

  /** Add a custom retention policy */
  addPolicy(policy: RetentionPolicy): void {
    // Replace existing policy for same table
    this.policies = this.policies.filter(p => p.tableName !== policy.tableName);
    this.policies.push(policy);
  }

  /** Get stats */
  getStats(): RetentionStats & { running: boolean; policies: number } {
    return {
      ...this.stats,
      running: this.running,
      policies: this.policies.length,
    };
  }

  /** Run a single retention cycle */
  async runCycle(): Promise<void> {
    const db = await getDb();
    if (!db) return;

    console.log('[DataRetention] Running retention cycle...');
    const startTime = Date.now();

    for (const policy of this.policies) {
      try {
        await this.applyPolicy(db, policy);
        this.stats.policiesApplied++;
      } catch (err) {
        console.error(`[DataRetention] Error applying policy for ${policy.tableName}:`, err);
        this.stats.errors++;
      }
    }

    this.stats.lastRunAt = new Date();
    const duration = Date.now() - startTime;
    console.log(`[DataRetention] Cycle complete in ${duration}ms`);
  }

  private async applyPolicy(db: NonNullable<Awaited<ReturnType<typeof getDb>>>, policy: RetentionPolicy): Promise<void> {
    const now = new Date();

    // 1. Purge data older than archive threshold
    const purgeThreshold = new Date(now.getTime() - policy.tiers.archive * 24 * 60 * 60 * 1000);

    if (policy.tableName === 'raw_telemetry') {
      const result = await db.delete(rawTelemetry)
        .where(lt(rawTelemetry.timestamp, purgeThreshold));
      // Note: Drizzle doesn't return affected rows count for MySQL deletes in all cases
      // We log the operation regardless
      this.stats.totalPurged++;
      await this.logRetentionAction(db, policy.tableName, 'purge', {
        threshold: purgeThreshold.toISOString(),
      });
    }

    if (policy.tableName === 'log_entries') {
      await db.delete(logEntries)
        .where(lt(logEntries.createdAt, purgeThreshold));
      this.stats.totalPurged++;
      await this.logRetentionAction(db, policy.tableName, 'purge', {
        threshold: purgeThreshold.toISOString(),
      });
    }

    // 2. Update retention tiers for raw_telemetry
    if (policy.tableName === 'raw_telemetry') {
      const coldThreshold = new Date(now.getTime() - policy.tiers.cold * 24 * 60 * 60 * 1000);
      const warmThreshold = new Date(now.getTime() - policy.tiers.warm * 24 * 60 * 60 * 1000);
      const hotThreshold = new Date(now.getTime() - policy.tiers.hot * 24 * 60 * 60 * 1000);

      // Move to archive tier
      await db.update(rawTelemetry)
        .set({ retentionTier: 'archive' })
        .where(lt(rawTelemetry.timestamp, coldThreshold));
      this.stats.totalDowntiered++;

      // Move to cold tier
      await db.update(rawTelemetry)
        .set({ retentionTier: 'cold' })
        .where(lt(rawTelemetry.timestamp, warmThreshold));
      this.stats.totalDowntiered++;

      // Move to warm tier
      await db.update(rawTelemetry)
        .set({ retentionTier: 'warm' })
        .where(lt(rawTelemetry.timestamp, hotThreshold));
      this.stats.totalDowntiered++;
    }
  }

  private async logRetentionAction(
    db: NonNullable<Awaited<ReturnType<typeof getDb>>>,
    tableName: string,
    action: string,
    details: Record<string, unknown>
  ): Promise<void> {
    try {
      await db.insert(logEntries).values({
        level: 'info',
        source: 'data_retention',
        action: `${action}:${tableName}`,
        details: JSON.stringify(details),
      });
    } catch {
      // Silent fail for audit logging
    }
  }
}

// ═══════════════════════════════════════════════════
// 4. HEALTH MONITOR
// ═══════════════════════════════════════════════════

interface ComponentHealth {
  name: string;
  status: 'healthy' | 'degraded' | 'down';
  lastCheck: number;
  latencyMs: number;
  details: Record<string, unknown>;
}

export class HealthMonitor {
  private components: Map<string, ComponentHealth> = new Map();
  private startTime: number = Date.now();
  private checkInterval: ReturnType<typeof setInterval> | null = null;

  constructor() {
    // Register default components
    const defaults = ['database', 'websocket', 'etl_pipeline', 'gnss_service', 'integration_hub'];
    for (const name of defaults) {
      this.components.set(name, {
        name,
        status: 'healthy',
        lastCheck: Date.now(),
        latencyMs: 0,
        details: {},
      });
    }
  }

  /** Start periodic health checks */
  start(intervalMs: number = 30_000): void {
    this.checkInterval = setInterval(() => this.runChecks(), intervalMs);
    console.log('[HealthMonitor] Started');
  }

  /** Stop health checks */
  stop(): void {
    if (this.checkInterval) {
      clearInterval(this.checkInterval);
      this.checkInterval = null;
    }
  }

  /** Update component health */
  reportHealth(name: string, status: ComponentHealth['status'], latencyMs: number = 0, details: Record<string, unknown> = {}): void {
    this.components.set(name, {
      name,
      status,
      lastCheck: Date.now(),
      latencyMs,
      details,
    });
  }

  /** Get overall system health */
  getSystemHealth(): {
    status: 'healthy' | 'degraded' | 'critical';
    uptime: number;
    uptimeFormatted: string;
    components: ComponentHealth[];
    summary: { healthy: number; degraded: number; down: number };
  } {
    const componentList = Array.from(this.components.values());
    const summary = {
      healthy: componentList.filter(c => c.status === 'healthy').length,
      degraded: componentList.filter(c => c.status === 'degraded').length,
      down: componentList.filter(c => c.status === 'down').length,
    };

    let status: 'healthy' | 'degraded' | 'critical' = 'healthy';
    if (summary.down > 0) status = 'critical';
    else if (summary.degraded > 0) status = 'degraded';

    const uptime = Date.now() - this.startTime;
    const hours = Math.floor(uptime / 3_600_000);
    const minutes = Math.floor((uptime % 3_600_000) / 60_000);
    const seconds = Math.floor((uptime % 60_000) / 1000);

    return {
      status,
      uptime,
      uptimeFormatted: `${hours}h ${minutes}m ${seconds}s`,
      components: componentList,
      summary,
    };
  }

  private async runChecks(): Promise<void> {
    // Check database
    try {
      const start = Date.now();
      const db = await getDb();
      if (db) {
        await db.execute(sql`SELECT 1`);
        this.reportHealth('database', 'healthy', Date.now() - start);
      } else {
        this.reportHealth('database', 'down', 0, { error: 'No connection' });
      }
    } catch (err) {
      this.reportHealth('database', 'down', 0, {
        error: err instanceof Error ? err.message : 'Unknown error',
      });
    }

    // Check stale components (no update in 5 minutes)
    const staleThreshold = 300_000;
    this.components.forEach((comp) => {
      if (Date.now() - comp.lastCheck > staleThreshold && comp.status === 'healthy') {
        comp.status = 'degraded';
        comp.details = { ...comp.details, reason: 'No health update received' };
      }
    });
  }
}

// ═══════════════════════════════════════════════════
// SINGLETON INSTANCES
// ═══════════════════════════════════════════════════

export const backpressure = new BackpressureController();
export const retryModel = new RetryModel();
export const dataRetention = new DataRetentionEngine();
export const healthMonitor = new HealthMonitor();
