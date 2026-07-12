/**
 * G.A.N.E — Telemetry API Router
 * =================================
 * Handles telemetry ingestion and spatial queries.
 * Designed for high-throughput (1Hz from millions of clients).
 */

import { z } from "zod";
import { eq, and, gte, lte, sql, desc } from "drizzle-orm";
import { publicProcedure, protectedProcedure, router } from "../_core/trpc";
import { getDb } from "../db";
import { rawTelemetry, vehicles } from "../../drizzle/schema";
import { haversineDistance, boundingBox } from "./geo";

const telemetryInputSchema = z.object({
  deviceId: z.string().min(1).max(64),
  lat: z.number().min(-90).max(90),
  lon: z.number().min(-180).max(180),
  alt: z.number().optional().default(0),
  velocity: z.number().optional().default(0),
  heading: z.number().optional().default(0),
  confidenceScore: z.number().min(0).max(1).optional().default(1.0),
  navigationMode: z.enum(["BOOTING", "OPTIMAL_FUSION", "DEGRADED_MODE", "EMERGENCY_BOUNDED"]).optional(),
  satellites: z.number().int().optional().default(0),
  hdop: z.number().optional(),
  batteryLevel: z.number().optional(),
  sensorStatusGnss: z.boolean().optional().default(true),
  sensorStatusImu: z.boolean().optional().default(true),
  sensorStatusVision: z.boolean().optional().default(false),
  sensorStatusNetwork: z.boolean().optional().default(true),
});

const nearbyQuerySchema = z.object({
  lat: z.number().min(-90).max(90),
  lon: z.number().min(-180).max(180),
  radiusMeters: z.number().min(1).max(50000).default(500),
  limit: z.number().int().min(1).max(1000).default(100),
  since: z.number().optional(), // timestamp in ms
});

const deviceHistorySchema = z.object({
  deviceId: z.string().min(1),
  from: z.number(), // timestamp in ms
  to: z.number(),   // timestamp in ms
  limit: z.number().int().min(1).max(10000).default(1000),
});

export const telemetryRouter = router({
  /** Ingest a single telemetry point */
  ingest: publicProcedure
    .input(telemetryInputSchema)
    .mutation(async ({ input }) => {
      const db = await getDb();
      if (!db) return { success: false, error: "Database not available" };

      try {
        await db.insert(rawTelemetry).values({
          deviceId: input.deviceId,
          timestamp: new Date(),
          lat: input.lat,
          lon: input.lon,
          alt: input.alt,
          velocity: input.velocity,
          heading: input.heading,
          confidenceScore: input.confidenceScore,
          navigationMode: input.navigationMode,
          satellites: input.satellites,
          hdop: input.hdop,
          batteryLevel: input.batteryLevel,
          sensorStatusGnss: input.sensorStatusGnss,
          sensorStatusImu: input.sensorStatusImu,
          sensorStatusVision: input.sensorStatusVision,
          sensorStatusNetwork: input.sensorStatusNetwork,
        });

        // Also update vehicle last-known position
        await db.update(vehicles)
          .set({
            lastLat: input.lat,
            lastLon: input.lon,
            lastHeading: input.heading,
            lastSpeed: input.velocity,
            lastSeen: new Date(),
            batteryLevel: input.batteryLevel,
            status: "active",
          })
          .where(eq(vehicles.deviceId, input.deviceId));

        return { success: true };
      } catch (error) {
        console.error("[Telemetry] Ingest error:", error);
        return { success: false, error: "Ingestion failed" };
      }
    }),

  /** Batch ingest telemetry (for offline sync) */
  ingestBatch: publicProcedure
    .input(z.object({ points: z.array(telemetryInputSchema).max(1000) }))
    .mutation(async ({ input }) => {
      const db = await getDb();
      if (!db) return { success: false, ingested: 0 };

      try {
        const values = input.points.map(p => ({
          deviceId: p.deviceId,
          timestamp: new Date(),
          lat: p.lat,
          lon: p.lon,
          alt: p.alt,
          velocity: p.velocity,
          heading: p.heading,
          confidenceScore: p.confidenceScore,
          navigationMode: p.navigationMode,
          satellites: p.satellites,
          hdop: p.hdop,
          batteryLevel: p.batteryLevel,
          sensorStatusGnss: p.sensorStatusGnss,
          sensorStatusImu: p.sensorStatusImu,
          sensorStatusVision: p.sensorStatusVision,
          sensorStatusNetwork: p.sensorStatusNetwork,
        }));

        // Batch insert in chunks of 100
        const chunkSize = 100;
        let ingested = 0;
        for (let i = 0; i < values.length; i += chunkSize) {
          const chunk = values.slice(i, i + chunkSize);
          await db.insert(rawTelemetry).values(chunk);
          ingested += chunk.length;
        }

        return { success: true, ingested };
      } catch (error) {
        console.error("[Telemetry] Batch ingest error:", error);
        return { success: false, ingested: 0 };
      }
    }),

  /** Find vehicles/telemetry near a point (spatial query) */
  nearby: publicProcedure
    .input(nearbyQuerySchema)
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return [];

      try {
        // Approximate degree offset for the radius
        // 1 degree lat ≈ 111,320m, 1 degree lon ≈ 111,320 * cos(lat)m
        const latDelta = input.radiusMeters / 111320;
        const lonDelta = input.radiusMeters / (111320 * Math.cos(input.lat * Math.PI / 180));

        const minLat = input.lat - latDelta;
        const maxLat = input.lat + latDelta;
        const minLon = input.lon - lonDelta;
        const maxLon = input.lon + lonDelta;

        // Query using bounding box + Haversine for precision
        const sinceDate = input.since ? new Date(input.since) : new Date(Date.now() - 300000); // default: last 5 min

        const results = await db.select()
          .from(rawTelemetry)
          .where(and(
            gte(rawTelemetry.lat, minLat),
            lte(rawTelemetry.lat, maxLat),
            gte(rawTelemetry.lon, minLon),
            lte(rawTelemetry.lon, maxLon),
            gte(rawTelemetry.timestamp, sinceDate),
          ))
          .orderBy(desc(rawTelemetry.timestamp))
          .limit(input.limit);

        // Filter by actual Haversine distance
        return results.filter(r => {
          const dist = haversineDistance(input.lat, input.lon, r.lat, r.lon);
          return dist <= input.radiusMeters;
        }).map(r => ({
          ...r,
          distance: Math.round(haversineDistance(input.lat, input.lon, r.lat, r.lon)),
        }));
      } catch (error) {
        console.error("[Telemetry] Nearby query error:", error);
        return [];
      }
    }),

  /** Get device telemetry history */
  deviceHistory: protectedProcedure
    .input(deviceHistorySchema)
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return [];

      try {
        return await db.select()
          .from(rawTelemetry)
          .where(and(
            eq(rawTelemetry.deviceId, input.deviceId),
            gte(rawTelemetry.timestamp, new Date(input.from)),
            lte(rawTelemetry.timestamp, new Date(input.to)),
          ))
          .orderBy(desc(rawTelemetry.timestamp))
          .limit(input.limit);
      } catch (error) {
        console.error("[Telemetry] History query error:", error);
        return [];
      }
    }),

  /** Get latest position for all active devices */
  activeDevices: protectedProcedure
    .query(async () => {
      const db = await getDb();
      if (!db) return [];

      try {
        // Get vehicles that were seen in the last 5 minutes
        const fiveMinAgo = new Date(Date.now() - 300000);
        return await db.select()
          .from(vehicles)
          .where(and(
            eq(vehicles.status, "active"),
            gte(vehicles.lastSeen, fiveMinAgo),
          ))
          .limit(1000);
      } catch (error) {
        console.error("[Telemetry] Active devices error:", error);
        return [];
      }
    }),
});

// Haversine imported from ./geo.ts — single source of truth
