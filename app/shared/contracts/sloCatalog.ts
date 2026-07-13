/**
 * G.A.N.E — SLO/SLI Catalog
 * ============================
 * Formal Service Level Objectives, Indicators, and Error Budgets.
 *
 * Every subsystem declares:
 *   - SLIs (what we measure)
 *   - SLOs (what we promise)
 *   - Error budgets (how much failure is tolerable)
 *   - Burn-rate alerts (when to page)
 *   - Runbook references
 */

// ─── Core Types ─────────────────────────────────────────

export type SLIMetricType = 'availability' | 'latency' | 'throughput' | 'error_rate' | 'freshness' | 'correctness' | 'durability';

export interface SLI {
  id: string;
  name: string;
  description: string;
  metricType: SLIMetricType;
  unit: string;
  /** How to compute: e.g. "good_events / total_events" */
  formula: string;
  /** Data source: e.g. "prometheus", "application_logs", "synthetic_probes" */
  source: string;
}

export interface SLO {
  id: string;
  name: string;
  subsystem: string;
  sli: SLI;
  /** Target as a fraction, e.g. 0.999 = 99.9% */
  target: number;
  /** Rolling window in seconds */
  windowSeconds: number;
  /** Error budget = 1 - target, expressed as fraction */
  errorBudget: number;
  /** Burn rate thresholds for alerting */
  burnRateAlerts: BurnRateAlert[];
  /** Runbook URL or reference */
  runbook: string;
  /** Consequence of breach */
  breachConsequence: string;
}

export interface BurnRateAlert {
  /** Multiplier of normal burn rate */
  burnRate: number;
  /** Window to evaluate (seconds) */
  windowSeconds: number;
  /** Severity: page, ticket, log */
  severity: 'page' | 'ticket' | 'log';
  /** Description */
  description: string;
}

export interface ErrorBudgetStatus {
  sloId: string;
  windowStart: number;
  windowEnd: number;
  totalEvents: number;
  goodEvents: number;
  badEvents: number;
  currentSLI: number;
  budgetRemaining: number;
  budgetConsumedPercent: number;
  isBudgetExhausted: boolean;
}

// ─── SLI Definitions ────────────────────────────────────

export const SLIs: Record<string, SLI> = {
  POSITION_AVAILABILITY: {
    id: 'sli-pos-avail',
    name: 'Position Fix Availability',
    description: 'Fraction of time a valid position fix is available',
    metricType: 'availability',
    unit: 'ratio',
    formula: 'time_with_fix / total_time',
    source: 'gnss_engine_metrics',
  },
  POSITION_LATENCY: {
    id: 'sli-pos-latency',
    name: 'Position Fix Latency',
    description: 'Time from satellite signal to computed position',
    metricType: 'latency',
    unit: 'milliseconds',
    formula: 'p99(fix_computation_time)',
    source: 'eskf_engine_metrics',
  },
  ROUTE_COMPUTATION_LATENCY: {
    id: 'sli-route-latency',
    name: 'Route Computation Latency',
    description: 'Time to compute optimal route from origin to destination',
    metricType: 'latency',
    unit: 'milliseconds',
    formula: 'p95(route_computation_time)',
    source: 'route_graph_metrics',
  },
  ETA_ACCURACY: {
    id: 'sli-eta-accuracy',
    name: 'ETA Prediction Accuracy',
    description: 'Fraction of ETAs within 10% of actual arrival time',
    metricType: 'correctness',
    unit: 'ratio',
    formula: 'count(abs(predicted - actual) < 0.1 * actual) / total_trips',
    source: 'eta_engine_metrics',
  },
  INCIDENT_DETECTION_LATENCY: {
    id: 'sli-incident-latency',
    name: 'Incident Detection Latency',
    description: 'Time from incident occurrence to system detection',
    metricType: 'latency',
    unit: 'seconds',
    formula: 'p95(detection_time - occurrence_time)',
    source: 'incident_engine_metrics',
  },
  TELEMETRY_THROUGHPUT: {
    id: 'sli-telemetry-throughput',
    name: 'Telemetry Ingestion Throughput',
    description: 'Events per second successfully ingested',
    metricType: 'throughput',
    unit: 'events/second',
    formula: 'count(ingested_events) / window_seconds',
    source: 'telemetry_pipeline_metrics',
  },
  DATA_FRESHNESS: {
    id: 'sli-data-freshness',
    name: 'Traffic Data Freshness',
    description: 'Age of most recent traffic data in the pipeline',
    metricType: 'freshness',
    unit: 'seconds',
    formula: 'now() - max(last_update_timestamp)',
    source: 'traffic_pipeline_metrics',
  },
  API_ERROR_RATE: {
    id: 'sli-api-errors',
    name: 'API Error Rate',
    description: 'Fraction of API requests returning 5xx errors',
    metricType: 'error_rate',
    unit: 'ratio',
    formula: 'count(status >= 500) / count(total_requests)',
    source: 'api_gateway_metrics',
  },
  OFFLINE_SYNC_DURABILITY: {
    id: 'sli-offline-durability',
    name: 'Offline Sync Durability',
    description: 'Fraction of offline events successfully synced on reconnect',
    metricType: 'durability',
    unit: 'ratio',
    formula: 'count(synced_events) / count(queued_events)',
    source: 'offline_sync_metrics',
  },
  MAP_RENDER_FPS: {
    id: 'sli-map-fps',
    name: 'Map Rendering FPS',
    description: 'Sustained frames per second during map interaction',
    metricType: 'throughput',
    unit: 'fps',
    formula: 'avg(frame_rate) over 5s window',
    source: 'performance_monitor_metrics',
  },
  V2X_MESSAGE_LATENCY: {
    id: 'sli-v2x-latency',
    name: 'V2X Message Latency',
    description: 'End-to-end latency for vehicle-to-everything messages',
    metricType: 'latency',
    unit: 'milliseconds',
    formula: 'p99(receive_time - send_time)',
    source: 'v2x_engine_metrics',
  },
  BATTERY_EFFICIENCY: {
    id: 'sli-battery-efficiency',
    name: 'Battery Consumption Efficiency',
    description: 'Battery drain rate relative to baseline navigation',
    metricType: 'correctness',
    unit: 'ratio',
    formula: 'actual_drain / baseline_drain',
    source: 'battery_optimizer_metrics',
  },
};

// ─── SLO Definitions ────────────────────────────────────

function makeBurnRateAlerts(target: number): BurnRateAlert[] {
  return [
    { burnRate: 14.4, windowSeconds: 3600, severity: 'page', description: `Burning ${(14.4 * (1 - target) * 100).toFixed(2)}% of budget per hour` },
    { burnRate: 6, windowSeconds: 21600, severity: 'page', description: `Burning ${(6 * (1 - target) * 100).toFixed(2)}% of budget per 6h` },
    { burnRate: 3, windowSeconds: 86400, severity: 'ticket', description: `Burning ${(3 * (1 - target) * 100).toFixed(2)}% of budget per day` },
    { burnRate: 1, windowSeconds: 259200, severity: 'log', description: `Normal burn rate over 3 days` },
  ];
}

export const SLOs: SLO[] = [
  {
    id: 'slo-position-availability',
    name: 'Position Fix Availability',
    subsystem: 'GNSS/ESKF',
    sli: SLIs.POSITION_AVAILABILITY,
    target: 0.999,
    windowSeconds: 2592000, // 30 days
    errorBudget: 0.001,
    burnRateAlerts: makeBurnRateAlerts(0.999),
    runbook: 'runbooks/position-availability.md',
    breachConsequence: 'Navigation degraded to dead-reckoning mode; user notified',
  },
  {
    id: 'slo-position-latency',
    name: 'Position Fix Latency (p99 < 100ms)',
    subsystem: 'ESKF',
    sli: SLIs.POSITION_LATENCY,
    target: 0.99,
    windowSeconds: 604800, // 7 days
    errorBudget: 0.01,
    burnRateAlerts: makeBurnRateAlerts(0.99),
    runbook: 'runbooks/position-latency.md',
    breachConsequence: 'Reduce ESKF update rate; switch to simplified Kalman',
  },
  {
    id: 'slo-route-latency',
    name: 'Route Computation (p95 < 500ms)',
    subsystem: 'RouteGraph',
    sli: SLIs.ROUTE_COMPUTATION_LATENCY,
    target: 0.95,
    windowSeconds: 604800,
    errorBudget: 0.05,
    burnRateAlerts: makeBurnRateAlerts(0.95),
    runbook: 'runbooks/route-latency.md',
    breachConsequence: 'Fall back to cached routes; disable real-time rerouting',
  },
  {
    id: 'slo-eta-accuracy',
    name: 'ETA Accuracy (within 10%)',
    subsystem: 'ETAEngine',
    sli: SLIs.ETA_ACCURACY,
    target: 0.85,
    windowSeconds: 2592000,
    errorBudget: 0.15,
    burnRateAlerts: makeBurnRateAlerts(0.85),
    runbook: 'runbooks/eta-accuracy.md',
    breachConsequence: 'Widen confidence intervals shown to user; retrain model',
  },
  {
    id: 'slo-incident-detection',
    name: 'Incident Detection (p95 < 30s)',
    subsystem: 'IncidentEngine',
    sli: SLIs.INCIDENT_DETECTION_LATENCY,
    target: 0.95,
    windowSeconds: 604800,
    errorBudget: 0.05,
    burnRateAlerts: makeBurnRateAlerts(0.95),
    runbook: 'runbooks/incident-detection.md',
    breachConsequence: 'Increase crowd report weight; lower confirmation threshold',
  },
  {
    id: 'slo-telemetry-throughput',
    name: 'Telemetry Ingestion (> 10k events/s)',
    subsystem: 'TelemetryPipeline',
    sli: SLIs.TELEMETRY_THROUGHPUT,
    target: 0.999,
    windowSeconds: 604800,
    errorBudget: 0.001,
    burnRateAlerts: makeBurnRateAlerts(0.999),
    runbook: 'runbooks/telemetry-throughput.md',
    breachConsequence: 'Activate backpressure; shed low-priority telemetry',
  },
  {
    id: 'slo-data-freshness',
    name: 'Traffic Data Freshness (< 60s)',
    subsystem: 'TrafficPipeline',
    sli: SLIs.DATA_FRESHNESS,
    target: 0.99,
    windowSeconds: 604800,
    errorBudget: 0.01,
    burnRateAlerts: makeBurnRateAlerts(0.99),
    runbook: 'runbooks/data-freshness.md',
    breachConsequence: 'Show stale data indicator; increase polling frequency',
  },
  {
    id: 'slo-api-errors',
    name: 'API Error Rate (< 0.1%)',
    subsystem: 'APIGateway',
    sli: SLIs.API_ERROR_RATE,
    target: 0.999,
    windowSeconds: 604800,
    errorBudget: 0.001,
    burnRateAlerts: makeBurnRateAlerts(0.999),
    runbook: 'runbooks/api-errors.md',
    breachConsequence: 'Circuit-break failing endpoints; serve cached responses',
  },
  {
    id: 'slo-offline-durability',
    name: 'Offline Sync Durability (> 99.9%)',
    subsystem: 'OfflineSync',
    sli: SLIs.OFFLINE_SYNC_DURABILITY,
    target: 0.999,
    windowSeconds: 2592000,
    errorBudget: 0.001,
    burnRateAlerts: makeBurnRateAlerts(0.999),
    runbook: 'runbooks/offline-sync.md',
    breachConsequence: 'Retry with exponential backoff; alert user of unsyncable items',
  },
  {
    id: 'slo-map-fps',
    name: 'Map Rendering (> 30 FPS sustained)',
    subsystem: 'MapRenderer',
    sli: SLIs.MAP_RENDER_FPS,
    target: 0.95,
    windowSeconds: 604800,
    errorBudget: 0.05,
    burnRateAlerts: makeBurnRateAlerts(0.95),
    runbook: 'runbooks/map-fps.md',
    breachConsequence: 'Reduce map detail level; disable 3D buildings; lower overlay count',
  },
  {
    id: 'slo-v2x-latency',
    name: 'V2X Message Latency (p99 < 100ms)',
    subsystem: 'V2XEngine',
    sli: SLIs.V2X_MESSAGE_LATENCY,
    target: 0.99,
    windowSeconds: 604800,
    errorBudget: 0.01,
    burnRateAlerts: makeBurnRateAlerts(0.99),
    runbook: 'runbooks/v2x-latency.md',
    breachConsequence: 'Increase safety margins; fall back to radar-only detection',
  },
  {
    id: 'slo-battery-efficiency',
    name: 'Battery Efficiency (< 1.5x baseline)',
    subsystem: 'BatteryOptimizer',
    sli: SLIs.BATTERY_EFFICIENCY,
    target: 0.9,
    windowSeconds: 2592000,
    errorBudget: 0.1,
    burnRateAlerts: makeBurnRateAlerts(0.9),
    runbook: 'runbooks/battery-efficiency.md',
    breachConsequence: 'Switch to power-saver mode; reduce GPS polling frequency',
  },
];

// ─── Error Budget Calculator ────────────────────────────

export class ErrorBudgetTracker {
  private windows = new Map<string, { good: number; bad: number; start: number }>();

  recordEvent(sloId: string, isGood: boolean): void {
    const slo = SLOs.find(s => s.id === sloId);
    if (!slo) return;

    const now = Date.now();
    let window = this.windows.get(sloId);
    if (!window || now - window.start > slo.windowSeconds * 1000) {
      window = { good: 0, bad: 0, start: now };
      this.windows.set(sloId, window);
    }

    if (isGood) window.good++;
    else window.bad++;
  }

  getStatus(sloId: string): ErrorBudgetStatus | null {
    const slo = SLOs.find(s => s.id === sloId);
    const window = this.windows.get(sloId);
    if (!slo || !window) return null;

    const total = window.good + window.bad;
    if (total === 0) return null;

    const currentSLI = window.good / total;
    const budgetUsed = Math.max(0, slo.target - currentSLI) / slo.errorBudget;

    return {
      sloId,
      windowStart: window.start,
      windowEnd: window.start + slo.windowSeconds * 1000,
      totalEvents: total,
      goodEvents: window.good,
      badEvents: window.bad,
      currentSLI,
      budgetRemaining: Math.max(0, 1 - budgetUsed),
      budgetConsumedPercent: Math.min(100, budgetUsed * 100),
      isBudgetExhausted: budgetUsed >= 1,
    };
  }

  getAllStatuses(): ErrorBudgetStatus[] {
    return SLOs.map(slo => this.getStatus(slo.id)).filter((s): s is ErrorBudgetStatus => s !== null);
  }
}

// ─── Runbook Registry ───────────────────────────────────

export interface Runbook {
  id: string;
  sloId: string;
  title: string;
  symptoms: string[];
  diagnosticSteps: string[];
  mitigationSteps: string[];
  escalationPath: string[];
  estimatedTTR: string;
}

export const RUNBOOKS: Runbook[] = [
  {
    id: 'rb-position-availability',
    sloId: 'slo-position-availability',
    title: 'Position Fix Availability Degradation',
    symptoms: [
      'Users report "No GPS" indicator',
      'ESKF engine fallback to PDR-only mode',
      'Position fix rate drops below 99.9%',
    ],
    diagnosticSteps: [
      '1. Check GNSS constellation status (multi-constellation dashboard)',
      '2. Verify antenna/receiver health metrics',
      '3. Check for ionospheric disturbances (space weather)',
      '4. Review urban canyon detection logs',
      '5. Check tunnel recovery engine state',
    ],
    mitigationSteps: [
      '1. Enable multi-constellation fallback (NavIC, QZSS, BeiDou)',
      '2. Increase PDR weight in sensor fusion',
      '3. Activate WiFi/BLE indoor positioning',
      '4. Notify affected users with degradation banner',
    ],
    escalationPath: [
      'L1: On-call SRE → check dashboards',
      'L2: GNSS team → constellation analysis',
      'L3: VP Engineering → coordinate with satellite operators',
    ],
    estimatedTTR: '15 minutes (L1) to 4 hours (L3)',
  },
  {
    id: 'rb-api-errors',
    sloId: 'slo-api-errors',
    title: 'API Error Rate Spike',
    symptoms: [
      '5xx error rate exceeds 0.1%',
      'Client-side retry storms detected',
      'Backpressure controller activating',
    ],
    diagnosticSteps: [
      '1. Check error logs for stack traces',
      '2. Verify database connection pool health',
      '3. Check memory/CPU utilization',
      '4. Review recent deployments',
      '5. Check downstream service health',
    ],
    mitigationSteps: [
      '1. Activate circuit breaker for failing endpoints',
      '2. Serve cached responses where possible',
      '3. Scale up compute if resource-constrained',
      '4. Rollback recent deployment if correlated',
    ],
    escalationPath: [
      'L1: On-call SRE → dashboard triage',
      'L2: Backend team → code-level investigation',
      'L3: Platform team → infrastructure remediation',
    ],
    estimatedTTR: '5 minutes (L1) to 2 hours (L3)',
  },
  {
    id: 'rb-telemetry-throughput',
    sloId: 'slo-telemetry-throughput',
    title: 'Telemetry Ingestion Throughput Drop',
    symptoms: [
      'Ingestion rate drops below 10k events/s',
      'Queue depth increasing',
      'Data freshness SLO at risk',
    ],
    diagnosticSteps: [
      '1. Check ETL pipeline health',
      '2. Verify message queue capacity',
      '3. Check consumer lag metrics',
      '4. Review data validation rejection rate',
    ],
    mitigationSteps: [
      '1. Activate backpressure shedding for low-priority events',
      '2. Scale consumer instances horizontally',
      '3. Increase batch size for bulk ingestion',
      '4. Temporarily disable non-critical enrichment steps',
    ],
    escalationPath: [
      'L1: Data team → pipeline monitoring',
      'L2: Infrastructure → scaling and capacity',
      'L3: Architecture → redesign ingestion path',
    ],
    estimatedTTR: '10 minutes (L1) to 6 hours (L3)',
  },
];

// ─── Failover Matrix ────────────────────────────────────

export interface FailoverScenario {
  id: string;
  name: string;
  triggerCondition: string;
  affectedSubsystems: string[];
  fallbackMode: string;
  recoveryStrategy: string;
  dataLossRisk: 'none' | 'minimal' | 'partial' | 'significant';
  estimatedRecoveryTime: string;
  userImpact: string;
  automatedResponse: boolean;
}

export const FAILOVER_MATRIX: FailoverScenario[] = [
  {
    id: 'fo-gnss-blackout',
    name: 'GNSS Constellation Blackout',
    triggerCondition: 'All GNSS constellations unavailable (jamming, solar event)',
    affectedSubsystems: ['ESKF', 'MultiConstellation', 'PositionFix'],
    fallbackMode: 'PDR + WiFi + BLE + Map Matching',
    recoveryStrategy: 'Monitor constellation status; auto-resume when signals return',
    dataLossRisk: 'none',
    estimatedRecoveryTime: 'Automatic on signal restoration',
    userImpact: 'Reduced position accuracy (10-50m vs 2-5m)',
    automatedResponse: true,
  },
  {
    id: 'fo-network-partition',
    name: 'Network Partition (Complete Offline)',
    triggerCondition: 'No cellular/WiFi connectivity for > 30 seconds',
    affectedSubsystems: ['API', 'LiveSharing', 'CrowdIntelligence', 'Telemetry'],
    fallbackMode: 'Offline-first with CRDT sync queue',
    recoveryStrategy: 'Queue all mutations; sync on reconnect with conflict resolution',
    dataLossRisk: 'minimal',
    estimatedRecoveryTime: 'Automatic on connectivity restoration',
    userImpact: 'No real-time traffic; cached routes only; no live sharing',
    automatedResponse: true,
  },
  {
    id: 'fo-database-failure',
    name: 'Primary Database Failure',
    triggerCondition: 'Database connection pool exhausted or primary node down',
    affectedSubsystems: ['API', 'Analytics', 'UserManagement', 'TripHistory'],
    fallbackMode: 'Read from replica; write to WAL buffer',
    recoveryStrategy: 'Failover to replica; replay WAL on primary recovery',
    dataLossRisk: 'minimal',
    estimatedRecoveryTime: '30 seconds (automated failover)',
    userImpact: 'Brief read-only mode; writes buffered',
    automatedResponse: true,
  },
  {
    id: 'fo-ml-model-failure',
    name: 'ML Model Inference Failure',
    triggerCondition: 'Model serving latency > 500ms or error rate > 5%',
    affectedSubsystems: ['MLPrediction', 'ETAEngine', 'CrowdIntelligence'],
    fallbackMode: 'Rule-based heuristics; historical averages',
    recoveryStrategy: 'Rollback to previous model version; investigate root cause',
    dataLossRisk: 'none',
    estimatedRecoveryTime: '5 minutes (automated rollback)',
    userImpact: 'Slightly less accurate predictions; wider confidence intervals',
    automatedResponse: true,
  },
  {
    id: 'fo-memory-pressure',
    name: 'Client Memory Pressure',
    triggerCondition: 'JS heap > 80% of device memory budget',
    affectedSubsystems: ['MapRenderer', 'DigitalTwin', 'AnalyticsEngine'],
    fallbackMode: 'Progressive feature shedding (3D → 2D → simplified)',
    recoveryStrategy: 'GC trigger; evict caches; reduce overlay count',
    dataLossRisk: 'none',
    estimatedRecoveryTime: 'Immediate (< 1 second)',
    userImpact: 'Reduced visual fidelity; fewer map layers',
    automatedResponse: true,
  },
  {
    id: 'fo-third-party-api',
    name: 'Third-Party API Degradation',
    triggerCondition: 'External API (weather, traffic, maps) latency > 5s or 5xx',
    affectedSubsystems: ['WeatherOverlay', 'TrafficPipeline', 'MapTiles'],
    fallbackMode: 'Cached data with staleness indicator',
    recoveryStrategy: 'Circuit breaker with exponential backoff; try alternate providers',
    dataLossRisk: 'none',
    estimatedRecoveryTime: '1-30 minutes (depends on provider)',
    userImpact: 'Stale weather/traffic data; "last updated" timestamp shown',
    automatedResponse: true,
  },
  {
    id: 'fo-security-breach',
    name: 'Security Incident Detected',
    triggerCondition: 'Anomaly detection flags potential breach (replay attack, credential stuffing)',
    affectedSubsystems: ['Authentication', 'API', 'UserData'],
    fallbackMode: 'Lockdown mode: read-only, no new sessions',
    recoveryStrategy: 'Rotate keys; invalidate sessions; forensic analysis',
    dataLossRisk: 'none',
    estimatedRecoveryTime: '1-24 hours (manual investigation required)',
    userImpact: 'Temporary service disruption; forced re-authentication',
    automatedResponse: false,
  },
];

// ─── Chaos Scenario Catalog ────────────────────────────

export interface ChaosScenario {
  id: string;
  name: string;
  description: string;
  targetSubsystem: string;
  injectionType: 'latency' | 'error' | 'resource' | 'network' | 'data';
  parameters: Record<string, number | string>;
  expectedBehavior: string;
  validationCriteria: string[];
  safetyGuards: string[];
}

export const CHAOS_SCENARIOS: ChaosScenario[] = [
  {
    id: 'chaos-gnss-jitter',
    name: 'GNSS Signal Jitter',
    description: 'Inject random noise into GNSS position fixes',
    targetSubsystem: 'ESKF',
    injectionType: 'data',
    parameters: { noiseStdDev: 50, durationSeconds: 300 },
    expectedBehavior: 'ESKF should filter noise; position should remain stable within 10m',
    validationCriteria: [
      'Position error < 10m during injection',
      'No false reroute triggers',
      'Sensor quality indicator shows degradation',
    ],
    safetyGuards: ['Auto-stop after 5 minutes', 'Only in staging environment'],
  },
  {
    id: 'chaos-api-latency',
    name: 'API Latency Injection',
    description: 'Add 2-5 second delay to all API responses',
    targetSubsystem: 'APIGateway',
    injectionType: 'latency',
    parameters: { minDelayMs: 2000, maxDelayMs: 5000, affectedEndpoints: '*' },
    expectedBehavior: 'Client should show loading states; offline cache should activate',
    validationCriteria: [
      'No client crashes or unhandled errors',
      'Offline mode activates within 10s',
      'User sees appropriate loading indicators',
    ],
    safetyGuards: ['Circuit breaker should trip', 'Backpressure controller activates'],
  },
  {
    id: 'chaos-memory-leak',
    name: 'Simulated Memory Leak',
    description: 'Gradually consume memory to test GC and shedding behavior',
    targetSubsystem: 'PerformanceMonitor',
    injectionType: 'resource',
    parameters: { leakRateMBPerMinute: 10, maxMB: 200 },
    expectedBehavior: 'Performance monitor should detect pressure and shed features',
    validationCriteria: [
      'Feature shedding activates before OOM',
      'Map degrades gracefully (3D → 2D → simplified)',
      'No data loss during degradation',
    ],
    safetyGuards: ['Hard cap at 200MB', 'Auto-cleanup on test end'],
  },
  {
    id: 'chaos-network-partition',
    name: 'Network Partition Simulation',
    description: 'Drop all network connectivity for configurable duration',
    targetSubsystem: 'OfflineSync',
    injectionType: 'network',
    parameters: { durationSeconds: 120, dropRate: 1.0 },
    expectedBehavior: 'Offline mode activates; all mutations queued; sync on restore',
    validationCriteria: [
      'No data loss during partition',
      'CRDT sync resolves conflicts on reconnect',
      'User notified of offline status within 5s',
    ],
    safetyGuards: ['Max duration 5 minutes', 'Manual override available'],
  },
  {
    id: 'chaos-db-slowdown',
    name: 'Database Slowdown',
    description: 'Inject 500ms-2s latency on all database queries',
    targetSubsystem: 'Database',
    injectionType: 'latency',
    parameters: { minDelayMs: 500, maxDelayMs: 2000 },
    expectedBehavior: 'API should serve cached data; writes should buffer',
    validationCriteria: [
      'API response time < 3s with caching',
      'No timeout errors surfaced to users',
      'Write buffer drains on recovery',
    ],
    safetyGuards: ['Connection pool monitoring', 'Auto-disable after 10 minutes'],
  },
];
