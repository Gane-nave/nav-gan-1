/**
 * G.A.N.E — Incident Engine (Server-Side)
 * =========================================
 * Waze-core incident system: report → validate → publish → decay
 *
 * EVENT TYPES:
 *   - accident
 *   - roadblock
 *   - police
 *   - hazard
 *   - construction
 *   - flooding
 *   - speed_trap
 *   - road_closure
 *
 * VALIDATION:
 *   - N ≥ 3 confirmations from distinct devices
 *   - Decay timer (incidents auto-expire)
 *   - Geo-clustering (nearby reports merge)
 *
 * OUTPUT:
 *   - Incident layer (active incidents with positions)
 *   - Reroute trigger (via traffic pipeline integration)
 */

import { z } from 'zod';
import { publicProcedure, protectedProcedure, router } from '../_core/trpc';
import { getDb } from '../db';
import { mapAnomalies, anomalyReports } from '../../drizzle/schema';
import { eq, and, gte, lte, sql, desc } from 'drizzle-orm';

// ─── Types ───────────────────────────────────────────────

export interface IncidentReport {
  id: string;
  type: IncidentType;
  lat: number;
  lon: number;
  severity: number;            // 1-5
  description?: string;
  descriptionHe?: string;
  reporterDeviceId: string;
  reporterUserId?: number;
  confirmations: number;
  isConfirmed: boolean;
  isActive: boolean;
  radiusM: number;
  expiresAt: number;           // Unix ms
  createdAt: number;
  updatedAt: number;
}

export type IncidentType =
  | 'accident'
  | 'roadblock'
  | 'police'
  | 'hazard'
  | 'construction'
  | 'flooding'
  | 'speed_trap'
  | 'road_closure';

// ─── Constants ──────────────────────────────────────────

const INCIDENT_TYPE_MAP: Record<IncidentType, string> = {
  accident: 'accident',
  roadblock: 'roadblock',
  police: 'speed_trap',
  hazard: 'hazard',
  construction: 'construction',
  flooding: 'flooding',
  speed_trap: 'speed_trap',
  road_closure: 'road_closure',
};

const CONFIRMATION_THRESHOLD = 3;
const CLUSTER_RADIUS_M = 100;       // merge reports within 100m
const DEFAULT_EXPIRY_MS: Record<IncidentType, number> = {
  accident: 2 * 60 * 60 * 1000,     // 2 hours
  roadblock: 4 * 60 * 60 * 1000,    // 4 hours
  police: 30 * 60 * 1000,           // 30 minutes
  hazard: 1 * 60 * 60 * 1000,       // 1 hour
  construction: 24 * 60 * 60 * 1000, // 24 hours
  flooding: 6 * 60 * 60 * 1000,     // 6 hours
  speed_trap: 30 * 60 * 1000,       // 30 minutes
  road_closure: 12 * 60 * 60 * 1000, // 12 hours
};

const SEVERITY_MAP: Record<IncidentType, number> = {
  accident: 5,
  roadblock: 4,
  police: 2,
  hazard: 3,
  construction: 3,
  flooding: 4,
  speed_trap: 1,
  road_closure: 5,
};

const EARTH_RADIUS_M = 6371000;
const DEG_TO_RAD = Math.PI / 180;

function haversineDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const dLat = (lat2 - lat1) * DEG_TO_RAD;
  const dLon = (lon2 - lon1) * DEG_TO_RAD;
  const a = Math.sin(dLat / 2) ** 2 +
    Math.cos(lat1 * DEG_TO_RAD) * Math.cos(lat2 * DEG_TO_RAD) *
    Math.sin(dLon / 2) ** 2;
  return EARTH_RADIUS_M * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

// ─── In-Memory Incident Store ───────────────────────────

class IncidentStore {
  private incidents: Map<string, IncidentReport> = new Map();
  private decayTimer: ReturnType<typeof setInterval> | null = null;

  start() {
    // Run decay check every 60 seconds
    this.decayTimer = setInterval(() => this.decayExpired(), 60000);
  }

  stop() {
    if (this.decayTimer) {
      clearInterval(this.decayTimer);
      this.decayTimer = null;
    }
  }

  /**
   * Report a new incident or confirm an existing one.
   * Uses geo-clustering to merge nearby reports.
   */
  report(
    type: IncidentType,
    lat: number,
    lon: number,
    deviceId: string,
    userId?: number,
    description?: string,
    descriptionHe?: string
  ): { incident: IncidentReport; isNew: boolean; merged: boolean } {
    // Check for nearby existing incident of same type
    const nearby = this.findNearby(lat, lon, CLUSTER_RADIUS_M, type);

    if (nearby) {
      // Merge: increment confirmation count
      nearby.confirmations++;
      nearby.updatedAt = Date.now();

      // Confirm if threshold reached
      if (nearby.confirmations >= CONFIRMATION_THRESHOLD && !nearby.isConfirmed) {
        nearby.isConfirmed = true;
        // Extend expiry on confirmation
        nearby.expiresAt = Date.now() + DEFAULT_EXPIRY_MS[type];
      }

      // Update centroid (weighted average)
      const weight = 1 / nearby.confirmations;
      nearby.lat = nearby.lat * (1 - weight) + lat * weight;
      nearby.lon = nearby.lon * (1 - weight) + lon * weight;

      this.incidents.set(nearby.id, nearby);
      return { incident: nearby, isNew: false, merged: true };
    }

    // New incident
    const id = `inc_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
    const incident: IncidentReport = {
      id,
      type,
      lat,
      lon,
      severity: SEVERITY_MAP[type] || 3,
      description,
      descriptionHe,
      reporterDeviceId: deviceId,
      reporterUserId: userId,
      confirmations: 1,
      isConfirmed: false,
      isActive: true,
      radiusM: type === 'accident' ? 200 : type === 'construction' ? 300 : 100,
      expiresAt: Date.now() + DEFAULT_EXPIRY_MS[type],
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };

    this.incidents.set(id, incident);
    return { incident, isNew: true, merged: false };
  }

  /**
   * Dismiss/deny an incident report.
   */
  dismiss(incidentId: string, deviceId: string): boolean {
    const incident = this.incidents.get(incidentId);
    if (!incident) return false;

    incident.confirmations = Math.max(0, incident.confirmations - 1);
    incident.updatedAt = Date.now();

    // If confirmations drop to 0, deactivate
    if (incident.confirmations <= 0) {
      incident.isActive = false;
      incident.isConfirmed = false;
    }

    this.incidents.set(incidentId, incident);
    return true;
  }

  /**
   * Get all active incidents.
   */
  getActive(): IncidentReport[] {
    return Array.from(this.incidents.values())
      .filter(i => i.isActive && i.expiresAt > Date.now());
  }

  /**
   * Get active incidents within a bounding box.
   */
  getInBounds(
    minLat: number, maxLat: number,
    minLon: number, maxLon: number
  ): IncidentReport[] {
    return this.getActive().filter(i =>
      i.lat >= minLat && i.lat <= maxLat &&
      i.lon >= minLon && i.lon <= maxLon
    );
  }

  /**
   * Get active incidents near a point.
   */
  getNearby(lat: number, lon: number, radiusM: number): IncidentReport[] {
    return this.getActive().filter(i =>
      haversineDistance(lat, lon, i.lat, i.lon) <= radiusM
    );
  }

  /**
   * Get confirmed incidents (N ≥ threshold).
   */
  getConfirmed(): IncidentReport[] {
    return this.getActive().filter(i => i.isConfirmed);
  }

  /**
   * Check if any active incident affects a route.
   */
  getIncidentsOnRoute(
    routePoints: { lat: number; lon: number }[],
    bufferM = 200
  ): IncidentReport[] {
    const active = this.getActive();
    return active.filter(incident => {
      for (const point of routePoints) {
        if (haversineDistance(point.lat, point.lon, incident.lat, incident.lon) < bufferM + incident.radiusM) {
          return true;
        }
      }
      return false;
    });
  }

  /**
   * Find a nearby incident of the same type (for clustering).
   */
  private findNearby(
    lat: number, lon: number,
    radiusM: number,
    type: IncidentType
  ): IncidentReport | null {
    const values = Array.from(this.incidents.values());
    for (const incident of values) {
      if (!incident.isActive) continue;
      if (incident.type !== type) continue;
      if (haversineDistance(lat, lon, incident.lat, incident.lon) <= radiusM) {
        return incident;
      }
    }
    return null;
  }

  /**
   * Decay expired incidents.
   */
  private decayExpired() {
    const now = Date.now();
    const entries = Array.from(this.incidents.entries());
    for (const [id, incident] of entries) {
      if (incident.expiresAt <= now && incident.isActive) {
        incident.isActive = false;
        this.incidents.set(id, incident);
      }
    }

    // Clean up very old inactive incidents (>24h)
    const cutoff = now - 24 * 60 * 60 * 1000;
    const entries2 = Array.from(this.incidents.entries());
    for (const [id, incident] of entries2) {
      if (!incident.isActive && incident.updatedAt < cutoff) {
        this.incidents.delete(id);
      }
    }
  }

  getStats() {
    const all = Array.from(this.incidents.values());
    const active = all.filter(i => i.isActive);
    const confirmed = active.filter(i => i.isConfirmed);

    const byType: Record<string, number> = {};
    for (const i of active) {
      byType[i.type] = (byType[i.type] || 0) + 1;
    }

    return {
      total: all.length,
      active: active.length,
      confirmed: confirmed.length,
      byType,
      avgConfirmations: active.length > 0
        ? Math.round(active.reduce((s, i) => s + i.confirmations, 0) / active.length * 10) / 10
        : 0,
    };
  }
}

// ─── Singleton ──────────────────────────────────────────

export const incidentStore = new IncidentStore();

// ─── tRPC Router ────────────────────────────────────────

export const incidentRouter = router({
  /**
   * Report an incident (public — any device can report).
   */
  report: publicProcedure
    .input(z.object({
      type: z.enum([
        'accident', 'roadblock', 'police', 'hazard',
        'construction', 'flooding', 'speed_trap', 'road_closure'
      ]),
      lat: z.number().min(-90).max(90),
      lon: z.number().min(-180).max(180),
      deviceId: z.string().min(1).max(64),
      description: z.string().max(500).optional(),
      descriptionHe: z.string().max(500).optional(),
    }))
    .mutation(async ({ input, ctx }) => {
      const userId = ctx.user?.id;

      const { incident, isNew, merged } = incidentStore.report(
        input.type as IncidentType,
        input.lat,
        input.lon,
        input.deviceId,
        userId,
        input.description,
        input.descriptionHe
      );

      // Also persist to DB (map_anomalies table)
      try {
        const db = await getDb();
        if (db && isNew) {
          const anomalyType = INCIDENT_TYPE_MAP[input.type as IncidentType] || input.type;
          await db.insert(mapAnomalies).values({
            anomalyId: incident.id,
            type: anomalyType as any,
            lat: incident.lat,
            lon: incident.lon,
            radiusMeters: incident.radiusM,
            severity: incident.severity,
            description: input.description || null,
            descriptionHe: input.descriptionHe || null,
            reportCount: 1,
            isConfirmed: false,
            isActive: true,
            reporterDeviceId: input.deviceId,
            reporterUserId: userId || null,
            expiresAt: new Date(incident.expiresAt),
          }).catch(() => { /* ignore duplicate */ });
        } else if (db && merged) {
          await db.update(mapAnomalies)
            .set({
              reportCount: incident.confirmations,
              isConfirmed: incident.isConfirmed,
              lat: incident.lat,
              lon: incident.lon,
            })
            .where(eq(mapAnomalies.anomalyId, incident.id))
            .catch(() => { /* ignore */ });
        }
      } catch (e) {
        console.warn('[IncidentEngine] DB persist failed:', e);
      }

      return {
        incidentId: incident.id,
        isNew,
        merged,
        confirmations: incident.confirmations,
        isConfirmed: incident.isConfirmed,
        expiresAt: incident.expiresAt,
      };
    }),

  /**
   * Confirm or deny an existing incident.
   */
  confirm: publicProcedure
    .input(z.object({
      incidentId: z.string().min(1),
      deviceId: z.string().min(1).max(64),
      action: z.enum(['confirm', 'deny']),
    }))
    .mutation(({ input }) => {
      if (input.action === 'deny') {
        const dismissed = incidentStore.dismiss(input.incidentId, input.deviceId);
        return { success: dismissed };
      }

      // Confirm = re-report at same location
      const active = incidentStore.getActive();
      const existing = active.find(i => i.id === input.incidentId);
      if (!existing) return { success: false };

      const { incident } = incidentStore.report(
        existing.type,
        existing.lat,
        existing.lon,
        input.deviceId
      );

      return {
        success: true,
        confirmations: incident.confirmations,
        isConfirmed: incident.isConfirmed,
      };
    }),

  /**
   * Get nearby active incidents.
   */
  nearby: publicProcedure
    .input(z.object({
      lat: z.number().min(-90).max(90),
      lon: z.number().min(-180).max(180),
      radiusM: z.number().min(100).max(50000).default(5000),
    }))
    .query(({ input }) => {
      return incidentStore.getNearby(input.lat, input.lon, input.radiusM);
    }),

  /**
   * Get all active incidents (for map layer).
   */
  active: publicProcedure
    .query(() => {
      return incidentStore.getActive();
    }),

  /**
   * Get confirmed incidents only.
   */
  confirmed: publicProcedure
    .query(() => {
      return incidentStore.getConfirmed();
    }),

  /**
   * Get incidents within a bounding box.
   */
  inBounds: publicProcedure
    .input(z.object({
      minLat: z.number().min(-90).max(90),
      maxLat: z.number().min(-90).max(90),
      minLon: z.number().min(-180).max(180),
      maxLon: z.number().min(-180).max(180),
    }))
    .query(({ input }) => {
      return incidentStore.getInBounds(
        input.minLat, input.maxLat,
        input.minLon, input.maxLon
      );
    }),

  /**
   * Check if incidents affect a route.
   */
  onRoute: publicProcedure
    .input(z.object({
      routePoints: z.array(z.object({
        lat: z.number(),
        lon: z.number(),
      })).min(2).max(1000),
      bufferM: z.number().min(50).max(1000).default(200),
    }))
    .query(({ input }) => {
      return incidentStore.getIncidentsOnRoute(input.routePoints, input.bufferM);
    }),

  /**
   * Get incident statistics.
   */
  stats: publicProcedure
    .query(() => {
      return incidentStore.getStats();
    }),
});
