/**
 * G.A.N.E — Server-Side Sentry Integration
 * ==========================================
 * Express error handler middleware and server-side error tracking.
 * Gracefully degrades when SENTRY_DSN is not configured.
 */
import * as Sentry from "@sentry/node";
import type { Request, Response, NextFunction, Express } from "express";
import { ENV } from "../_core/env";

let isInitialized = false;

/**
 * Initialize Sentry for server-side error tracking.
 * Must be called before Express routes are registered.
 */
export function initServerSentry(app: Express): void {
  if (isInitialized) return;
  if (!ENV.sentryDsn) {
    if (ENV.isProduction) {
      console.warn("[Sentry/Server] No DSN configured — server error tracking disabled");
    }
    return;
  }

  try {
    Sentry.init({
      dsn: ENV.sentryDsn,
      environment: ENV.isProduction ? "production" : "development",
      release: `gane-server@1.0.0`,

      // Performance monitoring
      tracesSampleRate: ENV.isProduction ? 0.1 : 1.0,

      // Integrations
      integrations: [
        Sentry.httpIntegration(),
        Sentry.expressIntegration(),
      ],

      // Filter out noisy errors
      ignoreErrors: [
        "ECONNRESET",
        "EPIPE",
        "ECONNREFUSED",
        "socket hang up",
      ],

      // Before send hook — enrich events
      beforeSend(event) {
        event.tags = {
          ...event.tags,
          app: "gane-server",
          platform: "node",
        };
        return event;
      },
    });

    // Sentry request handler (adds request context to events)
    Sentry.setupExpressErrorHandler(app);

    isInitialized = true;
    console.log("[Sentry/Server] Initialized successfully");
  } catch (err) {
    console.warn("[Sentry/Server] Failed to initialize:", (err as Error).message);
  }
}

/**
 * Express error handler middleware that reports to Sentry.
 * Should be registered AFTER all routes.
 */
export function sentryErrorHandler() {
  return (err: Error, req: Request, res: Response, next: NextFunction): void => {
    if (!isInitialized) {
      next(err);
      return;
    }

    Sentry.withScope((scope) => {
      // Add request context
      scope.setExtras({
        method: req.method,
        url: req.originalUrl,
        query: req.query,
        ip: req.ip,
        userAgent: req.headers["user-agent"],
      });

      // Add user context if available
      const user = (req as Request & { user?: { id?: string; openId?: string; name?: string } }).user;
      if (user) {
        scope.setUser({
          id: user.id || user.openId,
          username: user.name,
        });
      }

      // Tag by error type
      if (err.name) {
        scope.setTag("error.name", err.name);
      }

      Sentry.captureException(err);
    });

    next(err);
  };
}

/**
 * Capture a server-side error with context.
 */
export function captureServerError(
  error: Error,
  context?: Record<string, unknown>
): void {
  if (!isInitialized) {
    console.error("[Sentry/Server] Not initialized, logging error:", error.message);
    return;
  }

  Sentry.withScope((scope) => {
    if (context) {
      scope.setExtras(context);
    }
    Sentry.captureException(error);
  });
}

/**
 * Flush pending events before shutdown.
 */
export async function flushSentry(timeout = 2000): Promise<void> {
  if (!isInitialized) return;
  try {
    await Sentry.flush(timeout);
  } catch {
    // ignore flush errors during shutdown
  }
}
