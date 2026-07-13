/**
 * G.A.N.E — Server Module Barrel Export
 */

// ─── Routers ────────────────────────────────────────────
export { telemetryRouter } from './telemetryRouter';
export { anomalyRouter } from './anomalyRouter';
export { fleetRouter } from './fleetRouter';
export { incidentRouter, incidentStore } from './incidentEngine';
export { liveSharingRouter, liveSharingStore } from './liveSharing';
export { crowdRouter, crowdIntelligenceStore } from './crowdIntelligence';
export { analyticsRouter, analyticsPipelineStore } from './analyticsPipeline';

// ─── Infrastructure ─────────────────────────────────────
export { wsBridge } from './wsbridge';
export { ETLPipeline, getETLPipeline, startETLPipeline } from './etlPipeline';
export { haversineDistance, boundingBox, calculateRouteDistance, twoOptImprove } from './geo';
export { rateLimiter, securityHeaders, maxPayloadSize, requireJsonContentType, validateTelemetryPayload, sanitizeString, validateDeviceId, validateCoordinates, detectReplay } from './security';
export { integrationHub, IntegrationHub } from './integrationHub';
export type { NotificationPayload, AlertPriority, AlertCategory, ChannelType, NotificationResult } from './integrationHub';
export { TripManager } from './tripManager';
export { GeofenceEngine } from './geofenceEngine';
export { BackpressureController, RetryModel, DataRetentionEngine, HealthMonitor, backpressure, retryModel, dataRetention, healthMonitor } from './reliability';

// ─── Data Engines ───────────────────────────────────────
export { TrafficPipeline, trafficPipeline } from './trafficPipeline';
export type { SegmentTraffic, TrafficSnapshot, TrafficPipelineConfig } from './trafficPipeline';
export type { IncidentReport, IncidentType } from './incidentEngine';
export type { SharedLocation } from './liveSharing';
export type { SpeedSample, GridCell } from './crowdIntelligence';
export type { AnalyticsSnapshot, AnalyticsEvent } from './analyticsPipeline';

// ─── Observability ─────────────────────────────────────
export { observabilityRouter, serverTracer, ServerTracer } from './observabilityMiddleware';

// ─── Master Admin ─────────────────────────────────
export { masterAdminRouter } from './masterAdmin';

// ─── Content Moderation ────────────────────────────
export { contentModerationRouter, ModerationPipeline, ReputationEngine } from './contentModeration';
export type { ContentPayload, ModerationResult, ReputationScore, ContentType, ModerationAction } from './contentModeration';
