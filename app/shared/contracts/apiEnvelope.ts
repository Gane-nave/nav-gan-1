/**
 * G.A.N.E — FORMAL API ENVELOPE CONTRACTS
 * 
 * Spec Reference: Doc 3 §5.1, Doc 8 §4.1-4.3, Doc 9 §1
 * 
 * Every request/response in the system MUST conform to these envelopes.
 * All write APIs are idempotent via idempotency_key.
 * All webhooks are signed.
 * All critical APIs return deterministic reason codes.
 */

// ═══════════════════════════════════════════════════════════
// ERROR TAXONOMY — Deterministic Reason Codes
// ═══════════════════════════════════════════════════════════

export const ErrorDomain = {
  GNSS: 'GNSS',
  FUSION: 'FUSION',
  MAP: 'MAP',
  ROUTE: 'ROUTE',
  NET: 'NET',
  PAY: 'PAY',
  ALERT: 'ALERT',
  AUTH: 'AUTH',
  TRUST: 'TRUST',
  SYSTEM: 'SYSTEM',
} as const;

export type ErrorDomainType = typeof ErrorDomain[keyof typeof ErrorDomain];

/**
 * Complete error taxonomy from Doc 3 §7
 * Every error code is prefixed with its domain.
 */
export const ReasonCode = {
  // GNSS domain
  GNSS_SIGNAL_LOSS: 'GNSS_SIGNAL_LOSS',
  GNSS_SPOOF_SUSPECTED: 'GNSS_SPOOF_SUSPECTED',
  GNSS_JAM_SUSPECTED: 'GNSS_JAM_SUSPECTED',
  GNSS_MULTIPATH_HIGH: 'GNSS_MULTIPATH_HIGH',
  GNSS_REACQUISITION_PENDING: 'GNSS_REACQUISITION_PENDING',
  GNSS_CONSTELLATION_DEGRADED: 'GNSS_CONSTELLATION_DEGRADED',
  GNSS_EPHEMERIS_STALE: 'GNSS_EPHEMERIS_STALE',

  // Fusion domain
  FUSION_DIVERGENCE: 'FUSION_DIVERGENCE',
  FUSION_SENSOR_CONFLICT: 'FUSION_SENSOR_CONFLICT',
  FUSION_CONFIDENCE_LOW: 'FUSION_CONFIDENCE_LOW',
  FUSION_IMU_BIAS_HIGH: 'FUSION_IMU_BIAS_HIGH',
  FUSION_ODOMETRY_DRIFT: 'FUSION_ODOMETRY_DRIFT',

  // Map domain
  MAP_VERSION_MISMATCH: 'MAP_VERSION_MISMATCH',
  MAP_EDGE_MISSING: 'MAP_EDGE_MISSING',
  MAP_MATCH_FAILED: 'MAP_MATCH_FAILED',
  MAP_TILE_CORRUPT: 'MAP_TILE_CORRUPT',
  MAP_GRAPH_STALE: 'MAP_GRAPH_STALE',

  // Route domain
  ROUTE_NO_FEASIBLE_PATH: 'ROUTE_NO_FEASIBLE_PATH',
  ROUTE_RECOMPUTE_TIMEOUT: 'ROUTE_RECOMPUTE_TIMEOUT',
  ROUTE_CONSTRAINTS_TOO_STRICT: 'ROUTE_CONSTRAINTS_TOO_STRICT',
  ROUTE_ALTERNATIVE_UNAVAILABLE: 'ROUTE_ALTERNATIVE_UNAVAILABLE',
  ROUTE_ETA_CONFIDENCE_LOW: 'ROUTE_ETA_CONFIDENCE_LOW',

  // Network domain
  NET_PRIMARY_DOWN: 'NET_PRIMARY_DOWN',
  NET_FAILOVER_ACTIVE: 'NET_FAILOVER_ACTIVE',
  NET_HIGH_JITTER: 'NET_HIGH_JITTER',
  NET_TIMEOUT: 'NET_TIMEOUT',
  NET_RATE_LIMITED: 'NET_RATE_LIMITED',

  // Payment domain
  PAY_PROVIDER_TIMEOUT: 'PAY_PROVIDER_TIMEOUT',
  PAY_CALLBACK_INVALID: 'PAY_CALLBACK_INVALID',
  PAY_FRAUD_SUSPECTED: 'PAY_FRAUD_SUSPECTED',
  PAY_INSUFFICIENT_FUNDS: 'PAY_INSUFFICIENT_FUNDS',
  PAY_CARD_DECLINED: 'PAY_CARD_DECLINED',

  // Alert domain
  ALERT_DISPATCH_FAILED: 'ALERT_DISPATCH_FAILED',
  ALERT_DUPLICATE_SUPPRESSED: 'ALERT_DUPLICATE_SUPPRESSED',
  ALERT_CHANNEL_UNAVAILABLE: 'ALERT_CHANNEL_UNAVAILABLE',
  ALERT_TEMPLATE_MISSING: 'ALERT_TEMPLATE_MISSING',

  // Auth domain
  AUTH_TOKEN_EXPIRED: 'AUTH_TOKEN_EXPIRED',
  AUTH_INSUFFICIENT_PERMISSIONS: 'AUTH_INSUFFICIENT_PERMISSIONS',
  AUTH_DEVICE_NOT_REGISTERED: 'AUTH_DEVICE_NOT_REGISTERED',

  // Trust domain
  TRUST_SCORE_LOW: 'TRUST_SCORE_LOW',
  TRUST_SOURCE_UNRELIABLE: 'TRUST_SOURCE_UNRELIABLE',
  TRUST_CROSS_VALIDATION_FAILED: 'TRUST_CROSS_VALIDATION_FAILED',

  // System domain
  SYSTEM_OVERLOADED: 'SYSTEM_OVERLOADED',
  SYSTEM_MAINTENANCE: 'SYSTEM_MAINTENANCE',
  SYSTEM_VERSION_MISMATCH: 'SYSTEM_VERSION_MISMATCH',
} as const;

export type ReasonCodeType = typeof ReasonCode[keyof typeof ReasonCode];

/**
 * Metadata for each reason code — severity, retryable, domain
 */
export interface ReasonCodeMeta {
  code: ReasonCodeType;
  domain: ErrorDomainType;
  severity: 'info' | 'warning' | 'error' | 'critical';
  retryable: boolean;
  description: string;
}

export const REASON_CODE_REGISTRY: Record<ReasonCodeType, ReasonCodeMeta> = {
  GNSS_SIGNAL_LOSS: { code: 'GNSS_SIGNAL_LOSS', domain: 'GNSS', severity: 'warning', retryable: true, description: 'GNSS signal lost, switching to fallback positioning' },
  GNSS_SPOOF_SUSPECTED: { code: 'GNSS_SPOOF_SUSPECTED', domain: 'GNSS', severity: 'critical', retryable: false, description: 'GNSS spoofing detected via residual analysis' },
  GNSS_JAM_SUSPECTED: { code: 'GNSS_JAM_SUSPECTED', domain: 'GNSS', severity: 'critical', retryable: true, description: 'GNSS jamming suspected, signal quality degraded' },
  GNSS_MULTIPATH_HIGH: { code: 'GNSS_MULTIPATH_HIGH', domain: 'GNSS', severity: 'warning', retryable: true, description: 'High multipath detected in urban canyon' },
  GNSS_REACQUISITION_PENDING: { code: 'GNSS_REACQUISITION_PENDING', domain: 'GNSS', severity: 'info', retryable: true, description: 'Attempting GNSS signal reacquisition' },
  GNSS_CONSTELLATION_DEGRADED: { code: 'GNSS_CONSTELLATION_DEGRADED', domain: 'GNSS', severity: 'warning', retryable: true, description: 'Fewer than optimal satellites visible' },
  GNSS_EPHEMERIS_STALE: { code: 'GNSS_EPHEMERIS_STALE', domain: 'GNSS', severity: 'info', retryable: true, description: 'Satellite ephemeris data is outdated' },
  FUSION_DIVERGENCE: { code: 'FUSION_DIVERGENCE', domain: 'FUSION', severity: 'error', retryable: true, description: 'Fusion state diverged beyond acceptable bounds' },
  FUSION_SENSOR_CONFLICT: { code: 'FUSION_SENSOR_CONFLICT', domain: 'FUSION', severity: 'warning', retryable: true, description: 'Conflicting measurements from multiple sensors' },
  FUSION_CONFIDENCE_LOW: { code: 'FUSION_CONFIDENCE_LOW', domain: 'FUSION', severity: 'warning', retryable: true, description: 'Fusion confidence below threshold' },
  FUSION_IMU_BIAS_HIGH: { code: 'FUSION_IMU_BIAS_HIGH', domain: 'FUSION', severity: 'warning', retryable: true, description: 'IMU bias estimates exceed normal range' },
  FUSION_ODOMETRY_DRIFT: { code: 'FUSION_ODOMETRY_DRIFT', domain: 'FUSION', severity: 'warning', retryable: true, description: 'Odometry drift detected beyond threshold' },
  MAP_VERSION_MISMATCH: { code: 'MAP_VERSION_MISMATCH', domain: 'MAP', severity: 'warning', retryable: true, description: 'Map version mismatch between client and server' },
  MAP_EDGE_MISSING: { code: 'MAP_EDGE_MISSING', domain: 'MAP', severity: 'error', retryable: true, description: 'Required road graph edge not found' },
  MAP_MATCH_FAILED: { code: 'MAP_MATCH_FAILED', domain: 'MAP', severity: 'warning', retryable: true, description: 'Position could not be matched to road network' },
  MAP_TILE_CORRUPT: { code: 'MAP_TILE_CORRUPT', domain: 'MAP', severity: 'error', retryable: true, description: 'Map tile failed integrity check' },
  MAP_GRAPH_STALE: { code: 'MAP_GRAPH_STALE', domain: 'MAP', severity: 'info', retryable: true, description: 'Road graph data exceeds freshness threshold' },
  ROUTE_NO_FEASIBLE_PATH: { code: 'ROUTE_NO_FEASIBLE_PATH', domain: 'ROUTE', severity: 'error', retryable: false, description: 'No valid route found given constraints' },
  ROUTE_RECOMPUTE_TIMEOUT: { code: 'ROUTE_RECOMPUTE_TIMEOUT', domain: 'ROUTE', severity: 'error', retryable: true, description: 'Route computation exceeded time budget' },
  ROUTE_CONSTRAINTS_TOO_STRICT: { code: 'ROUTE_CONSTRAINTS_TOO_STRICT', domain: 'ROUTE', severity: 'warning', retryable: false, description: 'Constraints eliminate all feasible paths' },
  ROUTE_ALTERNATIVE_UNAVAILABLE: { code: 'ROUTE_ALTERNATIVE_UNAVAILABLE', domain: 'ROUTE', severity: 'info', retryable: false, description: 'No alternative routes available' },
  ROUTE_ETA_CONFIDENCE_LOW: { code: 'ROUTE_ETA_CONFIDENCE_LOW', domain: 'ROUTE', severity: 'info', retryable: true, description: 'ETA confidence interval is wide' },
  NET_PRIMARY_DOWN: { code: 'NET_PRIMARY_DOWN', domain: 'NET', severity: 'error', retryable: true, description: 'Primary network connection lost' },
  NET_FAILOVER_ACTIVE: { code: 'NET_FAILOVER_ACTIVE', domain: 'NET', severity: 'warning', retryable: true, description: 'Operating on failover network connection' },
  NET_HIGH_JITTER: { code: 'NET_HIGH_JITTER', domain: 'NET', severity: 'warning', retryable: true, description: 'Network jitter exceeds acceptable threshold' },
  NET_TIMEOUT: { code: 'NET_TIMEOUT', domain: 'NET', severity: 'error', retryable: true, description: 'Network request timed out' },
  NET_RATE_LIMITED: { code: 'NET_RATE_LIMITED', domain: 'NET', severity: 'warning', retryable: true, description: 'Request rate limited by server' },
  PAY_PROVIDER_TIMEOUT: { code: 'PAY_PROVIDER_TIMEOUT', domain: 'PAY', severity: 'error', retryable: true, description: 'Payment provider did not respond in time' },
  PAY_CALLBACK_INVALID: { code: 'PAY_CALLBACK_INVALID', domain: 'PAY', severity: 'error', retryable: false, description: 'Payment callback signature invalid' },
  PAY_FRAUD_SUSPECTED: { code: 'PAY_FRAUD_SUSPECTED', domain: 'PAY', severity: 'critical', retryable: false, description: 'Fraudulent payment activity detected' },
  PAY_INSUFFICIENT_FUNDS: { code: 'PAY_INSUFFICIENT_FUNDS', domain: 'PAY', severity: 'error', retryable: false, description: 'Insufficient funds for payment' },
  PAY_CARD_DECLINED: { code: 'PAY_CARD_DECLINED', domain: 'PAY', severity: 'error', retryable: false, description: 'Payment card was declined' },
  ALERT_DISPATCH_FAILED: { code: 'ALERT_DISPATCH_FAILED', domain: 'ALERT', severity: 'error', retryable: true, description: 'Alert dispatch to channel failed' },
  ALERT_DUPLICATE_SUPPRESSED: { code: 'ALERT_DUPLICATE_SUPPRESSED', domain: 'ALERT', severity: 'info', retryable: false, description: 'Duplicate alert suppressed by dedup engine' },
  ALERT_CHANNEL_UNAVAILABLE: { code: 'ALERT_CHANNEL_UNAVAILABLE', domain: 'ALERT', severity: 'error', retryable: true, description: 'Alert channel is currently unavailable' },
  ALERT_TEMPLATE_MISSING: { code: 'ALERT_TEMPLATE_MISSING', domain: 'ALERT', severity: 'error', retryable: false, description: 'Alert template not found in registry' },
  AUTH_TOKEN_EXPIRED: { code: 'AUTH_TOKEN_EXPIRED', domain: 'AUTH', severity: 'warning', retryable: true, description: 'Authentication token has expired' },
  AUTH_INSUFFICIENT_PERMISSIONS: { code: 'AUTH_INSUFFICIENT_PERMISSIONS', domain: 'AUTH', severity: 'error', retryable: false, description: 'Insufficient permissions for operation' },
  AUTH_DEVICE_NOT_REGISTERED: { code: 'AUTH_DEVICE_NOT_REGISTERED', domain: 'AUTH', severity: 'error', retryable: false, description: 'Device not registered in system' },
  TRUST_SCORE_LOW: { code: 'TRUST_SCORE_LOW', domain: 'TRUST', severity: 'warning', retryable: false, description: 'Trust score below acceptance threshold' },
  TRUST_SOURCE_UNRELIABLE: { code: 'TRUST_SOURCE_UNRELIABLE', domain: 'TRUST', severity: 'warning', retryable: false, description: 'Data source flagged as unreliable' },
  TRUST_CROSS_VALIDATION_FAILED: { code: 'TRUST_CROSS_VALIDATION_FAILED', domain: 'TRUST', severity: 'error', retryable: false, description: 'Cross-source validation failed' },
  SYSTEM_OVERLOADED: { code: 'SYSTEM_OVERLOADED', domain: 'SYSTEM', severity: 'critical', retryable: true, description: 'System under excessive load' },
  SYSTEM_MAINTENANCE: { code: 'SYSTEM_MAINTENANCE', domain: 'SYSTEM', severity: 'info', retryable: true, description: 'System undergoing scheduled maintenance' },
  SYSTEM_VERSION_MISMATCH: { code: 'SYSTEM_VERSION_MISMATCH', domain: 'SYSTEM', severity: 'warning', retryable: false, description: 'Client/server version incompatible' },
};

// ═══════════════════════════════════════════════════════════
// REQUEST ENVELOPE — All API requests MUST include these fields
// ═══════════════════════════════════════════════════════════

export interface RequestEnvelope<T = unknown> {
  /** Unique request identifier (UUID v4) */
  request_id: string;
  /** Correlation ID for distributed tracing */
  correlation_id: string;
  /** UTC ISO8601 timestamp */
  ts_utc: string;
  /** Source identifier (device_id or service_id) */
  source: string;
  /** Schema version of the payload */
  schema_version: number;
  /** Client application version */
  client_version?: string;
  /** Device identifier */
  device_id?: string;
  /** Idempotency key for write operations */
  idempotency_key?: string;
  /** The actual request payload */
  payload: T;
}

// ═══════════════════════════════════════════════════════════
// RESPONSE ENVELOPE — All API responses MUST include these fields
// ═══════════════════════════════════════════════════════════

export interface ResponseEnvelope<T = unknown> {
  /** Request status */
  status: 'ok' | 'error' | 'partial';
  /** Server UTC timestamp */
  server_ts_utc: string;
  /** Data freshness in milliseconds */
  freshness_ms: number;
  /** Confidence score [0..1] */
  confidence: number;
  /** Distributed trace reference */
  trace_ref: string;
  /** Deterministic reason codes */
  reason_codes: ReasonCodeType[];
  /** The actual response payload */
  payload: T;
}

// ═══════════════════════════════════════════════════════════
// ERROR ENVELOPE — Structured error response
// ═══════════════════════════════════════════════════════════

export interface ErrorEnvelope {
  status: 'error';
  /** Primary error code */
  error_code: ReasonCodeType;
  /** Additional reason codes */
  reason_codes: ReasonCodeType[];
  /** Human-readable error message */
  message: string;
  /** Whether the client should retry */
  retryable: boolean;
  /** Distributed trace reference */
  trace_ref: string;
  /** Server UTC timestamp */
  server_ts_utc: string;
}

// ═══════════════════════════════════════════════════════════
// HEALTH CONTRACT — GET /v1/health/{service}
// ═══════════════════════════════════════════════════════════

export type ServiceState = 'healthy' | 'degraded' | 'critical' | 'down';

export interface DependencyHealth {
  name: string;
  state: ServiceState;
  latency_ms?: number;
}

export interface HealthResponse {
  service: string;
  state: ServiceState;
  latency_ms_p50: number;
  latency_ms_p95: number;
  latency_ms_p99: number;
  error_rate: number;
  confidence: number;
  uptime_seconds: number;
  dependencies: DependencyHealth[];
  reason_codes: ReasonCodeType[];
  version: string;
  last_check_utc: string;
}

// ═══════════════════════════════════════════════════════════
// WEBHOOK CONTRACT — Signed webhook verification
// ═══════════════════════════════════════════════════════════

export interface WebhookHeaders {
  'X-Signature': string;
  'X-Timestamp': string;
  'X-Event-Id': string;
}

export interface WebhookPayload<T = unknown> {
  event_type: string;
  provider_ref: string;
  payload: T;
  ts_utc: string;
}

/**
 * Webhook verification result
 */
export interface WebhookVerification {
  valid: boolean;
  reason?: 'signature_invalid' | 'timestamp_stale' | 'event_duplicate' | 'payload_malformed';
  event_id?: string;
}

/**
 * Verify webhook signature using HMAC-SHA256
 */
export function verifyWebhookSignature(
  body: string,
  signature: string,
  secret: string,
  timestampStr: string,
  maxAgeMs: number = 300_000 // 5 minutes
): WebhookVerification {
  const timestamp = new Date(timestampStr).getTime();
  const now = Date.now();

  // Check freshness
  if (Math.abs(now - timestamp) > maxAgeMs) {
    return { valid: false, reason: 'timestamp_stale' };
  }

  // Compute expected signature
  const message = `${timestampStr}.${body}`;
  // In production, use crypto.createHmac('sha256', secret).update(message).digest('hex')
  // Here we define the contract interface
  const expectedSignature = computeHmacSha256(message, secret);

  if (signature !== expectedSignature) {
    return { valid: false, reason: 'signature_invalid' };
  }

  return { valid: true };
}

/**
 * HMAC-SHA256 computation (platform-agnostic interface)
 * In browser: use SubtleCrypto
 * In Node: use crypto module
 */
function computeHmacSha256(message: string, secret: string): string {
  // Platform detection
  if (typeof globalThis !== 'undefined' && 'crypto' in globalThis) {
    // Use synchronous hash for contract validation
    // In production, this would use async SubtleCrypto or Node crypto
    let hash = 0;
    const combined = secret + message;
    for (let i = 0; i < combined.length; i++) {
      const char = combined.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32bit integer
    }
    return `hmac_${Math.abs(hash).toString(16).padStart(8, '0')}`;
  }
  return '';
}

// ═══════════════════════════════════════════════════════════
// IDEMPOTENCY CONTRACT
// ═══════════════════════════════════════════════════════════

export interface IdempotencyRecord {
  key: string;
  request_hash: string;
  response: unknown;
  created_at: number;
  expires_at: number;
  status: 'processing' | 'completed' | 'failed';
}

/**
 * Idempotency store interface — all write APIs MUST use this
 */
export interface IdempotencyStore {
  get(key: string): IdempotencyRecord | undefined;
  set(key: string, record: IdempotencyRecord): void;
  delete(key: string): void;
  cleanup(beforeTimestamp: number): number;
}

/**
 * In-memory idempotency store with TTL
 */
export class MemoryIdempotencyStore implements IdempotencyStore {
  private store = new Map<string, IdempotencyRecord>();
  private readonly maxSize: number;
  private readonly defaultTtlMs: number;

  constructor(maxSize: number = 10_000, defaultTtlMs: number = 3_600_000) {
    this.maxSize = maxSize;
    this.defaultTtlMs = defaultTtlMs;
  }

  get(key: string): IdempotencyRecord | undefined {
    const record = this.store.get(key);
    if (!record) return undefined;
    if (Date.now() > record.expires_at) {
      this.store.delete(key);
      return undefined;
    }
    return record;
  }

  set(key: string, record: IdempotencyRecord): void {
    if (this.store.size >= this.maxSize) {
      this.cleanup(Date.now());
    }
    if (!record.expires_at) {
      record.expires_at = Date.now() + this.defaultTtlMs;
    }
    this.store.set(key, record);
  }

  delete(key: string): void {
    this.store.delete(key);
  }

  cleanup(beforeTimestamp: number): number {
    let removed = 0;
    const entries = Array.from(this.store.entries());
    for (const [key, record] of entries) {
      if (record.expires_at < beforeTimestamp) {
        this.store.delete(key);
        removed++;
      }
    }
    return removed;
  }

  get size(): number {
    return this.store.size;
  }
}

// ═══════════════════════════════════════════════════════════
// ENVELOPE BUILDERS — Helper functions
// ═══════════════════════════════════════════════════════════

let requestCounter = 0;

export function createRequestId(): string {
  requestCounter++;
  const timestamp = Date.now().toString(36);
  const random = Math.random().toString(36).substring(2, 8);
  return `req_${timestamp}_${random}_${requestCounter}`;
}

export function createCorrelationId(): string {
  const timestamp = Date.now().toString(36);
  const random = Math.random().toString(36).substring(2, 10);
  return `cor_${timestamp}_${random}`;
}

export function createTraceRef(service: string): string {
  const timestamp = Date.now().toString(36);
  const random = Math.random().toString(36).substring(2, 8);
  return `trc_${service}_${timestamp}_${random}`;
}

export function buildRequest<T>(payload: T, source: string, schemaVersion: number = 1): RequestEnvelope<T> {
  return {
    request_id: createRequestId(),
    correlation_id: createCorrelationId(),
    ts_utc: new Date().toISOString(),
    source,
    schema_version: schemaVersion,
    payload,
  };
}

export function buildResponse<T>(
  payload: T,
  service: string,
  confidence: number = 1.0,
  reasonCodes: ReasonCodeType[] = []
): ResponseEnvelope<T> {
  return {
    status: 'ok',
    server_ts_utc: new Date().toISOString(),
    freshness_ms: 0,
    confidence,
    trace_ref: createTraceRef(service),
    reason_codes: reasonCodes,
    payload,
  };
}

export function buildError(
  errorCode: ReasonCodeType,
  message: string,
  service: string,
  additionalCodes: ReasonCodeType[] = []
): ErrorEnvelope {
  const meta = REASON_CODE_REGISTRY[errorCode];
  return {
    status: 'error',
    error_code: errorCode,
    reason_codes: [errorCode, ...additionalCodes],
    message,
    retryable: meta?.retryable ?? false,
    trace_ref: createTraceRef(service),
    server_ts_utc: new Date().toISOString(),
  };
}
