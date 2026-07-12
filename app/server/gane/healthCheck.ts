/**
 * G.A.N.E — Health Check Endpoint
 * =================================
 * Comprehensive health check including Redis, database, and WebSocket status.
 * Endpoint: GET /api/health
 */
import type { Request, Response } from "express";
import { pubsub } from "./redisPubSub";
import { getCollabWSStats } from "./collabWsHandler";
import { getDb } from "../db";
import { sql } from "drizzle-orm";

const startTime = Date.now();

export async function healthCheckHandler(_req: Request, res: Response): Promise<void> {
  const pubsubStats = pubsub.getStats();
  const wsStats = getCollabWSStats();

  // Database check
  let dbStatus: { status: string; latencyMs: number } = { status: 'down', latencyMs: 0 };
  try {
    const dbStart = Date.now();
    const db = await getDb();
    if (db) {
      await db.execute(sql`SELECT 1`);
      dbStatus = { status: 'up', latencyMs: Date.now() - dbStart };
    }
  } catch {
    dbStatus = { status: 'down', latencyMs: 0 };
  }

  // CPU usage
  const cpuUsage = process.cpuUsage();
  const uptimeMs = Date.now() - startTime;

  const health = {
    status: "ok" as "ok" | "degraded" | "error",
    timestamp: new Date().toISOString(),
    uptime: process.uptime(),
    uptimeMs,
    version: '1.0.0',
    nodeVersion: process.version,
    services: {
      database: dbStatus,
      redis: {
        status: pubsubStats.isRedisAvailable ? "connected" : "fallback",
        mode: pubsubStats.mode,
        channels: pubsubStats.subscribedChannels,
        handlers: pubsubStats.totalHandlers,
        consecutiveFailures: pubsubStats.consecutiveFailures,
        lastPingMs: pubsubStats.lastPingMs,
      },
      websocket: {
        status: "running",
        activeConnections: wsStats.totalConnections,
        activeSubscriptions: wsStats.totalSubscriptions,
      },
    },
    memory: {
      rss: Math.round(process.memoryUsage().rss / 1024 / 1024),
      heapUsed: Math.round(process.memoryUsage().heapUsed / 1024 / 1024),
      heapTotal: Math.round(process.memoryUsage().heapTotal / 1024 / 1024),
      external: Math.round(process.memoryUsage().external / 1024 / 1024),
      unit: "MB",
    },
    cpu: {
      user: Math.round(cpuUsage.user / 1000),
      system: Math.round(cpuUsage.system / 1000),
      unit: 'ms',
    },
    scaling: {
      currentTier: 'starter',
      maxConcurrentUsers: 100,
      dbPoolSize: 10,
    },
  };

  // Determine overall status
  if (dbStatus.status === 'down') {
    health.status = "error";
  } else if (pubsubStats.consecutiveFailures > 0) {
    health.status = "degraded";
  }

  // In development, Vite HMR + TypeScript checker use significant heap.
  // Use RSS-based check instead of heap% which is misleading in dev.
  const rssMB = process.memoryUsage().rss / 1024 / 1024;
  if (rssMB > 1500) health.status = 'error';
  else if (rssMB > 1000 && health.status === 'ok') health.status = 'degraded';

  res.status(health.status === "ok" ? 200 : health.status === 'degraded' ? 200 : 503).json(health);
}
