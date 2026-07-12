/**
 * G.A.N.E — Trip Lifecycle Manager
 * ====================================
 * Full trip state machine: planned → active → paused → completed / cancelled
 *
 * Responsibilities:
 * - Trip creation with route, waypoints, and metadata
 * - State transitions with event logging
 * - ETA calculation and updates
 * - Trip event envelope (standardized event format)
 * - Trip replay data collection
 * - Integration with geofencing and alerts
 */

import { getDb } from "../db";
import { trips, tripEvents } from "../../drizzle/schema";
import { eq, desc } from "drizzle-orm";
import { TRPCError } from "@trpc/server";

async function requireDb() {
  const db = await getDb();
  if (!db) throw new TRPCError({ code: 'INTERNAL_SERVER_ERROR', message: 'Database unavailable' });
  return db;
}

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export type TripStatus = 'planned' | 'active' | 'paused' | 'completed' | 'cancelled' | 'failed';

export interface TripCreateInput {
  userId: number;
  deviceId: string;
  originLat: number;
  originLon: number;
  originAddress?: string;
  destLat?: number;
  destLon?: number;
  destAddress?: string;
  routeId?: string;
  navigationMode?: string;
}

export interface TripEventInput {
  tripId: string;
  deviceId: string;
  userId?: number;
  eventType: typeof tripEvents.$inferInsert['eventType'];
  lat?: number;
  lon?: number;
  accuracy?: number;
  confidence?: number;
  payload?: Record<string, unknown>;
}

// Valid state transitions
const VALID_TRANSITIONS: Record<TripStatus, TripStatus[]> = {
  planned:   ['active', 'cancelled'],
  active:    ['paused', 'completed', 'cancelled', 'failed'],
  paused:    ['active', 'cancelled'],
  completed: [],
  cancelled: [],
  failed:    [],
};

// ═══════════════════════════════════════════════════
// TRIP MANAGER
// ═══════════════════════════════════════════════════

export class TripManager {

  /**
   * Create a new trip
   */
  async createTrip(input: TripCreateInput): Promise<string> {
    const tripId = crypto.randomUUID();
    const now = Date.now();

    const db = await requireDb();
    await db.insert(trips).values({
      tripId,
      userId: input.userId,
      deviceId: input.deviceId,
      status: 'planned',
      originLat: input.originLat,
      originLon: input.originLon,
      originAddress: input.originAddress || null,
      destinationLat: input.destLat || null,
      destinationLon: input.destLon || null,
      destinationAddress: input.destAddress || null,
      routeId: input.routeId || null,
    });

    await this.emitEvent({
      tripId,
      deviceId: input.deviceId,
      userId: input.userId,
      eventType: 'trip_start',
      lat: input.originLat,
      lon: input.originLon,
      payload: {
        origin: input.originAddress || `${input.originLat},${input.originLon}`,
        destination: input.destAddress,
      },
    });

    return tripId;
  }

  /**
   * Transition trip state with validation
   */
  async transitionState(tripId: string, newStatus: TripStatus, deviceId: string, lat?: number, lon?: number): Promise<void> {
    const db = await requireDb();
    const trip = await db.select().from(trips).where(eq(trips.tripId, tripId)).limit(1);
    if (trip.length === 0) {
      throw new TRPCError({ code: 'NOT_FOUND', message: 'Trip not found' });
    }

    const currentStatus = trip[0].status as TripStatus;
    const allowed = VALID_TRANSITIONS[currentStatus] || [];

    if (!allowed.includes(newStatus)) {
      throw new TRPCError({
        code: 'BAD_REQUEST',
        message: `Cannot transition from '${currentStatus}' to '${newStatus}'`,
      });
    }

    const updates: Record<string, unknown> = {
      status: newStatus,
    };

    if (newStatus === 'active' && currentStatus === 'planned') {
      updates.startedAt = new Date();
    }

    if (newStatus === 'completed' || newStatus === 'failed') {
      updates.completedAt = new Date();
      if (trip[0].startedAt) {
        const startMs = new Date(trip[0].startedAt).getTime();
        updates.durationSeconds = Math.round((Date.now() - startMs) / 1000);
      }
    }

    await db.update(trips).set(updates).where(eq(trips.tripId, tripId));

    // Map status to event type
    const eventTypeMap: Record<string, TripEventInput['eventType']> = {
      active: 'trip_start',
      paused: 'trip_pause',
      completed: 'trip_end',
      cancelled: 'trip_end',
      failed: 'failure',
    };

    await this.emitEvent({
      tripId,
      deviceId,
      userId: trip[0].userId,
      eventType: eventTypeMap[newStatus] || 'trip_end',
      lat,
      lon,
      payload: { previousStatus: currentStatus, newStatus },
    });
  }

  /**
   * Update trip progress (distance, speed)
   */
  async updateProgress(
    tripId: string,
    deviceId: string,
    distanceTraveledM: number,
    avgSpeed: number,
    lat?: number,
    lon?: number
  ): Promise<void> {
    const db = await requireDb();
    await db.update(trips).set({
      distanceMeters: Math.round(distanceTraveledM),
      avgSpeed,
    }).where(eq(trips.tripId, tripId));

    await this.emitEvent({
      tripId,
      deviceId,
      eventType: 'eta_update',
      lat, lon,
      payload: {
        distanceTraveledM: Math.round(distanceTraveledM),
        avgSpeed,
      },
    });
  }

  /**
   * Get trip by tripId
   */
  async getTrip(tripId: string) {
    const db = await requireDb();
    const result = await db.select().from(trips).where(eq(trips.tripId, tripId)).limit(1);
    return result[0] || null;
  }

  /**
   * Get trips for a user
   */
  async getUserTrips(userId: number, limit = 50) {
    const db = await requireDb();
    return db.select().from(trips)
      .where(eq(trips.userId, userId))
      .orderBy(desc(trips.createdAt))
      .limit(limit);
  }

  /**
   * Get trip events for replay
   */
  async getTripEvents(tripId: string) {
    const db = await requireDb();
    return db.select().from(tripEvents)
      .where(eq(tripEvents.tripId, tripId))
      .orderBy(tripEvents.timestampServer);
  }

  /**
   * Emit a standardized trip event
   */
  private async emitEvent(input: TripEventInput): Promise<void> {
    const now = Date.now();
    const db = await requireDb();
    await db.insert(tripEvents).values({
      eventId: crypto.randomUUID(),
      tripId: input.tripId,
      deviceId: input.deviceId,
      userId: input.userId || null,
      eventType: input.eventType,
      timestampDevice: now,
      timestampServer: now,
      lat: input.lat || null,
      lon: input.lon || null,
      accuracy: input.accuracy || null,
      confidence: input.confidence || null,
      latencyMs: 0,
      outcome: 'success',
      payload: input.payload ? JSON.stringify(input.payload) : null,
    });
  }
}

// Singleton
export const tripManager = new TripManager();
