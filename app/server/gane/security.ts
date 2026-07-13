/**
 * G.A.N.E Security Hardening Layer
 * ==================================
 * 
 * - Rate limiting (sliding window per IP)
 * - Input sanitization (XSS prevention)
 * - Request validation (payload size, content-type)
 * - Coordinate bounds checking
 * - Device ID format validation
 * - Anomaly detection (brute force, replay attacks)
 */

import type { Request, Response, NextFunction } from "express";

// ─── Rate Limiter (Sliding Window) ───

interface RateLimitEntry {
  count: number;
  windowStart: number;
}

const rateLimitStore = new Map<string, RateLimitEntry>();

export function rateLimiter(
  maxRequests: number = 100,
  windowMs: number = 60_000
) {
  return (req: Request, res: Response, next: NextFunction) => {
    const key = req.ip || req.socket.remoteAddress || 'unknown';
    const now = Date.now();
    const entry = rateLimitStore.get(key);

    if (!entry || now - entry.windowStart > windowMs) {
      rateLimitStore.set(key, { count: 1, windowStart: now });
      return next();
    }

    if (entry.count >= maxRequests) {
      res.status(429).json({
        error: 'Too many requests',
        retryAfter: Math.ceil((entry.windowStart + windowMs - now) / 1000),
      });
      return;
    }

    entry.count++;
    next();
  };
}

// Periodic cleanup of expired entries
setInterval(() => {
  const now = Date.now();
  const entries = Array.from(rateLimitStore.entries());
  for (const [key, entry] of entries) {
    if (now - entry.windowStart > 120_000) {
      rateLimitStore.delete(key);
    }
  }
}, 60_000).unref?.();

// ─── Input Sanitization ───

/** Strip HTML tags and dangerous characters */
export function sanitizeString(input: string): string {
  return input
    .replace(/<[^>]*>/g, '')           // Strip HTML tags
    .replace(/[<>"'&]/g, '')           // Remove dangerous chars
    .replace(/javascript:/gi, '')       // Remove JS protocol
    .replace(/on\w+=/gi, '')           // Remove event handlers
    .trim();
}

/** Validate and sanitize a device ID */
export function validateDeviceId(deviceId: string): boolean {
  // Must be 1-64 chars, alphanumeric + hyphens + underscores
  return /^[a-zA-Z0-9_-]{1,64}$/.test(deviceId);
}

/** Validate coordinate bounds */
export function validateCoordinates(lat: number, lon: number): boolean {
  return (
    typeof lat === 'number' && !isNaN(lat) &&
    typeof lon === 'number' && !isNaN(lon) &&
    lat >= -90 && lat <= 90 &&
    lon >= -180 && lon <= 180
  );
}

/** Validate heading (0-360 degrees) */
export function validateHeading(heading: number): boolean {
  return typeof heading === 'number' && !isNaN(heading) && heading >= 0 && heading <= 360;
}

/** Validate speed (0-500 km/h reasonable range) */
export function validateSpeed(speed: number): boolean {
  return typeof speed === 'number' && !isNaN(speed) && speed >= 0 && speed <= 500;
}

// ─── Request Validation Middleware ───

/** Validate JSON payload size */
export function maxPayloadSize(maxBytes: number = 1_000_000) {
  return (req: Request, res: Response, next: NextFunction) => {
    const contentLength = parseInt(req.headers['content-length'] || '0', 10);
    if (contentLength > maxBytes) {
      res.status(413).json({ error: 'Payload too large' });
      return;
    }
    next();
  };
}

/** Validate content type for POST/PUT */
export function requireJsonContentType(req: Request, res: Response, next: NextFunction) {
  if (['POST', 'PUT', 'PATCH'].includes(req.method)) {
    const contentType = req.headers['content-type'];
    if (!contentType || !contentType.includes('application/json')) {
      // Allow tRPC requests which may have different content types
      if (req.path.startsWith('/api/trpc')) {
        return next();
      }
      res.status(415).json({ error: 'Content-Type must be application/json' });
      return;
    }
  }
  next();
}

// ─── Telemetry Validation ───

export interface ValidatedTelemetry {
  deviceId: string;
  lat: number;
  lon: number;
  alt?: number;
  velocity?: number;
  heading?: number;
  satellites?: number;
  hdop?: number;
  batteryLevel?: number;
}

/** Validate a telemetry payload */
export function validateTelemetryPayload(data: unknown): { valid: boolean; errors: string[]; data?: ValidatedTelemetry } {
  const errors: string[] = [];
  
  if (!data || typeof data !== 'object') {
    return { valid: false, errors: ['Invalid payload'] };
  }

  const d = data as Record<string, unknown>;

  // Required fields
  if (!d.deviceId || typeof d.deviceId !== 'string' || !validateDeviceId(d.deviceId)) {
    errors.push('Invalid deviceId');
  }
  if (typeof d.lat !== 'number' || typeof d.lon !== 'number' || !validateCoordinates(d.lat, d.lon)) {
    errors.push('Invalid coordinates');
  }

  // Optional fields
  if (d.velocity !== undefined && !validateSpeed(Number(d.velocity))) {
    errors.push('Invalid velocity');
  }
  if (d.heading !== undefined && !validateHeading(Number(d.heading))) {
    errors.push('Invalid heading');
  }

  if (errors.length > 0) {
    return { valid: false, errors };
  }

  return {
    valid: true,
    errors: [],
    data: {
      deviceId: sanitizeString(String(d.deviceId)),
      lat: Number(d.lat),
      lon: Number(d.lon),
      alt: d.alt !== undefined ? Number(d.alt) : undefined,
      velocity: d.velocity !== undefined ? Number(d.velocity) : undefined,
      heading: d.heading !== undefined ? Number(d.heading) : undefined,
      satellites: d.satellites !== undefined ? Number(d.satellites) : undefined,
      hdop: d.hdop !== undefined ? Number(d.hdop) : undefined,
      batteryLevel: d.batteryLevel !== undefined ? Number(d.batteryLevel) : undefined,
    },
  };
}

// ─── Replay Attack Detection ───

const recentRequests = new Map<string, number>();

/** Detect replay attacks by checking for duplicate request signatures */
export function detectReplay(deviceId: string, timestamp: number): boolean {
  const key = `${deviceId}_${timestamp}`;
  if (recentRequests.has(key)) {
    return true; // Replay detected
  }
  recentRequests.set(key, Date.now());
  return false;
}

// Cleanup old replay entries every 30 seconds
setInterval(() => {
  const cutoff = Date.now() - 30_000;
  const entries = Array.from(recentRequests.entries());
  for (const [key, ts] of entries) {
    if (ts < cutoff) {
      recentRequests.delete(key);
    }
  }
}, 30_000).unref?.();

// ─── Security Headers Middleware ───

export function securityHeaders(req: Request, res: Response, next: NextFunction) {
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-XSS-Protection', '1; mode=block');
  res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
  next();
}
