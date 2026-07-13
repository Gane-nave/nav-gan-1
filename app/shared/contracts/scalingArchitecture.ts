/**
 * Scaling Architecture & Load Testing Infrastructure
 * 
 * Defines:
 *  - Horizontal scaling configuration
 *  - Connection pooling strategy
 *  - Load test scenarios (k6-compatible)
 *  - Auto-scaling policies
 *  - Health check endpoints
 *  - Performance budgets per endpoint
 */

// ═══════════════════════════════════════════════════════════
// SCALING TIERS
// ═══════════════════════════════════════════════════════════

export const SCALING_TIERS = {
  starter: {
    name: 'Starter',
    maxInstances: 1,
    maxConcurrentUsers: 100,
    dbPoolSize: 10,
    wsConnections: 50,
    rateLimit: { requests: 100, windowMs: 60_000 },
    storage: '5GB',
    bandwidth: '10GB/month',
  },
  growth: {
    name: 'Growth',
    maxInstances: 3,
    maxConcurrentUsers: 1_000,
    dbPoolSize: 25,
    wsConnections: 500,
    rateLimit: { requests: 500, windowMs: 60_000 },
    storage: '50GB',
    bandwidth: '100GB/month',
  },
  scale: {
    name: 'Scale',
    maxInstances: 10,
    maxConcurrentUsers: 10_000,
    dbPoolSize: 50,
    wsConnections: 5_000,
    rateLimit: { requests: 2000, windowMs: 60_000 },
    storage: '500GB',
    bandwidth: '1TB/month',
  },
  enterprise: {
    name: 'Enterprise',
    maxInstances: 50,
    maxConcurrentUsers: 100_000,
    dbPoolSize: 100,
    wsConnections: 50_000,
    rateLimit: { requests: 10000, windowMs: 60_000 },
    storage: '5TB',
    bandwidth: '10TB/month',
  },
} as const;

// ═══════════════════════════════════════════════════════════
// AUTO-SCALING POLICIES
// ═══════════════════════════════════════════════════════════

export const AUTO_SCALING_POLICIES = {
  cpuBased: {
    metric: 'cpu_utilization',
    scaleUpThreshold: 70,    // % CPU
    scaleDownThreshold: 30,
    cooldownPeriod: 300,     // seconds
    minInstances: 1,
    maxInstances: 10,
    scaleUpStep: 1,
    scaleDownStep: 1,
  },
  requestBased: {
    metric: 'requests_per_second',
    scaleUpThreshold: 500,
    scaleDownThreshold: 100,
    cooldownPeriod: 180,
    minInstances: 1,
    maxInstances: 10,
    scaleUpStep: 2,
    scaleDownStep: 1,
  },
  memoryBased: {
    metric: 'memory_utilization',
    scaleUpThreshold: 80,    // % RAM
    scaleDownThreshold: 40,
    cooldownPeriod: 300,
    minInstances: 1,
    maxInstances: 10,
    scaleUpStep: 1,
    scaleDownStep: 1,
  },
  websocketBased: {
    metric: 'active_ws_connections',
    scaleUpThreshold: 1000,
    scaleDownThreshold: 200,
    cooldownPeriod: 120,
    minInstances: 1,
    maxInstances: 5,
    scaleUpStep: 1,
    scaleDownStep: 1,
  },
} as const;

// ═══════════════════════════════════════════════════════════
// CONNECTION POOLING
// ═══════════════════════════════════════════════════════════

export const CONNECTION_POOL_CONFIG = {
  database: {
    minConnections: 2,
    maxConnections: 50,
    acquireTimeout: 10_000,
    idleTimeout: 60_000,
    maxLifetime: 1800_000,   // 30 minutes
    statementTimeout: 30_000,
    healthCheckInterval: 30_000,
    retryAttempts: 3,
    retryDelay: 1000,
  },
  redis: {
    minConnections: 1,
    maxConnections: 20,
    connectTimeout: 5_000,
    commandTimeout: 5_000,
    retryAttempts: 3,
    retryDelay: 500,
    enableReadReplicas: true,
  },
  http: {
    maxSockets: 100,
    maxFreeSockets: 10,
    timeout: 30_000,
    keepAlive: true,
    keepAliveMsecs: 60_000,
  },
} as const;

// ═══════════════════════════════════════════════════════════
// PERFORMANCE BUDGETS (per endpoint)
// ═══════════════════════════════════════════════════════════

export const PERFORMANCE_BUDGETS = {
  // tRPC endpoints
  'trpc.auth.me': { p50: 50, p95: 150, p99: 300, maxRps: 1000 },
  'trpc.telemetry.ingest': { p50: 30, p95: 100, p99: 200, maxRps: 5000 },
  'trpc.telemetry.latest': { p50: 50, p95: 200, p99: 500, maxRps: 2000 },
  'trpc.poi.search': { p50: 100, p95: 500, p99: 1000, maxRps: 500 },
  'trpc.fleet.list': { p50: 80, p95: 300, p99: 600, maxRps: 500 },
  'trpc.incident.report': { p50: 100, p95: 300, p99: 500, maxRps: 200 },
  'trpc.collaboration.join': { p50: 50, p95: 150, p99: 300, maxRps: 100 },
  'trpc.analytics.query': { p50: 200, p95: 800, p99: 1500, maxRps: 100 },
  'trpc.payments.checkout': { p50: 500, p95: 2000, p99: 5000, maxRps: 50 },
  
  // WebSocket
  'ws.connect': { p50: 100, p95: 300, p99: 500, maxRps: 500 },
  'ws.message': { p50: 10, p95: 50, p99: 100, maxRps: 10000 },
  
  // Static assets
  'static.js': { p50: 50, p95: 200, p99: 500, maxRps: 2000 },
  'static.css': { p50: 30, p95: 100, p99: 200, maxRps: 2000 },
} as const;

// ═══════════════════════════════════════════════════════════
// LOAD TEST SCENARIOS (k6-compatible)
// ═══════════════════════════════════════════════════════════

export const LOAD_TEST_SCENARIOS = {
  smokeTest: {
    name: 'Smoke Test',
    description: 'Verify basic functionality under minimal load',
    stages: [
      { duration: '1m', target: 5 },
      { duration: '2m', target: 5 },
      { duration: '1m', target: 0 },
    ],
    thresholds: {
      http_req_duration: ['p(95)<500'],
      http_req_failed: ['rate<0.01'],
    },
  },
  loadTest: {
    name: 'Load Test',
    description: 'Normal production load simulation',
    stages: [
      { duration: '2m', target: 50 },
      { duration: '5m', target: 50 },
      { duration: '2m', target: 100 },
      { duration: '5m', target: 100 },
      { duration: '2m', target: 0 },
    ],
    thresholds: {
      http_req_duration: ['p(95)<1000', 'p(99)<2000'],
      http_req_failed: ['rate<0.05'],
      http_reqs: ['rate>100'],
    },
  },
  stressTest: {
    name: 'Stress Test',
    description: 'Find breaking point under increasing load',
    stages: [
      { duration: '2m', target: 100 },
      { duration: '5m', target: 200 },
      { duration: '5m', target: 500 },
      { duration: '5m', target: 1000 },
      { duration: '5m', target: 2000 },
      { duration: '5m', target: 0 },
    ],
    thresholds: {
      http_req_duration: ['p(95)<3000'],
      http_req_failed: ['rate<0.15'],
    },
  },
  spikeTest: {
    name: 'Spike Test',
    description: 'Sudden traffic spike simulation',
    stages: [
      { duration: '1m', target: 10 },
      { duration: '10s', target: 1000 },
      { duration: '3m', target: 1000 },
      { duration: '10s', target: 10 },
      { duration: '2m', target: 10 },
      { duration: '1m', target: 0 },
    ],
    thresholds: {
      http_req_duration: ['p(95)<5000'],
      http_req_failed: ['rate<0.20'],
    },
  },
  soakTest: {
    name: 'Soak Test',
    description: 'Extended duration test for memory leaks and degradation',
    stages: [
      { duration: '5m', target: 50 },
      { duration: '60m', target: 50 },
      { duration: '5m', target: 0 },
    ],
    thresholds: {
      http_req_duration: ['p(95)<1000'],
      http_req_failed: ['rate<0.02'],
    },
  },
  websocketTest: {
    name: 'WebSocket Load Test',
    description: 'Concurrent WebSocket connection stress test',
    stages: [
      { duration: '1m', target: 100 },
      { duration: '5m', target: 500 },
      { duration: '5m', target: 1000 },
      { duration: '2m', target: 0 },
    ],
    thresholds: {
      ws_connecting: ['p(95)<500'],
      ws_msgs_sent: ['rate>1000'],
    },
  },
} as const;

// ═══════════════════════════════════════════════════════════
// HEALTH CHECK CONTRACT
// ═══════════════════════════════════════════════════════════

export interface HealthCheckResult {
  status: 'healthy' | 'degraded' | 'unhealthy';
  timestamp: number;
  uptime: number;
  version: string;
  checks: {
    database: { status: 'up' | 'down'; latencyMs: number; poolSize: number; activeConnections: number };
    redis: { status: 'up' | 'down' | 'fallback'; latencyMs: number };
    memory: { heapUsed: number; heapTotal: number; rss: number; external: number };
    cpu: { user: number; system: number; loadAvg: [number, number, number] };
    disk: { used: number; total: number; percentUsed: number };
    websocket: { activeConnections: number; peakConnections: number };
    externalApis: {
      overpass: 'up' | 'down' | 'unknown';
      mapillary: 'up' | 'down' | 'unknown';
      googleMaps: 'up' | 'down' | 'unknown';
      stripe: 'up' | 'down' | 'unknown';
    };
  };
}

// ═══════════════════════════════════════════════════════════
// CIRCUIT BREAKER CONFIG
// ═══════════════════════════════════════════════════════════

export const CIRCUIT_BREAKER_CONFIG = {
  failureThreshold: 5,        // Failures before opening circuit
  successThreshold: 3,        // Successes before closing circuit
  timeout: 30_000,            // Time in open state before half-open
  monitorInterval: 10_000,    // Health check interval
  rollingWindow: 60_000,      // Window for failure counting
  services: {
    database: { failureThreshold: 3, timeout: 10_000 },
    redis: { failureThreshold: 5, timeout: 15_000 },
    overpass: { failureThreshold: 5, timeout: 30_000 },
    mapillary: { failureThreshold: 5, timeout: 30_000 },
    stripe: { failureThreshold: 3, timeout: 20_000 },
    googleMaps: { failureThreshold: 5, timeout: 30_000 },
  },
} as const;

// ═══════════════════════════════════════════════════════════
// DEPLOYMENT ARCHITECTURE
// ═══════════════════════════════════════════════════════════

export const DEPLOYMENT_ARCHITECTURE = {
  topology: 'horizontal-scaling',
  loadBalancer: {
    algorithm: 'least-connections',
    healthCheckPath: '/api/health',
    healthCheckInterval: 10,
    unhealthyThreshold: 3,
    stickySession: {
      enabled: true,
      cookieName: 'GANE_INSTANCE',
      ttl: 3600,
    },
  },
  instances: {
    web: {
      minReplicas: 1,
      maxReplicas: 10,
      cpu: '0.5-2 vCPU',
      memory: '512MB-2GB',
      autoscale: AUTO_SCALING_POLICIES.cpuBased,
    },
    worker: {
      minReplicas: 1,
      maxReplicas: 5,
      cpu: '1-4 vCPU',
      memory: '1GB-4GB',
      autoscale: AUTO_SCALING_POLICIES.requestBased,
    },
    websocket: {
      minReplicas: 1,
      maxReplicas: 5,
      cpu: '0.5-2 vCPU',
      memory: '512MB-2GB',
      autoscale: AUTO_SCALING_POLICIES.websocketBased,
    },
  },
  database: {
    type: 'TiDB Serverless',
    readReplicas: 2,
    connectionPool: CONNECTION_POOL_CONFIG.database,
    backup: {
      frequency: 'daily',
      retention: '30 days',
      pointInTimeRecovery: true,
    },
  },
  cdn: {
    provider: 'Cloudflare',
    cacheRules: [
      { path: '/assets/*', ttl: 31536000, immutable: true },
      { path: '/api/*', ttl: 0, noCache: true },
      { path: '/*.html', ttl: 300, staleWhileRevalidate: 60 },
    ],
  },
  monitoring: {
    metrics: ['Prometheus', 'Grafana'],
    logging: ['Structured JSON', 'Log aggregation'],
    tracing: ['OpenTelemetry'],
    alerting: ['PagerDuty', 'Slack', 'Email'],
  },
} as const;

// ═══════════════════════════════════════════════════════════
// K6 TEST SCRIPT GENERATOR
// ═══════════════════════════════════════════════════════════

export function generateK6Script(
  scenario: keyof typeof LOAD_TEST_SCENARIOS,
  baseUrl: string
): string {
  const config = LOAD_TEST_SCENARIOS[scenario];
  
  return `
// k6 Load Test: ${config.name}
// ${config.description}
// Generated by G.A.N.E Scaling Infrastructure

import http from 'k6/http';
import ws from 'k6/ws';
import { check, sleep } from 'k6';
import { Rate, Trend } from 'k6/metrics';

const errorRate = new Rate('errors');
const authLatency = new Trend('auth_latency');
const telemetryLatency = new Trend('telemetry_latency');
const poiLatency = new Trend('poi_latency');

export const options = {
  stages: ${JSON.stringify(config.stages, null, 4)},
  thresholds: ${JSON.stringify(config.thresholds, null, 4)},
};

const BASE_URL = '${baseUrl}';

export default function () {
  // 1. Health Check
  const health = http.get(\`\${BASE_URL}/api/health\`);
  check(health, { 'health OK': (r) => r.status === 200 });

  // 2. Auth Check
  const authStart = Date.now();
  const auth = http.get(\`\${BASE_URL}/api/trpc/auth.me\`, {
    headers: { 'Content-Type': 'application/json' },
  });
  authLatency.add(Date.now() - authStart);
  check(auth, { 'auth OK': (r) => r.status === 200 });

  // 3. Telemetry Ingest
  const telStart = Date.now();
  const telemetry = http.post(
    \`\${BASE_URL}/api/trpc/telemetry.ingest\`,
    JSON.stringify({
      lat: 32.0853 + Math.random() * 0.01,
      lon: 34.7818 + Math.random() * 0.01,
      speed: Math.random() * 120,
      heading: Math.random() * 360,
      accuracy: 5 + Math.random() * 20,
      timestamp: Date.now(),
    }),
    { headers: { 'Content-Type': 'application/json' } }
  );
  telemetryLatency.add(Date.now() - telStart);
  check(telemetry, { 'telemetry OK': (r) => r.status === 200 });

  // 4. POI Search
  const poiStart = Date.now();
  const poi = http.get(
    \`\${BASE_URL}/api/trpc/poi.search?input=\${encodeURIComponent(JSON.stringify({
      lat: 32.0853,
      lon: 34.7818,
      radiusMeters: 1000,
      limit: 20,
    }))}\`
  );
  poiLatency.add(Date.now() - poiStart);
  check(poi, { 'poi OK': (r) => r.status === 200 });

  errorRate.add(health.status !== 200 || auth.status !== 200);

  sleep(1 + Math.random() * 2);
}
`.trim();
}
