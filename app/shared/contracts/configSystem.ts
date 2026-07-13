/**
 * G.A.N.E — Configuration System Contract
 * ==========================================
 * 5-level hierarchical configuration with regional,
 * tenant, device, and experiment overrides.
 *
 * Hierarchy (highest priority wins):
 *   1. Experiment (A/B test overrides)
 *   2. Device (per-device tuning)
 *   3. Tenant (fleet/customer config)
 *   4. Regional (geo-specific settings)
 *   5. Global (system defaults)
 */

// ─── Types ──────────────────────────────────────────────

export type ConfigLevel = 'global' | 'regional' | 'tenant' | 'device' | 'experiment';
export type ConfigValueType = 'string' | 'number' | 'boolean' | 'json' | 'duration' | 'enum';

export interface ConfigKey {
  key: string;
  name: string;
  description: string;
  valueType: ConfigValueType;
  defaultValue: unknown;
  allowedLevels: ConfigLevel[];
  validation: string;
  sensitive: boolean;
  restartRequired: boolean;
  category: string;
}

export interface ConfigOverride {
  key: string;
  level: ConfigLevel;
  scope: string; // e.g., region ID, tenant ID, device ID, experiment ID
  value: unknown;
  setBy: string;
  setAt: number;
  expiresAt?: number;
  reason?: string;
}

export interface ConfigResolution {
  key: string;
  resolvedValue: unknown;
  resolvedLevel: ConfigLevel;
  resolvedScope: string;
  chain: Array<{ level: ConfigLevel; scope: string; value: unknown }>;
}

// ─── Config Key Registry ────────────────────────────────

export const CONFIG_KEYS: ConfigKey[] = [
  // ── Navigation ──
  {
    key: 'nav.gps.update_rate_hz',
    name: 'GPS Update Rate',
    description: 'How frequently to request GPS position updates',
    valueType: 'number',
    defaultValue: 1,
    allowedLevels: ['global', 'device', 'experiment'],
    validation: 'min: 0.1, max: 10',
    sensitive: false,
    restartRequired: false,
    category: 'navigation',
  },
  {
    key: 'nav.eskf.process_noise_q',
    name: 'ESKF Process Noise (Q)',
    description: 'Process noise covariance for Kalman filter tuning',
    valueType: 'number',
    defaultValue: 0.01,
    allowedLevels: ['global', 'regional', 'experiment'],
    validation: 'min: 0.001, max: 1.0',
    sensitive: false,
    restartRequired: false,
    category: 'navigation',
  },
  {
    key: 'nav.reroute.hysteresis_meters',
    name: 'Reroute Hysteresis Distance',
    description: 'Minimum deviation distance before triggering reroute',
    valueType: 'number',
    defaultValue: 50,
    allowedLevels: ['global', 'regional', 'tenant', 'experiment'],
    validation: 'min: 10, max: 500',
    sensitive: false,
    restartRequired: false,
    category: 'navigation',
  },
  {
    key: 'nav.map_matching.confidence_threshold',
    name: 'Map Matching Confidence Threshold',
    description: 'Minimum confidence to accept map-matched position',
    valueType: 'number',
    defaultValue: 0.7,
    allowedLevels: ['global', 'regional'],
    validation: 'min: 0.1, max: 1.0',
    sensitive: false,
    restartRequired: false,
    category: 'navigation',
  },

  // ── Traffic ──
  {
    key: 'traffic.crowd.min_confirmations',
    name: 'Crowd Report Min Confirmations',
    description: 'Number of independent reports needed to validate an incident',
    valueType: 'number',
    defaultValue: 3,
    allowedLevels: ['global', 'regional'],
    validation: 'min: 1, max: 10',
    sensitive: false,
    restartRequired: false,
    category: 'traffic',
  },
  {
    key: 'traffic.incident.ttl_minutes',
    name: 'Incident TTL',
    description: 'Time-to-live for unconfirmed incidents',
    valueType: 'number',
    defaultValue: 30,
    allowedLevels: ['global', 'regional'],
    validation: 'min: 5, max: 120',
    sensitive: false,
    restartRequired: false,
    category: 'traffic',
  },
  {
    key: 'traffic.speed_sample.outlier_threshold',
    name: 'Speed Sample Outlier Threshold',
    description: 'Z-score threshold for rejecting speed samples',
    valueType: 'number',
    defaultValue: 3.0,
    allowedLevels: ['global'],
    validation: 'min: 1.0, max: 5.0',
    sensitive: false,
    restartRequired: false,
    category: 'traffic',
  },

  // ── Performance ──
  {
    key: 'perf.memory_budget_mb',
    name: 'Memory Budget',
    description: 'Maximum JS heap usage before feature shedding',
    valueType: 'number',
    defaultValue: 256,
    allowedLevels: ['global', 'device'],
    validation: 'min: 64, max: 1024',
    sensitive: false,
    restartRequired: false,
    category: 'performance',
  },
  {
    key: 'perf.target_fps',
    name: 'Target FPS',
    description: 'Target frame rate for map rendering',
    valueType: 'number',
    defaultValue: 60,
    allowedLevels: ['global', 'device'],
    validation: 'min: 15, max: 120',
    sensitive: false,
    restartRequired: false,
    category: 'performance',
  },
  {
    key: 'perf.max_map_overlays',
    name: 'Max Map Overlays',
    description: 'Maximum number of simultaneous map overlay layers',
    valueType: 'number',
    defaultValue: 8,
    allowedLevels: ['global', 'device'],
    validation: 'min: 1, max: 20',
    sensitive: false,
    restartRequired: false,
    category: 'performance',
  },

  // ── Battery ──
  {
    key: 'battery.power_mode',
    name: 'Default Power Mode',
    description: 'Default battery optimization mode',
    valueType: 'enum',
    defaultValue: 'balanced',
    allowedLevels: ['global', 'device', 'experiment'],
    validation: 'enum: performance, balanced, power_saver, ultra_saver',
    sensitive: false,
    restartRequired: false,
    category: 'battery',
  },
  {
    key: 'battery.low_threshold_percent',
    name: 'Low Battery Threshold',
    description: 'Battery percentage to trigger power-saving mode',
    valueType: 'number',
    defaultValue: 20,
    allowedLevels: ['global'],
    validation: 'min: 5, max: 50',
    sensitive: false,
    restartRequired: false,
    category: 'battery',
  },

  // ── Privacy ──
  {
    key: 'privacy.anonymization_epsilon',
    name: 'Differential Privacy Epsilon',
    description: 'Privacy budget for differential privacy noise injection',
    valueType: 'number',
    defaultValue: 1.0,
    allowedLevels: ['global', 'regional'],
    validation: 'min: 0.1, max: 10.0',
    sensitive: false,
    restartRequired: false,
    category: 'privacy',
  },
  {
    key: 'privacy.data_retention_days',
    name: 'Data Retention Period',
    description: 'Days to retain user location data before purge',
    valueType: 'number',
    defaultValue: 90,
    allowedLevels: ['global', 'regional', 'tenant'],
    validation: 'min: 7, max: 365',
    sensitive: false,
    restartRequired: false,
    category: 'privacy',
  },

  // ── V2X ──
  {
    key: 'v2x.bsm_rate_hz',
    name: 'V2X BSM Broadcast Rate',
    description: 'Basic Safety Message broadcast frequency',
    valueType: 'number',
    defaultValue: 10,
    allowedLevels: ['global', 'regional'],
    validation: 'min: 1, max: 20',
    sensitive: false,
    restartRequired: false,
    category: 'v2x',
  },
  {
    key: 'v2x.pseudonym_rotation_seconds',
    name: 'V2X Pseudonym Rotation',
    description: 'How often to rotate V2X pseudonym certificates',
    valueType: 'number',
    defaultValue: 300,
    allowedLevels: ['global', 'regional'],
    validation: 'min: 60, max: 3600',
    sensitive: false,
    restartRequired: false,
    category: 'v2x',
  },

  // ── ML ──
  {
    key: 'ml.model_version',
    name: 'Active ML Model Version',
    description: 'Currently deployed ML model version for predictions',
    valueType: 'string',
    defaultValue: 'v1.0.0',
    allowedLevels: ['global', 'experiment'],
    validation: 'semver format',
    sensitive: false,
    restartRequired: false,
    category: 'ml',
  },
  {
    key: 'ml.canary_traffic_percent',
    name: 'ML Canary Traffic Percentage',
    description: 'Percentage of traffic routed to canary model',
    valueType: 'number',
    defaultValue: 0,
    allowedLevels: ['global', 'experiment'],
    validation: 'min: 0, max: 100',
    sensitive: false,
    restartRequired: false,
    category: 'ml',
  },

  // ── Feature Flags ──
  {
    key: 'feature.v2x_enabled',
    name: 'V2X Feature Flag',
    description: 'Enable V2X vehicle communication features',
    valueType: 'boolean',
    defaultValue: false,
    allowedLevels: ['global', 'regional', 'tenant', 'experiment'],
    validation: 'boolean',
    sensitive: false,
    restartRequired: false,
    category: 'features',
  },
  {
    key: 'feature.digital_twin_enabled',
    name: 'Digital Twin Feature Flag',
    description: 'Enable digital twin road network simulation',
    valueType: 'boolean',
    defaultValue: false,
    allowedLevels: ['global', 'tenant', 'experiment'],
    validation: 'boolean',
    sensitive: false,
    restartRequired: false,
    category: 'features',
  },
  {
    key: 'feature.ar_navigation_enabled',
    name: 'AR Navigation Feature Flag',
    description: 'Enable augmented reality navigation overlay',
    valueType: 'boolean',
    defaultValue: false,
    allowedLevels: ['global', 'device', 'experiment'],
    validation: 'boolean',
    sensitive: false,
    restartRequired: false,
    category: 'features',
  },
];

// ─── Config Resolver ────────────────────────────────────

export class ConfigResolver {
  private overrides: ConfigOverride[] = [];

  setOverride(override: ConfigOverride): void {
    // Remove existing override at same level+scope
    this.overrides = this.overrides.filter(
      o => !(o.key === override.key && o.level === override.level && o.scope === override.scope)
    );
    this.overrides.push(override);
  }

  removeOverride(key: string, level: ConfigLevel, scope: string): boolean {
    const before = this.overrides.length;
    this.overrides = this.overrides.filter(
      o => !(o.key === key && o.level === level && o.scope === scope)
    );
    return this.overrides.length < before;
  }

  resolve(
    key: string,
    context: { region?: string; tenantId?: string; deviceId?: string; experimentId?: string }
  ): ConfigResolution {
    const configKey = CONFIG_KEYS.find(k => k.key === key);
    if (!configKey) {
      return {
        key,
        resolvedValue: undefined,
        resolvedLevel: 'global',
        resolvedScope: 'default',
        chain: [],
      };
    }

    const chain: Array<{ level: ConfigLevel; scope: string; value: unknown }> = [];

    // Build resolution chain (highest priority first)
    const levels: Array<{ level: ConfigLevel; scope: string | undefined }> = [
      { level: 'experiment', scope: context.experimentId },
      { level: 'device', scope: context.deviceId },
      { level: 'tenant', scope: context.tenantId },
      { level: 'regional', scope: context.region },
      { level: 'global', scope: 'default' },
    ];

    for (const { level, scope } of levels) {
      if (!scope) continue;
      if (!configKey.allowedLevels.includes(level)) continue;

      const override = this.overrides.find(
        o => o.key === key && o.level === level && o.scope === scope
      );

      if (override) {
        // Check expiry
        if (override.expiresAt && override.expiresAt < Date.now()) continue;
        chain.push({ level, scope, value: override.value });
      }
    }

    // Add default
    chain.push({ level: 'global', scope: 'default', value: configKey.defaultValue });

    const resolved = chain[0];
    return {
      key,
      resolvedValue: resolved.value,
      resolvedLevel: resolved.level,
      resolvedScope: resolved.scope,
      chain,
    };
  }

  getOverrides(key?: string): ConfigOverride[] {
    if (key) return this.overrides.filter(o => o.key === key);
    return [...this.overrides];
  }

  getCategories(): string[] {
    return Array.from(new Set(CONFIG_KEYS.map(k => k.category)));
  }

  getKeysByCategory(category: string): ConfigKey[] {
    return CONFIG_KEYS.filter(k => k.category === category);
  }

  exportConfig(): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    for (const key of CONFIG_KEYS) {
      result[key.key] = key.defaultValue;
    }
    for (const override of this.overrides) {
      if (override.expiresAt && override.expiresAt < Date.now()) continue;
      result[override.key] = override.value;
    }
    return result;
  }
}

// ─── Experiment System ──────────────────────────────────

export interface Experiment {
  id: string;
  name: string;
  description: string;
  status: 'draft' | 'running' | 'paused' | 'completed' | 'aborted';
  startDate: number;
  endDate?: number;
  trafficPercent: number;
  configOverrides: Array<{ key: string; value: unknown }>;
  metrics: string[];
  hypothesis: string;
  successCriteria: string;
}

export const EXPERIMENT_TEMPLATES: Omit<Experiment, 'id' | 'status' | 'startDate'>[] = [
  {
    name: 'ESKF Process Noise Tuning',
    description: 'Test different process noise values for position accuracy',
    trafficPercent: 10,
    configOverrides: [{ key: 'nav.eskf.process_noise_q', value: 0.005 }],
    metrics: ['position_accuracy_cep', 'position_fix_rate'],
    hypothesis: 'Lower process noise improves accuracy in open-sky conditions',
    successCriteria: 'CEP improvement > 10% without degrading fix rate',
  },
  {
    name: 'ML Model Canary Deployment',
    description: 'Route small traffic percentage to new ML model version',
    trafficPercent: 5,
    configOverrides: [
      { key: 'ml.model_version', value: 'v2.0.0-rc1' },
      { key: 'ml.canary_traffic_percent', value: 5 },
    ],
    metrics: ['eta_accuracy', 'prediction_latency', 'error_rate'],
    hypothesis: 'New model improves ETA accuracy by 15%',
    successCriteria: 'ETA accuracy improvement > 10%, latency < 200ms, error rate < 1%',
  },
  {
    name: 'Battery Optimization A/B',
    description: 'Compare balanced vs power_saver mode impact on user experience',
    trafficPercent: 50,
    configOverrides: [{ key: 'battery.power_mode', value: 'power_saver' }],
    metrics: ['battery_drain_rate', 'position_accuracy', 'user_satisfaction'],
    hypothesis: 'Power saver mode reduces battery drain by 30% with acceptable accuracy',
    successCriteria: 'Battery drain reduction > 20%, position accuracy degradation < 15%',
  },
];
