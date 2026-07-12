/**
 * Benchmark Harness — Automated Performance Regression Testing
 * Spec reference: P1 System Gaps
 *
 * Features:
 *   - Define benchmarks with SLO targets
 *   - Run benchmarks with statistical analysis (mean, p50, p95, p99, stddev)
 *   - Compare against baselines for regression detection
 *   - Trend analysis over time
 *   - Automated suite execution with pass/fail reporting
 *   - JSON-serializable results for persistence
 */

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

export type BenchmarkStatus = 'pending' | 'running' | 'passed' | 'failed' | 'error' | 'skipped';
export type SLOUnit = 'ms' | 's' | 'fps' | 'bytes' | 'percent' | 'count';
export type RegressionSeverity = 'none' | 'minor' | 'major' | 'critical';

/** Service Level Objective definition */
export interface SLO {
  name: string;
  target: number;
  unit: SLOUnit;
  comparison: 'lt' | 'lte' | 'gt' | 'gte' | 'eq';
  description?: string;
}

/** A single benchmark definition */
export interface BenchmarkDef {
  id: string;
  name: string;
  nameHe?: string;
  category: string;
  description?: string;
  fn: () => Promise<number> | number;    // returns the measured value
  slo: SLO;
  warmupRuns?: number;
  iterations?: number;
  timeoutMs?: number;
  tags?: string[];
  enabled?: boolean;
}

/** Statistical summary of benchmark runs */
export interface BenchmarkStats {
  mean: number;
  median: number;
  p50: number;
  p95: number;
  p99: number;
  min: number;
  max: number;
  stddev: number;
  samples: number;
  outliers: number;
}

/** Result of a single benchmark */
export interface BenchmarkResult {
  id: string;
  name: string;
  category: string;
  status: BenchmarkStatus;
  stats: BenchmarkStats;
  slo: SLO;
  sloMet: boolean;
  rawValues: number[];
  timestamp: number;
  durationMs: number;
  error?: string;
}

/** Baseline snapshot for regression comparison */
export interface Baseline {
  id: string;
  benchmarkId: string;
  stats: BenchmarkStats;
  timestamp: number;
  label: string;
  commitHash?: string;
}

/** Regression analysis result */
export interface RegressionResult {
  benchmarkId: string;
  benchmarkName: string;
  severity: RegressionSeverity;
  currentMean: number;
  baselineMean: number;
  changePercent: number;
  sloStillMet: boolean;
  details: string;
}

/** Suite execution result */
export interface SuiteResult {
  suiteId: string;
  suiteName: string;
  startedAt: number;
  completedAt: number;
  totalDurationMs: number;
  results: BenchmarkResult[];
  regressions: RegressionResult[];
  passed: number;
  failed: number;
  errors: number;
  skipped: number;
  overallStatus: BenchmarkStatus;
}

/** Trend data point */
export interface TrendPoint {
  timestamp: number;
  mean: number;
  p95: number;
  sloMet: boolean;
  label?: string;
}

/** Harness configuration */
export interface HarnessConfig {
  defaultIterations: number;
  defaultWarmupRuns: number;
  defaultTimeoutMs: number;
  regressionThresholdPercent: number;    // % increase to flag regression
  criticalRegressionPercent: number;     // % increase for critical
  outlierZScore: number;                 // z-score threshold for outlier removal
  maxHistorySize: number;               // max trend points per benchmark
  enableConsoleOutput: boolean;
}

// ═══════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════

const DEFAULTS: HarnessConfig = {
  defaultIterations: 100,
  defaultWarmupRuns: 5,
  defaultTimeoutMs: 10_000,
  regressionThresholdPercent: 15,
  criticalRegressionPercent: 50,
  outlierZScore: 2.5,
  maxHistorySize: 500,
  enableConsoleOutput: false,
};

// ═══════════════════════════════════════════════════════════
// STATISTICAL HELPERS
// ═══════════════════════════════════════════════════════════

function computeStats(values: number[], outlierZScore: number): BenchmarkStats {
  if (values.length === 0) {
    return { mean: 0, median: 0, p50: 0, p95: 0, p99: 0, min: 0, max: 0, stddev: 0, samples: 0, outliers: 0 };
  }

  const sorted = [...values].sort((a, b) => a - b);
  const n = sorted.length;

  // Basic stats
  const sum = sorted.reduce((a, b) => a + b, 0);
  const mean = sum / n;
  const variance = sorted.reduce((acc, v) => acc + (v - mean) ** 2, 0) / n;
  const stddev = Math.sqrt(variance);

  // Remove outliers
  let filtered = sorted;
  let outliers = 0;
  if (stddev > 0 && n > 10) {
    filtered = sorted.filter(v => {
      const z = Math.abs((v - mean) / stddev);
      if (z > outlierZScore) { outliers++; return false; }
      return true;
    });
  }

  const fn = filtered.length;
  const fSum = filtered.reduce((a, b) => a + b, 0);
  const fMean = fn > 0 ? fSum / fn : mean;
  const fVariance = fn > 0 ? filtered.reduce((acc, v) => acc + (v - fMean) ** 2, 0) / fn : variance;

  return {
    mean: fMean,
    median: percentile(filtered, 50),
    p50: percentile(filtered, 50),
    p95: percentile(filtered, 95),
    p99: percentile(filtered, 99),
    min: filtered[0] ?? 0,
    max: filtered[fn - 1] ?? 0,
    stddev: Math.sqrt(fVariance),
    samples: fn,
    outliers,
  };
}

function percentile(sorted: number[], p: number): number {
  if (sorted.length === 0) return 0;
  const idx = (p / 100) * (sorted.length - 1);
  const lower = Math.floor(idx);
  const upper = Math.ceil(idx);
  if (lower === upper) return sorted[lower];
  return sorted[lower] + (sorted[upper] - sorted[lower]) * (idx - lower);
}

function checkSLO(value: number, slo: SLO): boolean {
  switch (slo.comparison) {
    case 'lt': return value < slo.target;
    case 'lte': return value <= slo.target;
    case 'gt': return value > slo.target;
    case 'gte': return value >= slo.target;
    case 'eq': return Math.abs(value - slo.target) < 0.001;
    default: return false;
  }
}

// ═══════════════════════════════════════════════════════════
// BUILT-IN BENCHMARKS (G.A.N.E SLOs)
// ═══════════════════════════════════════════════════════════

export function createGANEBenchmarks(): BenchmarkDef[] {
  return [
    {
      id: 'slo-position-fix',
      name: 'Position Fix Latency',
      nameHe: 'זמן תיקון מיקום',
      category: 'navigation',
      description: 'Time to compute a fused position fix from GNSS+IMU+Vision',
      fn: async () => {
        const start = performance.now();
        // Simulate ESKF position fix computation
        const state = new Float64Array(16);
        for (let i = 0; i < 16; i++) state[i] = Math.random();
        // Matrix multiplication simulation (15x15 covariance update)
        const P = new Float64Array(225);
        const F = new Float64Array(225);
        for (let i = 0; i < 225; i++) { P[i] = Math.random(); F[i] = Math.random(); }
        // P = F * P * F^T
        for (let i = 0; i < 15; i++) {
          for (let j = 0; j < 15; j++) {
            let sum = 0;
            for (let k = 0; k < 15; k++) sum += F[i * 15 + k] * P[k * 15 + j];
            P[i * 15 + j] = sum;
          }
        }
        return performance.now() - start;
      },
      slo: { name: 'Position Fix', target: 1000, unit: 'ms', comparison: 'lt', description: 'Must complete in <1s' },
      iterations: 200,
      warmupRuns: 10,
      tags: ['core', 'navigation'],
    },
    {
      id: 'slo-reroute-latency',
      name: 'Reroute Computation',
      nameHe: 'חישוב מסלול חלופי',
      category: 'routing',
      description: 'Time to compute an alternative route on deviation',
      fn: async () => {
        const start = performance.now();
        // Simulate A* pathfinding on a 1000-node graph
        const nodes = 1000;
        const distances = new Float64Array(nodes);
        const visited = new Uint8Array(nodes);
        distances.fill(Infinity);
        distances[0] = 0;
        for (let step = 0; step < nodes; step++) {
          let minDist = Infinity, minNode = -1;
          for (let i = 0; i < nodes; i++) {
            if (!visited[i] && distances[i] < minDist) { minDist = distances[i]; minNode = i; }
          }
          if (minNode === -1) break;
          visited[minNode] = 1;
          // Relax neighbors (simulate 4-6 edges per node)
          const edgeCount = 4 + Math.floor(Math.random() * 3);
          for (let e = 0; e < edgeCount; e++) {
            const neighbor = Math.floor(Math.random() * nodes);
            const weight = Math.random() * 10 + 1;
            if (distances[minNode] + weight < distances[neighbor]) {
              distances[neighbor] = distances[minNode] + weight;
            }
          }
        }
        return performance.now() - start;
      },
      slo: { name: 'Reroute', target: 500, unit: 'ms', comparison: 'lt', description: 'Must complete in <500ms' },
      iterations: 50,
      warmupRuns: 5,
      tags: ['core', 'routing'],
    },
    {
      id: 'slo-alert-delivery',
      name: 'Alert Delivery Latency',
      nameHe: 'זמן מסירת התראה',
      category: 'alerts',
      description: 'Time from alert trigger to UI notification',
      fn: async () => {
        const start = performance.now();
        // Simulate alert processing pipeline
        const alert = {
          id: Math.random().toString(36),
          priority: 'high',
          message: 'Test alert',
          timestamp: Date.now(),
        };
        // Priority queue insertion
        const queue: typeof alert[] = [];
        for (let i = 0; i < 100; i++) {
          queue.push({ ...alert, id: Math.random().toString(36) });
        }
        queue.sort((a, b) => a.timestamp - b.timestamp);
        // Deduplication check
        const seen = new Set<string>();
        const deduped = queue.filter(a => {
          if (seen.has(a.id)) return false;
          seen.add(a.id);
          return true;
        });
        // Template rendering
        const rendered = deduped.map(a => `[${a.priority}] ${a.message}`);
        void rendered;
        return performance.now() - start;
      },
      slo: { name: 'Alert Delivery', target: 300, unit: 'ms', comparison: 'lt', description: 'Must complete in <300ms' },
      iterations: 100,
      warmupRuns: 10,
      tags: ['core', 'alerts'],
    },
    {
      id: 'slo-eta-update',
      name: 'ETA Recalculation',
      nameHe: 'עדכון זמן הגעה',
      category: 'navigation',
      description: 'Time to recalculate ETA based on current conditions',
      fn: async () => {
        const start = performance.now();
        // Simulate per-segment ETA calculation
        const segments = 50;
        let totalEta = 0;
        for (let i = 0; i < segments; i++) {
          const distanceM = 200 + Math.random() * 800;
          const baseSpeed = 30 + Math.random() * 90; // km/h
          const trafficFactor = 0.5 + Math.random() * 0.5;
          const weatherFactor = 0.8 + Math.random() * 0.2;
          const effectiveSpeed = baseSpeed * trafficFactor * weatherFactor;
          totalEta += (distanceM / 1000) / effectiveSpeed * 3600;
        }
        void totalEta;
        return performance.now() - start;
      },
      slo: { name: 'ETA Update', target: 200, unit: 'ms', comparison: 'lt', description: 'Must complete in <200ms' },
      iterations: 200,
      warmupRuns: 10,
      tags: ['core', 'navigation'],
    },
    {
      id: 'slo-telemetry-ingest',
      name: 'Telemetry Ingestion',
      nameHe: 'קליטת טלמטריה',
      category: 'data',
      description: 'Time to process and validate a telemetry packet',
      fn: async () => {
        const start = performance.now();
        // Simulate telemetry validation + storage
        const packet = {
          deviceId: 'dev-' + Math.random().toString(36).slice(2, 8),
          lat: 31.5 + Math.random() * 2,
          lon: 34 + Math.random() * 2,
          speed: Math.random() * 120,
          heading: Math.random() * 360,
          altitude: Math.random() * 1000,
          accuracy: 1 + Math.random() * 20,
          timestamp: Date.now(),
          sensors: {
            accelerometer: [Math.random(), Math.random(), Math.random()],
            gyroscope: [Math.random(), Math.random(), Math.random()],
            magnetometer: [Math.random(), Math.random(), Math.random()],
          },
        };
        // Validation
        const valid = packet.lat >= -90 && packet.lat <= 90 && packet.lon >= -180 && packet.lon <= 180;
        // Coordinate transform
        const rad = packet.lat * Math.PI / 180;
        const x = Math.cos(rad) * Math.cos(packet.lon * Math.PI / 180);
        const y = Math.cos(rad) * Math.sin(packet.lon * Math.PI / 180);
        const z = Math.sin(rad);
        void [valid, x, y, z];
        return performance.now() - start;
      },
      slo: { name: 'Telemetry Ingest', target: 50, unit: 'ms', comparison: 'lt', description: 'Must complete in <50ms' },
      iterations: 500,
      warmupRuns: 20,
      tags: ['core', 'data'],
    },
    {
      id: 'slo-policy-eval',
      name: 'Policy Evaluation',
      nameHe: 'הערכת מדיניות',
      category: 'policy',
      description: 'Time to evaluate all routing policies against a trip context',
      fn: async () => {
        const start = performance.now();
        // Simulate evaluating 50 policy rules
        const rules = 50;
        let matchCount = 0;
        for (let i = 0; i < rules; i++) {
          const conditions = 3 + Math.floor(Math.random() * 5);
          let allMatch = true;
          for (let c = 0; c < conditions; c++) {
            const fieldValue = Math.random() * 100;
            const threshold = Math.random() * 100;
            if (fieldValue > threshold) allMatch = false;
          }
          if (allMatch) matchCount++;
        }
        void matchCount;
        return performance.now() - start;
      },
      slo: { name: 'Policy Eval', target: 50, unit: 'ms', comparison: 'lt', description: 'Must complete in <50ms' },
      iterations: 200,
      warmupRuns: 10,
      tags: ['core', 'policy'],
    },
  ];
}

// ═══════════════════════════════════════════════════════════
// BENCHMARK HARNESS
// ═══════════════════════════════════════════════════════════

export class BenchmarkHarness {
  private config: HarnessConfig;
  private benchmarks: Map<string, BenchmarkDef> = new Map();
  private baselines: Map<string, Baseline> = new Map();
  private history: Map<string, TrendPoint[]> = new Map();
  private suiteResults: SuiteResult[] = [];
  private listeners = new Map<string, Set<(data: unknown) => void>>();

  constructor(config: Partial<HarnessConfig> = {}) {
    this.config = { ...DEFAULTS, ...config };
  }

  // ── Benchmark Registration ───────────────────────────

  register(def: BenchmarkDef): void {
    this.benchmarks.set(def.id, { ...def, enabled: def.enabled !== false });
  }

  registerAll(defs: BenchmarkDef[]): void {
    for (const def of defs) this.register(def);
  }

  unregister(id: string): boolean {
    return this.benchmarks.delete(id);
  }

  getBenchmark(id: string): BenchmarkDef | undefined {
    return this.benchmarks.get(id);
  }

  getAllBenchmarks(): BenchmarkDef[] {
    return Array.from(this.benchmarks.values());
  }

  getCategories(): string[] {
    const cats = new Set<string>();
    for (const b of Array.from(this.benchmarks.values())) cats.add(b.category);
    return Array.from(cats);
  }

  // ── Running Benchmarks ───────────────────────────────

  async runOne(id: string): Promise<BenchmarkResult> {
    const def = this.benchmarks.get(id);
    if (!def) throw new Error(`Benchmark not found: ${id}`);
    if (!def.enabled) {
      return {
        id: def.id, name: def.name, category: def.category,
        status: 'skipped', stats: computeStats([], this.config.outlierZScore),
        slo: def.slo, sloMet: false, rawValues: [],
        timestamp: Date.now(), durationMs: 0,
      };
    }

    const start = performance.now();
    const iterations = def.iterations || this.config.defaultIterations;
    const warmup = def.warmupRuns || this.config.defaultWarmupRuns;
    const timeout = def.timeoutMs || this.config.defaultTimeoutMs;

    this.emit('benchmark:start', { id: def.id, name: def.name });

    try {
      // Warmup
      for (let i = 0; i < warmup; i++) {
        await Promise.resolve(def.fn());
      }

      // Measured runs
      const values: number[] = [];
      const deadline = performance.now() + timeout;

      for (let i = 0; i < iterations; i++) {
        if (performance.now() > deadline) break;
        const value = await Promise.resolve(def.fn());
        values.push(value);
      }

      const stats = computeStats(values, this.config.outlierZScore);
      const sloMet = checkSLO(stats.p95, def.slo);
      const status: BenchmarkStatus = sloMet ? 'passed' : 'failed';

      const result: BenchmarkResult = {
        id: def.id, name: def.name, category: def.category,
        status, stats, slo: def.slo, sloMet, rawValues: values,
        timestamp: Date.now(), durationMs: performance.now() - start,
      };

      // Record trend
      this.addTrendPoint(def.id, stats, sloMet);
      this.emit('benchmark:complete', result);

      return result;
    } catch (err) {
      const result: BenchmarkResult = {
        id: def.id, name: def.name, category: def.category,
        status: 'error', stats: computeStats([], this.config.outlierZScore),
        slo: def.slo, sloMet: false, rawValues: [],
        timestamp: Date.now(), durationMs: performance.now() - start,
        error: err instanceof Error ? err.message : String(err),
      };
      this.emit('benchmark:error', result);
      return result;
    }
  }

  async runCategory(category: string): Promise<BenchmarkResult[]> {
    const benchmarks = Array.from(this.benchmarks.values()).filter(b => b.category === category);
    const results: BenchmarkResult[] = [];
    for (const b of Array.from(benchmarks)) {
      results.push(await this.runOne(b.id));
    }
    return results;
  }

  async runAll(): Promise<SuiteResult> {
    const suiteId = `suite-${Date.now()}`;
    const startedAt = Date.now();

    this.emit('suite:start', { suiteId });

    const results: BenchmarkResult[] = [];
    for (const b of Array.from(this.benchmarks.values())) {
      results.push(await this.runOne(b.id));
    }

    // Check regressions against baselines
    const regressions = this.detectRegressions(results);

    const passed = results.filter(r => r.status === 'passed').length;
    const failed = results.filter(r => r.status === 'failed').length;
    const errors = results.filter(r => r.status === 'error').length;
    const skipped = results.filter(r => r.status === 'skipped').length;

    const suiteResult: SuiteResult = {
      suiteId,
      suiteName: 'G.A.N.E Performance Suite',
      startedAt,
      completedAt: Date.now(),
      totalDurationMs: Date.now() - startedAt,
      results,
      regressions,
      passed, failed, errors, skipped,
      overallStatus: failed > 0 || errors > 0 ? 'failed' : 'passed',
    };

    this.suiteResults.push(suiteResult);
    this.emit('suite:complete', suiteResult);

    if (this.config.enableConsoleOutput) {
      this.printReport(suiteResult);
    }

    return suiteResult;
  }

  // ── Baselines ────────────────────────────────────────

  setBaseline(benchmarkId: string, label: string, commitHash?: string): Baseline | null {
    const trend = this.history.get(benchmarkId);
    if (!trend || trend.length === 0) return null;

    const lastPoint = trend[trend.length - 1];
    const baseline: Baseline = {
      id: `baseline-${Date.now()}`,
      benchmarkId,
      stats: {
        mean: lastPoint.mean,
        median: lastPoint.mean,
        p50: lastPoint.mean,
        p95: lastPoint.p95,
        p99: lastPoint.p95 * 1.1,
        min: lastPoint.mean * 0.5,
        max: lastPoint.p95 * 1.5,
        stddev: (lastPoint.p95 - lastPoint.mean) / 2,
        samples: 0,
        outliers: 0,
      },
      timestamp: Date.now(),
      label,
      commitHash,
    };

    this.baselines.set(benchmarkId, baseline);
    this.emit('baseline:set', baseline);
    return baseline;
  }

  setBaselineFromResult(result: BenchmarkResult, label: string, commitHash?: string): Baseline {
    const baseline: Baseline = {
      id: `baseline-${Date.now()}`,
      benchmarkId: result.id,
      stats: result.stats,
      timestamp: Date.now(),
      label,
      commitHash,
    };
    this.baselines.set(result.id, baseline);
    return baseline;
  }

  getBaseline(benchmarkId: string): Baseline | undefined {
    return this.baselines.get(benchmarkId);
  }

  clearBaselines(): void {
    this.baselines.clear();
  }

  // ── Regression Detection ─────────────────────────────

  detectRegressions(results: BenchmarkResult[]): RegressionResult[] {
    const regressions: RegressionResult[] = [];

    for (const result of results) {
      if (result.status === 'skipped' || result.status === 'error') continue;

      const baseline = this.baselines.get(result.id);
      if (!baseline) continue;

      const changePercent = baseline.stats.mean > 0
        ? ((result.stats.mean - baseline.stats.mean) / baseline.stats.mean) * 100
        : 0;

      let severity: RegressionSeverity = 'none';
      if (changePercent > this.config.criticalRegressionPercent) {
        severity = 'critical';
      } else if (changePercent > this.config.regressionThresholdPercent) {
        severity = 'major';
      } else if (changePercent > this.config.regressionThresholdPercent / 2) {
        severity = 'minor';
      }

      if (severity !== 'none') {
        regressions.push({
          benchmarkId: result.id,
          benchmarkName: result.name,
          severity,
          currentMean: result.stats.mean,
          baselineMean: baseline.stats.mean,
          changePercent: Math.round(changePercent * 100) / 100,
          sloStillMet: result.sloMet,
          details: `${result.name}: ${severity} regression (${changePercent > 0 ? '+' : ''}${changePercent.toFixed(1)}% from baseline)`,
        });
      }
    }

    return regressions;
  }

  // ── Trend Analysis ───────────────────────────────────

  getTrend(benchmarkId: string): TrendPoint[] {
    return this.history.get(benchmarkId) || [];
  }

  getAllTrends(): Map<string, TrendPoint[]> {
    return new Map(this.history);
  }

  clearHistory(): void {
    this.history.clear();
  }

  // ── Suite History ────────────────────────────────────

  getSuiteResults(): SuiteResult[] {
    return [...this.suiteResults];
  }

  getLastSuiteResult(): SuiteResult | undefined {
    return this.suiteResults[this.suiteResults.length - 1];
  }

  // ── Report ───────────────────────────────────────────

  generateReport(suiteResult: SuiteResult): string {
    const lines: string[] = [
      `# G.A.N.E Performance Report`,
      ``,
      `**Suite:** ${suiteResult.suiteName}`,
      `**Status:** ${suiteResult.overallStatus.toUpperCase()}`,
      `**Duration:** ${suiteResult.totalDurationMs.toFixed(0)}ms`,
      `**Results:** ${suiteResult.passed} passed, ${suiteResult.failed} failed, ${suiteResult.errors} errors, ${suiteResult.skipped} skipped`,
      ``,
      `## SLO Results`,
      ``,
      `| Benchmark | Mean | P95 | P99 | SLO Target | Status |`,
      `|-----------|------|-----|-----|------------|--------|`,
    ];

    for (const r of suiteResult.results) {
      if (r.status === 'skipped') continue;
      const status = r.sloMet ? 'PASS' : 'FAIL';
      lines.push(`| ${r.name} | ${r.stats.mean.toFixed(2)}${r.slo.unit} | ${r.stats.p95.toFixed(2)}${r.slo.unit} | ${r.stats.p99.toFixed(2)}${r.slo.unit} | ${r.slo.comparison}${r.slo.target}${r.slo.unit} | ${status} |`);
    }

    if (suiteResult.regressions.length > 0) {
      lines.push(``, `## Regressions`, ``);
      for (const reg of suiteResult.regressions) {
        lines.push(`- **${reg.severity.toUpperCase()}**: ${reg.details}`);
      }
    }

    return lines.join('\n');
  }

  // ── Events ───────────────────────────────────────────

  on(event: string, handler: (data: unknown) => void): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(handler);
    return () => { this.listeners.get(event)?.delete(handler); };
  }

  // ── Cleanup ──────────────────────────────────────────

  destroy(): void {
    this.benchmarks.clear();
    this.baselines.clear();
    this.history.clear();
    this.suiteResults = [];
    this.listeners.clear();
  }

  // ── Private ──────────────────────────────────────────

  private addTrendPoint(benchmarkId: string, stats: BenchmarkStats, sloMet: boolean): void {
    if (!this.history.has(benchmarkId)) {
      this.history.set(benchmarkId, []);
    }
    const trend = this.history.get(benchmarkId)!;
    trend.push({
      timestamp: Date.now(),
      mean: stats.mean,
      p95: stats.p95,
      sloMet,
    });
    if (trend.length > this.config.maxHistorySize) {
      trend.splice(0, trend.length - this.config.maxHistorySize);
    }
  }

  private printReport(suite: SuiteResult): void {
    console.log(`\n${'═'.repeat(60)}`);
    console.log(`  G.A.N.E BENCHMARK SUITE — ${suite.overallStatus.toUpperCase()}`);
    console.log(`${'═'.repeat(60)}`);
    for (const r of suite.results) {
      const icon = r.status === 'passed' ? '[PASS]' : r.status === 'failed' ? '[FAIL]' : r.status === 'error' ? '[ERR]' : '[SKIP]';
      console.log(`  ${icon} ${r.name}: mean=${r.stats.mean.toFixed(2)}${r.slo.unit} p95=${r.stats.p95.toFixed(2)}${r.slo.unit} (target: ${r.slo.comparison}${r.slo.target}${r.slo.unit})`);
    }
    console.log(`${'─'.repeat(60)}`);
    console.log(`  ${suite.passed} passed | ${suite.failed} failed | ${suite.errors} errors | ${suite.skipped} skipped | ${suite.totalDurationMs.toFixed(0)}ms`);
    console.log(`${'═'.repeat(60)}\n`);
  }

  private emit(event: string, data: unknown): void {
    const handlers = this.listeners.get(event);
    if (handlers) {
      for (const handler of Array.from(handlers)) {
        try { handler(data); } catch { /* swallow */ }
      }
    }
  }
}

// ═══════════════════════════════════════════════════════════
// SINGLETON
// ═══════════════════════════════════════════════════════════

let _instance: BenchmarkHarness | null = null;

export function getBenchmarkHarness(config?: Partial<HarnessConfig>): BenchmarkHarness {
  if (!_instance) {
    _instance = new BenchmarkHarness(config);
    _instance.registerAll(createGANEBenchmarks());
  }
  return _instance;
}

export function resetBenchmarkHarness(): void {
  _instance?.destroy();
  _instance = null;
}
