/**
 * G.A.N.E — SCHEMA REGISTRY
 * 
 * Spec Reference: Doc 3 §6, Doc 9 §3
 * 
 * All schemas are versioned. Backward compatibility is mandatory.
 * Breaking changes require a new version.
 * Each schema defines required/optional fields, validation rules,
 * default behavior, and deprecation window.
 */

// ═══════════════════════════════════════════════════════════
// SCHEMA REGISTRY FRAMEWORK
// ═══════════════════════════════════════════════════════════

export interface SchemaField {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'object' | 'array' | 'enum' | 'timestamp' | 'uuid' | 'geopoint';
  required: boolean;
  description: string;
  defaultValue?: unknown;
  validation?: string; // Human-readable validation rule
  enumValues?: string[];
  deprecated?: boolean;
  deprecatedSince?: number; // Schema version
  replacedBy?: string;
}

export interface SchemaVersion {
  version: number;
  fields: SchemaField[];
  createdAt: string;
  changelog: string;
  backwardCompatible: boolean;
}

export interface SchemaDefinition {
  id: string;
  name: string;
  domain: string;
  description: string;
  currentVersion: number;
  versions: SchemaVersion[];
}

export interface SchemaValidationResult {
  valid: boolean;
  errors: { field: string; message: string; code: string }[];
  warnings: { field: string; message: string }[];
}

// ═══════════════════════════════════════════════════════════
// SCHEMA VALIDATOR
// ═══════════════════════════════════════════════════════════

export class SchemaValidator {
  private schemas = new Map<string, SchemaDefinition>();

  register(schema: SchemaDefinition): void {
    this.schemas.set(schema.id, schema);
  }

  getSchema(id: string): SchemaDefinition | undefined {
    return this.schemas.get(id);
  }

  validate(schemaId: string, data: Record<string, unknown>, version?: number): SchemaValidationResult {
    const schema = this.schemas.get(schemaId);
    if (!schema) {
      return { valid: false, errors: [{ field: '', message: `Schema ${schemaId} not found`, code: 'SCHEMA_NOT_FOUND' }], warnings: [] };
    }

    const v = version ?? schema.currentVersion;
    const schemaVersion = schema.versions.find(sv => sv.version === v);
    if (!schemaVersion) {
      return { valid: false, errors: [{ field: '', message: `Version ${v} not found for schema ${schemaId}`, code: 'VERSION_NOT_FOUND' }], warnings: [] };
    }

    const errors: { field: string; message: string; code: string }[] = [];
    const warnings: { field: string; message: string }[] = [];

    for (const field of schemaVersion.fields) {
      const value = data[field.name];

      // Check required
      if (field.required && (value === undefined || value === null)) {
        errors.push({ field: field.name, message: `Required field '${field.name}' is missing`, code: 'REQUIRED_FIELD_MISSING' });
        continue;
      }

      // Check deprecated
      if (field.deprecated && value !== undefined) {
        warnings.push({ field: field.name, message: `Field '${field.name}' is deprecated since v${field.deprecatedSince}${field.replacedBy ? `, use '${field.replacedBy}' instead` : ''}` });
      }

      // Type check (basic)
      if (value !== undefined && value !== null) {
        const typeValid = this.checkType(value, field.type);
        if (!typeValid) {
          errors.push({ field: field.name, message: `Field '${field.name}' expected type '${field.type}', got '${typeof value}'`, code: 'TYPE_MISMATCH' });
        }

        // Enum check
        if (field.type === 'enum' && field.enumValues && !field.enumValues.includes(String(value))) {
          errors.push({ field: field.name, message: `Field '${field.name}' must be one of: ${field.enumValues.join(', ')}`, code: 'ENUM_INVALID' });
        }
      }
    }

    return { valid: errors.length === 0, errors, warnings };
  }

  private checkType(value: unknown, expectedType: string): boolean {
    switch (expectedType) {
      case 'string': return typeof value === 'string';
      case 'number': return typeof value === 'number' && !isNaN(value as number);
      case 'boolean': return typeof value === 'boolean';
      case 'object': return typeof value === 'object' && !Array.isArray(value);
      case 'array': return Array.isArray(value);
      case 'enum': return typeof value === 'string';
      case 'timestamp': return typeof value === 'string' || typeof value === 'number';
      case 'uuid': return typeof value === 'string' && /^[0-9a-f-]{36}$/i.test(value);
      case 'geopoint': return typeof value === 'object' && value !== null && 'lat' in value && 'lon' in value;
      default: return true;
    }
  }

  listSchemas(): SchemaDefinition[] {
    return Array.from(this.schemas.values());
  }

  checkCompatibility(schemaId: string, fromVersion: number, toVersion: number): { compatible: boolean; breakingChanges: string[] } {
    const schema = this.schemas.get(schemaId);
    if (!schema) return { compatible: false, breakingChanges: ['Schema not found'] };

    const fromSchema = schema.versions.find(v => v.version === fromVersion);
    const toSchema = schema.versions.find(v => v.version === toVersion);
    if (!fromSchema || !toSchema) return { compatible: false, breakingChanges: ['Version not found'] };

    const breakingChanges: string[] = [];
    const fromFields = new Map(fromSchema.fields.map(f => [f.name, f]));

    // Check for removed required fields
    for (const [name, field] of Array.from(fromFields.entries())) {
      const toField = toSchema.fields.find(f => f.name === name);
      if (!toField && field.required) {
        breakingChanges.push(`Required field '${name}' was removed`);
      }
    }

    // Check for new required fields without defaults
    for (const field of toSchema.fields) {
      if (field.required && !fromFields.has(field.name) && field.defaultValue === undefined) {
        breakingChanges.push(`New required field '${field.name}' has no default value`);
      }
    }

    return { compatible: breakingChanges.length === 0, breakingChanges };
  }
}

// ═══════════════════════════════════════════════════════════
// CANONICAL SCHEMAS — Doc 3 §6
// ═══════════════════════════════════════════════════════════

export const CANONICAL_EVENT_SCHEMA: SchemaDefinition = {
  id: 'gane.event.v1',
  name: 'Canonical Event',
  domain: 'core',
  description: 'Base event schema for all system events',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'event_id', type: 'uuid', required: true, description: 'Unique event identifier' },
      { name: 'ts_utc', type: 'timestamp', required: true, description: 'UTC timestamp' },
      { name: 'seq_no', type: 'number', required: true, description: 'Monotonic sequence number' },
      { name: 'source', type: 'string', required: true, description: 'Producer service identifier' },
      { name: 'entity_type', type: 'string', required: true, description: 'Entity type (device, route, incident, etc.)' },
      { name: 'entity_id', type: 'string', required: true, description: 'Entity identifier' },
      { name: 'event_type', type: 'string', required: true, description: 'Event type name' },
      { name: 'schema_version', type: 'number', required: true, description: 'Schema version number' },
      { name: 'confidence', type: 'number', required: true, description: 'Confidence score [0..1]', validation: '0 <= value <= 1' },
      { name: 'freshness_ms', type: 'number', required: true, description: 'Data freshness in milliseconds' },
      { name: 'payload', type: 'object', required: true, description: 'Event-specific payload' },
      { name: 'reason_codes', type: 'array', required: false, description: 'Deterministic reason codes', defaultValue: [] },
      { name: 'correlation_id', type: 'string', required: false, description: 'Distributed tracing correlation ID' },
      { name: 'signature', type: 'string', required: false, description: 'Event signature for verification' },
    ],
  }],
};

export const DEVICE_SCHEMA: SchemaDefinition = {
  id: 'gane.device.v1',
  name: 'Device',
  domain: 'identity',
  description: 'Device registration and capabilities',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'device_id', type: 'string', required: true, description: 'Unique device identifier' },
      { name: 'user_id', type: 'string', required: false, description: 'Associated user identifier' },
      { name: 'platform', type: 'enum', required: true, description: 'Device platform', enumValues: ['iOS', 'Android', 'Web', 'Car', 'Desktop'] },
      { name: 'hardware_profile', type: 'string', required: true, description: 'Hardware profile identifier' },
      { name: 'gnss_capabilities', type: 'object', required: true, description: 'GNSS receiver capabilities' },
      { name: 'sensors', type: 'object', required: true, description: 'Available sensor types' },
      { name: 'app_version', type: 'string', required: true, description: 'Application version string' },
      { name: 'created_at', type: 'timestamp', required: true, description: 'Registration timestamp' },
      { name: 'updated_at', type: 'timestamp', required: true, description: 'Last update timestamp' },
    ],
  }],
};

export const POSITION_SCHEMA: SchemaDefinition = {
  id: 'gane.position.v1',
  name: 'Position Sample',
  domain: 'positioning',
  description: 'Resolved position with confidence and source breakdown',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'sample_id', type: 'uuid', required: true, description: 'Unique sample identifier' },
      { name: 'device_id', type: 'string', required: true, description: 'Source device' },
      { name: 'ts_utc', type: 'timestamp', required: true, description: 'Sample timestamp' },
      { name: 'lat', type: 'number', required: true, description: 'Latitude in degrees', validation: '-90 <= value <= 90' },
      { name: 'lon', type: 'number', required: true, description: 'Longitude in degrees', validation: '-180 <= value <= 180' },
      { name: 'alt_m', type: 'number', required: true, description: 'Altitude in meters' },
      { name: 'velocity_mps', type: 'number', required: true, description: 'Speed in meters per second' },
      { name: 'heading_deg', type: 'number', required: true, description: 'Heading in degrees [0..360)' },
      { name: 'covariance', type: 'array', required: false, description: '3x3 position covariance matrix (flattened)' },
      { name: 'confidence', type: 'number', required: true, description: 'Position confidence [0..1]' },
      { name: 'mode', type: 'enum', required: true, description: 'Positioning mode', enumValues: ['full_gnss', 'fusion', 'ins_only', 'bounded', 'dead_reckoning'] },
      { name: 'integrity_state', type: 'enum', required: true, description: 'Integrity assessment', enumValues: ['nominal', 'degraded', 'warning', 'critical'] },
      { name: 'source_breakdown', type: 'array', required: false, description: 'Per-source contribution weights' },
    ],
  }],
};

export const ROUTE_SCHEMA: SchemaDefinition = {
  id: 'gane.route.v1',
  name: 'Route',
  domain: 'routing',
  description: 'Computed route with segments and cost breakdown',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'route_id', type: 'uuid', required: true, description: 'Unique route identifier' },
      { name: 'user_id', type: 'string', required: false, description: 'Route owner' },
      { name: 'mode', type: 'enum', required: true, description: 'Transport mode', enumValues: ['driving', 'walking', 'cycling', 'transit'] },
      { name: 'origin', type: 'geopoint', required: true, description: 'Route start point' },
      { name: 'destination', type: 'geopoint', required: true, description: 'Route end point' },
      { name: 'waypoints', type: 'array', required: false, description: 'Intermediate waypoints', defaultValue: [] },
      { name: 'segments', type: 'array', required: true, description: 'Route segments' },
      { name: 'total_distance_m', type: 'number', required: true, description: 'Total distance in meters' },
      { name: 'eta_seconds', type: 'number', required: true, description: 'Estimated time of arrival in seconds' },
      { name: 'confidence', type: 'number', required: true, description: 'Route confidence [0..1]' },
      { name: 'route_score', type: 'object', required: true, description: 'Multi-objective cost breakdown' },
      { name: 'alternatives', type: 'array', required: false, description: 'Alternative route IDs', defaultValue: [] },
      { name: 'created_at', type: 'timestamp', required: true, description: 'Route computation timestamp' },
      { name: 'expires_at', type: 'timestamp', required: true, description: 'Route validity expiration' },
    ],
  }],
};

export const INCIDENT_SCHEMA: SchemaDefinition = {
  id: 'gane.incident.v1',
  name: 'Incident',
  domain: 'trust',
  description: 'Reported incident with trust scoring',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'incident_id', type: 'uuid', required: true, description: 'Unique incident identifier' },
      { name: 'type', type: 'enum', required: true, description: 'Incident type', enumValues: ['accident', 'hazard', 'closure', 'police', 'weather', 'roadwork', 'congestion', 'emergency'] },
      { name: 'status', type: 'enum', required: true, description: 'Incident lifecycle status', enumValues: ['pending', 'validated', 'rejected', 'resolved'] },
      { name: 'position', type: 'geopoint', required: true, description: 'Incident location' },
      { name: 'severity', type: 'enum', required: true, description: 'Severity level', enumValues: ['low', 'medium', 'high', 'critical'] },
      { name: 'truth_probability', type: 'number', required: true, description: 'Bayesian truth score [0..1]' },
      { name: 'evidence_refs', type: 'array', required: false, description: 'Evidence object references', defaultValue: [] },
      { name: 'source_count', type: 'number', required: true, description: 'Number of independent sources' },
      { name: 'reporter_id', type: 'string', required: true, description: 'Original reporter identifier' },
      { name: 'created_at', type: 'timestamp', required: true, description: 'Report timestamp' },
      { name: 'updated_at', type: 'timestamp', required: true, description: 'Last update timestamp' },
      { name: 'expires_at', type: 'timestamp', required: false, description: 'Auto-expiration timestamp' },
    ],
  }],
};

export const TRUST_SCHEMA: SchemaDefinition = {
  id: 'gane.trust.v1',
  name: 'Trust Record',
  domain: 'trust',
  description: 'Trust scoring for entities (users, sources, devices)',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'trust_id', type: 'uuid', required: true, description: 'Unique trust record identifier' },
      { name: 'subject_type', type: 'enum', required: true, description: 'Entity type', enumValues: ['user', 'source', 'incident', 'device'] },
      { name: 'subject_id', type: 'string', required: true, description: 'Entity identifier' },
      { name: 'trust_score', type: 'number', required: true, description: 'Overall trust score [0..1]' },
      { name: 'reliability_score', type: 'number', required: true, description: 'Historical reliability [0..1]' },
      { name: 'anomaly_score', type: 'number', required: true, description: 'Anomaly detection score [0..1]' },
      { name: 'evidence_count', type: 'number', required: true, description: 'Number of evidence points' },
      { name: 'decay_rate', type: 'number', required: true, description: 'Trust decay rate per hour' },
      { name: 'updated_at', type: 'timestamp', required: true, description: 'Last update timestamp' },
    ],
  }],
};

export const ALERT_SCHEMA: SchemaDefinition = {
  id: 'gane.alert.v1',
  name: 'Alert',
  domain: 'alerting',
  description: 'Alert dispatch record',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'alert_id', type: 'uuid', required: true, description: 'Unique alert identifier' },
      { name: 'channel', type: 'enum', required: true, description: 'Delivery channel', enumValues: ['in_app', 'email', 'push', 'in_app', 'sms'] },
      { name: 'recipient', type: 'string', required: true, description: 'Recipient identifier' },
      { name: 'template_id', type: 'string', required: true, description: 'Alert template identifier' },
      { name: 'payload', type: 'object', required: true, description: 'Alert-specific payload' },
      { name: 'priority', type: 'enum', required: true, description: 'Alert priority', enumValues: ['low', 'normal', 'high', 'critical'] },
      { name: 'status', type: 'enum', required: true, description: 'Delivery status', enumValues: ['queued', 'sent', 'delivered', 'failed'] },
      { name: 'retry_count', type: 'number', required: true, description: 'Number of delivery attempts', defaultValue: 0 },
      { name: 'created_at', type: 'timestamp', required: true, description: 'Alert creation timestamp' },
      { name: 'delivered_at', type: 'timestamp', required: false, description: 'Delivery confirmation timestamp' },
    ],
  }],
};

export const PAYMENT_SCHEMA: SchemaDefinition = {
  id: 'gane.payment.v1',
  name: 'Payment',
  domain: 'payments',
  description: 'Payment transaction record',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'payment_id', type: 'uuid', required: true, description: 'Unique payment identifier' },
      { name: 'customer_id', type: 'string', required: true, description: 'Customer identifier' },
      { name: 'provider', type: 'string', required: true, description: 'Payment provider name' },
      { name: 'provider_ref', type: 'string', required: false, description: 'Provider reference ID' },
      { name: 'amount_minor', type: 'number', required: true, description: 'Amount in minor units (cents/agorot)' },
      { name: 'currency', type: 'string', required: true, description: 'ISO 4217 currency code' },
      { name: 'status', type: 'enum', required: true, description: 'Payment status', enumValues: ['pending', 'authorized', 'captured', 'failed', 'refunded'] },
      { name: 'plan_id', type: 'string', required: false, description: 'Subscription plan identifier' },
      { name: 'failure_reason', type: 'string', required: false, description: 'Failure reason if applicable' },
      { name: 'created_at', type: 'timestamp', required: true, description: 'Payment initiation timestamp' },
      { name: 'updated_at', type: 'timestamp', required: true, description: 'Last status update timestamp' },
    ],
  }],
};

export const MAP_VERSION_SCHEMA: SchemaDefinition = {
  id: 'gane.mapversion.v1',
  name: 'Map Version',
  domain: 'geo',
  description: 'Map data version tracking',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'map_version_id', type: 'uuid', required: true, description: 'Unique version identifier' },
      { name: 'source_name', type: 'string', required: true, description: 'Data source name' },
      { name: 'region_code', type: 'string', required: true, description: 'Geographic region code' },
      { name: 'version_tag', type: 'string', required: true, description: 'Version tag string' },
      { name: 'effective_from', type: 'timestamp', required: true, description: 'Version effective date' },
      { name: 'rollback_from', type: 'string', required: false, description: 'Previous version for rollback' },
      { name: 'graph_ref', type: 'string', required: true, description: 'Road graph reference' },
      { name: 'tile_pack_ref', type: 'string', required: true, description: 'Tile package reference' },
      { name: 'validation_status', type: 'enum', required: true, description: 'Validation state', enumValues: ['pending', 'valid', 'rejected'] },
    ],
  }],
};

export const REPLAY_SESSION_SCHEMA: SchemaDefinition = {
  id: 'gane.replay.v1',
  name: 'Replay Session',
  domain: 'observability',
  description: 'Replay session for deterministic testing',
  currentVersion: 1,
  versions: [{
    version: 1,
    createdAt: '2026-04-01',
    changelog: 'Initial version',
    backwardCompatible: true,
    fields: [
      { name: 'replay_id', type: 'uuid', required: true, description: 'Unique replay session identifier' },
      { name: 'source_session_id', type: 'string', required: true, description: 'Original session to replay' },
      { name: 'time_range', type: 'object', required: true, description: 'Start/end timestamps' },
      { name: 'event_refs', type: 'array', required: true, description: 'Event references to replay' },
      { name: 'scenario_type', type: 'enum', required: true, description: 'Scenario classification', enumValues: ['real_world', 'simulated', 'failure_injection'] },
      { name: 'deterministic_result_hash', type: 'string', required: false, description: 'Hash of deterministic output' },
      { name: 'created_at', type: 'timestamp', required: true, description: 'Replay creation timestamp' },
    ],
  }],
};

// ═══════════════════════════════════════════════════════════
// REGISTRY SINGLETON
// ═══════════════════════════════════════════════════════════

export function createGaneSchemaRegistry(): SchemaValidator {
  const registry = new SchemaValidator();
  registry.register(CANONICAL_EVENT_SCHEMA);
  registry.register(DEVICE_SCHEMA);
  registry.register(POSITION_SCHEMA);
  registry.register(ROUTE_SCHEMA);
  registry.register(INCIDENT_SCHEMA);
  registry.register(TRUST_SCHEMA);
  registry.register(ALERT_SCHEMA);
  registry.register(PAYMENT_SCHEMA);
  registry.register(MAP_VERSION_SCHEMA);
  registry.register(REPLAY_SESSION_SCHEMA);
  return registry;
}
