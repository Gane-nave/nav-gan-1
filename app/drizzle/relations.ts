import { relations } from "drizzle-orm";
import {
  users, devices, trips, routes, waypoints, alerts, paymentEvents,
  tripEvents, logEntries, integrationChannels, geofences,
  rawTelemetry, mapAnomalies, anomalyReports,
  fleets, vehicles, missions, deltaUpdates,
  adminActions, featureFlags, appConfig, userBlocks, adminNotifications,
  collaborationSessions, collaborationParticipants,
  sharedMarkers, sharedAnnotations, collaborationEvents, collaborationInvites,
  userNotifications, notificationPreferences,
} from "./schema";

// ─── Users ───
export const usersRelations = relations(users, ({ one, many }) => ({
  fleet: one(fleets, { fields: [users.fleetId], references: [fleets.id] }),
  devices: many(devices),
  trips: many(trips),
  vehicles: many(vehicles),
  missions: many(missions),
  alerts: many(alerts),
  notifications: many(userNotifications),
  notificationPreferences: one(notificationPreferences, { fields: [users.id], references: [notificationPreferences.userId] }),
}));

// ─── Devices ───
export const devicesRelations = relations(devices, ({ one }) => ({
  user: one(users, { fields: [devices.userId], references: [users.id] }),
}));

// ─── Trips ───
export const tripsRelations = relations(trips, ({ one, many }) => ({
  user: one(users, { fields: [trips.userId], references: [users.id] }),
  device: one(devices, { fields: [trips.deviceId], references: [devices.id] }),
  routes: many(routes),
  events: many(tripEvents),
}));

// ─── Routes ───
export const routesRelations = relations(routes, ({ one, many }) => ({
  trip: one(trips, { fields: [routes.tripId], references: [trips.id] }),
  user: one(users, { fields: [routes.userId], references: [users.id] }),
  waypoints: many(waypoints),
}));

// ─── Waypoints ───
export const waypointsRelations = relations(waypoints, ({ one }) => ({
  route: one(routes, { fields: [waypoints.routeId], references: [routes.id] }),
  trip: one(trips, { fields: [waypoints.tripId], references: [trips.id] }),
}));

// ─── Alerts ───
export const alertsRelations = relations(alerts, ({ one }) => ({
  user: one(users, { fields: [alerts.userId], references: [users.id] }),
  trip: one(trips, { fields: [alerts.tripId], references: [trips.id] }),
}));

// ─── Payment Events ───
export const paymentEventsRelations = relations(paymentEvents, ({ one }) => ({
  user: one(users, { fields: [paymentEvents.userId], references: [users.id] }),
  trip: one(trips, { fields: [paymentEvents.tripId], references: [trips.id] }),
}));

// ─── Trip Events ───
export const tripEventsRelations = relations(tripEvents, ({ one }) => ({
  trip: one(trips, { fields: [tripEvents.tripId], references: [trips.id] }),
}));

// ─── Fleets ───
export const fleetsRelations = relations(fleets, ({ one, many }) => ({
  owner: one(users, { fields: [fleets.ownerId], references: [users.id] }),
  vehicles: many(vehicles),
  missions: many(missions),
}));

// ─── Vehicles ───
export const vehiclesRelations = relations(vehicles, ({ one }) => ({
  fleet: one(fleets, { fields: [vehicles.fleetId], references: [fleets.id] }),
  driver: one(users, { fields: [vehicles.driverId], references: [users.id] }),
}));

// ─── Missions ───
export const missionsRelations = relations(missions, ({ one }) => ({
  fleet: one(fleets, { fields: [missions.fleetId], references: [fleets.id] }),
  vehicle: one(vehicles, { fields: [missions.vehicleId], references: [vehicles.id] }),
}));

// ─── Anomaly Reports ───
export const anomalyReportsRelations = relations(anomalyReports, ({ one }) => ({
  anomaly: one(mapAnomalies, { fields: [anomalyReports.anomalyId], references: [mapAnomalies.anomalyId] }),
}));

// ─── Delta Updates ───
export const deltaUpdatesRelations = relations(deltaUpdates, ({ one }) => ({
  anomaly: one(mapAnomalies, { fields: [deltaUpdates.anomalyId], references: [mapAnomalies.anomalyId] }),
}));

// ─── Collaboration Sessions ───
export const collaborationSessionsRelations = relations(collaborationSessions, ({ many }) => ({
  participants: many(collaborationParticipants),
  markers: many(sharedMarkers),
  annotations: many(sharedAnnotations),
  events: many(collaborationEvents),
  invites: many(collaborationInvites),
}));

// ─── Collaboration Participants ───
export const collaborationParticipantsRelations = relations(collaborationParticipants, ({ one }) => ({
  session: one(collaborationSessions, {
    fields: [collaborationParticipants.sessionId],
    references: [collaborationSessions.sessionId],
  }),
}));

// ─── Shared Markers ───
export const sharedMarkersRelations = relations(sharedMarkers, ({ one }) => ({
  session: one(collaborationSessions, {
    fields: [sharedMarkers.sessionId],
    references: [collaborationSessions.sessionId],
  }),
}));

// ─── Shared Annotations ───
export const sharedAnnotationsRelations = relations(sharedAnnotations, ({ one }) => ({
  session: one(collaborationSessions, {
    fields: [sharedAnnotations.sessionId],
    references: [collaborationSessions.sessionId],
  }),
}));

// ─── Collaboration Events ───
export const collaborationEventsRelations = relations(collaborationEvents, ({ one }) => ({
  session: one(collaborationSessions, {
    fields: [collaborationEvents.sessionId],
    references: [collaborationSessions.sessionId],
  }),
}));

// ─── Collaboration Invites ───
export const collaborationInvitesRelations = relations(collaborationInvites, ({ one }) => ({
  session: one(collaborationSessions, {
    fields: [collaborationInvites.sessionId],
    references: [collaborationSessions.sessionId],
  }),
}));

// ─── User Notifications ───
export const userNotificationsRelations = relations(userNotifications, ({ one }) => ({
  user: one(users, { fields: [userNotifications.userId], references: [users.id] }),
}));

// ─── Notification Preferences ───
export const notificationPreferencesRelations = relations(notificationPreferences, ({ one }) => ({
  user: one(users, { fields: [notificationPreferences.userId], references: [users.id] }),
}));
