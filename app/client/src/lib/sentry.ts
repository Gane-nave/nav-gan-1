/**
 * G.A.N.E — Sentry Client SDK Initialization
 * =============================================
 * Initializes Sentry for error tracking, performance monitoring,
 * and session replay on the client side.
 *
 * Environment variable: VITE_SENTRY_DSN
 */
import * as Sentry from "@sentry/react";

const SENTRY_DSN = import.meta.env.VITE_SENTRY_DSN || "";
const IS_PRODUCTION = import.meta.env.PROD;
const APP_VERSION = import.meta.env.VITE_APP_VERSION || "1.0.0";

let isInitialized = false;

export function initSentry(): void {
  if (isInitialized) return;
  if (!SENTRY_DSN) {
    if (IS_PRODUCTION) {
      console.warn("[Sentry] No DSN configured — error tracking disabled");
    }
    return;
  }

  try {
    Sentry.init({
      dsn: SENTRY_DSN,
      environment: IS_PRODUCTION ? "production" : "development",
      release: `gane@${APP_VERSION}`,

      // Performance monitoring
      tracesSampleRate: IS_PRODUCTION ? 0.1 : 1.0,

      // Session replay for debugging
      replaysSessionSampleRate: IS_PRODUCTION ? 0.01 : 0.1,
      replaysOnErrorSampleRate: 1.0,

      // Integrations
      integrations: [
        Sentry.browserTracingIntegration(),
        Sentry.replayIntegration({
          maskAllText: false,
          blockAllMedia: false,
        }),
        Sentry.breadcrumbsIntegration({
          console: true,
          dom: true,
          fetch: true,
          history: true,
          xhr: true,
        }),
      ],

      // Filter out noisy errors
      ignoreErrors: [
        // Browser extensions
        "ResizeObserver loop",
        "ResizeObserver loop completed with undelivered notifications",
        // Network errors (handled by retry logic)
        "Failed to fetch",
        "Load failed",
        "NetworkError",
        // Auth redirects (expected behavior)
        "Please login (10001)",
      ],

      // Don't send PII by default
      sendDefaultPii: false,

      // Normalize depth for context
      normalizeDepth: 5,

      // Before send hook — enrich or filter events
      beforeSend(event, hint) {
        const error = hint.originalException;

        // Add custom tags
        event.tags = {
          ...event.tags,
          app: "gane-nav",
          platform: "web",
        };

        // Add user context if available
        const userStr = localStorage.getItem("gane_user_context");
        if (userStr) {
          try {
            const user = JSON.parse(userStr);
            event.user = {
              id: user.id,
              username: user.name,
            };
          } catch {
            // ignore parse errors
          }
        }

        // Add browser context
        event.contexts = {
          ...event.contexts,
          browser_state: {
            online: navigator.onLine,
            url: window.location.href,
            viewport: `${window.innerWidth}x${window.innerHeight}`,
            language: navigator.language,
          },
        };

        return event;
      },

      // Before breadcrumb — filter noisy breadcrumbs
      beforeBreadcrumb(breadcrumb) {
        // Filter out debug collector requests
        if (
          breadcrumb.category === "fetch" &&
          breadcrumb.data?.url?.includes("/__manus__/")
        ) {
          return null;
        }
        // Filter out metrics endpoint
        if (
          breadcrumb.category === "fetch" &&
          breadcrumb.data?.url?.includes("/metrics")
        ) {
          return null;
        }
        return breadcrumb;
      },
    });

    isInitialized = true;
    console.log("[Sentry] Initialized successfully");
  } catch (err) {
    console.warn("[Sentry] Failed to initialize:", (err as Error).message);
  }
}

/**
 * Capture an error with additional context.
 */
export function captureError(
  error: Error,
  context?: Record<string, unknown>
): void {
  if (!isInitialized) {
    console.error("[Sentry] Not initialized, logging error:", error);
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
 * Capture a message with severity level.
 */
export function captureMessage(
  message: string,
  level: "info" | "warning" | "error" = "info"
): void {
  if (!isInitialized) return;
  Sentry.captureMessage(message, level);
}

/**
 * Set user context for all future events.
 */
export function setUser(user: { id: string; name?: string; email?: string } | null): void {
  if (!isInitialized) return;
  if (user) {
    Sentry.setUser({ id: user.id, username: user.name, email: user.email });
    // Also store for beforeSend hook
    localStorage.setItem("gane_user_context", JSON.stringify(user));
  } else {
    Sentry.setUser(null);
    localStorage.removeItem("gane_user_context");
  }
}

/**
 * Add a breadcrumb for tracking user actions.
 */
export function addBreadcrumb(
  category: string,
  message: string,
  data?: Record<string, unknown>
): void {
  if (!isInitialized) return;
  Sentry.addBreadcrumb({
    category,
    message,
    data,
    level: "info",
    timestamp: Date.now() / 1000,
  });
}

/**
 * Start a performance transaction.
 */
export function startTransaction(name: string, op: string) {
  if (!isInitialized) return undefined;
  return Sentry.startInactiveSpan({ name, op });
}

// Re-export Sentry for direct access when needed
export { Sentry };
