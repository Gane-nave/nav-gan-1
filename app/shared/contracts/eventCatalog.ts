/**
 * G.A.N.E — EVENT CATALOG
 * 
 * Spec Reference: Doc 9 §4
 * 
 * Complete catalog of all system events.
 * Every event has a typed payload, schema version, and routing metadata.
 * Events are the backbone of the Event Bus and Observability Stack.
 */

// ═══════════════════════════════════════════════════════════
// EVENT DOMAINS
// ═══════════════════════════════════════════════════════════

export const EventDomain = {
  POSITIONING: 'positioning',
  ROUTING: 'routing',
  INCIDENT: 'incident',
  ALERT: 'alert',
  TRUST: 'trust',
  PAYMENT: 'payment',
  SYNC: 'sync',
  SYSTEM: 'system',
  CROWD: 'crowd',
  ANALYTICS: 'analytics',
  V2X: 'v2x',
  MAP: 'map',
} as const;

export type EventDomainType = typeof EventDomain[keyof typeof EventDomain];

// ═══════════════════════════════════════════════════════════
// EVENT ROUTING METADATA
// ═══════════════════════════════════════════════════════════

export interface EventRoutingMeta {
  /** Event type identifier */
  type: string;
  /** Domain this event belongs to */
  domain: EventDomainType;
  /** Schema version */
  schema_version: number;
  /** Priority: 0 (highest) to 3 (lowest) */
  priority: 0 | 1 | 2 | 3;
  /** Whether this event requires exactly-once delivery */
  exactly_once: boolean;
  /** Whether this event should be persisted to event store */
  persistent: boolean;
  /** Maximum delivery latency in milliseconds */
  max_latency_ms: number;
  /** TTL in milliseconds (0 = no expiry) */
  ttl_ms: number;
  /** Partitioning key expression */
  partition_key: string;
}

// ═══════════════════════════════════════════════════════════
// EVENT PAYLOADS — Typed payloads for each event
// ═══════════════════════════════════════════════════════════

// --- Positioning Events ---

export interface PositionFixPayload {
  device_id: string;
  lat: number;
  lon: number;
  alt_m: number;
  velocity_mps: number;
  heading_deg: number;
  confidence: number;
  mode: 'full_gnss' | 'fusion' | 'ins_only' | 'bounded' | 'dead_reckoning';
  covariance_trace: number;
  satellites_used: number;
  hdop: number;
}

export interface PositionDegradedPayload {
  device_id: string;
  from_mode: string;
  to_mode: string;
  reason: string;
  confidence_drop: number;
  recovery_estimate_ms: number;
}

export interface PositionLostPayload {
  device_id: string;
  last_known_lat: number;
  last_known_lon: number;
  last_fix_age_ms: number;
  fallback_mode: string;
}

export interface PositionRecoveredPayload {
  device_id: string;
  recovery_time_ms: number;
  new_mode: string;
  confidence: number;
}

export interface SpoofDetectedPayload {
  device_id: string;
  detection_method: 'residual_analysis' | 'clock_drift' | 'constellation_mismatch' | 'position_jump';
  confidence: number;
  affected_constellations: string[];
}

// --- Routing Events ---

export interface RouteComputedPayload {
  route_id: string;
  origin: { lat: number; lon: number };
  destination: { lat: number; lon: number };
  distance_m: number;
  eta_seconds: number;
  alternatives_count: number;
  provider: string;
  computation_time_ms: number;
}

export interface RerouteTriggeredPayload {
  route_id: string;
  reason: 'deviation' | 'incident' | 'traffic' | 'user_request' | 'policy';
  deviation_m: number;
  new_route_id: string;
  eta_delta_seconds: number;
}

export interface ETACorrectedPayload {
  route_id: string;
  previous_eta_seconds: number;
  corrected_eta_seconds: number;
  correction_source: 'traffic' | 'ml_prediction' | 'crowd' | 'historical';
  confidence: number;
}

export interface NavigationStartedPayload {
  route_id: string;
  device_id: string;
  mode: 'driving' | 'walking' | 'cycling' | 'transit';
}

export interface NavigationEndedPayload {
  route_id: string;
  device_id: string;
  reason: 'arrived' | 'cancelled' | 'timeout' | 'error';
  actual_duration_seconds: number;
  actual_distance_m: number;
}

// --- Incident Events ---

export interface IncidentReportedPayload {
  incident_id: string;
  type: string;
  lat: number;
  lon: number;
  severity: 'low' | 'medium' | 'high' | 'critical';
  reporter_id: string;
  description?: string;
}

export interface IncidentValidatedPayload {
  incident_id: string;
  truth_probability: number;
  source_count: number;
  validation_method: 'bayesian' | 'cross_source' | 'manual';
}

export interface IncidentResolvedPayload {
  incident_id: string;
  resolution_method: 'auto_expire' | 'manual' | 'counter_evidence';
  duration_seconds: number;
}

// --- Alert Events ---

export interface AlertDispatchedPayload {
  alert_id: string;
  channel: 'in_app' | 'email' | 'push' | 'in_app' | 'sms';
  recipient: string;
  template_id: string;
  priority: 'low' | 'normal' | 'high' | 'critical';
}

export interface AlertDeliveredPayload {
  alert_id: string;
  channel: string;
  delivery_time_ms: number;
}

export interface AlertFailedPayload {
  alert_id: string;
  channel: string;
  failure_reason: string;
  retry_count: number;
  will_retry: boolean;
}

// --- Trust Events ---

export interface TrustScoreUpdatedPayload {
  subject_type: 'user' | 'source' | 'incident' | 'device';
  subject_id: string;
  previous_score: number;
  new_score: number;
  reason: string;
}

export interface TrustViolationPayload {
  subject_type: string;
  subject_id: string;
  violation_type: 'spoofing' | 'spam' | 'false_report' | 'abuse';
  evidence: string;
  action_taken: 'warning' | 'throttle' | 'suspend' | 'ban';
}

// --- Payment Events ---

export interface PaymentInitiatedPayload {
  payment_id: string;
  customer_id: string;
  amount_minor: number;
  currency: string;
  provider: string;
}

export interface PaymentCompletedPayload {
  payment_id: string;
  provider_ref: string;
  amount_minor: number;
  currency: string;
}

export interface PaymentFailedPayload {
  payment_id: string;
  failure_reason: string;
  retryable: boolean;
}

// --- Sync Events ---

export interface SyncStartedPayload {
  device_id: string;
  sync_type: 'full' | 'incremental' | 'conflict_resolution';
  pending_changes: number;
}

export interface SyncCompletedPayload {
  device_id: string;
  changes_applied: number;
  conflicts_resolved: number;
  duration_ms: number;
}

// --- System Events ---

export interface ServiceHealthChangedPayload {
  service: string;
  previous_state: string;
  new_state: string;
  reason: string;
}

export interface CircuitBreakerTrippedPayload {
  service: string;
  failure_count: number;
  threshold: number;
  recovery_time_ms: number;
}

export interface SLOViolationPayload {
  slo_name: string;
  metric: string;
  threshold: number;
  actual_value: number;
  window_seconds: number;
}

// --- Crowd Events ---

export interface CrowdContributionPayload {
  contributor_id: string;
  contribution_type: 'speed' | 'incident' | 'road_condition' | 'poi';
  lat: number;
  lon: number;
  trust_score: number;
}

export interface CrowdConsensusPayload {
  topic_id: string;
  consensus_value: unknown;
  participant_count: number;
  confidence: number;
}

// --- V2X Events ---

export interface V2XMessagePayload {
  message_type: 'BSM' | 'SPAT' | 'MAP' | 'TIM' | 'PSM' | 'RSA';
  sender_id: string;
  lat: number;
  lon: number;
  payload: Record<string, unknown>;
}

// --- Map Events ---

export interface MapVersionUpdatedPayload {
  region_code: string;
  previous_version: string;
  new_version: string;
  change_type: 'incremental' | 'full';
}

export interface MapTileCorruptPayload {
  tile_id: string;
  zoom_level: number;
  checksum_expected: string;
  checksum_actual: string;
}

// ═══════════════════════════════════════════════════════════
// EVENT TYPE MAP — Maps event type string to payload type
// ═══════════════════════════════════════════════════════════

export interface EventTypeMap {
  // Positioning
  'position.fix': PositionFixPayload;
  'position.degraded': PositionDegradedPayload;
  'position.lost': PositionLostPayload;
  'position.recovered': PositionRecoveredPayload;
  'position.spoof_detected': SpoofDetectedPayload;

  // Routing
  'route.computed': RouteComputedPayload;
  'route.reroute_triggered': RerouteTriggeredPayload;
  'route.eta_corrected': ETACorrectedPayload;
  'route.navigation_started': NavigationStartedPayload;
  'route.navigation_ended': NavigationEndedPayload;

  // Incident
  'incident.reported': IncidentReportedPayload;
  'incident.validated': IncidentValidatedPayload;
  'incident.resolved': IncidentResolvedPayload;

  // Alert
  'alert.dispatched': AlertDispatchedPayload;
  'alert.delivered': AlertDeliveredPayload;
  'alert.failed': AlertFailedPayload;

  // Trust
  'trust.score_updated': TrustScoreUpdatedPayload;
  'trust.violation': TrustViolationPayload;

  // Payment
  'payment.initiated': PaymentInitiatedPayload;
  'payment.completed': PaymentCompletedPayload;
  'payment.failed': PaymentFailedPayload;

  // Sync
  'sync.started': SyncStartedPayload;
  'sync.completed': SyncCompletedPayload;

  // System
  'system.health_changed': ServiceHealthChangedPayload;
  'system.circuit_breaker_tripped': CircuitBreakerTrippedPayload;
  'system.slo_violation': SLOViolationPayload;

  // Crowd
  'crowd.contribution': CrowdContributionPayload;
  'crowd.consensus': CrowdConsensusPayload;

  // V2X
  'v2x.message': V2XMessagePayload;

  // Map
  'map.version_updated': MapVersionUpdatedPayload;
  'map.tile_corrupt': MapTileCorruptPayload;
}

export type EventTypeName = keyof EventTypeMap;

// ═══════════════════════════════════════════════════════════
// EVENT ROUTING TABLE — Routing metadata for each event type
// ═══════════════════════════════════════════════════════════

export const EVENT_ROUTING_TABLE: Record<EventTypeName, EventRoutingMeta> = {
  'position.fix': { type: 'position.fix', domain: 'positioning', schema_version: 1, priority: 0, exactly_once: false, persistent: false, max_latency_ms: 100, ttl_ms: 5000, partition_key: 'device_id' },
  'position.degraded': { type: 'position.degraded', domain: 'positioning', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 200, ttl_ms: 60000, partition_key: 'device_id' },
  'position.lost': { type: 'position.lost', domain: 'positioning', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 100, ttl_ms: 120000, partition_key: 'device_id' },
  'position.recovered': { type: 'position.recovered', domain: 'positioning', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 200, ttl_ms: 60000, partition_key: 'device_id' },
  'position.spoof_detected': { type: 'position.spoof_detected', domain: 'positioning', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 50, ttl_ms: 300000, partition_key: 'device_id' },

  'route.computed': { type: 'route.computed', domain: 'routing', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 500, ttl_ms: 3600000, partition_key: 'route_id' },
  'route.reroute_triggered': { type: 'route.reroute_triggered', domain: 'routing', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 300, ttl_ms: 600000, partition_key: 'route_id' },
  'route.eta_corrected': { type: 'route.eta_corrected', domain: 'routing', schema_version: 1, priority: 1, exactly_once: false, persistent: false, max_latency_ms: 1000, ttl_ms: 300000, partition_key: 'route_id' },
  'route.navigation_started': { type: 'route.navigation_started', domain: 'routing', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 1000, ttl_ms: 0, partition_key: 'route_id' },
  'route.navigation_ended': { type: 'route.navigation_ended', domain: 'routing', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 2000, ttl_ms: 0, partition_key: 'route_id' },

  'incident.reported': { type: 'incident.reported', domain: 'incident', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 300, ttl_ms: 86400000, partition_key: 'incident_id' },
  'incident.validated': { type: 'incident.validated', domain: 'incident', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 500, ttl_ms: 86400000, partition_key: 'incident_id' },
  'incident.resolved': { type: 'incident.resolved', domain: 'incident', schema_version: 1, priority: 2, exactly_once: true, persistent: true, max_latency_ms: 2000, ttl_ms: 0, partition_key: 'incident_id' },

  'alert.dispatched': { type: 'alert.dispatched', domain: 'alert', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 300, ttl_ms: 3600000, partition_key: 'alert_id' },
  'alert.delivered': { type: 'alert.delivered', domain: 'alert', schema_version: 1, priority: 2, exactly_once: true, persistent: true, max_latency_ms: 5000, ttl_ms: 0, partition_key: 'alert_id' },
  'alert.failed': { type: 'alert.failed', domain: 'alert', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 1000, ttl_ms: 3600000, partition_key: 'alert_id' },

  'trust.score_updated': { type: 'trust.score_updated', domain: 'trust', schema_version: 1, priority: 2, exactly_once: false, persistent: true, max_latency_ms: 2000, ttl_ms: 0, partition_key: 'subject_id' },
  'trust.violation': { type: 'trust.violation', domain: 'trust', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 500, ttl_ms: 0, partition_key: 'subject_id' },

  'payment.initiated': { type: 'payment.initiated', domain: 'payment', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 2000, ttl_ms: 0, partition_key: 'payment_id' },
  'payment.completed': { type: 'payment.completed', domain: 'payment', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 2000, ttl_ms: 0, partition_key: 'payment_id' },
  'payment.failed': { type: 'payment.failed', domain: 'payment', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 2000, ttl_ms: 0, partition_key: 'payment_id' },

  'sync.started': { type: 'sync.started', domain: 'sync', schema_version: 1, priority: 2, exactly_once: false, persistent: false, max_latency_ms: 5000, ttl_ms: 60000, partition_key: 'device_id' },
  'sync.completed': { type: 'sync.completed', domain: 'sync', schema_version: 1, priority: 2, exactly_once: true, persistent: true, max_latency_ms: 5000, ttl_ms: 0, partition_key: 'device_id' },

  'system.health_changed': { type: 'system.health_changed', domain: 'system', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 500, ttl_ms: 0, partition_key: 'service' },
  'system.circuit_breaker_tripped': { type: 'system.circuit_breaker_tripped', domain: 'system', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 200, ttl_ms: 0, partition_key: 'service' },
  'system.slo_violation': { type: 'system.slo_violation', domain: 'system', schema_version: 1, priority: 0, exactly_once: true, persistent: true, max_latency_ms: 300, ttl_ms: 0, partition_key: 'slo_name' },

  'crowd.contribution': { type: 'crowd.contribution', domain: 'crowd', schema_version: 1, priority: 2, exactly_once: false, persistent: true, max_latency_ms: 5000, ttl_ms: 3600000, partition_key: 'contributor_id' },
  'crowd.consensus': { type: 'crowd.consensus', domain: 'crowd', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 2000, ttl_ms: 0, partition_key: 'topic_id' },

  'v2x.message': { type: 'v2x.message', domain: 'v2x', schema_version: 1, priority: 0, exactly_once: false, persistent: false, max_latency_ms: 100, ttl_ms: 10000, partition_key: 'sender_id' },

  'map.version_updated': { type: 'map.version_updated', domain: 'map', schema_version: 1, priority: 2, exactly_once: true, persistent: true, max_latency_ms: 5000, ttl_ms: 0, partition_key: 'region_code' },
  'map.tile_corrupt': { type: 'map.tile_corrupt', domain: 'map', schema_version: 1, priority: 1, exactly_once: true, persistent: true, max_latency_ms: 2000, ttl_ms: 86400000, partition_key: 'tile_id' },
};

// ═══════════════════════════════════════════════════════════
// TYPED EVENT — Full event with routing + payload
// ═══════════════════════════════════════════════════════════

export interface TypedEvent<T extends EventTypeName = EventTypeName> {
  event_id: string;
  type: T;
  ts_utc: string;
  seq_no: number;
  source: string;
  schema_version: number;
  payload: EventTypeMap[T];
  correlation_id?: string;
  routing: EventRoutingMeta;
}

/**
 * Create a typed event with automatic routing metadata
 */
let globalSeqNo = 0;

export function createTypedEvent<T extends EventTypeName>(
  type: T,
  payload: EventTypeMap[T],
  source: string,
  correlationId?: string
): TypedEvent<T> {
  globalSeqNo++;
  const routing = EVENT_ROUTING_TABLE[type];
  const ts = Date.now().toString(36);
  const rnd = Math.random().toString(36).substring(2, 8);

  return {
    event_id: `evt_${ts}_${rnd}`,
    type,
    ts_utc: new Date().toISOString(),
    seq_no: globalSeqNo,
    source,
    schema_version: routing.schema_version,
    payload,
    correlation_id: correlationId,
    routing,
  };
}

/**
 * Get all events for a specific domain
 */
export function getEventsByDomain(domain: EventDomainType): EventTypeName[] {
  return (Object.keys(EVENT_ROUTING_TABLE) as EventTypeName[]).filter(
    key => EVENT_ROUTING_TABLE[key].domain === domain
  );
}

/**
 * Get all critical events (priority 0)
 */
export function getCriticalEvents(): EventTypeName[] {
  return (Object.keys(EVENT_ROUTING_TABLE) as EventTypeName[]).filter(
    key => EVENT_ROUTING_TABLE[key].priority === 0
  );
}

/**
 * Get all persistent events
 */
export function getPersistentEvents(): EventTypeName[] {
  return (Object.keys(EVENT_ROUTING_TABLE) as EventTypeName[]).filter(
    key => EVENT_ROUTING_TABLE[key].persistent
  );
}
