/**
 * G.A.N.E — Database Connection Pool Optimizer
 * ===============================================
 * Optimized connection pooling for high-concurrency workloads.
 * Wraps the drizzle ORM with connection pool configuration
 * suitable for thousands of concurrent users.
 */

import { drizzle } from "drizzle-orm/mysql2";
import mysql from "mysql2/promise";

let _pool: mysql.Pool | null = null;
let _pooledDb: any = null;

// ─── Pool Configuration ───
const POOL_CONFIG = {
  connectionLimit: 20,          // Max concurrent connections
  maxIdle: 10,                  // Max idle connections
  idleTimeout: 60_000,          // Close idle connections after 60s
  enableKeepAlive: true,        // TCP keep-alive
  keepAliveInitialDelay: 10_000, // Keep-alive probe after 10s
  waitForConnections: true,     // Queue requests when pool is full
  queueLimit: 100,              // Max queued requests (0 = unlimited)
  connectTimeout: 10_000,       // Connection timeout 10s
};

/**
 * Get the optimized pooled database connection.
 * Falls back to the standard getDb() if pool creation fails.
 */
export async function getPooledDb() {
  if (_pooledDb) return _pooledDb;

  const dbUrl = process.env.DATABASE_URL;
  if (!dbUrl) return null;

  try {
    _pool = mysql.createPool({
      uri: dbUrl,
      ...POOL_CONFIG,
    });

    _pooledDb = drizzle(_pool);
    console.log("[DB Pool] Connection pool initialized with", POOL_CONFIG.connectionLimit, "max connections");
    return _pooledDb;
  } catch (error) {
    console.warn("[DB Pool] Failed to create pool, falling back to standard connection:", error);
    return null;
  }
}

/**
 * Get pool statistics for monitoring.
 */
export function getPoolStats(): {
  totalConnections: number;
  freeConnections: number;
  queueLength: number;
} | null {
  if (!_pool) return null;

  const pool = _pool as any;
  return {
    totalConnections: pool.pool?._allConnections?.length ?? 0,
    freeConnections: pool.pool?._freeConnections?.length ?? 0,
    queueLength: pool.pool?._connectionQueue?.length ?? 0,
  };
}

/**
 * Gracefully close the pool on shutdown.
 */
export async function closePool(): Promise<void> {
  if (_pool) {
    await _pool.end();
    _pool = null;
    _pooledDb = null;
    console.log("[DB Pool] Connection pool closed");
  }
}
