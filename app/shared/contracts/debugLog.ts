/**
 * G.A.N.E — Debug / Diagnostic Log Helper (shared)
 * ===================================================
 * Replaces scattered `console.log` / `console.debug` calls with a single
 * routed entry point that:
 *   - In dev (`import.meta.env.DEV` where available / `NODE_ENV !== 'production'`
 *     otherwise), prints to `console[level]` with a module tag.
 *   - In prod, silences `debug` and `info`, forwards `warn` / `error`
 *     to a registered reporter (Sentry, OTel collector, etc.).
 *
 * Addresses `AUDIT_CODE_LEVEL_PASS2.md` §2.2 — 47 stray console.log calls.
 *
 * The reporter is pluggable to avoid a hard dependency on Sentry in the
 * shared layer; `client/src/lib/sentry.ts` registers the real one.
 */

export type LogLevel = "debug" | "info" | "warn" | "error";

export interface LogEvent {
  tag: string;
  level: LogLevel;
  message: string;
  meta?: Record<string, unknown>;
  timestamp: number;
}

export type LogReporter = (ev: LogEvent) => void;

let reporter: LogReporter | null = null;
let overrideDevMode: boolean | null = null;

export function setLogReporter(r: LogReporter | null): void {
  reporter = r;
}

/** Force dev/prod mode (tests only). */
export function __setLogDevMode(v: boolean | null): void {
  overrideDevMode = v;
}

function isDev(): boolean {
  if (overrideDevMode !== null) return overrideDevMode;
  const env = (globalThis as unknown as { process?: { env?: { NODE_ENV?: string } } }).process?.env;
  if (env && typeof env.NODE_ENV === "string") return env.NODE_ENV !== "production";
  // Browser: default to dev (Vite dev server) · production build usually injects NODE_ENV.
  return true;
}

export function log(
  tag: string,
  level: LogLevel,
  message: string,
  meta?: Record<string, unknown>,
): void {
  const ev: LogEvent = { tag, level, message, meta, timestamp: Date.now() };
  const dev = isDev();

  if (dev) {
    const prefix = `[${tag}]`;
    const payload = meta === undefined ? [prefix, message] : [prefix, message, meta];
    switch (level) {
      case "debug": console.debug(...payload); break;
      case "info":  console.info(...payload);  break;
      case "warn":  console.warn(...payload);  break;
      case "error": console.error(...payload); break;
    }
  } else if (level === "warn" || level === "error") {
    // Prod · still surface warnings/errors through the reporter + console
    if (reporter) {
      try { reporter(ev); } catch { /* reporter must never throw */ }
    }
    if (level === "error") console.error(`[${tag}]`, message, meta ?? "");
  }

  // Always hand events to the reporter when one is registered, so tests can
  // observe the full stream without depending on console shape.
  if (!dev && reporter && level !== "warn" && level !== "error") {
    try { reporter(ev); } catch { /* swallow */ }
  }
}

// ── Module-scoped convenience factory ─────────────────────────────────────

export interface ScopedLogger {
  debug(message: string, meta?: Record<string, unknown>): void;
  info(message: string, meta?: Record<string, unknown>): void;
  warn(message: string, meta?: Record<string, unknown>): void;
  error(message: string, meta?: Record<string, unknown>): void;
}

/**
 * Module-level logger. Usage:
 *   const logger = createLogger('chaosTesting');
 *   logger.debug('scenario injected', { scenario });
 */
export function createLogger(tag: string): ScopedLogger {
  return {
    debug: (m, meta) => log(tag, "debug", m, meta),
    info:  (m, meta) => log(tag, "info",  m, meta),
    warn:  (m, meta) => log(tag, "warn",  m, meta),
    error: (m, meta) => log(tag, "error", m, meta),
  };
}
