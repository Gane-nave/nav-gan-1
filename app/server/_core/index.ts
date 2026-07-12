import "dotenv/config";
// OpenTelemetry MUST be initialized before any other imports
import { initTracing, shutdownTracing } from "../gane/tracing";
initTracing();

import express from "express";
import { createServer } from "http";
import net from "net";
import { createExpressMiddleware } from "@trpc/server/adapters/express";
import { registerOAuthRoutes } from "./oauth";
import { appRouter } from "../routers";
import { createContext } from "./context";
import { serveStatic, setupVite } from "./vite";
import { wsBridge } from "../gane/wsbridge";
import { startETLPipeline } from "../gane/etlPipeline";
import { rateLimiter, securityHeaders } from "../gane/security";
import { dataRetention, healthMonitor, backpressure } from "../gane/reliability";
import { createSSEHandler } from "../gane/collaborationRouter";
import { createCollabWSHandler } from "../gane/collabWsHandler";
import { pubsub } from "../gane/redisPubSub";
import { startCleanupJob, stopCleanupJob } from "../gane/cleanupJob";
import { metricsMiddleware, metricsHandler, startMetricsCollection, stopMetricsCollection } from "../gane/metricsCollector";
import { healthCheckHandler } from "../gane/healthCheck";
import { alertWebhookHandler, alertHistoryHandler } from "../gane/alertWebhook";
import { initServerSentry, sentryErrorHandler, flushSentry } from "../gane/sentryServer";
import { stripeWebhookHandler } from "../gane/stripeWebhook";

function isPortAvailable(port: number): Promise<boolean> {
  return new Promise(resolve => {
    const server = net.createServer();
    server.listen(port, () => {
      server.close(() => resolve(true));
    });
    server.on("error", () => resolve(false));
  });
}

async function findAvailablePort(startPort: number = 3000): Promise<number> {
  for (let port = startPort; port < startPort + 20; port++) {
    if (await isPortAvailable(port)) {
      return port;
    }
  }
  throw new Error(`No available port found starting from ${startPort}`);
}

async function startServer() {
  const app = express();
  const server = createServer(app);

  // ─── Sentry Server-Side Error Tracking ───
  initServerSentry(app);

  // ─── Security Middleware ───
  app.use(securityHeaders);

  // ─── Prometheus Metrics ───
  app.use(metricsMiddleware);
  app.get("/metrics", metricsHandler);
  app.get("/api/health", healthCheckHandler);
  app.post("/api/webhooks/alerts", alertWebhookHandler);
  app.get("/api/alerts/history", alertHistoryHandler);
  // Rate limit only API routes (not static assets or page loads)
  app.use('/api', rateLimiter(200, 60_000)); // 200 req/min per IP
  app.use('/ws', rateLimiter(50, 60_000));   // 50 WS upgrades/min per IP

  // Stripe webhook MUST be before express.json() to receive raw body
  app.post("/api/stripe/webhook", express.raw({ type: "application/json" }), stripeWebhookHandler);

  // Configure body parser with larger size limit for file uploads
  app.use(express.json({ limit: "50mb" }));
  app.use(express.urlencoded({ limit: "50mb", extended: true }));

  // OAuth callback under /api/oauth/callback
  registerOAuthRoutes(app);

  // SSE endpoint for real-time collaboration
  app.get("/api/collab/stream", createSSEHandler());

  // tRPC API
  app.use(
    "/api/trpc",
    createExpressMiddleware({
      router: appRouter,
      createContext,
    })
  );

  // Sentry error handler (must be after routes, before static)
  app.use(sentryErrorHandler());

  // development mode uses Vite, production mode uses static files
  if (process.env.NODE_ENV === "development") {
    await setupVite(app, server);
  } else {
    serveStatic(app);
  }

  const preferredPort = parseInt(process.env.PORT || "3000");
  const port = await findAvailablePort(preferredPort);

  if (port !== preferredPort) {
    console.log(`Port ${preferredPort} is busy, using port ${port} instead`);
  }

  // Initialize G.A.N.E WebSocket Bridge (telemetry/fleet)
  wsBridge.init(server);

  // Initialize Collaboration WebSocket Handler (real-time collab)
  const collabWss = createCollabWSHandler(server);

  // Initialize Redis Pub/Sub (uses ENV.redisUrl, falls back to EventEmitter)
  const { ENV: serverEnv } = await import("./env");
  await pubsub.connect(serverEnv.redisUrl || undefined);

  server.listen(port, () => {
    console.log(`Server running on http://localhost:${port}/`);
    // Start ETL pipeline for data archiving with anonymization
    startETLPipeline();
    // Start data retention engine (runs daily)
    dataRetention.start();
    // Start health monitor (checks every 30s)
    healthMonitor.start();
    // Start collaboration cleanup job (runs every 60s)
    startCleanupJob();
    // Start Prometheus gauge collection (every 15s)
    startMetricsCollection();
  });

  // Graceful shutdown
  const shutdown = async () => {
    console.log("[Server] Shutting down gracefully...");
    wsBridge.shutdown();
    collabWss.close();
    pubsub.disconnect();
    dataRetention.stop();
    healthMonitor.stop();
    stopCleanupJob();
    stopMetricsCollection();
    backpressure.destroy();
    shutdownTracing();
    await flushSentry();
    server.close(() => {
      console.log("[Server] Closed.");
      process.exit(0);
    });
    // Force exit after 10s
    setTimeout(() => process.exit(1), 10_000);
  };

  process.on("SIGTERM", shutdown);
  process.on("SIGINT", shutdown);
}

startServer().catch(console.error);
