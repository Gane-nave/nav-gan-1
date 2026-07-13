import {
  int, bigint, mysqlEnum, mysqlTable, text, timestamp, varchar,
  float, double, boolean, json, index, uniqueIndex
} from "drizzle-orm/mysql-core";

/**
 * G.A.N.E — Database Schema (Planetary Data Matrix)
 * ===================================================
 * MySQL with Drizzle ORM — optimized for:
 * - Time-series telemetry ingestion (1Hz from millions of clients)
 * - Geospatial queries (vehicles within radius, anomaly clustering)
 * - Crowdsourced SLAM (anomaly aggregation + self-healing maps)
 * - Fleet management (C4ISR multi-tenant)
 * - Route optimization (VRP solver data)
 * - Trip lifecycle management
 * - Multi-channel integration orchestration
 */

// ═══════════════════════════════════════════════════
// 1. USERS (Core Auth)
// ═══════════════════════════════════════════════════
export const users = mysqlTable("users", {
  id: int("id").autoincrement().primaryKey(),
  openId: varchar("openId", { length: 64 }).notNull().unique(),
  name: text("name"),
  email: varchar("email", { length: 320 }),
  loginMethod: varchar("loginMethod", { length: 64 }),
  role: mysqlEnum("role", ["user", "admin", "dispatcher", "driver", "ems", "sports"]).default("user").notNull(),
  uiProfile: mysqlEnum("uiProfile", ["private", "sports", "ems", "logistics"]).default("private").notNull(),
  fleetId: int("fleetId"),
  preferences: json("preferences"),                   // { language, units, theme, notifications }
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
  lastSignedIn: timestamp("lastSignedIn").defaultNow().notNull(),
});

export type User = typeof users.$inferSelect;
export type InsertUser = typeof users.$inferInsert;

// ═══════════════════════════════════════════════════
// 2. DEVICES (Device Registry — Section 10)
// ═══════════════════════════════════════════════════
/**
 * Each user can have multiple devices. Devices are the source
 * of telemetry, GNSS data, and sensor readings.
 * Ownership: User → Device → Trip
 */
export const devices = mysqlTable("devices", {
  id: int("id").autoincrement().primaryKey(),
  deviceId: varchar("deviceId", { length: 64 }).notNull().unique(),
  userId: int("userId").notNull(),
  name: varchar("name", { length: 128 }),
  platform: mysqlEnum("platform", ["ios", "android", "web", "embedded", "obd2"]).default("android"),
  osVersion: varchar("osVersion", { length: 32 }),
  appVersion: varchar("appVersion", { length: 32 }),
  capabilities: json("capabilities"),                  // { gnss: true, imu: true, camera: false, ... }
  pushToken: text("pushToken"),
  isActive: boolean("isActive").default(true),
  lastSeenAt: timestamp("lastSeenAt"),
  lastLat: double("lastLat"),
  lastLon: double("lastLon"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_devices_user").on(table.userId),
  index("idx_devices_active").on(table.isActive),
]);

export type Device = typeof devices.$inferSelect;
export type InsertDevice = typeof devices.$inferInsert;

// ═══════════════════════════════════════════════════
// 3. TRIPS (Trip Lifecycle — Section 9/10)
// ═══════════════════════════════════════════════════
/**
 * Full trip lifecycle: start → route_compute → navigate → reroute* → arrival/failure
 * Each trip has a complete audit trail via tripEvents.
 */
export const trips = mysqlTable("trips", {
  id: int("id").autoincrement().primaryKey(),
  tripId: varchar("tripId", { length: 64 }).notNull().unique(),
  userId: int("userId").notNull(),
  deviceId: varchar("deviceId", { length: 64 }).notNull(),
  status: mysqlEnum("status", [
    "planned", "active", "paused", "completed", "cancelled", "failed"
  ]).default("planned").notNull(),
  originLat: double("originLat").notNull(),
  originLon: double("originLon").notNull(),
  originAddress: text("originAddress"),
  destinationLat: double("destinationLat"),
  destinationLon: double("destinationLon"),
  destinationAddress: text("destinationAddress"),
  routeId: varchar("routeId", { length: 64 }),
  distanceMeters: float("distanceMeters"),
  durationSeconds: float("durationSeconds"),
  avgSpeed: float("avgSpeed"),
  maxSpeed: float("maxSpeed"),
  rerouteCount: int("rerouteCount").default(0),
  gnssQualityAvg: float("gnssQualityAvg"),
  startedAt: timestamp("startedAt"),
  completedAt: timestamp("completedAt"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_trips_user").on(table.userId),
  index("idx_trips_device").on(table.deviceId),
  index("idx_trips_status").on(table.status),
  index("idx_trips_started").on(table.startedAt),
]);

export type Trip = typeof trips.$inferSelect;
export type InsertTrip = typeof trips.$inferInsert;

// ═══════════════════════════════════════════════════
// 4. ROUTES (Computed Routes — Section 7/10)
// ═══════════════════════════════════════════════════
export const routes = mysqlTable("routes", {
  id: int("id").autoincrement().primaryKey(),
  routeId: varchar("routeId", { length: 64 }).notNull().unique(),
  tripId: varchar("tripId", { length: 64 }),
  userId: int("userId"),
  originLat: double("originLat").notNull(),
  originLon: double("originLon").notNull(),
  destinationLat: double("destinationLat").notNull(),
  destinationLon: double("destinationLon").notNull(),
  polyline: text("polyline"),                          // encoded polyline
  distanceMeters: float("distanceMeters"),
  durationSeconds: float("durationSeconds"),
  trafficDelaySeconds: float("trafficDelaySeconds"),
  tollCost: float("tollCost"),
  routeType: mysqlEnum("routeType", [
    "fastest", "shortest", "eco", "avoid_tolls", "avoid_highways", "scenic"
  ]).default("fastest"),
  constraints: json("constraints"),                    // { avoidTolls, avoidHighways, vehicleType, ... }
  alternatives: json("alternatives"),                  // Array of alternative route summaries
  computeTimeMs: int("computeTimeMs"),
  isActive: boolean("isActive").default(true),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_routes_trip").on(table.tripId),
  index("idx_routes_user").on(table.userId),
]);

export type Route = typeof routes.$inferSelect;
export type InsertRoute = typeof routes.$inferInsert;

// ═══════════════════════════════════════════════════
// 5. WAYPOINTS (Route Waypoints — Section 10)
// ═══════════════════════════════════════════════════
export const waypoints = mysqlTable("waypoints", {
  id: int("id").autoincrement().primaryKey(),
  routeId: varchar("routeId", { length: 64 }).notNull(),
  tripId: varchar("tripId", { length: 64 }),
  sequence: int("sequence").notNull(),
  lat: double("lat").notNull(),
  lon: double("lon").notNull(),
  address: text("address"),
  name: varchar("name", { length: 256 }),
  type: mysqlEnum("type", ["origin", "destination", "via", "stop", "fuel", "rest", "charging"]).default("via"),
  arrivalTime: timestamp("arrivalTime"),
  departureTime: timestamp("departureTime"),
  dwellSeconds: int("dwellSeconds"),
  isReached: boolean("isReached").default(false),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_waypoints_route").on(table.routeId),
  index("idx_waypoints_trip").on(table.tripId),
]);

export type Waypoint = typeof waypoints.$inferSelect;
export type InsertWaypoint = typeof waypoints.$inferInsert;

// ═══════════════════════════════════════════════════
// 6. ALERTS (Unified Alerting — Section 10)
// ═══════════════════════════════════════════════════
export const alerts = mysqlTable("alerts", {
  id: int("id").autoincrement().primaryKey(),
  alertId: varchar("alertId", { length: 64 }).notNull().unique(),
  userId: int("userId"),
  deviceId: varchar("deviceId", { length: 64 }),
  tripId: varchar("tripId", { length: 64 }),
  type: mysqlEnum("type", [
    "traffic", "accident", "weather", "geofence_enter", "geofence_exit",
    "speed_violation", "gnss_degradation", "gnss_spoofing", "gnss_jamming",
    "route_deviation", "sos", "maintenance", "payment", "system"
  ]).notNull(),
  severity: mysqlEnum("severity", ["info", "warning", "critical", "emergency"]).default("info").notNull(),
  title: varchar("title", { length: 256 }).notNull(),
  message: text("message"),
  lat: double("lat"),
  lon: double("lon"),
  channels: json("channels"),                          // ["push", "in_app", "email", "sms"]
  deliveryStatus: json("deliveryStatus"),              // { push: "sent", in_app: "delivered", ... }
  isRead: boolean("isRead").default(false),
  isAcknowledged: boolean("isAcknowledged").default(false),
  expiresAt: timestamp("expiresAt"),
  metadata: json("metadata"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_alerts_user").on(table.userId),
  index("idx_alerts_type").on(table.type),
  index("idx_alerts_severity").on(table.severity),
  index("idx_alerts_created").on(table.createdAt),
  index("idx_alerts_trip").on(table.tripId),
]);

export type Alert = typeof alerts.$inferSelect;
export type InsertAlert = typeof alerts.$inferInsert;

// ═══════════════════════════════════════════════════
// 7. PAYMENT EVENTS (Section 8/10)
// ═══════════════════════════════════════════════════
export const paymentEvents = mysqlTable("payment_events", {
  id: int("id").autoincrement().primaryKey(),
  eventId: varchar("eventId", { length: 64 }).notNull().unique(),
  userId: int("userId").notNull(),
  tripId: varchar("tripId", { length: 64 }),
  type: mysqlEnum("type", [
    "toll", "parking", "fuel", "charging", "subscription", "fine", "refund"
  ]).notNull(),
  amount: float("amount").notNull(),
  currency: varchar("currency", { length: 3 }).default("ILS"),
  status: mysqlEnum("status", ["pending", "completed", "failed", "refunded"]).default("pending"),
  provider: varchar("provider", { length: 64 }),
  externalRef: varchar("externalRef", { length: 128 }),
  lat: double("lat"),
  lon: double("lon"),
  metadata: json("metadata"),
  alertSent: boolean("alertSent").default(false),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_payment_user").on(table.userId),
  index("idx_payment_trip").on(table.tripId),
  index("idx_payment_type").on(table.type),
  index("idx_payment_status").on(table.status),
]);

export type PaymentEvent = typeof paymentEvents.$inferSelect;
export type InsertPaymentEvent = typeof paymentEvents.$inferInsert;

// ═══════════════════════════════════════════════════
// 8. TRIP EVENTS (Audit Trail — Section 9)
// ═══════════════════════════════════════════════════
/**
 * Every event in the G.A.N.E is encapsulated in a standardized envelope.
 * The envelope is the atomic unit of communication and is immutable once published.
 * Core fields: event_id, trip_id, timestamp_device, timestamp_server, location, accuracy, confidence, latency, outcome
 */
export const tripEvents = mysqlTable("trip_events", {
  id: bigint("id", { mode: "number" }).autoincrement().primaryKey(),
  eventId: varchar("eventId", { length: 64 }).notNull().unique(),
  tripId: varchar("tripId", { length: 64 }).notNull(),
  deviceId: varchar("deviceId", { length: 64 }).notNull(),
  userId: int("userId"),
  eventType: mysqlEnum("eventType", [
    "trip_start", "trip_end", "trip_pause", "trip_resume",
    "route_compute", "reroute", "arrival", "failure",
    "gnss_degradation", "gnss_recovery", "gnss_spoofing_detected",
    "sync_event", "notification_dispatch", "payment_alert",
    "geofence_enter", "geofence_exit", "speed_violation",
    "waypoint_reached", "eta_update", "traffic_update"
  ]).notNull(),
  timestampDevice: bigint("timestampDevice", { mode: "number" }).notNull(),  // ms epoch from device
  timestampServer: bigint("timestampServer", { mode: "number" }).notNull(),  // ms epoch from server
  lat: double("lat"),
  lon: double("lon"),
  accuracy: float("accuracy"),
  confidence: float("confidence"),
  latencyMs: int("latencyMs"),
  outcome: mysqlEnum("outcome", ["success", "failure", "degraded", "timeout"]).default("success"),
  payload: json("payload"),                            // event-specific data
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_trip_events_trip").on(table.tripId),
  index("idx_trip_events_type").on(table.eventType),
  index("idx_trip_events_device").on(table.deviceId),
  index("idx_trip_events_ts").on(table.timestampServer),
]);

export type TripEvent = typeof tripEvents.$inferSelect;
export type InsertTripEvent = typeof tripEvents.$inferInsert;

// ═══════════════════════════════════════════════════
// 9. LOG ENTRIES (Structured Audit Logs — Section 12)
// ═══════════════════════════════════════════════════
export const logEntries = mysqlTable("log_entries", {
  id: bigint("id", { mode: "number" }).autoincrement().primaryKey(),
  level: mysqlEnum("level", ["debug", "info", "warn", "error", "critical"]).default("info").notNull(),
  source: varchar("source", { length: 64 }).notNull(),  // module/service name
  action: varchar("action", { length: 128 }).notNull(),
  userId: int("userId"),
  deviceId: varchar("deviceId", { length: 64 }),
  ipAddress: varchar("ipAddress", { length: 45 }),
  userAgent: text("userAgent"),
  details: json("details"),
  errorStack: text("errorStack"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_logs_level").on(table.level),
  index("idx_logs_source").on(table.source),
  index("idx_logs_user").on(table.userId),
  index("idx_logs_created").on(table.createdAt),
]);

export type LogEntry = typeof logEntries.$inferSelect;
export type InsertLogEntry = typeof logEntries.$inferInsert;

// ═══════════════════════════════════════════════════
// 10. INTEGRATION CHANNELS (Section 8)
// ═══════════════════════════════════════════════════
/**
 * Configuration for multi-channel integrations:
 * In-app, Gmail, Google Drive, payment providers.
 */
export const integrationChannels = mysqlTable("integration_channels", {
  id: int("id").autoincrement().primaryKey(),
  userId: int("userId").notNull(),
  channelType: mysqlEnum("channelType", [
    "in_app", "gmail", "google_drive", "sms", "push", "webhook"
  ]).notNull(),
  isEnabled: boolean("isEnabled").default(true),
  config: json("config"),                              // channel-specific config (phone, email, folder, etc.)
  lastUsedAt: timestamp("lastUsedAt"),
  errorCount: int("errorCount").default(0),
  lastError: text("lastError"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_channels_user").on(table.userId),
  index("idx_channels_type").on(table.channelType),
  uniqueIndex("idx_channels_unique").on(table.userId, table.channelType),
]);

export type IntegrationChannel = typeof integrationChannels.$inferSelect;
export type InsertIntegrationChannel = typeof integrationChannels.$inferInsert;

// ═══════════════════════════════════════════════════
// 11. GEOFENCES (Section 4 — Geofencing Engine)
// ═══════════════════════════════════════════════════
export const geofences = mysqlTable("geofences", {
  id: int("id").autoincrement().primaryKey(),
  fenceId: varchar("fenceId", { length: 64 }).notNull().unique(),
  userId: int("userId"),
  fleetId: int("fleetId"),
  name: varchar("name", { length: 128 }).notNull(),
  type: mysqlEnum("type", ["circle", "polygon"]).default("circle"),
  centerLat: double("centerLat"),
  centerLon: double("centerLon"),
  radiusMeters: float("radiusMeters"),
  polygon: json("polygon"),                            // Array of { lat, lon } for polygon type
  triggerOn: mysqlEnum("triggerOn", ["enter", "exit", "both"]).default("both"),
  alertChannels: json("alertChannels"),                // ["push", "in_app", "email"]
  isActive: boolean("isActive").default(true),
  metadata: json("metadata"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_geofences_user").on(table.userId),
  index("idx_geofences_fleet").on(table.fleetId),
  index("idx_geofences_center").on(table.centerLat, table.centerLon),
  index("idx_geofences_active").on(table.isActive),
]);

export type Geofence = typeof geofences.$inferSelect;
export type InsertGeofence = typeof geofences.$inferInsert;

// ═══════════════════════════════════════════════════
// 12. RAW TELEMETRY (Time-Series)
// ═══════════════════════════════════════════════════
/**
 * High-frequency telemetry from all connected devices.
 * Designed for time-series queries with composite indexes.
 * In production, partition by timestamp for performance.
 */
export const rawTelemetry = mysqlTable("raw_telemetry", {
  id: bigint("id", { mode: "number" }).autoincrement().primaryKey(),
  deviceId: varchar("deviceId", { length: 64 }).notNull(),
  tripId: varchar("tripId", { length: 64 }),
  timestamp: timestamp("timestamp").notNull(),
  lat: double("lat").notNull(),
  lon: double("lon").notNull(),
  alt: float("alt").default(0),
  velocity: float("velocity").default(0),
  heading: float("heading").default(0),
  confidenceScore: float("confidenceScore").default(1.0),
  navigationMode: mysqlEnum("navigationMode", [
    "BOOTING", "OPTIMAL_FUSION", "DEGRADED_MODE", "EMERGENCY_BOUNDED"
  ]).default("OPTIMAL_FUSION"),
  satellites: int("satellites").default(0),
  hdop: float("hdop"),
  batteryLevel: float("batteryLevel"),
  sensorStatusGnss: boolean("sensorStatusGnss").default(true),
  sensorStatusImu: boolean("sensorStatusImu").default(true),
  sensorStatusVision: boolean("sensorStatusVision").default(false),
  sensorStatusNetwork: boolean("sensorStatusNetwork").default(true),
  retentionTier: mysqlEnum("retentionTier", ["hot", "warm", "cold", "archive"]).default("hot"),
}, (table) => [
  index("idx_telemetry_device_time").on(table.deviceId, table.timestamp),
  index("idx_telemetry_time").on(table.timestamp),
  index("idx_telemetry_lat_lon").on(table.lat, table.lon),
  index("idx_telemetry_trip").on(table.tripId),
  index("idx_telemetry_retention").on(table.retentionTier),
]);

export type RawTelemetry = typeof rawTelemetry.$inferSelect;
export type InsertRawTelemetry = typeof rawTelemetry.$inferInsert;

// ═══════════════════════════════════════════════════
// 13. MAP ANOMALIES (Crowdsourced SLAM)
// ═══════════════════════════════════════════════════
export const mapAnomalies = mysqlTable("map_anomalies", {
  id: int("id").autoincrement().primaryKey(),
  anomalyId: varchar("anomalyId", { length: 64 }).notNull().unique(),
  type: mysqlEnum("type", [
    "roadblock", "pothole", "construction", "accident",
    "signal_jamming", "new_road", "road_closure", "flooding",
    "speed_trap", "hazard", "other"
  ]).notNull(),
  lat: double("lat").notNull(),
  lon: double("lon").notNull(),
  radiusMeters: float("radiusMeters").default(10),
  severity: int("severity").default(1),
  description: text("description"),
  descriptionHe: text("descriptionHe"),
  reportCount: int("reportCount").default(1),
  confirmedAt: timestamp("confirmedAt"),
  isConfirmed: boolean("isConfirmed").default(false),
  isActive: boolean("isActive").default(true),
  reporterDeviceId: varchar("reporterDeviceId", { length: 64 }),
  reporterUserId: int("reporterUserId"),
  expiresAt: timestamp("expiresAt"),
  metadata: json("metadata"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_anomalies_lat_lon").on(table.lat, table.lon),
  index("idx_anomalies_type").on(table.type),
  index("idx_anomalies_active").on(table.isActive, table.isConfirmed),
  index("idx_anomalies_created").on(table.createdAt),
]);

export type MapAnomaly = typeof mapAnomalies.$inferSelect;
export type InsertMapAnomaly = typeof mapAnomalies.$inferInsert;

// ═══════════════════════════════════════════════════
// 14. ANOMALY REPORTS (Individual Reports for Aggregation)
// ═══════════════════════════════════════════════════
export const anomalyReports = mysqlTable("anomaly_reports", {
  id: int("id").autoincrement().primaryKey(),
  anomalyId: varchar("anomalyId", { length: 64 }).notNull(),
  deviceId: varchar("deviceId", { length: 64 }).notNull(),
  userId: int("userId"),
  lat: double("lat").notNull(),
  lon: double("lon").notNull(),
  type: varchar("type", { length: 32 }).notNull(),
  confidence: float("confidence").default(1.0),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_reports_anomaly").on(table.anomalyId),
  index("idx_reports_device").on(table.deviceId),
  uniqueIndex("idx_reports_unique").on(table.anomalyId, table.deviceId),
]);

export type AnomalyReport = typeof anomalyReports.$inferSelect;
export type InsertAnomalyReport = typeof anomalyReports.$inferInsert;

// ═══════════════════════════════════════════════════
// 15. FLEETS (C4ISR Multi-Tenant)
// ═══════════════════════════════════════════════════
export const fleets = mysqlTable("fleets", {
  id: int("id").autoincrement().primaryKey(),
  name: varchar("name", { length: 128 }).notNull(),
  nameHe: varchar("nameHe", { length: 128 }),
  ownerId: int("ownerId").notNull(),
  type: mysqlEnum("type", ["delivery", "emergency", "logistics", "transit", "private"]).default("private"),
  maxVehicles: int("maxVehicles").default(50),
  isActive: boolean("isActive").default(true),
  settings: json("settings"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
});

export type Fleet = typeof fleets.$inferSelect;
export type InsertFleet = typeof fleets.$inferInsert;

// ═══════════════════════════════════════════════════
// 16. VEHICLES (Fleet Members)
// ═══════════════════════════════════════════════════
export const vehicles = mysqlTable("vehicles", {
  id: int("id").autoincrement().primaryKey(),
  fleetId: int("fleetId").notNull(),
  driverId: int("driverId"),
  deviceId: varchar("deviceId", { length: 64 }).notNull().unique(),
  name: varchar("name", { length: 128 }),
  licensePlate: varchar("licensePlate", { length: 20 }),
  type: mysqlEnum("type", ["car", "truck", "van", "motorcycle", "bicycle", "ambulance", "firetruck", "police"]).default("car"),
  status: mysqlEnum("status", ["active", "idle", "offline", "maintenance", "emergency"]).default("idle"),
  lastLat: double("lastLat"),
  lastLon: double("lastLon"),
  lastHeading: float("lastHeading"),
  lastSpeed: float("lastSpeed"),
  lastSeen: timestamp("lastSeen"),
  batteryLevel: float("batteryLevel"),
  fuelLevel: float("fuelLevel"),
  dimensions: json("dimensions"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_vehicles_fleet").on(table.fleetId),
  index("idx_vehicles_driver").on(table.driverId),
  index("idx_vehicles_status").on(table.status),
  index("idx_vehicles_location").on(table.lastLat, table.lastLon),
]);

export type Vehicle = typeof vehicles.$inferSelect;
export type InsertVehicle = typeof vehicles.$inferInsert;

// ═══════════════════════════════════════════════════
// 17. MISSIONS (Route Assignments / Ghost Tailing)
// ═══════════════════════════════════════════════════
export const missions = mysqlTable("missions", {
  id: int("id").autoincrement().primaryKey(),
  fleetId: int("fleetId").notNull(),
  vehicleId: int("vehicleId"),
  driverId: int("driverId"),
  dispatcherId: int("dispatcherId"),
  status: mysqlEnum("status", [
    "pending", "assigned", "in_progress", "completed", "cancelled", "failed"
  ]).default("pending"),
  priority: int("priority").default(3),
  type: mysqlEnum("type", ["delivery", "pickup", "patrol", "emergency", "custom"]).default("delivery"),
  originLat: double("originLat"),
  originLon: double("originLon"),
  originAddress: text("originAddress"),
  destinationLat: double("destinationLat"),
  destinationLon: double("destinationLon"),
  destinationAddress: text("destinationAddress"),
  waypoints: json("waypoints"),
  routePolyline: text("routePolyline"),
  estimatedDistance: float("estimatedDistance"),
  estimatedDuration: float("estimatedDuration"),
  actualDistance: float("actualDistance"),
  actualDuration: float("actualDuration"),
  timeWindowStart: timestamp("timeWindowStart"),
  timeWindowEnd: timestamp("timeWindowEnd"),
  notes: text("notes"),
  isGhostTailing: boolean("isGhostTailing").default(false),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
  completedAt: timestamp("completedAt"),
}, (table) => [
  index("idx_missions_fleet").on(table.fleetId),
  index("idx_missions_vehicle").on(table.vehicleId),
  index("idx_missions_status").on(table.status),
  index("idx_missions_created").on(table.createdAt),
]);

export type Mission = typeof missions.$inferSelect;
export type InsertMission = typeof missions.$inferInsert;

// ═══════════════════════════════════════════════════
// 18. DELTA UPDATES (Map Update Broadcasts)
// ═══════════════════════════════════════════════════
export const deltaUpdates = mysqlTable("delta_updates", {
  id: int("id").autoincrement().primaryKey(),
  anomalyId: varchar("anomalyId", { length: 64 }).notNull(),
  type: varchar("type", { length: 32 }).notNull(),
  payload: json("payload").notNull(),
  geofenceLat: double("geofenceLat").notNull(),
  geofenceLon: double("geofenceLon").notNull(),
  geofenceRadiusM: float("geofenceRadiusM").default(5000),
  broadcastCount: int("broadcastCount").default(0),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_delta_anomaly").on(table.anomalyId),
  index("idx_delta_geofence").on(table.geofenceLat, table.geofenceLon),
  index("idx_delta_created").on(table.createdAt),
]);

export type DeltaUpdate = typeof deltaUpdates.$inferSelect;
export type InsertDeltaUpdate = typeof deltaUpdates.$inferInsert;

// ═══════════════════════════════════════════════════
// 19. ADMIN ACTIONS LOG (Master Admin Audit Trail)
// ═══════════════════════════════════════════════════
/**
 * Every admin action is logged immutably for full audit trail.
 * Supports targeting: individual, group, or all users.
 */
export const adminActions = mysqlTable("admin_actions", {
  id: int("id").autoincrement().primaryKey(),
  actionId: varchar("actionId", { length: 64 }).notNull().unique(),
  adminUserId: int("adminUserId").notNull(),
  actionType: mysqlEnum("actionType", [
    "block_user", "unblock_user", "promote_user", "demote_user",
    "hide_content", "show_content", "update_content", "delete_content",
    "toggle_feature", "update_config", "maintenance_mode",
    "send_notification", "reset_user", "force_logout",
    "ai_command", "bulk_action"
  ]).notNull(),
  targetScope: mysqlEnum("targetScope", ["individual", "group", "all"]).default("individual").notNull(),
  targetUserIds: json("targetUserIds"),              // Array of user IDs for individual/group
  description: text("description"),
  previousValue: json("previousValue"),              // State before the action
  newValue: json("newValue"),                        // State after the action
  aiPrompt: text("aiPrompt"),                        // If triggered by AI bot, the original prompt
  status: mysqlEnum("status", ["pending", "completed", "failed", "rolled_back"]).default("completed").notNull(),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_admin_actions_admin").on(table.adminUserId),
  index("idx_admin_actions_type").on(table.actionType),
  index("idx_admin_actions_scope").on(table.targetScope),
  index("idx_admin_actions_created").on(table.createdAt),
]);

export type AdminAction = typeof adminActions.$inferSelect;
export type InsertAdminAction = typeof adminActions.$inferInsert;

// ═══════════════════════════════════════════════════
// 20. FEATURE FLAGS (Dynamic Feature Control)
// ═══════════════════════════════════════════════════
export const featureFlags = mysqlTable("feature_flags", {
  id: int("id").autoincrement().primaryKey(),
  key: varchar("key", { length: 128 }).notNull().unique(),
  label: varchar("label", { length: 256 }).notNull(),
  description: text("description"),
  isEnabled: boolean("isEnabled").default(true).notNull(),
  scope: mysqlEnum("scope", ["global", "group", "individual"]).default("global").notNull(),
  targetUserIds: json("targetUserIds"),              // For group/individual scope
  metadata: json("metadata"),
  updatedBy: int("updatedBy"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
});

export type FeatureFlag = typeof featureFlags.$inferSelect;
export type InsertFeatureFlag = typeof featureFlags.$inferInsert;

// ═══════════════════════════════════════════════════
// 21. APP CONFIG (Dynamic Application Settings)
// ═══════════════════════════════════════════════════
export const appConfig = mysqlTable("app_config", {
  id: int("id").autoincrement().primaryKey(),
  key: varchar("key", { length: 128 }).notNull().unique(),
  value: json("value").notNull(),
  label: varchar("label", { length: 256 }),
  category: mysqlEnum("category", [
    "general", "navigation", "notifications", "security",
    "performance", "ui", "integrations", "limits"
  ]).default("general").notNull(),
  updatedBy: int("updatedBy"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
});

export type AppConfigRow = typeof appConfig.$inferSelect;
export type InsertAppConfig = typeof appConfig.$inferInsert;

// ═══════════════════════════════════════════════════
// 22. USER BLOCKS (Blocked Users Registry)
// ═══════════════════════════════════════════════════
export const userBlocks = mysqlTable("user_blocks", {
  id: int("id").autoincrement().primaryKey(),
  userId: int("userId").notNull(),
  blockedBy: int("blockedBy").notNull(),
  reason: text("reason"),
  expiresAt: timestamp("expiresAt"),
  isActive: boolean("isActive").default(true).notNull(),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_blocks_user").on(table.userId),
  index("idx_blocks_active").on(table.isActive),
  uniqueIndex("idx_blocks_unique").on(table.userId, table.blockedBy),
]);

export type UserBlock = typeof userBlocks.$inferSelect;
export type InsertUserBlock = typeof userBlocks.$inferInsert;

// ═══════════════════════════════════════════════════
// 23. ADMIN NOTIFICATIONS (Targeted Notifications)
// ═══════════════════════════════════════════════════
export const adminNotifications = mysqlTable("admin_notifications", {
  id: int("id").autoincrement().primaryKey(),
  notificationId: varchar("notificationId", { length: 64 }).notNull().unique(),
  title: varchar("title", { length: 256 }).notNull(),
  message: text("message").notNull(),
  type: mysqlEnum("type", ["info", "warning", "success", "error", "announcement"]).default("info").notNull(),
  targetScope: mysqlEnum("targetScope", ["individual", "group", "all"]).default("all").notNull(),
  targetUserIds: json("targetUserIds"),
  sentBy: int("sentBy").notNull(),
  isActive: boolean("isActive").default(true).notNull(),
  expiresAt: timestamp("expiresAt"),
  readBy: json("readBy"),                            // Array of user IDs who read it
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_admin_notif_scope").on(table.targetScope),
  index("idx_admin_notif_active").on(table.isActive),
  index("idx_admin_notif_created").on(table.createdAt),
]);

export type AdminNotification = typeof adminNotifications.$inferSelect;
export type InsertAdminNotification = typeof adminNotifications.$inferInsert;

// ═══════════════════════════════════════════════════
// 24. COLLABORATION SESSIONS (Real-Time Map Editing)
// ═══════════════════════════════════════════════════
/**
 * A collaboration session represents a shared map workspace.
 * Multiple users can join a session and see each other's cursors,
 * markers, and annotations in real-time.
 */
export const collaborationSessions = mysqlTable("collaboration_sessions", {
  id: int("id").autoincrement().primaryKey(),
  sessionId: varchar("sessionId", { length: 64 }).notNull().unique(),
  name: varchar("name", { length: 256 }).notNull(),
  createdBy: int("createdBy").notNull(),
  isActive: boolean("isActive").default(true).notNull(),
  centerLat: double("centerLat"),
  centerLon: double("centerLon"),
  zoomLevel: int("zoomLevel").default(12),
  maxParticipants: int("maxParticipants").default(20),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_collab_session_creator").on(table.createdBy),
  index("idx_collab_session_active").on(table.isActive),
]);

export type CollaborationSession = typeof collaborationSessions.$inferSelect;
export type InsertCollaborationSession = typeof collaborationSessions.$inferInsert;

// ═══════════════════════════════════════════════════
// 25. COLLABORATION PARTICIPANTS (Session Members)
// ═══════════════════════════════════════════════════
export const collaborationParticipants = mysqlTable("collaboration_participants", {
  id: int("id").autoincrement().primaryKey(),
  sessionId: varchar("sessionId", { length: 64 }).notNull(),
  userId: int("userId").notNull(),
  displayName: varchar("displayName", { length: 128 }),
  color: varchar("color", { length: 7 }).notNull(),     // hex color e.g. #FF5733
  cursorLat: double("cursorLat"),
  cursorLon: double("cursorLon"),
  isOnline: boolean("isOnline").default(true).notNull(),
  lastHeartbeat: timestamp("lastHeartbeat").defaultNow().notNull(),
  joinedAt: timestamp("joinedAt").defaultNow().notNull(),
}, (table) => [
  index("idx_collab_part_session").on(table.sessionId),
  index("idx_collab_part_user").on(table.userId),
  index("idx_collab_part_online").on(table.isOnline),
]);

export type CollaborationParticipant = typeof collaborationParticipants.$inferSelect;
export type InsertCollaborationParticipant = typeof collaborationParticipants.$inferInsert;

// ═══════════════════════════════════════════════════
// 26. SHARED MARKERS (Collaborative Map Markers)
// ═══════════════════════════════════════════════════
export const sharedMarkers = mysqlTable("shared_markers", {
  id: int("id").autoincrement().primaryKey(),
  markerId: varchar("markerId", { length: 64 }).notNull().unique(),
  sessionId: varchar("sessionId", { length: 64 }).notNull(),
  userId: int("userId").notNull(),
  lat: double("lat").notNull(),
  lon: double("lon").notNull(),
  label: varchar("label", { length: 256 }),
  description: text("description"),
  icon: varchar("icon", { length: 64 }).default("pin"),
  color: varchar("color", { length: 7 }).default("#00e5ff"),
  isActive: boolean("isActive").default(true).notNull(),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_shared_markers_session").on(table.sessionId),
  index("idx_shared_markers_user").on(table.userId),
  index("idx_shared_markers_active").on(table.isActive),
]);

export type SharedMarker = typeof sharedMarkers.$inferSelect;
export type InsertSharedMarker = typeof sharedMarkers.$inferInsert;

// ═══════════════════════════════════════════════════
// 27. SHARED ANNOTATIONS (Collaborative Annotations)
// ═══════════════════════════════════════════════════
export const sharedAnnotations = mysqlTable("shared_annotations", {
  id: int("id").autoincrement().primaryKey(),
  annotationId: varchar("annotationId", { length: 64 }).notNull().unique(),
  sessionId: varchar("sessionId", { length: 64 }).notNull(),
  userId: int("userId").notNull(),
  type: mysqlEnum("type", ["text", "route", "area", "measurement", "arrow"]).notNull(),
  data: json("data").notNull(),                          // { points, text, style, ... }
  color: varchar("color", { length: 7 }).default("#00e5ff"),
  isActive: boolean("isActive").default(true).notNull(),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
  updatedAt: timestamp("updatedAt").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_shared_annot_session").on(table.sessionId),
  index("idx_shared_annot_user").on(table.userId),
  index("idx_shared_annot_type").on(table.type),
]);

export type SharedAnnotation = typeof sharedAnnotations.$inferSelect;
export type InsertSharedAnnotation = typeof sharedAnnotations.$inferInsert;

// ═══════════════════════════════════════════════════
// 28. COLLABORATION EVENTS (Activity Log)
// ═══════════════════════════════════════════════════
export const collaborationEvents = mysqlTable("collaboration_events", {
  id: int("id").autoincrement().primaryKey(),
  sessionId: varchar("sessionId", { length: 64 }).notNull(),
  userId: int("userId").notNull(),
  type: mysqlEnum("type", [
    "user_joined", "user_left", "marker_added", "marker_moved",
    "marker_deleted", "annotation_added", "annotation_deleted",
    "cursor_moved", "session_created", "session_ended"
  ]).notNull(),
  payload: json("payload"),
  createdAt: timestamp("createdAt").defaultNow().notNull(),
}, (table) => [
  index("idx_collab_events_session").on(table.sessionId),
  index("idx_collab_events_user").on(table.userId),
  index("idx_collab_events_created").on(table.createdAt),
]);

export type CollaborationEvent = typeof collaborationEvents.$inferSelect;
export type InsertCollaborationEvent = typeof collaborationEvents.$inferInsert;

// ═══════════════════════════════════════════════════
// COLLABORATION INVITES
// ═══════════════════════════════════════════════════
/**
 * Invite tokens allow users to join collaboration sessions via shareable URLs.
 * Tokens expire after 24 hours and can have usage limits.
 */
export const collaborationInvites = mysqlTable("collaboration_invites", {
  id: int("id").autoincrement().primaryKey(),
  inviteToken: varchar("invite_token", { length: 64 }).notNull().unique(),
  sessionId: varchar("session_id", { length: 64 }).notNull(),
  createdBy: int("created_by").notNull(),
  maxUses: int("max_uses").default(0), // 0 = unlimited
  usedCount: int("used_count").default(0).notNull(),
  expiresAt: timestamp("expires_at").notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
  isActive: boolean("is_active").default(true).notNull(),
}, (table) => [
  index("idx_invite_token").on(table.inviteToken),
  index("idx_invite_session").on(table.sessionId),
]);

export type CollaborationInvite = typeof collaborationInvites.$inferSelect;
export type InsertCollaborationInvite = typeof collaborationInvites.$inferInsert;


// ═══════════════════════════════════════════════════
// 25. USER NOTIFICATIONS (In-App Notification System)
// ═══════════════════════════════════════════════════
/**
 * Per-user notifications with read/unread tracking.
 * Supports multiple notification types and optional metadata
 * for deep-linking or contextual actions.
 */
export const userNotifications = mysqlTable("user_notifications", {
  id: int("id").autoincrement().primaryKey(),
  notificationId: varchar("notification_id", { length: 64 }).notNull().unique(),
  userId: int("user_id").notNull(),                     // Target user
  type: mysqlEnum("type", ["info", "success", "warning", "error", "system", "collaboration", "admin"]).default("info").notNull(),
  title: varchar("title", { length: 256 }).notNull(),
  message: text("message").notNull(),
  isRead: boolean("is_read").default(false).notNull(),
  metadata: json("metadata"),                            // { link?, sessionId?, actionType?, sourceUserId? }
  createdAt: timestamp("created_at").defaultNow().notNull(),
  readAt: timestamp("read_at"),
  expiresAt: timestamp("expires_at"),                    // Optional auto-expiry
}, (table) => [
  index("idx_user_notif_user").on(table.userId),
  index("idx_user_notif_read").on(table.userId, table.isRead),
  index("idx_user_notif_created").on(table.createdAt),
  index("idx_user_notif_type").on(table.type),
]);

export type UserNotification = typeof userNotifications.$inferSelect;
export type InsertUserNotification = typeof userNotifications.$inferInsert;

// ═══════════════════════════════════════════════════
// 26. NOTIFICATION PREFERENCES (Per-User Settings)
// ═══════════════════════════════════════════════════
/**
 * Per-user notification preferences controlling which
 * notification types are enabled and delivery preferences.
 */
export const notificationPreferences = mysqlTable("notification_preferences", {
  id: int("id").autoincrement().primaryKey(),
  userId: int("user_id").notNull().unique(),
  enableInfo: boolean("enable_info").default(true).notNull(),
  enableSuccess: boolean("enable_success").default(true).notNull(),
  enableWarning: boolean("enable_warning").default(true).notNull(),
  enableError: boolean("enable_error").default(true).notNull(),
  enableSystem: boolean("enable_system").default(true).notNull(),
  enableCollaboration: boolean("enable_collaboration").default(true).notNull(),
  enableAdmin: boolean("enable_admin").default(true).notNull(),
  enableSound: boolean("enable_sound").default(true).notNull(),
  enableToast: boolean("enable_toast").default(true).notNull(),
  updatedAt: timestamp("updated_at").defaultNow().onUpdateNow().notNull(),
}, (table) => [
  index("idx_notif_prefs_user").on(table.userId),
]);

export type NotificationPreference = typeof notificationPreferences.$inferSelect;
export type InsertNotificationPreference = typeof notificationPreferences.$inferInsert;

// ═══════════════════════════════════════════════════════════
// 33. POINTS OF INTEREST (POI Database)
// ═══════════════════════════════════════════════════════════
export const pois = mysqlTable("pois", {
  id: int("id").primaryKey().autoincrement(),
  externalId: varchar("external_id", { length: 255 }),       // OSM node ID or Google Place ID
  source: mysqlEnum("source", ["osm", "google", "user", "import"]).notNull(),
  name: varchar("name", { length: 500 }).notNull(),
  nameLocal: varchar("name_local", { length: 500 }),         // Local language name
  category: varchar("category", { length: 100 }).notNull(),  // restaurant, gas_station, hospital, etc.
  subcategory: varchar("subcategory", { length: 100 }),
  lat: text("lat").notNull(),
  lon: text("lon").notNull(),
  address: text("address"),
  phone: varchar("phone", { length: 50 }),
  website: varchar("website", { length: 500 }),
  rating: text("rating"),                                     // Average rating (0-5)
  ratingCount: int("rating_count").default(0),
  priceLevel: int("price_level"),                             // 1-4 ($-$$$$)
  openNow: boolean("open_now"),
  hours: json("hours"),                                       // { mon: "09:00-22:00", ... }
  photos: json("photos"),                                     // [{ url, attribution }]
  tags: json("tags"),                                         // ["wifi", "parking", "wheelchair"]
  metadata: json("metadata"),                                 // Provider-specific extra data
  boundingBox: varchar("bounding_box", { length: 100 }),      // "lat1,lon1,lat2,lon2" for spatial queries
  lastUpdated: timestamp("last_updated").defaultNow().onUpdateNow().notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
}, (table) => [
  index("idx_pois_category").on(table.category),
  index("idx_pois_source").on(table.source),
  index("idx_pois_external").on(table.externalId),
  index("idx_pois_bbox").on(table.boundingBox),
]);

export type POI = typeof pois.$inferSelect;
export type InsertPOI = typeof pois.$inferInsert;
