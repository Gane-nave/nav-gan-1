/**
 * G.A.N.E — Data Lineage Model
 * ===============================
 * Provenance tracking per data point, data flow graph,
 * transformation audit, and freshness contracts.
 */

// ─── Types ──────────────────────────────────────────────

export type DataClassification = 'public' | 'internal' | 'confidential' | 'restricted';
export type DataFormat = 'json' | 'protobuf' | 'binary' | 'csv' | 'geojson' | 'nmea';
export type TransformationType = 'filter' | 'aggregate' | 'enrich' | 'anonymize' | 'validate' | 'normalize' | 'fuse' | 'predict';

export interface DataSource {
  id: string;
  name: string;
  type: 'sensor' | 'api' | 'database' | 'user_input' | 'computed' | 'external_feed';
  format: DataFormat;
  classification: DataClassification;
  refreshRate: string;
  owner: string;
  schema: string;
  qualityMetrics: {
    accuracy: string;
    completeness: string;
    timeliness: string;
  };
}

export interface DataTransformation {
  id: string;
  name: string;
  type: TransformationType;
  inputSources: string[];
  outputSources: string[];
  logic: string;
  latencyBudget: string;
  idempotent: boolean;
  reversible: boolean;
}

export interface DataFlowEdge {
  from: string;
  to: string;
  transformation: string;
  protocol: string;
  encrypted: boolean;
  batchOrStream: 'batch' | 'stream' | 'micro_batch';
  retentionPolicy: string;
}

// ─── Data Sources ───────────────────────────────────────

export const DATA_SOURCES: DataSource[] = [
  {
    id: 'ds-gnss-raw',
    name: 'Raw GNSS Observations',
    type: 'sensor',
    format: 'nmea',
    classification: 'confidential',
    refreshRate: '1 Hz',
    owner: 'GNSS Engine',
    schema: 'NMEA 0183 / RINEX',
    qualityMetrics: {
      accuracy: 'CEP 2-5m (open sky), 10-50m (urban)',
      completeness: '> 99% in open sky, 60-80% in urban',
      timeliness: '< 1 second',
    },
  },
  {
    id: 'ds-imu-raw',
    name: 'Raw IMU Measurements',
    type: 'sensor',
    format: 'binary',
    classification: 'internal',
    refreshRate: '100 Hz',
    owner: 'PDR Engine',
    schema: '{ accel: Vec3, gyro: Vec3, mag: Vec3, timestamp: number }',
    qualityMetrics: {
      accuracy: 'Accelerometer: ±0.01 m/s², Gyroscope: ±0.01 rad/s',
      completeness: '> 99.9% (hardware dependent)',
      timeliness: '< 10 milliseconds',
    },
  },
  {
    id: 'ds-fused-position',
    name: 'Fused Position (ESKF Output)',
    type: 'computed',
    format: 'json',
    classification: 'confidential',
    refreshRate: '10 Hz',
    owner: 'ESKF Engine',
    schema: '{ lat, lng, alt, heading, speed, accuracy, timestamp }',
    qualityMetrics: {
      accuracy: 'CEP 1-3m (fused), 5-15m (degraded)',
      completeness: '> 99.9% (PDR fills gaps)',
      timeliness: '< 100 milliseconds',
    },
  },
  {
    id: 'ds-traffic-live',
    name: 'Live Traffic Data',
    type: 'external_feed',
    format: 'json',
    classification: 'internal',
    refreshRate: '30 seconds',
    owner: 'Traffic Pipeline',
    schema: '{ segmentId, speed, congestionLevel, timestamp }',
    qualityMetrics: {
      accuracy: 'Speed: ±5 km/h, Congestion: 4-level classification',
      completeness: '> 95% of major road segments',
      timeliness: '< 60 seconds',
    },
  },
  {
    id: 'ds-crowd-reports',
    name: 'Crowd-Sourced Reports',
    type: 'user_input',
    format: 'json',
    classification: 'internal',
    refreshRate: 'Event-driven',
    owner: 'Crowd Intelligence',
    schema: '{ type, location, description, userId, timestamp, confirmations }',
    qualityMetrics: {
      accuracy: 'Validated by N≥3 confirmation rule',
      completeness: 'Dependent on user density',
      timeliness: '< 5 seconds from report to ingestion',
    },
  },
  {
    id: 'ds-weather',
    name: 'Weather Data',
    type: 'external_feed',
    format: 'json',
    classification: 'public',
    refreshRate: '5 minutes',
    owner: 'Weather Service',
    schema: '{ temperature, precipitation, visibility, windSpeed, alerts }',
    qualityMetrics: {
      accuracy: 'Temperature: ±1°C, Precipitation: ±10%',
      completeness: '> 99% for covered regions',
      timeliness: '< 5 minutes',
    },
  },
  {
    id: 'ds-v2x-bsm',
    name: 'V2X Basic Safety Messages',
    type: 'external_feed',
    format: 'protobuf',
    classification: 'restricted',
    refreshRate: '10 Hz',
    owner: 'V2X Engine',
    schema: 'SAE J2735 BSM Part II',
    qualityMetrics: {
      accuracy: 'Position: ±1.5m, Speed: ±0.5 m/s',
      completeness: 'Dependent on V2X penetration rate',
      timeliness: '< 100 milliseconds',
    },
  },
  {
    id: 'ds-user-trips',
    name: 'User Trip Records',
    type: 'database',
    format: 'json',
    classification: 'confidential',
    refreshRate: 'Event-driven (trip start/end)',
    owner: 'Trip Manager',
    schema: '{ tripId, userId, origin, destination, waypoints, startTime, endTime, distance }',
    qualityMetrics: {
      accuracy: 'Distance: ±2%, Duration: ±1 second',
      completeness: '100% for completed trips',
      timeliness: 'Real-time during trip, persisted on completion',
    },
  },
  {
    id: 'ds-ml-predictions',
    name: 'ML Model Predictions',
    type: 'computed',
    format: 'json',
    classification: 'internal',
    refreshRate: 'On-demand (per route request)',
    owner: 'ML Prediction Engine',
    schema: '{ prediction, confidence, modelVersion, features, timestamp }',
    qualityMetrics: {
      accuracy: 'ETA: ±10% (p85), Speed: ±5 km/h (p90)',
      completeness: '100% for requested segments',
      timeliness: '< 200 milliseconds inference',
    },
  },
  {
    id: 'ds-telemetry',
    name: 'Device Telemetry',
    type: 'sensor',
    format: 'json',
    classification: 'confidential',
    refreshRate: '1-10 Hz (adaptive)',
    owner: 'Telemetry Pipeline',
    schema: '{ deviceId, position, speed, heading, battery, networkType, timestamp }',
    qualityMetrics: {
      accuracy: 'Position: ±5m, Speed: ±2 km/h',
      completeness: '> 95% uptime per device',
      timeliness: '< 5 seconds to server',
    },
  },
];

// ─── Transformations ────────────────────────────────────

export const TRANSFORMATIONS: DataTransformation[] = [
  {
    id: 'tx-eskf-fusion',
    name: 'ESKF Sensor Fusion',
    type: 'fuse',
    inputSources: ['ds-gnss-raw', 'ds-imu-raw'],
    outputSources: ['ds-fused-position'],
    logic: 'Extended Square-root Kalman Filter with 15-state vector (position, velocity, attitude, biases)',
    latencyBudget: '< 10ms per update',
    idempotent: false,
    reversible: false,
  },
  {
    id: 'tx-traffic-aggregate',
    name: 'Traffic Data Aggregation',
    type: 'aggregate',
    inputSources: ['ds-telemetry', 'ds-crowd-reports'],
    outputSources: ['ds-traffic-live'],
    logic: 'Weighted average of crowd speed samples per road segment with outlier rejection',
    latencyBudget: '< 5 seconds',
    idempotent: true,
    reversible: false,
  },
  {
    id: 'tx-anonymize-telemetry',
    name: 'Telemetry Anonymization',
    type: 'anonymize',
    inputSources: ['ds-telemetry'],
    outputSources: ['ds-telemetry'],
    logic: 'Remove deviceId, truncate coordinates to 3 decimal places, add Laplace noise (ε=1.0)',
    latencyBudget: '< 1ms',
    idempotent: true,
    reversible: false,
  },
  {
    id: 'tx-incident-validate',
    name: 'Incident Report Validation',
    type: 'validate',
    inputSources: ['ds-crowd-reports'],
    outputSources: ['ds-crowd-reports'],
    logic: 'N≥3 confirmation rule, geo-clustering (500m radius), decay timer (30 min TTL)',
    latencyBudget: '< 100ms',
    idempotent: true,
    reversible: false,
  },
  {
    id: 'tx-ml-predict',
    name: 'ML Speed/ETA Prediction',
    type: 'predict',
    inputSources: ['ds-traffic-live', 'ds-weather', 'ds-fused-position'],
    outputSources: ['ds-ml-predictions'],
    logic: 'Gradient-boosted tree model with time-of-day, day-of-week, weather, and historical features',
    latencyBudget: '< 200ms',
    idempotent: true,
    reversible: false,
  },
  {
    id: 'tx-v2x-plausibility',
    name: 'V2X Plausibility Check',
    type: 'validate',
    inputSources: ['ds-v2x-bsm'],
    outputSources: ['ds-v2x-bsm'],
    logic: 'Physics-based validation: speed < 300 km/h, acceleration < 15 m/s², position within road network',
    latencyBudget: '< 10ms',
    idempotent: true,
    reversible: false,
  },
];

// ─── Data Flow Graph ────────────────────────────────────

export const DATA_FLOW_EDGES: DataFlowEdge[] = [
  { from: 'ds-gnss-raw', to: 'ds-fused-position', transformation: 'tx-eskf-fusion', protocol: 'In-process', encrypted: false, batchOrStream: 'stream', retentionPolicy: 'None (real-time only)' },
  { from: 'ds-imu-raw', to: 'ds-fused-position', transformation: 'tx-eskf-fusion', protocol: 'In-process', encrypted: false, batchOrStream: 'stream', retentionPolicy: 'None (real-time only)' },
  { from: 'ds-telemetry', to: 'ds-traffic-live', transformation: 'tx-traffic-aggregate', protocol: 'tRPC/HTTPS', encrypted: true, batchOrStream: 'micro_batch', retentionPolicy: '7 days hot, 30 days warm' },
  { from: 'ds-crowd-reports', to: 'ds-traffic-live', transformation: 'tx-incident-validate', protocol: 'tRPC/HTTPS', encrypted: true, batchOrStream: 'stream', retentionPolicy: '30 days' },
  { from: 'ds-traffic-live', to: 'ds-ml-predictions', transformation: 'tx-ml-predict', protocol: 'In-process', encrypted: false, batchOrStream: 'stream', retentionPolicy: '24 hours' },
  { from: 'ds-weather', to: 'ds-ml-predictions', transformation: 'tx-ml-predict', protocol: 'HTTPS', encrypted: true, batchOrStream: 'batch', retentionPolicy: '7 days' },
  { from: 'ds-v2x-bsm', to: 'ds-v2x-bsm', transformation: 'tx-v2x-plausibility', protocol: 'DSRC/C-V2X', encrypted: true, batchOrStream: 'stream', retentionPolicy: '1 hour' },
  { from: 'ds-fused-position', to: 'ds-user-trips', transformation: 'tx-anonymize-telemetry', protocol: 'In-process', encrypted: false, batchOrStream: 'stream', retentionPolicy: '90 days' },
];

// ─── Freshness Contracts ────────────────────────────────

export interface FreshnessContract {
  sourceId: string;
  maxAge: string;
  maxAgeMs: number;
  staleBehavior: string;
  monitoringMetric: string;
}

export const FRESHNESS_CONTRACTS: FreshnessContract[] = [
  { sourceId: 'ds-fused-position', maxAge: '1 second', maxAgeMs: 1000, staleBehavior: 'Show last known position with "GPS Searching" indicator', monitoringMetric: 'position_age_seconds' },
  { sourceId: 'ds-traffic-live', maxAge: '60 seconds', maxAgeMs: 60000, staleBehavior: 'Show data with "Updated X minutes ago" timestamp', monitoringMetric: 'traffic_data_age_seconds' },
  { sourceId: 'ds-weather', maxAge: '5 minutes', maxAgeMs: 300000, staleBehavior: 'Show cached weather with staleness indicator', monitoringMetric: 'weather_data_age_seconds' },
  { sourceId: 'ds-crowd-reports', maxAge: '30 minutes', maxAgeMs: 1800000, staleBehavior: 'Fade out old reports; remove after TTL', monitoringMetric: 'report_age_seconds' },
  { sourceId: 'ds-v2x-bsm', maxAge: '500 milliseconds', maxAgeMs: 500, staleBehavior: 'Discard stale BSMs; do not display', monitoringMetric: 'bsm_age_ms' },
  { sourceId: 'ds-ml-predictions', maxAge: '5 minutes', maxAgeMs: 300000, staleBehavior: 'Show prediction with wider confidence interval', monitoringMetric: 'prediction_age_seconds' },
];

// ─── Helpers ────────────────────────────────────────────

export function getSourceById(id: string): DataSource | undefined {
  return DATA_SOURCES.find(s => s.id === id);
}

export function getUpstreamSources(sourceId: string): string[] {
  const edges = DATA_FLOW_EDGES.filter(e => e.to === sourceId);
  return edges.map(e => e.from);
}

export function getDownstreamSources(sourceId: string): string[] {
  const edges = DATA_FLOW_EDGES.filter(e => e.from === sourceId);
  return edges.map(e => e.to);
}

export function getTransformationChain(sourceId: string): DataTransformation[] {
  const chain: DataTransformation[] = [];
  const visited = new Set<string>();

  function walk(id: string) {
    if (visited.has(id)) return;
    visited.add(id);
    const edges = DATA_FLOW_EDGES.filter(e => e.to === id);
    for (const edge of edges) {
      const tx = TRANSFORMATIONS.find(t => t.id === edge.transformation);
      if (tx) chain.push(tx);
      walk(edge.from);
    }
  }

  walk(sourceId);
  return chain.reverse();
}
