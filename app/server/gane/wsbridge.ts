/**
 * G.A.N.E — WebSocket Bridge
 * =============================
 * Real-time binary data stream for telemetry, delta updates, and fleet tracking.
 * Uses binary protocol for sub-15ms latency.
 * 
 * Protocol:
 * - Client sends: JSON { type, payload } or binary telemetry packets
 * - Server sends: JSON events or binary delta push
 * 
 * Channels:
 * - telemetry:  Device → Server (1Hz position updates)
 * - deltas:     Server → Device (map anomaly broadcasts)
 * - fleet:      Server → Dispatcher (vehicle positions)
 * - commands:   Dispatcher → Device (mission injection)
 */

import type { Server as HttpServer } from "http";
import { WebSocketServer, WebSocket } from "ws";
import { getDb } from "../db";
import { rawTelemetry, vehicles, deltaUpdates, mapAnomalies } from "../../drizzle/schema";
import { eq, and, gte, lte, desc } from "drizzle-orm";

// ─── Types ───

interface ConnectedClient {
  ws: WebSocket;
  deviceId: string;
  lat: number;
  lon: number;
  role: "device" | "dispatcher" | "viewer";
  fleetId?: number;
  lastSeen: number;
  subscriptions: Set<string>;
}

interface WSMessage {
  type: string;
  payload: Record<string, unknown>;
}

// ─── Binary Protocol ───

/**
 * Binary telemetry packet (32 bytes):
 * [0-3]   float32  lat
 * [4-7]   float32  lon
 * [8-11]  float32  alt
 * [12-15] float32  velocity
 * [16-19] float32  heading
 * [20-23] float32  confidence
 * [24-27] uint32   satellites
 * [28-31] uint32   flags (gnss|imu|vision|network)
 */
const BINARY_PACKET_SIZE = 32;

function decodeBinaryTelemetry(buffer: ArrayBuffer): {
  lat: number; lon: number; alt: number; velocity: number;
  heading: number; confidence: number; satellites: number; flags: number;
} {
  const view = new DataView(buffer);
  return {
    lat: view.getFloat32(0, true),
    lon: view.getFloat32(4, true),
    alt: view.getFloat32(8, true),
    velocity: view.getFloat32(12, true),
    heading: view.getFloat32(16, true),
    confidence: view.getFloat32(20, true),
    satellites: view.getUint32(24, true),
    flags: view.getUint32(28, true),
  };
}

function encodeBinaryTelemetry(data: {
  lat: number; lon: number; alt: number; velocity: number;
  heading: number; confidence: number; satellites: number; flags: number;
}): ArrayBuffer {
  const buffer = new ArrayBuffer(BINARY_PACKET_SIZE);
  const view = new DataView(buffer);
  view.setFloat32(0, data.lat, true);
  view.setFloat32(4, data.lon, true);
  view.setFloat32(8, data.alt, true);
  view.setFloat32(12, data.velocity, true);
  view.setFloat32(16, data.heading, true);
  view.setFloat32(20, data.confidence, true);
  view.setUint32(24, data.satellites, true);
  view.setUint32(28, data.flags, true);
  return buffer;
}

// ─── WebSocket Server ───

class GANEWebSocketBridge {
  private wss: WebSocketServer | null = null;
  private clients = new Map<string, ConnectedClient>();
  private deltaCheckInterval: ReturnType<typeof setInterval> | null = null;
  private cleanupInterval: ReturnType<typeof setInterval> | null = null;
  private lastDeltaCheck = Date.now();

  /** Initialize WebSocket server on the existing HTTP server */
  init(server: HttpServer) {
    this.wss = new WebSocketServer({ server, path: "/ws/gane" });

    this.wss.on("connection", (ws: WebSocket, req: unknown) => {
      const clientId = `client_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;

      const client: ConnectedClient = {
        ws,
        deviceId: clientId,
        lat: 0,
        lon: 0,
        role: "device",
        lastSeen: Date.now(),
        subscriptions: new Set(["deltas"]),
      };

      this.clients.set(clientId, client);
      console.log(`[WS] Client connected: ${clientId} (total: ${this.clients.size})`);

      // Send welcome
      this.send(ws, {
        type: "welcome",
        payload: { clientId, serverTime: Date.now(), protocol: "gane-v1" },
      });

      ws.on("message", (data: Buffer | ArrayBuffer | Buffer[], isBinary: boolean) => {
        try {
          if (isBinary && data instanceof Buffer && data.length === BINARY_PACKET_SIZE) {
            // Binary telemetry packet
            const ab = data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength) as ArrayBuffer;
            const telemetry = decodeBinaryTelemetry(ab);
            this.handleBinaryTelemetry(clientId, telemetry);
          } else {
            // JSON message
            const msg = JSON.parse(data.toString()) as WSMessage;
            this.handleMessage(clientId, msg);
          }
        } catch (err) {
          console.error(`[WS] Message parse error from ${clientId}:`, err);
        }
      });

      ws.on("close", () => {
        this.clients.delete(clientId);
        console.log(`[WS] Client disconnected: ${clientId} (total: ${this.clients.size})`);
      });

      ws.on("error", (err: Error) => {
        console.error(`[WS] Error from ${clientId}:`, err);
        this.clients.delete(clientId);
      });
    });

    // Start delta broadcast checker (every 2 seconds)
    this.deltaCheckInterval = setInterval(() => this.broadcastDeltas(), 2000);

    // Cleanup stale connections (every 30 seconds)
    this.cleanupInterval = setInterval(() => this.cleanupStale(), 30000);

    console.log("[WS] G.A.N.E WebSocket Bridge initialized on /ws/gane");
  }

  /** Handle JSON messages */
  private async handleMessage(clientId: string, msg: WSMessage) {
    const client = this.clients.get(clientId);
    if (!client) return;

    client.lastSeen = Date.now();

    switch (msg.type) {
      case "register": {
        // Register device with ID and role
        const { deviceId, role, fleetId } = msg.payload as {
          deviceId?: string; role?: string; fleetId?: number;
        };
        if (deviceId) client.deviceId = deviceId;
        if (role === "dispatcher" || role === "viewer") client.role = role as "dispatcher" | "viewer";
        if (fleetId) client.fleetId = fleetId;

        // Dispatchers subscribe to fleet channel
        if (client.role === "dispatcher" && client.fleetId) {
          client.subscriptions.add(`fleet:${client.fleetId}`);
        }

        this.send(client.ws, {
          type: "registered",
          payload: { deviceId: client.deviceId, role: client.role },
        });
        break;
      }

      case "telemetry": {
        // JSON telemetry (fallback for non-binary clients)
        const { lat, lon, alt, velocity, heading, confidence, satellites } = msg.payload as {
          lat: number; lon: number; alt?: number; velocity?: number;
          heading?: number; confidence?: number; satellites?: number;
        };
        client.lat = lat;
        client.lon = lon;

        // Persist to DB (fire-and-forget)
        this.persistTelemetry(client.deviceId, {
          lat, lon, alt: alt ?? 0, velocity: velocity ?? 0,
          heading: heading ?? 0, confidence: confidence ?? 1.0,
          satellites: satellites ?? 0, flags: 0xF,
        });

        // Broadcast to fleet dispatchers
        this.broadcastToFleet(client, {
          type: "vehicle_position",
          payload: {
            deviceId: client.deviceId,
            lat, lon, velocity, heading,
            timestamp: Date.now(),
          },
        });
        break;
      }

      case "subscribe": {
        const { channel } = msg.payload as { channel: string };
        client.subscriptions.add(channel);
        break;
      }

      case "unsubscribe": {
        const { channel } = msg.payload as { channel: string };
        client.subscriptions.delete(channel);
        break;
      }

      case "anomaly_report": {
        // Quick anomaly report via WebSocket
        const { type, lat, lon, severity, description } = msg.payload as {
          type: string; lat: number; lon: number; severity?: number; description?: string;
        };

        // Broadcast immediately to nearby clients
        this.broadcastToNearby(lat, lon, 5000, {
          type: "anomaly_alert",
          payload: {
            anomalyType: type,
            lat, lon, severity: severity ?? 1,
            description,
            reportedBy: client.deviceId,
            timestamp: Date.now(),
          },
        }, clientId);
        break;
      }

      case "mission_command": {
        // Dispatcher → Vehicle command (Ghost Tailing)
        if (client.role !== "dispatcher") {
          this.send(client.ws, { type: "error", payload: { message: "Unauthorized" } });
          return;
        }

        const { targetDeviceId, command, data } = msg.payload as {
          targetDeviceId: string; command: string; data?: unknown;
        };

        // Find target vehicle
        const targetClient = Array.from(this.clients.values()).find(c => c.deviceId === targetDeviceId);
        if (targetClient) {
          this.send(targetClient.ws, {
            type: "mission_command",
            payload: { command, data, from: client.deviceId, timestamp: Date.now() },
          });
        }
        break;
      }

      case "ping": {
        this.send(client.ws, { type: "pong", payload: { serverTime: Date.now() } });
        break;
      }
    }
  }

  /** Handle binary telemetry packet */
  private handleBinaryTelemetry(clientId: string, data: {
    lat: number; lon: number; alt: number; velocity: number;
    heading: number; confidence: number; satellites: number; flags: number;
  }) {
    const client = this.clients.get(clientId);
    if (!client) return;

    client.lat = data.lat;
    client.lon = data.lon;
    client.lastSeen = Date.now();

    // Persist (fire-and-forget)
    this.persistTelemetry(client.deviceId, data);

    // Broadcast to fleet dispatchers (as binary for speed)
    this.broadcastToFleet(client, {
      type: "vehicle_position",
      payload: {
        deviceId: client.deviceId,
        lat: data.lat,
        lon: data.lon,
        velocity: data.velocity,
        heading: data.heading,
        alt: data.alt,
        confidence: data.confidence,
        satellites: data.satellites,
        timestamp: Date.now(),
      },
    });
  }

  /** Persist telemetry to database */
  private async persistTelemetry(deviceId: string, data: {
    lat: number; lon: number; alt: number; velocity: number;
    heading: number; confidence: number; satellites: number; flags: number;
  }) {
    try {
      const db = await getDb();
      if (!db) return;

      await db.insert(rawTelemetry).values({
        deviceId,
        timestamp: new Date(),
        lat: data.lat,
        lon: data.lon,
        alt: data.alt,
        velocity: data.velocity,
        heading: data.heading,
        confidenceScore: data.confidence,
        satellites: data.satellites,
        sensorStatusGnss: !!(data.flags & 0x1),
        sensorStatusImu: !!(data.flags & 0x2),
        sensorStatusVision: !!(data.flags & 0x4),
        sensorStatusNetwork: !!(data.flags & 0x8),
      });

      // Update vehicle last-known position
      await db.update(vehicles)
        .set({
          lastLat: data.lat,
          lastLon: data.lon,
          lastHeading: data.heading,
          lastSpeed: data.velocity,
          lastSeen: new Date(),
          status: "active",
        })
        .where(eq(vehicles.deviceId, deviceId));
    } catch (err) {
      // Fire-and-forget, don't crash on DB errors
    }
  }

  /** Broadcast delta updates to nearby clients */
  private async broadcastDeltas() {
    try {
      const db = await getDb();
      if (!db) return;

      const since = new Date(this.lastDeltaCheck);
      this.lastDeltaCheck = Date.now();

      const deltas = await db.select()
        .from(deltaUpdates)
        .where(gte(deltaUpdates.createdAt, since))
        .orderBy(desc(deltaUpdates.createdAt))
        .limit(50);

      for (const delta of deltas) {
        const radius = delta.geofenceRadiusM ?? 5000;
        this.broadcastToNearby(delta.geofenceLat, delta.geofenceLon, radius, {
          type: "delta_update",
          payload: delta.payload as Record<string, unknown>,
        });

        // Increment broadcast count
        await db.update(deltaUpdates)
          .set({ broadcastCount: (delta.broadcastCount ?? 0) + 1 })
          .where(eq(deltaUpdates.id, delta.id));
      }
    } catch (err) {
      // Silent fail for delta broadcast
    }
  }

  /** Broadcast message to clients near a location */
  private broadcastToNearby(lat: number, lon: number, radiusM: number, msg: WSMessage, excludeId?: string) {
    const latDelta = radiusM / 111320;
    const lonDelta = radiusM / (111320 * Math.cos(lat * Math.PI / 180));

    Array.from(this.clients.entries()).forEach(([id, client]) => {
      if (id === excludeId) return;
      if (!client.subscriptions.has("deltas")) return;
      if (client.lat === 0 && client.lon === 0) return;

      if (
        client.lat >= lat - latDelta && client.lat <= lat + latDelta &&
        client.lon >= lon - lonDelta && client.lon <= lon + lonDelta
      ) {
        this.send(client.ws, msg);
      }
    });
  }

  /** Broadcast to fleet dispatchers */
  private broadcastToFleet(sourceClient: ConnectedClient, msg: WSMessage) {
    if (!sourceClient.fleetId) return;

    const channel = `fleet:${sourceClient.fleetId}`;
    Array.from(this.clients.values()).forEach(client => {
      if (client.role === "dispatcher" && client.subscriptions.has(channel)) {
        this.send(client.ws, msg);
      }
    });
  }

  /** Send JSON message to a WebSocket */
  private send(ws: WebSocket, msg: WSMessage) {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(msg));
    }
  }

  /** Broadcast to all clients subscribed to a specific channel */
  broadcastToChannel(channel: string, msg: WSMessage) {
    Array.from(this.clients.values()).forEach(client => {
      if (client.subscriptions.has(channel)) {
        this.send(client.ws, msg);
      }
    });
  }

  /** Broadcast to ALL connected clients */
  broadcastToAll(msg: WSMessage) {
    Array.from(this.clients.values()).forEach(client => {
      this.send(client.ws, msg);
    });
  }

  /** Cleanup stale connections */
  private cleanupStale() {
    const staleThreshold = Date.now() - 120000; // 2 minutes
    Array.from(this.clients.entries()).forEach(([id, client]) => {
      if (client.lastSeen < staleThreshold) {
        client.ws.terminate();
        this.clients.delete(id);
      }
    });
  }

  /** Get connection stats */
  getStats() {
    let devices = 0, dispatchers = 0, viewers = 0;
    Array.from(this.clients.values()).forEach(client => {
      if (client.role === "device") devices++;
      else if (client.role === "dispatcher") dispatchers++;
      else viewers++;
    });
    return {
      totalConnections: this.clients.size,
      devices,
      dispatchers,
      viewers,
    };
  }

  /** Shutdown */
  shutdown() {
    if (this.deltaCheckInterval) clearInterval(this.deltaCheckInterval);
    if (this.cleanupInterval) clearInterval(this.cleanupInterval);
    Array.from(this.clients.values()).forEach(client => {
      client.ws.close();
    });
    this.clients.clear();
    this.wss?.close();
  }
}

// Singleton
export const wsBridge = new GANEWebSocketBridge();
