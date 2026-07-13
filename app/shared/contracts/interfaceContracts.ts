/**
 * G.A.N.E — INTERFACE CONTRACTS BETWEEN MODULES
 * 
 * Spec Reference: Doc 9 §5
 * 
 * Every module boundary has a typed contract.
 * Contracts define inputs, outputs, error conditions, and SLOs.
 * These are the "seams" of the system — if a contract is violated,
 * the system MUST fail deterministically with a reason code.
 */

// ═══════════════════════════════════════════════════════════
// POSITIONING → ROUTING CONTRACT
// ═══════════════════════════════════════════════════════════

export interface PositionToRouting {
  /** Resolved position with confidence */
  position: {
    lat: number;
    lon: number;
    alt_m: number;
    velocity_mps: number;
    heading_deg: number;
    confidence: number;
    covariance_trace: number;
    mode: 'full_gnss' | 'fusion' | 'ins_only' | 'bounded' | 'dead_reckoning';
  };
  /** Map-matched position (if available) */
  map_matched?: {
    edge_id: string;
    offset: number;
    confidence: number;
  };
  /** Timestamp of the fix */
  ts_utc: string;
  /** SLO: position_fix must arrive within 1000ms */
  slo_max_latency_ms: 1000;
}

// ═══════════════════════════════════════════════════════════
// ROUTING → NAVIGATION CONTRACT
// ═══════════════════════════════════════════════════════════

export interface RoutingToNavigation {
  route_id: string;
  segments: RouteSegment[];
  total_distance_m: number;
  eta_seconds: number;
  eta_confidence: number;
  alternatives: string[];
  constraints_applied: string[];
  /** SLO: route computation must complete within 3000ms */
  slo_max_computation_ms: 3000;
}

export interface RouteSegment {
  segment_id: string;
  from: { lat: number; lon: number };
  to: { lat: number; lon: number };
  distance_m: number;
  duration_seconds: number;
  road_name?: string;
  road_class: 'motorway' | 'trunk' | 'primary' | 'secondary' | 'tertiary' | 'residential' | 'service';
  speed_limit_kmh?: number;
  instruction?: string;
  maneuver_type?: 'straight' | 'turn_left' | 'turn_right' | 'u_turn' | 'merge' | 'exit' | 'roundabout';
}

// ═══════════════════════════════════════════════════════════
// INCIDENT → ROUTING CONTRACT
// ═══════════════════════════════════════════════════════════

export interface IncidentToRouting {
  incident_id: string;
  type: string;
  position: { lat: number; lon: number };
  radius_m: number;
  severity: 'low' | 'medium' | 'high' | 'critical';
  truth_probability: number;
  /** Minimum truth probability for routing to consider this incident */
  routing_threshold: 0.6;
  /** Estimated duration in seconds */
  estimated_duration_seconds?: number;
  /** Affected road segments */
  affected_edges: string[];
}

// ═══════════════════════════════════════════════════════════
// TRUST → INCIDENT CONTRACT
// ═══════════════════════════════════════════════════════════

export interface TrustToIncident {
  subject_id: string;
  subject_type: 'user' | 'source' | 'device';
  trust_score: number;
  reliability_score: number;
  /** Minimum trust score for incident acceptance */
  acceptance_threshold: 0.3;
  /** Weight multiplier for Bayesian fusion */
  weight_multiplier: number;
}

// ═══════════════════════════════════════════════════════════
// CROWD → TRUST CONTRACT
// ═══════════════════════════════════════════════════════════

export interface CrowdToTrust {
  contributor_id: string;
  contribution_type: string;
  position: { lat: number; lon: number };
  timestamp: string;
  raw_value: unknown;
  /** Number of independent confirmations */
  confirmation_count: number;
  /** Spatial proximity of confirmations (meters) */
  confirmation_radius_m: number;
}

// ═══════════════════════════════════════════════════════════
// ALERT → DELIVERY CONTRACT
// ═══════════════════════════════════════════════════════════

export interface AlertToDelivery {
  alert_id: string;
  channel: 'in_app' | 'email' | 'push' | 'in_app' | 'sms';
  recipient: string;
  template_id: string;
  template_vars: Record<string, string>;
  priority: 'low' | 'normal' | 'high' | 'critical';
  /** SLO: alert dispatch must complete within 300ms */
  slo_max_dispatch_ms: 300;
  /** Maximum retry attempts */
  max_retries: 3;
  /** Retry backoff base in milliseconds */
  retry_backoff_base_ms: 1000;
}

// ═══════════════════════════════════════════════════════════
// ANALYTICS → PREDICTION CONTRACT
// ═══════════════════════════════════════════════════════════

export interface AnalyticsToPrediction {
  /** Feature vector for ML prediction */
  features: {
    hour_of_day: number;
    day_of_week: number;
    is_holiday: boolean;
    weather_code: string;
    historical_speed_avg: number;
    historical_speed_std: number;
    current_congestion_level: number;
    incident_count_nearby: number;
    event_count_nearby: number;
  };
  /** Prediction target */
  target: 'traffic_speed' | 'eta_correction' | 'incident_probability' | 'congestion_level';
  /** Required confidence for the prediction to be used */
  min_confidence: number;
  /** Maximum acceptable prediction latency */
  slo_max_latency_ms: 500;
}

// ═══════════════════════════════════════════════════════════
// SYNC → STORAGE CONTRACT
// ═══════════════════════════════════════════════════════════

export interface SyncToStorage {
  /** Entity type being synced */
  entity_type: string;
  /** Sync direction */
  direction: 'push' | 'pull' | 'bidirectional';
  /** Conflict resolution strategy */
  conflict_strategy: 'last_write_wins' | 'server_wins' | 'client_wins' | 'merge';
  /** Maximum batch size */
  max_batch_size: number;
  /** Compression enabled */
  compress: boolean;
  /** Delta sync supported */
  delta_sync: boolean;
}

// ═══════════════════════════════════════════════════════════
// V2X → POSITIONING CONTRACT
// ═══════════════════════════════════════════════════════════

export interface V2XToPositioning {
  message_type: 'BSM' | 'SPAT' | 'MAP' | 'TIM' | 'PSM' | 'RSA';
  sender_position: { lat: number; lon: number };
  sender_velocity_mps: number;
  sender_heading_deg: number;
  /** Signal quality [0..1] */
  signal_quality: number;
  /** Message freshness in milliseconds */
  freshness_ms: number;
  /** Maximum acceptable freshness for positioning use */
  max_freshness_ms: 500;
}

// ═══════════════════════════════════════════════════════════
// MAP → ROUTING CONTRACT
// ═══════════════════════════════════════════════════════════

export interface MapToRouting {
  /** Map version identifier */
  version_id: string;
  /** Region code */
  region_code: string;
  /** Graph format */
  graph_format: 'adjacency_list' | 'edge_list' | 'compressed';
  /** Number of nodes */
  node_count: number;
  /** Number of edges */
  edge_count: number;
  /** Last update timestamp */
  updated_at: string;
  /** Validation status */
  validated: boolean;
}

// ═══════════════════════════════════════════════════════════
// POLICY → ROUTING CONTRACT
// ═══════════════════════════════════════════════════════════

export interface PolicyToRouting {
  /** Active policy rules that affect routing */
  active_rules: {
    rule_id: string;
    type: 'speed_limit' | 'zone_restriction' | 'vehicle_class' | 'time_window' | 'emergency_override';
    constraint: Record<string, unknown>;
    priority: number;
  }[];
  /** Emergency override active */
  emergency_override: boolean;
  /** Compliance requirements */
  compliance_requirements: string[];
}

// ═══════════════════════════════════════════════════════════
// BATTERY → POSITIONING CONTRACT
// ═══════════════════════════════════════════════════════════

export interface BatteryToPositioning {
  /** Current power mode */
  power_mode: 'performance' | 'balanced' | 'power_saver' | 'ultra_saver' | 'emergency';
  /** Recommended GNSS update interval in milliseconds */
  gnss_interval_ms: number;
  /** Recommended sensor fusion rate in Hz */
  fusion_rate_hz: number;
  /** Whether to use WiFi/BLE positioning */
  use_auxiliary_positioning: boolean;
  /** Maximum allowed CPU usage percentage */
  max_cpu_percent: number;
}

// ═══════════════════════════════════════════════════════════
// PRIVACY → ALL MODULES CONTRACT
// ═══════════════════════════════════════════════════════════

export interface PrivacyContract {
  /** User consent level */
  consent_level: 'full' | 'limited' | 'minimal' | 'none';
  /** Data collection allowed */
  collect_position: boolean;
  collect_telemetry: boolean;
  collect_usage: boolean;
  share_crowd_data: boolean;
  /** Data retention period in days */
  retention_days: number;
  /** Anonymization required */
  anonymize: boolean;
  /** Encryption required for storage */
  encrypt_at_rest: boolean;
  /** Encryption required for transit */
  encrypt_in_transit: boolean;
}

// ═══════════════════════════════════════════════════════════
// CONTRACT REGISTRY — All contracts in one place
// ═══════════════════════════════════════════════════════════

export interface ContractRegistry {
  'positioning→routing': PositionToRouting;
  'routing→navigation': RoutingToNavigation;
  'incident→routing': IncidentToRouting;
  'trust→incident': TrustToIncident;
  'crowd→trust': CrowdToTrust;
  'alert→delivery': AlertToDelivery;
  'analytics→prediction': AnalyticsToPrediction;
  'sync→storage': SyncToStorage;
  'v2x→positioning': V2XToPositioning;
  'map→routing': MapToRouting;
  'policy→routing': PolicyToRouting;
  'battery→positioning': BatteryToPositioning;
  'privacy→all': PrivacyContract;
}

export type ContractName = keyof ContractRegistry;

/**
 * Contract SLO definitions
 */
export interface ContractSLO {
  contract: ContractName;
  metric: string;
  threshold: number;
  unit: 'ms' | 'percent' | 'count' | 'score';
  measurement_window_seconds: number;
}

export const CONTRACT_SLOS: ContractSLO[] = [
  { contract: 'positioning→routing', metric: 'position_fix_latency', threshold: 1000, unit: 'ms', measurement_window_seconds: 60 },
  { contract: 'routing→navigation', metric: 'route_computation_time', threshold: 3000, unit: 'ms', measurement_window_seconds: 300 },
  { contract: 'incident→routing', metric: 'incident_propagation_time', threshold: 500, unit: 'ms', measurement_window_seconds: 60 },
  { contract: 'trust→incident', metric: 'trust_evaluation_time', threshold: 200, unit: 'ms', measurement_window_seconds: 60 },
  { contract: 'alert→delivery', metric: 'alert_dispatch_time', threshold: 300, unit: 'ms', measurement_window_seconds: 60 },
  { contract: 'analytics→prediction', metric: 'prediction_latency', threshold: 500, unit: 'ms', measurement_window_seconds: 300 },
  { contract: 'v2x→positioning', metric: 'v2x_message_freshness', threshold: 500, unit: 'ms', measurement_window_seconds: 10 },
];
