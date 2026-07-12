/**
 * G.A.N.E — Fleet Management Router (C4ISR)
 * ============================================
 * God's-Eye admin terminal with 1Hz live tracking.
 * VRP solver for multi-stop route optimization.
 * Remote Mission Injection (Ghost Tailing).
 */

import { z } from "zod";
import { eq, and, gte, lte, desc, sql, inArray } from "drizzle-orm";
import { protectedProcedure, router } from "../_core/trpc";
import { getDb } from "../db";
import { fleets, vehicles, missions, users } from "../../drizzle/schema";
import { nanoid } from "nanoid";
import { haversineDistance, calculateRouteDistance, twoOptImprove } from "./geo";

const createFleetSchema = z.object({
  name: z.string().min(1).max(128),
  nameHe: z.string().optional(),
  type: z.enum(["delivery", "emergency", "logistics", "transit", "private"]).default("private"),
  maxVehicles: z.number().int().min(1).max(10000).default(50),
});

const addVehicleSchema = z.object({
  fleetId: z.number().int(),
  name: z.string().optional(),
  licensePlate: z.string().optional(),
  type: z.enum(["car", "truck", "van", "motorcycle", "bicycle", "ambulance", "firetruck", "police"]).default("car"),
  deviceId: z.string().min(1).max(64),
  dimensions: z.object({
    length: z.number().optional(),
    width: z.number().optional(),
    height: z.number().optional(),
    weight: z.number().optional(),
  }).optional(),
});

const createMissionSchema = z.object({
  fleetId: z.number().int(),
  vehicleId: z.number().int().optional(),
  priority: z.number().int().min(1).max(5).default(3),
  type: z.enum(["delivery", "pickup", "patrol", "emergency", "custom"]).default("delivery"),
  originLat: z.number().optional(),
  originLon: z.number().optional(),
  originAddress: z.string().optional(),
  destinationLat: z.number().optional(),
  destinationLon: z.number().optional(),
  destinationAddress: z.string().optional(),
  waypoints: z.array(z.object({
    lat: z.number(),
    lon: z.number(),
    address: z.string().optional(),
    timeWindowStart: z.number().optional(),
    timeWindowEnd: z.number().optional(),
  })).optional(),
  timeWindowStart: z.number().optional(),
  timeWindowEnd: z.number().optional(),
  notes: z.string().optional(),
  isGhostTailing: z.boolean().default(false),
});

const vrpSolveSchema = z.object({
  fleetId: z.number().int(),
  depotLat: z.number(),
  depotLon: z.number(),
  stops: z.array(z.object({
    id: z.string(),
    lat: z.number(),
    lon: z.number(),
    demand: z.number().default(1),
    timeWindowStart: z.number().optional(),
    timeWindowEnd: z.number().optional(),
    serviceDuration: z.number().default(300), // seconds
  })).min(1).max(200),
  vehicleCapacity: z.number().default(20),
  maxVehicles: z.number().default(10),
});

export const fleetRouter = router({
  /** Create a new fleet */
  create: protectedProcedure
    .input(createFleetSchema)
    .mutation(async ({ input, ctx }) => {
      const db = await getDb();
      if (!db) return { success: false, error: "Database not available" };

      try {
        const result = await db.insert(fleets).values({
          name: input.name,
          nameHe: input.nameHe,
          ownerId: ctx.user.id,
          type: input.type,
          maxVehicles: input.maxVehicles,
        });

        return { success: true, fleetId: Number(result[0].insertId) };
      } catch (error) {
        console.error("[Fleet] Create error:", error);
        return { success: false, error: "Failed to create fleet" };
      }
    }),

  /** List user's fleets */
  list: protectedProcedure
    .query(async ({ ctx }) => {
      const db = await getDb();
      if (!db) return [];

      try {
        if (ctx.user.role === "admin") {
          return await db.select().from(fleets).orderBy(desc(fleets.createdAt));
        }
        return await db.select().from(fleets)
          .where(eq(fleets.ownerId, ctx.user.id))
          .orderBy(desc(fleets.createdAt));
      } catch (error) {
        console.error("[Fleet] List error:", error);
        return [];
      }
    }),

  /** Add a vehicle to a fleet */
  addVehicle: protectedProcedure
    .input(addVehicleSchema)
    .mutation(async ({ input }) => {
      const db = await getDb();
      if (!db) return { success: false };

      try {
        const result = await db.insert(vehicles).values({
          fleetId: input.fleetId,
          deviceId: input.deviceId,
          name: input.name,
          licensePlate: input.licensePlate,
          type: input.type,
          dimensions: input.dimensions,
        });

        return { success: true, vehicleId: Number(result[0].insertId) };
      } catch (error) {
        console.error("[Fleet] Add vehicle error:", error);
        return { success: false };
      }
    }),

  /** Get all vehicles in a fleet with live positions */
  vehicles: protectedProcedure
    .input(z.object({ fleetId: z.number().int() }))
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return [];

      try {
        return await db.select().from(vehicles)
          .where(eq(vehicles.fleetId, input.fleetId))
          .orderBy(desc(vehicles.lastSeen));
      } catch (error) {
        console.error("[Fleet] Vehicles error:", error);
        return [];
      }
    }),

  /** Create a mission (route assignment) */
  createMission: protectedProcedure
    .input(createMissionSchema)
    .mutation(async ({ input, ctx }) => {
      const db = await getDb();
      if (!db) return { success: false };

      try {
        const result = await db.insert(missions).values({
          fleetId: input.fleetId,
          vehicleId: input.vehicleId,
          dispatcherId: ctx.user.id,
          priority: input.priority,
          type: input.type,
          originLat: input.originLat,
          originLon: input.originLon,
          originAddress: input.originAddress,
          destinationLat: input.destinationLat,
          destinationLon: input.destinationLon,
          destinationAddress: input.destinationAddress,
          waypoints: input.waypoints,
          timeWindowStart: input.timeWindowStart ? new Date(input.timeWindowStart) : undefined,
          timeWindowEnd: input.timeWindowEnd ? new Date(input.timeWindowEnd) : undefined,
          notes: input.notes,
          isGhostTailing: input.isGhostTailing,
          status: input.vehicleId ? "assigned" : "pending",
        });

        return { success: true, missionId: Number(result[0].insertId) };
      } catch (error) {
        console.error("[Fleet] Create mission error:", error);
        return { success: false };
      }
    }),

  /** Get missions for a fleet */
  missions: protectedProcedure
    .input(z.object({
      fleetId: z.number().int(),
      status: z.enum(["pending", "assigned", "in_progress", "completed", "cancelled", "failed"]).optional(),
      limit: z.number().int().default(50),
    }))
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return [];

      try {
        const conditions = [eq(missions.fleetId, input.fleetId)];
        if (input.status) {
          conditions.push(eq(missions.status, input.status));
        }

        return await db.select().from(missions)
          .where(and(...conditions))
          .orderBy(desc(missions.createdAt))
          .limit(input.limit);
      } catch (error) {
        console.error("[Fleet] Missions error:", error);
        return [];
      }
    }),

  /** Update mission status */
  updateMissionStatus: protectedProcedure
    .input(z.object({
      missionId: z.number().int(),
      status: z.enum(["pending", "assigned", "in_progress", "completed", "cancelled", "failed"]),
    }))
    .mutation(async ({ input }) => {
      const db = await getDb();
      if (!db) return { success: false };

      try {
        const updateData: Record<string, unknown> = { status: input.status };
        if (input.status === "completed") {
          updateData.completedAt = new Date();
        }

        await db.update(missions)
          .set(updateData)
          .where(eq(missions.id, input.missionId));

        return { success: true };
      } catch (error) {
        console.error("[Fleet] Update mission error:", error);
        return { success: false };
      }
    }),

  /** VRP Solver — Nearest-Neighbor + 2-Opt improvement */
  solveVRP: protectedProcedure
    .input(vrpSolveSchema)
    .mutation(async ({ input }) => {
      const { depotLat, depotLon, stops, vehicleCapacity, maxVehicles } = input;

      // ─── Phase 1: Nearest-Neighbor Construction ───
      const routes: { vehicleIndex: number; stops: typeof stops; totalDistance: number }[] = [];
      const unassigned = [...stops];

      for (let v = 0; v < maxVehicles && unassigned.length > 0; v++) {
        const route: typeof stops = [];
        let currentLat = depotLat;
        let currentLon = depotLon;
        let currentDemand = 0;

        while (unassigned.length > 0) {
          // Find nearest unassigned stop
          let bestIdx = -1;
          let bestDist = Infinity;

          for (let i = 0; i < unassigned.length; i++) {
            if (currentDemand + unassigned[i].demand > vehicleCapacity) continue;
            const dist = haversineDistance(currentLat, currentLon, unassigned[i].lat, unassigned[i].lon);
            if (dist < bestDist) {
              bestDist = dist;
              bestIdx = i;
            }
          }

          if (bestIdx === -1) break; // capacity full or no reachable stops

          const stop = unassigned.splice(bestIdx, 1)[0];
          route.push(stop);
          currentLat = stop.lat;
          currentLon = stop.lon;
          currentDemand += stop.demand;
        }

        if (route.length > 0) {
          // ─── Phase 2: 2-Opt Improvement ───
          const improved = twoOptImprove(route, depotLat, depotLon);
          const totalDistance = calculateRouteDistance(improved, depotLat, depotLon);
          routes.push({ vehicleIndex: v, stops: improved, totalDistance });
        }
      }

      return {
        success: true,
        routes,
        totalVehicles: routes.length,
        totalDistance: routes.reduce((sum, r) => sum + r.totalDistance, 0),
        unassignedStops: unassigned.length,
      };
    }),

  /** Fleet statistics dashboard */
  stats: protectedProcedure
    .input(z.object({ fleetId: z.number().int() }))
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return null;

      try {
        const allVehicles = await db.select().from(vehicles)
          .where(eq(vehicles.fleetId, input.fleetId));

        const allMissions = await db.select().from(missions)
          .where(eq(missions.fleetId, input.fleetId));

        const now = Date.now();
        const fiveMinAgo = new Date(now - 300000);

        return {
          totalVehicles: allVehicles.length,
          activeVehicles: allVehicles.filter(v => v.status === "active" && v.lastSeen && v.lastSeen >= fiveMinAgo).length,
          idleVehicles: allVehicles.filter(v => v.status === "idle").length,
          offlineVehicles: allVehicles.filter(v => v.status === "offline" || !v.lastSeen || v.lastSeen < fiveMinAgo).length,
          totalMissions: allMissions.length,
          activeMissions: allMissions.filter(m => m.status === "in_progress").length,
          pendingMissions: allMissions.filter(m => m.status === "pending" || m.status === "assigned").length,
          completedMissions: allMissions.filter(m => m.status === "completed").length,
          avgBattery: allVehicles.reduce((sum, v) => sum + (v.batteryLevel ?? 100), 0) / Math.max(allVehicles.length, 1),
        };
      } catch (error) {
        console.error("[Fleet] Stats error:", error);
        return null;
      }
    }),
});

// VRP helpers (haversineDistance, calculateRouteDistance, twoOptImprove) imported from ./geo.ts
