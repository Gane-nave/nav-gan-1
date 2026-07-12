/**
 * G.A.N.E — Geofencing Engine
 * ===============================
 * Server-side geofence management with real-time enter/exit detection.
 *
 * Features:
 * - Circular and polygon geofences
 * - Per-user and global geofences
 * - Enter/exit event emission
 * - Integration with alert system
 * - Batch position checking for fleet
 */

import { getDb } from "../db";
import { geofences, alerts } from "../../drizzle/schema";
import { eq, and } from "drizzle-orm";

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export interface GeofenceCheck {
  deviceId: string;
  userId: number;
  lat: number;
  lon: number;
  tripId?: string;
}

export interface GeofenceEvent {
  geofenceId: string;
  name: string;
  type: 'enter' | 'exit';
  deviceId: string;
  userId: number;
  lat: number;
  lon: number;
  timestamp: number;
}

// In-memory state: deviceId → set of geofence IDs the device is currently inside
const deviceGeofenceState = new Map<string, Set<string>>();

const EARTH_RADIUS = 6_371_000;
const DEG2RAD = Math.PI / 180;

// ═══════════════════════════════════════════════════
// GEOFENCE ENGINE
// ═══════════════════════════════════════════════════

export class GeofenceEngine {

  /**
   * Check a position against all active geofences for a user
   * Returns list of enter/exit events
   */
  async checkPosition(check: GeofenceCheck): Promise<GeofenceEvent[]> {
    const db = await getDb();
    if (!db) return [];

    // Get all active geofences for this user (and global ones)
    const fences = await db.select().from(geofences)
      .where(eq(geofences.isActive, true))
      .limit(500);

    const userFences = fences.filter(f =>
      f.userId === null || f.userId === check.userId
    );

    const events: GeofenceEvent[] = [];
    const currentInside = new Set<string>();

    // Get previous state
    const prevInside = deviceGeofenceState.get(check.deviceId) || new Set<string>();

    for (const fence of userFences) {
      const isInside = this.isInsideGeofence(
        check.lat, check.lon,
        fence.centerLat || 0, fence.centerLon || 0,
        fence.radiusMeters || 0,
        fence.polygon ? (typeof fence.polygon === 'string' ? JSON.parse(fence.polygon) : fence.polygon as Array<{lat:number;lon:number}>) : null
      );

      if (isInside) {
        currentInside.add(fence.fenceId);
      }

      const wasInside = prevInside.has(fence.fenceId);

      if (isInside && !wasInside) {
        // ENTER event
        events.push({
          geofenceId: fence.fenceId,
          name: fence.name,
          type: 'enter',
          deviceId: check.deviceId,
          userId: check.userId,
          lat: check.lat,
          lon: check.lon,
          timestamp: Date.now(),
        });
      } else if (!isInside && wasInside) {
        // EXIT event
        events.push({
          geofenceId: fence.fenceId,
          name: fence.name,
          type: 'exit',
          deviceId: check.deviceId,
          userId: check.userId,
          lat: check.lat,
          lon: check.lon,
          timestamp: Date.now(),
        });
      }
    }

    // Update state
    deviceGeofenceState.set(check.deviceId, currentInside);

    // Emit alerts for events
    for (const event of events) {
      await this.emitAlert(event, check.tripId);
    }

    return events;
  }

  /**
   * Create a new geofence
   */
  async createGeofence(input: {
    userId: number | null;
    name: string;
    lat: number;
    lon: number;
    radiusMeters: number;
    polygon?: Array<{ lat: number; lon: number }>;
    triggerOn: 'enter' | 'exit' | 'both';
    metadata?: Record<string, unknown>;
  }): Promise<string> {
    const db = await getDb();
    if (!db) throw new Error('Database unavailable');

    const geofenceId = crypto.randomUUID();

    await db.insert(geofences).values({
      fenceId: geofenceId,
      userId: input.userId,
      name: input.name,
      type: input.polygon ? 'polygon' : 'circle',
      centerLat: input.lat,
      centerLon: input.lon,
      radiusMeters: input.radiusMeters,
      polygon: input.polygon ? JSON.stringify(input.polygon) : null,
      triggerOn: input.triggerOn,
      isActive: true,
      metadata: input.metadata ? JSON.stringify(input.metadata) : null,
    });

    return geofenceId;
  }

  /**
   * Check if a point is inside a geofence (circle or polygon)
   */
  private isInsideGeofence(
    lat: number, lon: number,
    fenceLat: number, fenceLon: number,
    radiusMeters: number,
    polygon: Array<{ lat: number; lon: number }> | null
  ): boolean {
    if (polygon && polygon.length >= 3) {
      return this.isInsidePolygon(lat, lon, polygon);
    }
    // Circular geofence
    const distance = this.haversine(lat, lon, fenceLat, fenceLon);
    return distance <= radiusMeters;
  }

  /**
   * Ray-casting algorithm for point-in-polygon
   */
  private isInsidePolygon(lat: number, lon: number, polygon: Array<{ lat: number; lon: number }>): boolean {
    let inside = false;
    const n = polygon.length;

    for (let i = 0, j = n - 1; i < n; j = i++) {
      const xi = polygon[i].lon, yi = polygon[i].lat;
      const xj = polygon[j].lon, yj = polygon[j].lat;

      const intersect = ((yi > lat) !== (yj > lat)) &&
        (lon < (xj - xi) * (lat - yi) / (yj - yi) + xi);

      if (intersect) inside = !inside;
    }

    return inside;
  }

  private haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const dLat = (lat2 - lat1) * DEG2RAD;
    const dLon = (lon2 - lon1) * DEG2RAD;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * DEG2RAD) * Math.cos(lat2 * DEG2RAD) *
      Math.sin(dLon / 2) ** 2;
    return EARTH_RADIUS * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  /**
   * Emit an alert for a geofence event
   */
  private async emitAlert(event: GeofenceEvent, tripId?: string): Promise<void> {
    const db = await getDb();
    if (!db) return;

    const alertType = event.type === 'enter' ? 'geofence_enter' : 'geofence_exit';

    await db.insert(alerts).values({
      alertId: crypto.randomUUID(),
      userId: event.userId,
      deviceId: event.deviceId,
      tripId: tripId || null,
      type: alertType,
      severity: 'info',
      title: `${event.type === 'enter' ? 'Entered' : 'Left'} ${event.name}`,
      message: `Device ${event.deviceId} ${event.type === 'enter' ? 'entered' : 'exited'} geofence "${event.name}"`,
      lat: event.lat,
      lon: event.lon,
      channels: JSON.stringify(['push']),
      metadata: JSON.stringify({ geofenceId: event.geofenceId }),
    });
  }

  /**
   * Clear device state (e.g., on disconnect)
   */
  clearDeviceState(deviceId: string): void {
    deviceGeofenceState.delete(deviceId);
  }
}

// Singleton
export const geofenceEngine = new GeofenceEngine();
