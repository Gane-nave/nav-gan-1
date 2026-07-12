/**
 * G.A.N.E — DATA ENGINEERING DEPTH
 * 
 * Spec Reference: Doc 3 §6, Doc 8 §9-12, Doc 9 §9
 * 
 * 1. Feature Store Design
 * 2. Map Diff Protocol
 * 3. Offline Package Manifest
 * 4. Compaction Strategy
 * 5. Retention Policies
 * 6. Provenance Model
 * 7. Consistency Model
 */

// ═══════════════════════════════════════════════════════════
// 1. FEATURE STORE — ML Feature Management
// ═══════════════════════════════════════════════════════════

export interface FeatureDefinition {
  name: string;
  group: string;
  type: 'numeric' | 'categorical' | 'boolean' | 'timestamp' | 'vector';
  source: 'realtime' | 'batch' | 'derived';
  freshness_sla_seconds: number;
  computation: string;
  dependencies: string[];
  version: number;
}

export const FEATURE_DEFINITIONS: FeatureDefinition[] = [
  // Traffic features
  { name: 'segment_speed_avg', group: 'traffic', type: 'numeric', source: 'realtime', freshness_sla_seconds: 60, computation: 'avg(speed) over segment in 5min window', dependencies: ['position.fix'], version: 1 },
  { name: 'segment_speed_std', group: 'traffic', type: 'numeric', source: 'realtime', freshness_sla_seconds: 60, computation: 'stddev(speed) over segment in 5min window', dependencies: ['position.fix'], version: 1 },
  { name: 'segment_congestion_level', group: 'traffic', type: 'numeric', source: 'derived', freshness_sla_seconds: 60, computation: '1 - (segment_speed_avg / speed_limit)', dependencies: ['segment_speed_avg'], version: 1 },
  { name: 'segment_flow_rate', group: 'traffic', type: 'numeric', source: 'realtime', freshness_sla_seconds: 300, computation: 'count(unique_devices) over segment in 5min window', dependencies: ['position.fix'], version: 1 },
  // Temporal features
  { name: 'hour_of_day', group: 'temporal', type: 'numeric', source: 'derived', freshness_sla_seconds: 3600, computation: 'extract(hour from current_timestamp)', dependencies: [], version: 1 },
  { name: 'day_of_week', group: 'temporal', type: 'numeric', source: 'derived', freshness_sla_seconds: 86400, computation: 'extract(dow from current_timestamp)', dependencies: [], version: 1 },
  { name: 'is_holiday', group: 'temporal', type: 'boolean', source: 'batch', freshness_sla_seconds: 86400, computation: 'lookup(holiday_calendar, date)', dependencies: [], version: 1 },
  { name: 'is_rush_hour', group: 'temporal', type: 'boolean', source: 'derived', freshness_sla_seconds: 3600, computation: 'hour_of_day in [7,8,9,16,17,18]', dependencies: ['hour_of_day'], version: 1 },
  // Weather features
  { name: 'weather_code', group: 'weather', type: 'categorical', source: 'batch', freshness_sla_seconds: 1800, computation: 'api_fetch(weather_service)', dependencies: [], version: 1 },
  { name: 'precipitation_mm', group: 'weather', type: 'numeric', source: 'batch', freshness_sla_seconds: 1800, computation: 'api_fetch(weather_service).precipitation', dependencies: [], version: 1 },
  { name: 'visibility_km', group: 'weather', type: 'numeric', source: 'batch', freshness_sla_seconds: 1800, computation: 'api_fetch(weather_service).visibility', dependencies: [], version: 1 },
  // Incident features
  { name: 'incident_count_radius_1km', group: 'incident', type: 'numeric', source: 'realtime', freshness_sla_seconds: 60, computation: 'count(active_incidents) within 1km', dependencies: ['incident.reported'], version: 1 },
  { name: 'incident_severity_max', group: 'incident', type: 'numeric', source: 'realtime', freshness_sla_seconds: 60, computation: 'max(severity) of active_incidents within 1km', dependencies: ['incident.reported'], version: 1 },
  // Historical features
  { name: 'historical_speed_segment_hour', group: 'historical', type: 'numeric', source: 'batch', freshness_sla_seconds: 86400, computation: 'avg(speed) by segment,hour over 30 days', dependencies: [], version: 1 },
  { name: 'historical_incident_rate', group: 'historical', type: 'numeric', source: 'batch', freshness_sla_seconds: 86400, computation: 'count(incidents) / days by segment over 90 days', dependencies: [], version: 1 },
  // Device features
  { name: 'device_battery_level', group: 'device', type: 'numeric', source: 'realtime', freshness_sla_seconds: 60, computation: 'battery.level', dependencies: [], version: 1 },
  { name: 'device_signal_strength', group: 'device', type: 'numeric', source: 'realtime', freshness_sla_seconds: 30, computation: 'gnss.signal_strength', dependencies: [], version: 1 },
  { name: 'device_position_confidence', group: 'device', type: 'numeric', source: 'realtime', freshness_sla_seconds: 10, computation: 'eskf.confidence', dependencies: ['position.fix'], version: 1 },
];

export interface FeatureVector {
  timestamp: number;
  features: Record<string, number | string | boolean>;
  metadata: {
    source_versions: Record<string, number>;
    staleness_ms: Record<string, number>;
    quality_score: number;
  };
}

// ═══════════════════════════════════════════════════════════
// 2. MAP DIFF PROTOCOL
// ═══════════════════════════════════════════════════════════

export interface MapDiffHeader {
  region_code: string;
  base_version: string;
  target_version: string;
  diff_type: 'incremental' | 'full';
  created_at: string;
  size_bytes: number;
  checksum_sha256: string;
  operations_count: number;
}

export type MapDiffOperation =
  | { op: 'add_node'; node_id: string; lat: number; lon: number; properties: Record<string, unknown> }
  | { op: 'remove_node'; node_id: string }
  | { op: 'modify_node'; node_id: string; changes: Record<string, unknown> }
  | { op: 'add_edge'; edge_id: string; from_node: string; to_node: string; properties: Record<string, unknown> }
  | { op: 'remove_edge'; edge_id: string }
  | { op: 'modify_edge'; edge_id: string; changes: Record<string, unknown> }
  | { op: 'add_restriction'; restriction_id: string; edges: string[]; type: string }
  | { op: 'remove_restriction'; restriction_id: string };

export interface MapDiffPackage {
  header: MapDiffHeader;
  operations: MapDiffOperation[];
  validation: {
    pre_node_count: number;
    post_node_count: number;
    pre_edge_count: number;
    post_edge_count: number;
    affected_tiles: string[];
  };
}

// ═══════════════════════════════════════════════════════════
// 3. OFFLINE PACKAGE MANIFEST
// ═══════════════════════════════════════════════════════════

export interface OfflinePackageManifest {
  package_id: string;
  region_code: string;
  version: string;
  created_at: string;
  expires_at: string;
  total_size_bytes: number;
  checksum_sha256: string;
  assets: OfflineAsset[];
  dependencies: string[];
  min_app_version: string;
}

export interface OfflineAsset {
  asset_id: string;
  type: 'map_tiles' | 'route_graph' | 'poi_data' | 'ml_model' | 'voice_pack' | 'style_data';
  path: string;
  size_bytes: number;
  checksum_sha256: string;
  compression: 'none' | 'gzip' | 'brotli' | 'zstd';
  priority: 'required' | 'recommended' | 'optional';
  zoom_range?: { min: number; max: number };
}

// ═══════════════════════════════════════════════════════════
// 4. COMPACTION STRATEGY
// ═══════════════════════════════════════════════════════════

export interface CompactionPolicy {
  entity: string;
  strategy: 'time_window' | 'count_based' | 'size_based' | 'hybrid';
  trigger: CompactionTrigger;
  retention_after_compaction: string;
  aggregation: string;
}

export interface CompactionTrigger {
  type: 'schedule' | 'threshold' | 'manual';
  schedule_cron?: string;
  size_threshold_mb?: number;
  count_threshold?: number;
}

export const COMPACTION_POLICIES: CompactionPolicy[] = [
  {
    entity: 'position_history',
    strategy: 'time_window',
    trigger: { type: 'schedule', schedule_cron: '0 0 3 * * *' }, // 3 AM daily
    retention_after_compaction: '30d',
    aggregation: 'downsample to 1-minute intervals after 24h, 5-minute after 7d',
  },
  {
    entity: 'telemetry_events',
    strategy: 'hybrid',
    trigger: { type: 'threshold', size_threshold_mb: 100, count_threshold: 1000000 },
    retention_after_compaction: '90d',
    aggregation: 'aggregate to hourly summaries after 7d, daily after 30d',
  },
  {
    entity: 'incident_reports',
    strategy: 'count_based',
    trigger: { type: 'schedule', schedule_cron: '0 0 4 * * 0' }, // 4 AM Sunday
    retention_after_compaction: '365d',
    aggregation: 'keep all resolved, archive expired after 30d',
  },
  {
    entity: 'crowd_contributions',
    strategy: 'time_window',
    trigger: { type: 'schedule', schedule_cron: '0 0 2 * * *' }, // 2 AM daily
    retention_after_compaction: '14d',
    aggregation: 'merge into consensus values after 24h',
  },
  {
    entity: 'analytics_metrics',
    strategy: 'time_window',
    trigger: { type: 'schedule', schedule_cron: '0 0 1 * * *' }, // 1 AM daily
    retention_after_compaction: '365d',
    aggregation: 'rollup to 1h after 7d, 1d after 30d, 1w after 90d',
  },
  {
    entity: 'event_bus_log',
    strategy: 'size_based',
    trigger: { type: 'threshold', size_threshold_mb: 50 },
    retention_after_compaction: '7d',
    aggregation: 'keep only persistent events, drop transient after TTL',
  },
];

// ═══════════════════════════════════════════════════════════
// 5. RETENTION POLICIES
// ═══════════════════════════════════════════════════════════

export interface RetentionPolicy {
  entity: string;
  hot_retention: string;
  warm_retention: string;
  cold_retention: string;
  archive_retention: string;
  deletion_strategy: 'hard_delete' | 'soft_delete' | 'anonymize';
  legal_hold_exempt: boolean;
  gdpr_relevant: boolean;
}

export const RETENTION_POLICIES: RetentionPolicy[] = [
  { entity: 'user_positions', hot_retention: '24h', warm_retention: '7d', cold_retention: '30d', archive_retention: '90d', deletion_strategy: 'anonymize', legal_hold_exempt: false, gdpr_relevant: true },
  { entity: 'route_history', hot_retention: '7d', warm_retention: '30d', cold_retention: '90d', archive_retention: '365d', deletion_strategy: 'soft_delete', legal_hold_exempt: false, gdpr_relevant: true },
  { entity: 'incident_reports', hot_retention: '30d', warm_retention: '90d', cold_retention: '365d', archive_retention: '5y', deletion_strategy: 'soft_delete', legal_hold_exempt: true, gdpr_relevant: false },
  { entity: 'telemetry', hot_retention: '7d', warm_retention: '30d', cold_retention: '90d', archive_retention: '365d', deletion_strategy: 'hard_delete', legal_hold_exempt: false, gdpr_relevant: false },
  { entity: 'user_accounts', hot_retention: 'active', warm_retention: 'active', cold_retention: '30d_after_deletion', archive_retention: '90d_after_deletion', deletion_strategy: 'anonymize', legal_hold_exempt: false, gdpr_relevant: true },
  { entity: 'payment_records', hot_retention: '90d', warm_retention: '365d', cold_retention: '7y', archive_retention: '10y', deletion_strategy: 'soft_delete', legal_hold_exempt: true, gdpr_relevant: true },
  { entity: 'audit_logs', hot_retention: '30d', warm_retention: '90d', cold_retention: '365d', archive_retention: '7y', deletion_strategy: 'soft_delete', legal_hold_exempt: true, gdpr_relevant: false },
  { entity: 'ml_training_data', hot_retention: '30d', warm_retention: '90d', cold_retention: '365d', archive_retention: '3y', deletion_strategy: 'anonymize', legal_hold_exempt: false, gdpr_relevant: true },
];

// ═══════════════════════════════════════════════════════════
// 6. PROVENANCE MODEL
// ═══════════════════════════════════════════════════════════

export interface ProvenanceRecord {
  entity_id: string;
  entity_type: string;
  lineage: ProvenanceNode[];
  created_at: string;
  last_modified_at: string;
  version: number;
}

export interface ProvenanceNode {
  step: number;
  operation: 'create' | 'transform' | 'merge' | 'validate' | 'anonymize' | 'aggregate';
  source: string;
  timestamp: string;
  actor: string;
  input_refs: string[];
  output_ref: string;
  parameters: Record<string, unknown>;
  checksum: string;
}

/**
 * Track data provenance for audit and compliance
 */
export class ProvenanceTracker {
  private records = new Map<string, ProvenanceRecord>();

  create(entityId: string, entityType: string, source: string, actor: string): ProvenanceRecord {
    const now = new Date().toISOString();
    const record: ProvenanceRecord = {
      entity_id: entityId,
      entity_type: entityType,
      lineage: [{
        step: 0,
        operation: 'create',
        source,
        timestamp: now,
        actor,
        input_refs: [],
        output_ref: entityId,
        parameters: {},
        checksum: this.computeChecksum(entityId, now),
      }],
      created_at: now,
      last_modified_at: now,
      version: 1,
    };
    this.records.set(entityId, record);
    return record;
  }

  addStep(
    entityId: string,
    operation: ProvenanceNode['operation'],
    source: string,
    actor: string,
    inputRefs: string[],
    parameters: Record<string, unknown> = {}
  ): ProvenanceRecord | undefined {
    const record = this.records.get(entityId);
    if (!record) return undefined;

    const now = new Date().toISOString();
    record.lineage.push({
      step: record.lineage.length,
      operation,
      source,
      timestamp: now,
      actor,
      input_refs: inputRefs,
      output_ref: entityId,
      parameters,
      checksum: this.computeChecksum(entityId, now),
    });
    record.last_modified_at = now;
    record.version++;
    return record;
  }

  getRecord(entityId: string): ProvenanceRecord | undefined {
    return this.records.get(entityId);
  }

  getLineage(entityId: string): ProvenanceNode[] {
    return this.records.get(entityId)?.lineage ?? [];
  }

  private computeChecksum(entityId: string, timestamp: string): string {
    // Simple hash for provenance integrity
    let hash = 0;
    const str = `${entityId}:${timestamp}`;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash |= 0;
    }
    return Math.abs(hash).toString(16).padStart(8, '0');
  }
}

// ═══════════════════════════════════════════════════════════
// 7. CONSISTENCY MODEL
// ═══════════════════════════════════════════════════════════

export type ConsistencyLevel = 'strong' | 'eventual' | 'causal' | 'read_your_writes' | 'monotonic_reads';

export interface ConsistencyRequirement {
  entity: string;
  write_consistency: ConsistencyLevel;
  read_consistency: ConsistencyLevel;
  conflict_resolution: 'last_write_wins' | 'vector_clock' | 'crdt' | 'manual';
  staleness_tolerance_ms: number;
}

export const CONSISTENCY_REQUIREMENTS: ConsistencyRequirement[] = [
  { entity: 'user_position', write_consistency: 'eventual', read_consistency: 'read_your_writes', conflict_resolution: 'last_write_wins', staleness_tolerance_ms: 1000 },
  { entity: 'route_state', write_consistency: 'strong', read_consistency: 'strong', conflict_resolution: 'last_write_wins', staleness_tolerance_ms: 0 },
  { entity: 'incident', write_consistency: 'eventual', read_consistency: 'causal', conflict_resolution: 'crdt', staleness_tolerance_ms: 5000 },
  { entity: 'user_profile', write_consistency: 'strong', read_consistency: 'read_your_writes', conflict_resolution: 'last_write_wins', staleness_tolerance_ms: 0 },
  { entity: 'payment', write_consistency: 'strong', read_consistency: 'strong', conflict_resolution: 'manual', staleness_tolerance_ms: 0 },
  { entity: 'trust_score', write_consistency: 'eventual', read_consistency: 'monotonic_reads', conflict_resolution: 'vector_clock', staleness_tolerance_ms: 10000 },
  { entity: 'crowd_data', write_consistency: 'eventual', read_consistency: 'eventual', conflict_resolution: 'crdt', staleness_tolerance_ms: 30000 },
  { entity: 'map_data', write_consistency: 'strong', read_consistency: 'eventual', conflict_resolution: 'vector_clock', staleness_tolerance_ms: 86400000 },
  { entity: 'analytics', write_consistency: 'eventual', read_consistency: 'eventual', conflict_resolution: 'last_write_wins', staleness_tolerance_ms: 300000 },
];

/**
 * Vector Clock for causal consistency
 */
export class VectorClock {
  private clock: Map<string, number>;

  constructor(initial?: Record<string, number>) {
    this.clock = new Map(initial ? Object.entries(initial) : []);
  }

  increment(nodeId: string): void {
    this.clock.set(nodeId, (this.clock.get(nodeId) ?? 0) + 1);
  }

  merge(other: VectorClock): VectorClock {
    const merged = new VectorClock();
    for (const [k, v] of Array.from(this.clock.entries())) {
      merged.clock.set(k, Math.max(v, other.clock.get(k) ?? 0));
    }
    for (const [k, v] of Array.from(other.clock.entries())) {
      if (!merged.clock.has(k)) {
        merged.clock.set(k, v);
      }
    }
    return merged;
  }

  happensBefore(other: VectorClock): boolean {
    let atLeastOneLess = false;
    for (const [k, v] of Array.from(this.clock.entries())) {
      const otherV = other.clock.get(k) ?? 0;
      if (v > otherV) return false;
      if (v < otherV) atLeastOneLess = true;
    }
    for (const [k] of Array.from(other.clock.entries())) {
      if (!this.clock.has(k)) atLeastOneLess = true;
    }
    return atLeastOneLess;
  }

  isConcurrent(other: VectorClock): boolean {
    return !this.happensBefore(other) && !other.happensBefore(this);
  }

  toJSON(): Record<string, number> {
    const result: Record<string, number> = {};
    for (const [k, v] of Array.from(this.clock.entries())) {
      result[k] = v;
    }
    return result;
  }
}
