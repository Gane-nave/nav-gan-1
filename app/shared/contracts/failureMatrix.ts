/**
 * G.A.N.E — Failure Matrix Contract
 * ====================================
 * Formal failure case catalog with fallback modes,
 * recovery strategies, and dependency chains.
 */

// ─── Types ──────────────────────────────────────────────

export type FailureSeverity = 'info' | 'warning' | 'degraded' | 'critical' | 'fatal';
export type RecoveryType = 'automatic' | 'semi_automatic' | 'manual';

export interface FailureCase {
  id: string;
  name: string;
  severity: FailureSeverity;
  subsystem: string;
  trigger: string;
  detection: string;
  detectionLatency: string;
  fallbackMode: string;
  recoveryType: RecoveryType;
  recoverySteps: string[];
  recoveryTime: string;
  dataLoss: string;
  userNotification: string;
  dependencies: string[];
  testScenario: string;
}

// ─── Failure Cases ──────────────────────────────────────

export const FAILURE_CASES: FailureCase[] = [
  // ── GNSS Failures ──
  {
    id: 'fail-gnss-total-loss',
    name: 'Total GNSS Signal Loss',
    severity: 'degraded',
    subsystem: 'ESKF / MultiConstellation',
    trigger: 'All GNSS constellations unavailable (tunnel, deep urban canyon, jamming)',
    detection: 'ESKF reports no satellite fixes for > 5 seconds',
    detectionLatency: '< 5 seconds',
    fallbackMode: 'PDR (Pedestrian Dead Reckoning) + Map Matching + WiFi/BLE positioning',
    recoveryType: 'automatic',
    recoverySteps: [
      '1. ESKF switches to PDR-only mode',
      '2. Map matching constrains position to road network',
      '3. WiFi/BLE scans for indoor positioning if available',
      '4. On signal restoration, ESKF re-initializes with fresh fix',
    ],
    recoveryTime: 'Automatic on signal restoration (< 2 seconds)',
    dataLoss: 'None — position estimates continue via PDR',
    userNotification: 'Subtle "GPS Searching" indicator; position accuracy badge changes',
    dependencies: ['PDR engine', 'Map matching engine', 'WiFi scanner'],
    testScenario: 'chaos-gnss-jitter: inject 100% signal loss for 60 seconds',
  },
  {
    id: 'fail-gnss-multipath',
    name: 'GNSS Multipath Interference',
    severity: 'warning',
    subsystem: 'ESKF / UrbanCanyon',
    trigger: 'Signal reflections in urban environment causing position scatter',
    detection: 'Position variance exceeds 3σ threshold; HDOP > 5',
    detectionLatency: '< 2 seconds',
    fallbackMode: 'Urban canyon mode: increased IMU weight, reduced GNSS weight in fusion',
    recoveryType: 'automatic',
    recoverySteps: [
      '1. UrbanCanyon engine detects building geometry',
      '2. ESKF reduces GNSS measurement noise covariance',
      '3. Map matching provides additional constraint',
      '4. Position smoothing applied to reduce jitter',
    ],
    recoveryTime: 'Continuous adaptation (< 1 second)',
    dataLoss: 'None',
    userNotification: 'Position accuracy indicator shows wider circle',
    dependencies: ['UrbanCanyon engine', 'Map matching', 'Building geometry database'],
    testScenario: 'Inject multipath noise with 50m scatter for 120 seconds',
  },

  // ── Network Failures ──
  {
    id: 'fail-network-complete',
    name: 'Complete Network Loss',
    severity: 'degraded',
    subsystem: 'OfflineSync / API',
    trigger: 'No cellular or WiFi connectivity',
    detection: 'Navigator.onLine === false OR 3 consecutive API timeouts',
    detectionLatency: '< 10 seconds',
    fallbackMode: 'Full offline mode with CRDT sync queue',
    recoveryType: 'automatic',
    recoverySteps: [
      '1. OfflineSync engine activates',
      '2. All mutations queued to IndexedDB with CRDT timestamps',
      '3. Cached map tiles and routes served from local storage',
      '4. On reconnect, CRDT sync resolves conflicts',
      '5. Telemetry batch-uploaded with backfill timestamps',
    ],
    recoveryTime: 'Automatic on connectivity restoration (< 5 seconds)',
    dataLoss: 'None — all data queued and synced',
    userNotification: '"Offline Mode" banner; cached data indicator',
    dependencies: ['OfflineSync engine', 'IndexedDB', 'Service Worker'],
    testScenario: 'chaos-network-partition: drop all connectivity for 120 seconds',
  },
  {
    id: 'fail-network-degraded',
    name: 'Degraded Network (High Latency)',
    severity: 'warning',
    subsystem: 'API / WebSocket',
    trigger: 'Network latency > 2 seconds or packet loss > 10%',
    detection: 'Moving average of API response times exceeds threshold',
    detectionLatency: '< 15 seconds',
    fallbackMode: 'Reduced polling frequency; batch requests; serve cached data',
    recoveryType: 'automatic',
    recoverySteps: [
      '1. Reduce API polling frequency (60s → 120s)',
      '2. Batch multiple requests into single calls',
      '3. Serve cached traffic/weather data with staleness indicator',
      '4. Disable non-critical features (live sharing updates)',
      '5. Resume normal operation when latency drops below threshold',
    ],
    recoveryTime: 'Automatic when network improves (< 10 seconds)',
    dataLoss: 'None',
    userNotification: '"Slow connection" indicator; data freshness timestamps',
    dependencies: ['Performance monitor', 'Cache layer'],
    testScenario: 'Inject 3-second latency on all API calls for 60 seconds',
  },

  // ── Database Failures ──
  {
    id: 'fail-db-connection',
    name: 'Database Connection Pool Exhaustion',
    severity: 'critical',
    subsystem: 'Database / API',
    trigger: 'All connection pool slots occupied; new queries rejected',
    detection: 'Connection acquire timeout; error rate spike',
    detectionLatency: '< 5 seconds',
    fallbackMode: 'Read from cache; write to WAL buffer; reject non-critical writes',
    recoveryType: 'semi_automatic',
    recoverySteps: [
      '1. Circuit breaker trips for database writes',
      '2. Cached data served for read queries',
      '3. Critical writes buffered in WAL',
      '4. Alert sent to on-call SRE',
      '5. Auto-scale connection pool if possible',
      '6. Drain long-running queries',
    ],
    recoveryTime: '30 seconds (auto-scale) to 15 minutes (manual intervention)',
    dataLoss: 'Minimal — WAL buffer preserves writes',
    userNotification: '"Service temporarily limited" for write operations',
    dependencies: ['Connection pool manager', 'Cache layer', 'WAL buffer'],
    testScenario: 'chaos-db-slowdown: inject 2-second query latency',
  },

  // ── Client-Side Failures ──
  {
    id: 'fail-memory-pressure',
    name: 'Client Memory Pressure',
    severity: 'degraded',
    subsystem: 'PerformanceMonitor / MapRenderer',
    trigger: 'JS heap usage > 80% of device memory budget',
    detection: 'Performance monitor polls performance.memory every 5 seconds',
    detectionLatency: '< 5 seconds',
    fallbackMode: 'Progressive feature shedding: 3D → 2D → simplified map',
    recoveryType: 'automatic',
    recoverySteps: [
      '1. Disable 3D building rendering',
      '2. Reduce map overlay count (keep traffic, remove weather)',
      '3. Clear tile cache for off-screen areas',
      '4. Disable particle effects and animations',
      '5. If still high, switch to simplified map renderer',
      '6. Force GC hint via structured clone trick',
    ],
    recoveryTime: 'Immediate (< 1 second per shedding step)',
    dataLoss: 'None — visual fidelity reduced only',
    userNotification: 'Subtle "Performance mode" indicator',
    dependencies: ['PerformanceMonitor', 'Quality scaling system'],
    testScenario: 'chaos-memory-leak: allocate 10MB/min until 200MB',
  },
  {
    id: 'fail-webgl-crash',
    name: 'WebGL Context Loss',
    severity: 'degraded',
    subsystem: 'MapRenderer / WebGPU',
    trigger: 'GPU driver crash or resource exhaustion',
    detection: 'webglcontextlost event on canvas',
    detectionLatency: '< 1 second',
    fallbackMode: 'Canvas 2D fallback renderer with simplified map',
    recoveryType: 'automatic',
    recoverySteps: [
      '1. Catch webglcontextlost event',
      '2. Switch to Canvas 2D fallback renderer',
      '3. Attempt WebGL context restoration after 5 seconds',
      '4. If restoration fails 3 times, stay on Canvas 2D',
    ],
    recoveryTime: '< 2 seconds to fallback; 5-15 seconds to restore WebGL',
    dataLoss: 'None',
    userNotification: 'Brief flash during renderer switch',
    dependencies: ['Canvas 2D fallback renderer'],
    testScenario: 'Force webglcontextlost event via extension.loseContext()',
  },

  // ── ML/AI Failures ──
  {
    id: 'fail-ml-inference',
    name: 'ML Model Inference Failure',
    severity: 'warning',
    subsystem: 'MLPrediction / ETAEngine',
    trigger: 'Model serving latency > 500ms or inference error',
    detection: 'Inference timeout or error counter exceeds threshold',
    detectionLatency: '< 10 seconds',
    fallbackMode: 'Rule-based heuristics; historical averages',
    recoveryType: 'automatic',
    recoverySteps: [
      '1. Switch to rule-based fallback (speed limits, historical patterns)',
      '2. Widen confidence intervals in predictions',
      '3. Attempt model reload from cache',
      '4. If persistent, rollback to previous model version',
    ],
    recoveryTime: '< 5 seconds (fallback); 5 minutes (model rollback)',
    dataLoss: 'None — predictions continue with lower accuracy',
    userNotification: 'ETA shown with wider range; "approximate" label',
    dependencies: ['Rule-based fallback engine', 'Model version registry'],
    testScenario: 'Inject 1-second latency on all ML inference calls',
  },
];

// ─── Dependency Chain ───────────────────────────────────

export interface DependencyNode {
  id: string;
  name: string;
  type: 'engine' | 'service' | 'database' | 'external' | 'infrastructure';
  criticality: 'critical' | 'high' | 'medium' | 'low';
  dependsOn: string[];
  dependedBy: string[];
  healthCheck: string;
  timeoutMs: number;
}

export const DEPENDENCY_GRAPH: DependencyNode[] = [
  {
    id: 'dep-eskf',
    name: 'ESKF Engine',
    type: 'engine',
    criticality: 'critical',
    dependsOn: ['dep-gnss', 'dep-imu', 'dep-map-data'],
    dependedBy: ['dep-navigation', 'dep-route-graph', 'dep-digital-twin'],
    healthCheck: 'eskf.getState().isInitialized',
    timeoutMs: 100,
  },
  {
    id: 'dep-gnss',
    name: 'GNSS Receiver',
    type: 'infrastructure',
    criticality: 'critical',
    dependsOn: [],
    dependedBy: ['dep-eskf', 'dep-multi-constellation'],
    healthCheck: 'navigator.geolocation.getCurrentPosition',
    timeoutMs: 5000,
  },
  {
    id: 'dep-imu',
    name: 'IMU Sensors',
    type: 'infrastructure',
    criticality: 'high',
    dependsOn: [],
    dependedBy: ['dep-eskf', 'dep-pdr'],
    healthCheck: 'DeviceMotionEvent listener active',
    timeoutMs: 1000,
  },
  {
    id: 'dep-map-data',
    name: 'Map Tile Service',
    type: 'external',
    criticality: 'high',
    dependsOn: ['dep-network'],
    dependedBy: ['dep-eskf', 'dep-map-renderer', 'dep-route-graph'],
    healthCheck: 'Tile request returns 200',
    timeoutMs: 5000,
  },
  {
    id: 'dep-network',
    name: 'Network Connectivity',
    type: 'infrastructure',
    criticality: 'high',
    dependsOn: [],
    dependedBy: ['dep-api', 'dep-map-data', 'dep-live-sharing', 'dep-telemetry'],
    healthCheck: 'navigator.onLine && fetch("/api/health")',
    timeoutMs: 3000,
  },
  {
    id: 'dep-api',
    name: 'API Gateway',
    type: 'service',
    criticality: 'critical',
    dependsOn: ['dep-network', 'dep-database'],
    dependedBy: ['dep-incident', 'dep-crowd', 'dep-analytics', 'dep-admin'],
    healthCheck: 'GET /api/health returns 200',
    timeoutMs: 5000,
  },
  {
    id: 'dep-database',
    name: 'Database (TiDB)',
    type: 'database',
    criticality: 'critical',
    dependsOn: [],
    dependedBy: ['dep-api', 'dep-analytics', 'dep-admin'],
    healthCheck: 'SELECT 1 returns within 1s',
    timeoutMs: 5000,
  },
  {
    id: 'dep-navigation',
    name: 'Navigation Manager',
    type: 'engine',
    criticality: 'critical',
    dependsOn: ['dep-eskf', 'dep-route-graph', 'dep-map-renderer'],
    dependedBy: [],
    healthCheck: 'navigationManager.getState().isActive',
    timeoutMs: 200,
  },
  {
    id: 'dep-route-graph',
    name: 'Route Graph Engine',
    type: 'engine',
    criticality: 'high',
    dependsOn: ['dep-eskf', 'dep-map-data'],
    dependedBy: ['dep-navigation', 'dep-reroute'],
    healthCheck: 'routeGraph.isInitialized()',
    timeoutMs: 500,
  },
  {
    id: 'dep-map-renderer',
    name: 'Map Renderer',
    type: 'engine',
    criticality: 'high',
    dependsOn: ['dep-map-data'],
    dependedBy: ['dep-navigation'],
    healthCheck: 'WebGL context active',
    timeoutMs: 1000,
  },
];

// ─── Helpers ────────────────────────────────────────────

export function getFailuresBySeverity(severity: FailureSeverity): FailureCase[] {
  return FAILURE_CASES.filter(f => f.severity === severity);
}

export function getCriticalDependencies(): DependencyNode[] {
  return DEPENDENCY_GRAPH.filter(d => d.criticality === 'critical');
}

export function getDependencyChain(nodeId: string): string[] {
  const visited = new Set<string>();
  const chain: string[] = [];

  function walk(id: string) {
    if (visited.has(id)) return;
    visited.add(id);
    chain.push(id);
    const node = DEPENDENCY_GRAPH.find(d => d.id === id);
    if (node) {
      for (const dep of node.dependsOn) {
        walk(dep);
      }
    }
  }

  walk(nodeId);
  return chain;
}
