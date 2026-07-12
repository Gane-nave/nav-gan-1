/**
 * G.A.N.E — Anomaly & SLAM Router
 * ==================================
 * Crowdsourced SLAM: anomaly reporting, aggregation, and delta push.
 * When N >= 3 independent reports within 10m radius → Global Map Update.
 */

import { z } from "zod";
import { eq, and, gte, lte, desc, sql } from "drizzle-orm";
import { publicProcedure, protectedProcedure, router } from "../_core/trpc";
import { getDb } from "../db";
import { mapAnomalies, anomalyReports, deltaUpdates } from "../../drizzle/schema";
import { nanoid } from "nanoid";
import { haversineDistance, boundingBox } from "./geo";

const SLAM_THRESHOLD = 3;        // N reports to confirm
const CLUSTER_RADIUS_M = 10;     // meters for clustering

const reportAnomalySchema = z.object({
  type: z.enum([
    "roadblock", "pothole", "construction", "accident",
    "signal_jamming", "new_road", "road_closure", "flooding",
    "speed_trap", "hazard", "other"
  ]),
  lat: z.number().min(-90).max(90),
  lon: z.number().min(-180).max(180),
  description: z.string().optional(),
  descriptionHe: z.string().optional(),
  severity: z.number().int().min(1).max(5).optional().default(1),
  deviceId: z.string().min(1).max(64),
});

const nearbyAnomaliesSchema = z.object({
  lat: z.number().min(-90).max(90),
  lon: z.number().min(-180).max(180),
  radiusMeters: z.number().min(1).max(50000).default(2000),
  activeOnly: z.boolean().optional().default(true),
  limit: z.number().int().min(1).max(500).default(100),
});

export const anomalyRouter = router({
  /** Report a map anomaly */
  report: publicProcedure
    .input(reportAnomalySchema)
    .mutation(async ({ input, ctx }) => {
      const db = await getDb();
      if (!db) return { success: false, error: "Database not available" };

      try {
        // Find existing anomaly within cluster radius
        const latDelta = CLUSTER_RADIUS_M / 111320;
        const lonDelta = CLUSTER_RADIUS_M / (111320 * Math.cos(input.lat * Math.PI / 180));

        const existing = await db.select()
          .from(mapAnomalies)
          .where(and(
            eq(mapAnomalies.type, input.type),
            eq(mapAnomalies.isActive, true),
            gte(mapAnomalies.lat, input.lat - latDelta),
            lte(mapAnomalies.lat, input.lat + latDelta),
            gte(mapAnomalies.lon, input.lon - lonDelta),
            lte(mapAnomalies.lon, input.lon + lonDelta),
          ))
          .limit(1);

        let anomalyId: string;
        let isNewlyConfirmed = false;

        if (existing.length > 0) {
          // Existing anomaly — add report and increment count
          anomalyId = existing[0].anomalyId;
          const newCount = (existing[0].reportCount ?? 0) + 1;

          // Check if this device already reported
          const existingReport = await db.select()
            .from(anomalyReports)
            .where(and(
              eq(anomalyReports.anomalyId, anomalyId),
              eq(anomalyReports.deviceId, input.deviceId),
            ))
            .limit(1);

          if (existingReport.length > 0) {
            return { success: true, anomalyId, alreadyReported: true };
          }

          // Update anomaly count
          const updateData: Record<string, unknown> = {
            reportCount: newCount,
            severity: Math.max(existing[0].severity ?? 1, input.severity),
          };

          // SLAM trigger: N >= 3 → confirm anomaly
          if (newCount >= SLAM_THRESHOLD && !existing[0].isConfirmed) {
            updateData.isConfirmed = true;
            updateData.confirmedAt = new Date();
            isNewlyConfirmed = true;
          }

          await db.update(mapAnomalies)
            .set(updateData)
            .where(eq(mapAnomalies.anomalyId, anomalyId));
        } else {
          // New anomaly
          anomalyId = nanoid(16);
          await db.insert(mapAnomalies).values({
            anomalyId,
            type: input.type,
            lat: input.lat,
            lon: input.lon,
            severity: input.severity,
            description: input.description,
            descriptionHe: input.descriptionHe,
            reportCount: 1,
            reporterDeviceId: input.deviceId,
            reporterUserId: ctx.user?.id,
            // Auto-expire in 24h for temporary anomalies
            expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000),
          });
        }

        // Add individual report
        await db.insert(anomalyReports).values({
          anomalyId,
          deviceId: input.deviceId,
          userId: ctx.user?.id,
          lat: input.lat,
          lon: input.lon,
          type: input.type,
          confidence: 1.0,
        });

        // If newly confirmed, create delta update for broadcast
        if (isNewlyConfirmed) {
          await db.insert(deltaUpdates).values({
            anomalyId,
            type: input.type,
            payload: {
              action: "confirm_anomaly",
              anomalyId,
              type: input.type,
              lat: input.lat,
              lon: input.lon,
              severity: input.severity,
              description: input.description,
            },
            geofenceLat: input.lat,
            geofenceLon: input.lon,
            geofenceRadiusM: 5000, // 5km broadcast radius
          });
        }

        return { success: true, anomalyId, confirmed: isNewlyConfirmed };
      } catch (error) {
        console.error("[Anomaly] Report error:", error);
        return { success: false, error: "Report failed" };
      }
    }),

  /** Get anomalies near a location */
  nearby: publicProcedure
    .input(nearbyAnomaliesSchema)
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return [];

      try {
        const latDelta = input.radiusMeters / 111320;
        const lonDelta = input.radiusMeters / (111320 * Math.cos(input.lat * Math.PI / 180));

        const conditions = [
          gte(mapAnomalies.lat, input.lat - latDelta),
          lte(mapAnomalies.lat, input.lat + latDelta),
          gte(mapAnomalies.lon, input.lon - lonDelta),
          lte(mapAnomalies.lon, input.lon + lonDelta),
        ];

        if (input.activeOnly) {
          conditions.push(eq(mapAnomalies.isActive, true));
        }

        const results = await db.select()
          .from(mapAnomalies)
          .where(and(...conditions))
          .orderBy(desc(mapAnomalies.createdAt))
          .limit(input.limit);

        // Filter by actual distance and add distance field
        return results
          .map(r => ({
            ...r,
            distance: Math.round(haversineDistance(input.lat, input.lon, r.lat, r.lon)),
          }))
          .filter(r => r.distance <= input.radiusMeters)
          .sort((a, b) => a.distance - b.distance);
      } catch (error) {
        console.error("[Anomaly] Nearby query error:", error);
        return [];
      }
    }),

  /** Get pending delta updates for a geofence (for WebSocket push) */
  pendingDeltas: publicProcedure
    .input(z.object({
      lat: z.number(),
      lon: z.number(),
      radiusMeters: z.number().default(5000),
      since: z.number().optional(),
    }))
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return [];

      try {
        const latDelta = input.radiusMeters / 111320;
        const lonDelta = input.radiusMeters / (111320 * Math.cos(input.lat * Math.PI / 180));
        const sinceDate = input.since ? new Date(input.since) : new Date(Date.now() - 3600000);

        return await db.select()
          .from(deltaUpdates)
          .where(and(
            gte(deltaUpdates.geofenceLat, input.lat - latDelta),
            lte(deltaUpdates.geofenceLat, input.lat + latDelta),
            gte(deltaUpdates.geofenceLon, input.lon - lonDelta),
            lte(deltaUpdates.geofenceLon, input.lon + lonDelta),
            gte(deltaUpdates.createdAt, sinceDate),
          ))
          .orderBy(desc(deltaUpdates.createdAt))
          .limit(50);
      } catch (error) {
        console.error("[Anomaly] Delta query error:", error);
        return [];
      }
    }),

  /** Dismiss/deactivate an anomaly (admin or original reporter) */
  dismiss: protectedProcedure
    .input(z.object({ anomalyId: z.string() }))
    .mutation(async ({ input, ctx }) => {
      const db = await getDb();
      if (!db) return { success: false };

      try {
        await db.update(mapAnomalies)
          .set({ isActive: false })
          .where(eq(mapAnomalies.anomalyId, input.anomalyId));

        // Create delta update for dismissal
        const anomaly = await db.select()
          .from(mapAnomalies)
          .where(eq(mapAnomalies.anomalyId, input.anomalyId))
          .limit(1);

        if (anomaly.length > 0) {
          await db.insert(deltaUpdates).values({
            anomalyId: input.anomalyId,
            type: "dismiss",
            payload: { action: "dismiss_anomaly", anomalyId: input.anomalyId },
            geofenceLat: anomaly[0].lat,
            geofenceLon: anomaly[0].lon,
            geofenceRadiusM: 5000,
          });
        }

        return { success: true };
      } catch (error) {
        console.error("[Anomaly] Dismiss error:", error);
        return { success: false };
      }
    }),
});

// Haversine imported from ./geo.ts — single source of truth
