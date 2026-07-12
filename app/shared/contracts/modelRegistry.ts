/**
 * G.A.N.E — ML Ops Registry & Feature Store
 * =============================================
 * Model versioning, canary deployment, shadow mode,
 * rollback, feature store, and drift detection.
 */

// ─── Model Registry Types ───────────────────────────────

export type ModelStage = 'development' | 'staging' | 'canary' | 'production' | 'archived' | 'rollback';
export type ModelFramework = 'tensorflow' | 'pytorch' | 'onnx' | 'xgboost' | 'lightgbm' | 'custom';

export interface ModelVersion {
  id: string;
  modelName: string;
  version: string;
  stage: ModelStage;
  framework: ModelFramework;
  artifactPath: string;
  inputSchema: Record<string, string>;
  outputSchema: Record<string, string>;
  metrics: ModelMetrics;
  trainingConfig: TrainingConfig;
  deploymentConfig: DeploymentConfig;
  createdAt: number;
  promotedAt?: number;
  retiredAt?: number;
  createdBy: string;
  changelog: string;
}

export interface ModelMetrics {
  accuracy: number;
  precision: number;
  recall: number;
  f1Score: number;
  latencyP50Ms: number;
  latencyP99Ms: number;
  throughputRPS: number;
  modelSizeMB: number;
  customMetrics: Record<string, number>;
}

export interface TrainingConfig {
  datasetVersion: string;
  datasetSize: number;
  epochs: number;
  batchSize: number;
  learningRate: number;
  optimizer: string;
  regularization: string;
  validationSplit: number;
  randomSeed: number;
  trainingDurationMinutes: number;
}

export interface DeploymentConfig {
  minInstances: number;
  maxInstances: number;
  cpuRequest: string;
  memoryRequest: string;
  gpuRequired: boolean;
  maxBatchSize: number;
  timeoutMs: number;
  warmupRequests: number;
}

// ─── Model Registry ─────────────────────────────────────

export const REGISTERED_MODELS: ModelVersion[] = [
  {
    id: 'model-eta-v1',
    modelName: 'eta_predictor',
    version: '1.0.0',
    stage: 'production',
    framework: 'xgboost',
    artifactPath: 's3://gane-models/eta_predictor/v1.0.0/',
    inputSchema: {
      segment_length_km: 'float',
      current_speed_kmh: 'float',
      time_of_day_minutes: 'int',
      day_of_week: 'int',
      weather_code: 'int',
      congestion_level: 'float',
      historical_avg_speed: 'float',
    },
    outputSchema: {
      eta_seconds: 'float',
      confidence: 'float',
      congestion_prediction: 'float',
    },
    metrics: {
      accuracy: 0.87,
      precision: 0.85,
      recall: 0.89,
      f1Score: 0.87,
      latencyP50Ms: 12,
      latencyP99Ms: 45,
      throughputRPS: 5000,
      modelSizeMB: 15,
      customMetrics: { mape: 0.08, within_10_percent: 0.85 },
    },
    trainingConfig: {
      datasetVersion: 'trips-2025-q4',
      datasetSize: 2_500_000,
      epochs: 100,
      batchSize: 1024,
      learningRate: 0.01,
      optimizer: 'adam',
      regularization: 'l2(0.001)',
      validationSplit: 0.2,
      randomSeed: 42,
      trainingDurationMinutes: 45,
    },
    deploymentConfig: {
      minInstances: 2,
      maxInstances: 10,
      cpuRequest: '500m',
      memoryRequest: '256Mi',
      gpuRequired: false,
      maxBatchSize: 64,
      timeoutMs: 200,
      warmupRequests: 100,
    },
    createdAt: Date.now() - 90 * 86400000,
    promotedAt: Date.now() - 85 * 86400000,
    createdBy: 'ml-team',
    changelog: 'Initial production model trained on Q4 2025 trip data',
  },
  {
    id: 'model-congestion-v1',
    modelName: 'congestion_classifier',
    version: '1.0.0',
    stage: 'production',
    framework: 'lightgbm',
    artifactPath: 's3://gane-models/congestion_classifier/v1.0.0/',
    inputSchema: {
      segment_id: 'string',
      current_speed_ratio: 'float',
      vehicle_count: 'int',
      time_of_day: 'int',
      is_holiday: 'bool',
      weather_severity: 'int',
    },
    outputSchema: {
      congestion_level: 'int',
      probability: 'float',
    },
    metrics: {
      accuracy: 0.91,
      precision: 0.89,
      recall: 0.93,
      f1Score: 0.91,
      latencyP50Ms: 5,
      latencyP99Ms: 18,
      throughputRPS: 15000,
      modelSizeMB: 8,
      customMetrics: { auc_roc: 0.95 },
    },
    trainingConfig: {
      datasetVersion: 'traffic-2025-q4',
      datasetSize: 10_000_000,
      epochs: 200,
      batchSize: 2048,
      learningRate: 0.05,
      optimizer: 'gbdt',
      regularization: 'l1(0.01)',
      validationSplit: 0.15,
      randomSeed: 42,
      trainingDurationMinutes: 30,
    },
    deploymentConfig: {
      minInstances: 3,
      maxInstances: 15,
      cpuRequest: '250m',
      memoryRequest: '128Mi',
      gpuRequired: false,
      maxBatchSize: 128,
      timeoutMs: 100,
      warmupRequests: 200,
    },
    createdAt: Date.now() - 60 * 86400000,
    promotedAt: Date.now() - 55 * 86400000,
    createdBy: 'ml-team',
    changelog: 'LightGBM congestion classifier with 4-level output',
  },
  {
    id: 'model-anomaly-v1',
    modelName: 'anomaly_detector',
    version: '1.0.0',
    stage: 'production',
    framework: 'custom',
    artifactPath: 's3://gane-models/anomaly_detector/v1.0.0/',
    inputSchema: {
      speed_history: 'float[]',
      acceleration_history: 'float[]',
      heading_changes: 'float[]',
      position_variance: 'float',
    },
    outputSchema: {
      is_anomaly: 'bool',
      anomaly_score: 'float',
      anomaly_type: 'string',
    },
    metrics: {
      accuracy: 0.94,
      precision: 0.92,
      recall: 0.88,
      f1Score: 0.90,
      latencyP50Ms: 3,
      latencyP99Ms: 10,
      throughputRPS: 25000,
      modelSizeMB: 2,
      customMetrics: { false_positive_rate: 0.03 },
    },
    trainingConfig: {
      datasetVersion: 'telemetry-2025-q4',
      datasetSize: 50_000_000,
      epochs: 50,
      batchSize: 4096,
      learningRate: 0.001,
      optimizer: 'isolation_forest',
      regularization: 'none',
      validationSplit: 0.2,
      randomSeed: 42,
      trainingDurationMinutes: 120,
    },
    deploymentConfig: {
      minInstances: 5,
      maxInstances: 20,
      cpuRequest: '100m',
      memoryRequest: '64Mi',
      gpuRequired: false,
      maxBatchSize: 256,
      timeoutMs: 50,
      warmupRequests: 500,
    },
    createdAt: Date.now() - 45 * 86400000,
    promotedAt: Date.now() - 40 * 86400000,
    createdBy: 'ml-team',
    changelog: 'Isolation forest anomaly detector for telemetry streams',
  },
];

// ─── Canary Deployment ──────────────────────────────────

export interface CanaryDeployment {
  modelName: string;
  productionVersion: string;
  canaryVersion: string;
  trafficPercent: number;
  startedAt: number;
  metrics: {
    production: Partial<ModelMetrics>;
    canary: Partial<ModelMetrics>;
  };
  autoPromoteThreshold: Record<string, number>;
  autoRollbackThreshold: Record<string, number>;
  status: 'running' | 'promoting' | 'rolling_back' | 'completed';
}

export interface ShadowDeployment {
  modelName: string;
  productionVersion: string;
  shadowVersion: string;
  startedAt: number;
  requestsProcessed: number;
  divergenceRate: number;
  metrics: {
    production: Partial<ModelMetrics>;
    shadow: Partial<ModelMetrics>;
  };
  status: 'running' | 'completed' | 'aborted';
}

// ─── Feature Store ──────────────────────────────────────

export type FeatureType = 'numeric' | 'categorical' | 'boolean' | 'embedding' | 'timestamp';
export type FeatureSource = 'realtime' | 'batch' | 'on_demand';

export interface Feature {
  name: string;
  type: FeatureType;
  source: FeatureSource;
  description: string;
  entity: string;
  freshness: string;
  freshnessMs: number;
  computeLogic: string;
  dependencies: string[];
  owner: string;
}

export const FEATURE_STORE: Feature[] = [
  {
    name: 'segment_avg_speed_15min',
    type: 'numeric',
    source: 'realtime',
    description: 'Average speed on road segment over last 15 minutes',
    entity: 'road_segment',
    freshness: '1 minute',
    freshnessMs: 60000,
    computeLogic: 'AVG(speed) WHERE segment_id = ? AND timestamp > NOW() - 15min',
    dependencies: ['ds-telemetry'],
    owner: 'traffic-team',
  },
  {
    name: 'user_trip_count_30d',
    type: 'numeric',
    source: 'batch',
    description: 'Number of trips by user in last 30 days',
    entity: 'user',
    freshness: '1 hour',
    freshnessMs: 3600000,
    computeLogic: 'COUNT(*) FROM trips WHERE user_id = ? AND started_at > NOW() - 30d',
    dependencies: ['ds-user-trips'],
    owner: 'analytics-team',
  },
  {
    name: 'weather_severity_current',
    type: 'categorical',
    source: 'realtime',
    description: 'Current weather severity level (0=clear, 1=light, 2=moderate, 3=severe)',
    entity: 'region',
    freshness: '5 minutes',
    freshnessMs: 300000,
    computeLogic: 'CASE WHEN precipitation > 10 THEN 3 WHEN precipitation > 5 THEN 2 WHEN precipitation > 1 THEN 1 ELSE 0 END',
    dependencies: ['ds-weather'],
    owner: 'weather-team',
  },
  {
    name: 'segment_incident_active',
    type: 'boolean',
    source: 'realtime',
    description: 'Whether an active incident exists on this segment',
    entity: 'road_segment',
    freshness: '30 seconds',
    freshnessMs: 30000,
    computeLogic: 'EXISTS(SELECT 1 FROM incidents WHERE segment_id = ? AND status = active)',
    dependencies: ['ds-crowd-reports'],
    owner: 'incident-team',
  },
  {
    name: 'device_battery_level',
    type: 'numeric',
    source: 'realtime',
    description: 'Current device battery percentage',
    entity: 'device',
    freshness: '30 seconds',
    freshnessMs: 30000,
    computeLogic: 'navigator.getBattery().level * 100',
    dependencies: [],
    owner: 'client-team',
  },
  {
    name: 'segment_historical_speed_dow_hour',
    type: 'numeric',
    source: 'batch',
    description: 'Historical average speed by day-of-week and hour',
    entity: 'road_segment',
    freshness: '24 hours',
    freshnessMs: 86400000,
    computeLogic: 'AVG(speed) GROUP BY segment_id, day_of_week, hour_of_day',
    dependencies: ['ds-telemetry'],
    owner: 'analytics-team',
  },
];

// ─── Drift Detection ────────────────────────────────────

export interface DriftMetric {
  featureName: string;
  baselineDistribution: { mean: number; std: number; min: number; max: number };
  currentDistribution: { mean: number; std: number; min: number; max: number };
  psiScore: number; // Population Stability Index
  ksStatistic: number; // Kolmogorov-Smirnov
  isDrifted: boolean;
  driftThreshold: number;
  lastChecked: number;
}

export interface ModelDriftReport {
  modelName: string;
  version: string;
  reportDate: number;
  featureDrift: DriftMetric[];
  predictionDrift: {
    baselineAccuracy: number;
    currentAccuracy: number;
    degradationPercent: number;
    isSignificant: boolean;
  };
  recommendation: 'no_action' | 'monitor' | 'retrain' | 'rollback';
  nextRetrainDate?: number;
}

// ─── Helpers ────────────────────────────────────────────

export function getModelByName(name: string): ModelVersion[] {
  return REGISTERED_MODELS.filter(m => m.modelName === name);
}

export function getProductionModel(name: string): ModelVersion | undefined {
  return REGISTERED_MODELS.find(m => m.modelName === name && m.stage === 'production');
}

export function getFeaturesByEntity(entity: string): Feature[] {
  return FEATURE_STORE.filter(f => f.entity === entity);
}

export function getStaleFeatures(maxFreshnessMs: number): Feature[] {
  return FEATURE_STORE.filter(f => f.freshnessMs > maxFreshnessMs);
}

export function calculatePSI(baseline: number[], current: number[], bins = 10): number {
  const range = Math.max(...baseline, ...current) - Math.min(...baseline, ...current);
  const binWidth = range / bins;
  const minVal = Math.min(...baseline, ...current);

  let psi = 0;
  for (let i = 0; i < bins; i++) {
    const lo = minVal + i * binWidth;
    const hi = lo + binWidth;
    const baseCount = baseline.filter(v => v >= lo && v < hi).length / baseline.length || 0.0001;
    const currCount = current.filter(v => v >= lo && v < hi).length / current.length || 0.0001;
    psi += (currCount - baseCount) * Math.log(currCount / baseCount);
  }
  return psi;
}
