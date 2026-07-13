/**
 * Tests for Policy Engine, Multi-Provider Routing, and Benchmark Harness
 */
import { describe, it, expect, beforeEach } from 'vitest';

// ── Policy Engine Tests ─────────────────────────────────────
describe('PolicyEngine', () => {
  let PolicyEngine: any;
  let parseDSL: any;

  beforeEach(async () => {
    const mod = await import('../../client/src/engine/policyEngine');
    PolicyEngine = mod.PolicyEngine;
    parseDSL = mod.parseDSL;
  });

  // Helper to create a valid PolicyContext
  function makeContext(overrides: Record<string, any> = {}): any {
    return {
      vehicle: { class: 'car' as const, weight: 1500 },
      route: {
        origin: { lat: 32.08, lon: 34.78 },
        destination: { lat: 32.07, lon: 34.77 },
        distanceKm: 5,
        zones: ['zone-1'],
      },
      driver: { id: 'drv-1', score: 80 },
      time: { hour: 10, dayOfWeek: 2 },
      environment: { weather: 'clear' },
      ...overrides,
    };
  }

  it('initializes with built-in compliance rules', () => {
    const engine = new PolicyEngine();
    expect(engine).toBeDefined();
    // Engine comes with 6 built-in compliance rules
    expect(engine.getAllRules().length).toBe(6);
  });

  it('adds and retrieves rules', () => {
    const engine = new PolicyEngine();
    const initialCount = engine.getAllRules().length;
    const rule = {
      id: 'test-rule-1',
      name: 'Speed Limit',
      description: 'Limit speed in school zones',
      priority: 'high' as const,
      enabled: true,
      version: 1,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      conditions: {
        logic: 'and' as const,
        conditions: [
          { field: 'environment.weather', op: 'eq' as const, value: 'storm' },
        ],
      },
      actions: [
        { type: 'alert' as const, message: 'Storm warning' },
      ],
      tags: ['safety', 'school'],
      region: 'IL' as const,
      vehicleClasses: ['car' as const],
    };
    engine.addRule(rule);
    expect(engine.getRule('test-rule-1')).toBeDefined();
    expect(engine.getRule('test-rule-1')!.name).toBe('Speed Limit');
    expect(engine.getAllRules()).toHaveLength(initialCount + 1);
  });

  it('removes rules', () => {
    const engine = new PolicyEngine();
    engine.addRule({
      id: 'remove-me',
      name: 'Temp',
      priority: 'low',
      enabled: true,
      version: 1,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      conditions: { logic: 'and', conditions: [] },
      actions: [],
    });
    expect(engine.removeRule('remove-me')).toBe(true);
    expect(engine.getRule('remove-me')).toBeUndefined();
    expect(engine.removeRule('nonexistent')).toBe(false);
  });

  it('enables and disables rules', () => {
    const engine = new PolicyEngine();
    engine.addRule({
      id: 'toggle-rule',
      name: 'Toggle',
      priority: 'medium',
      enabled: true,
      version: 1,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      conditions: { logic: 'and', conditions: [] },
      actions: [],
    });
    expect(engine.disableRule('toggle-rule')).toBe(true);
    expect(engine.getRule('toggle-rule')!.enabled).toBe(false);
    expect(engine.enableRule('toggle-rule')).toBe(true);
    expect(engine.getRule('toggle-rule')!.enabled).toBe(true);
  });

  it('evaluates rules against context', () => {
    const engine = new PolicyEngine();
    engine.addRule({
      id: 'storm-check',
      name: 'Storm Check',
      priority: 'high',
      enabled: true,
      version: 1,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      conditions: {
        logic: 'and',
        conditions: [
          { field: 'environment.weather', op: 'eq', value: 'storm' },
        ],
      },
      actions: [
        { type: 'alert', message: 'Storm detected' },
      ],
    });

    const result = engine.evaluate(makeContext({ environment: { weather: 'storm' } }));
    expect(result).toBeDefined();
    expect(typeof result.allowed).toBe('boolean');
    expect(typeof result.evaluationMs).toBe('number');
    expect(result.ruleCount).toBeGreaterThan(0);
    // The storm-check rule should match
    expect(result.matchCount).toBeGreaterThanOrEqual(1);
    expect(result.alerts.length).toBeGreaterThanOrEqual(1);
  });

  it('handles emergency override', () => {
    const engine = new PolicyEngine();
    engine.activateEmergencyOverride('Emergency evacuation');
    expect(engine.isEmergencyOverrideActive()).toBe(true);

    const result = engine.evaluate(makeContext());
    // Emergency override should bypass all rules
    expect(result.allowed).toBe(true);
    expect(result.modifications.emergencyOverride).toBe(true);

    engine.deactivateEmergencyOverride();
    expect(engine.isEmergencyOverrideActive()).toBe(false);
  });

  it('filters rules by priority', () => {
    const engine = new PolicyEngine();
    // Clear default rules by hot-reloading empty
    engine.hotReload([]);
    const priorities = ['emergency', 'critical', 'high', 'medium', 'low'] as const;
    priorities.forEach((p) => {
      engine.addRule({
        id: `rule-${p}`,
        name: `Rule ${p}`,
        priority: p,
        enabled: true,
        version: 1,
        createdAt: Date.now(),
        updatedAt: Date.now(),
        conditions: { logic: 'and', conditions: [] },
        actions: [],
      });
    });

    expect(engine.getRulesByPriority('emergency')).toHaveLength(1);
    expect(engine.getRulesByPriority('high')).toHaveLength(1);
    expect(engine.getAllRules()).toHaveLength(5);
  });

  it('filters rules by tag', () => {
    const engine = new PolicyEngine();
    engine.hotReload([]);
    engine.addRule({
      id: 'tagged-1',
      name: 'Tagged',
      priority: 'medium',
      enabled: true,
      version: 1,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      conditions: { logic: 'and', conditions: [] },
      actions: [],
      tags: ['safety', 'school'],
    });
    engine.addRule({
      id: 'tagged-2',
      name: 'Tagged 2',
      priority: 'medium',
      enabled: true,
      version: 1,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      conditions: { logic: 'and', conditions: [] },
      actions: [],
      tags: ['compliance'],
    });

    expect(engine.getRulesByTag('safety')).toHaveLength(1);
    expect(engine.getRulesByTag('compliance')).toHaveLength(1);
    expect(engine.getRulesByTag('nonexistent')).toHaveLength(0);
  });

  it('filters rules by region', () => {
    const engine = new PolicyEngine();
    engine.hotReload([]);
    engine.addRule({
      id: 'eu-rule',
      name: 'EU Rule',
      priority: 'medium',
      enabled: true,
      version: 1,
      createdAt: Date.now(),
      updatedAt: Date.now(),
      conditions: { logic: 'and', conditions: [] },
      actions: [],
      region: 'EU',
    });

    expect(engine.getRulesByRegion('EU')).toHaveLength(1);
    expect(engine.getRulesByRegion('US')).toHaveLength(0);
  });

  it('hot-reloads rules', () => {
    const engine = new PolicyEngine();
    engine.hotReload([
      {
        id: 'new-rule',
        name: 'New',
        priority: 'high',
        enabled: true,
        version: 1,
        createdAt: Date.now(),
        updatedAt: Date.now(),
        conditions: { logic: 'and', conditions: [] },
        actions: [],
      },
    ]);

    expect(engine.getRule('new-rule')).toBeDefined();
    expect(engine.getAllRules()).toHaveLength(1);
  });

  it('maintains audit log', () => {
    const engine = new PolicyEngine({ auditEnabled: true });
    engine.evaluate(makeContext());

    const log = engine.getAuditLog();
    expect(log.length).toBeGreaterThanOrEqual(1);

    engine.clearAuditLog();
    expect(engine.getAuditLog()).toHaveLength(0);
  });

  it('parseDSL creates a valid rule from DSL string', () => {
    const dsl = `
RULE speed_limit
WHEN environment.weather eq storm
ACTION alert Storm warning
PRIORITY high
TAGS safety,weather
REGION IL
    `;
    const rule = parseDSL(dsl);
    expect(rule).toBeDefined();
    expect(rule.id).toBeDefined();
    expect(rule.priority).toBe('high');
    expect(rule.tags).toContain('safety');
    expect(rule.tags).toContain('weather');
  });
});

// ── Multi-Provider Routing Tests ────────────────────────────
describe('MultiProviderRouter', () => {
  let MultiProviderRouter: any;

  beforeEach(async () => {
    const mod = await import('../../client/src/engine/multiProviderRouting');
    MultiProviderRouter = mod.MultiProviderRouter;
  });

  it('initializes with default providers', () => {
    const router = new MultiProviderRouter();
    expect(router).toBeDefined();
    const health = router.getAllProviderHealth();
    expect(health.length).toBeGreaterThanOrEqual(1);
  });

  it('routes with internal provider (no API key needed)', async () => {  // OSM data loading may take time
    const router = new MultiProviderRouter();
    const result = await router.route({
      origin: { lat: 32.0853, lon: 34.7818 },
      destination: { lat: 32.0700, lon: 34.7700 },
      profile: 'driving',
    });

    expect(result).toBeDefined();
    expect(result.primary).toBeDefined();
    expect(result.primary.legs.length).toBeGreaterThanOrEqual(1);
    expect(result.primary.totalDurationS).toBeGreaterThan(0);
    expect(result.primary.totalDistanceM).toBeGreaterThan(0);
    expect(result.providerUsed).toBeDefined();
  }, 30000);

  it('enables and disables providers', () => {
    const router = new MultiProviderRouter();
    router.disableProvider('mapbox');
    const health = router.getProviderHealth('mapbox');
    expect(health?.status).toBe('disabled');

    router.enableProvider('mapbox', 'test-key');
    const health2 = router.getProviderHealth('mapbox');
    expect(health2?.status).not.toBe('disabled');
  });

  it('returns provider health information', () => {
    const router = new MultiProviderRouter();
    const health = router.getAllProviderHealth();
    expect(Array.isArray(health)).toBe(true);
    health.forEach((h: any) => {
      expect(h.id).toBeDefined();
      expect(h.status).toBeDefined();
      expect(typeof h.totalRequests).toBe('number');
    });
  });

  it('returns provider configs', () => {
    const router = new MultiProviderRouter();
    const configs = router.getProviderConfigs();
    expect(Array.isArray(configs)).toBe(true);
    expect(configs.length).toBeGreaterThanOrEqual(1);
  });

  it('sets provider priority', () => {
    const router = new MultiProviderRouter();
    router.setProviderPriority('internal', 1);
    const configs = router.getProviderConfigs();
    const internal = configs.find((c: any) => c.id === 'internal');
    expect(internal?.priority).toBe(1);
  });

  it('routes in parallel when requested', async () => {
    const router = new MultiProviderRouter();
    const result = await router.routeParallel({
      origin: { lat: 32.0853, lon: 34.7818 },
      destination: { lat: 32.0700, lon: 34.7700 },
      profile: 'driving',
    });

    expect(result).toBeDefined();
    expect(result.primary).toBeDefined();
    expect(result.primary.totalDistanceM).toBeGreaterThan(0);
  });
});

// ── Benchmark Harness Tests ─────────────────────────────────
describe('BenchmarkHarness', () => {
  let BenchmarkHarness: any;
  let createGANEBenchmarks: any;

  beforeEach(async () => {
    const mod = await import('../../client/src/engine/benchmarkHarness');
    BenchmarkHarness = mod.BenchmarkHarness;
    createGANEBenchmarks = mod.createGANEBenchmarks;
  });

  it('initializes with default config', () => {
    const harness = new BenchmarkHarness();
    expect(harness).toBeDefined();
    expect(harness.getAllBenchmarks()).toEqual([]);
  });

  it('registers and retrieves benchmarks', () => {
    const harness = new BenchmarkHarness();
    harness.register({
      id: 'test-bench-1',
      name: 'Test Benchmark',
      category: 'unit',
      fn: async () => 42,
      slo: { name: 'latency', target: 100, unit: 'ms', comparison: 'lt' },
    });

    expect(harness.getBenchmark('test-bench-1')).toBeDefined();
    expect(harness.getAllBenchmarks()).toHaveLength(1);
  });

  it('unregisters benchmarks', () => {
    const harness = new BenchmarkHarness();
    harness.register({
      id: 'remove-bench',
      name: 'Remove Me',
      category: 'unit',
      fn: async () => 1,
      slo: { name: 'latency', target: 100, unit: 'ms', comparison: 'lt' },
    });

    expect(harness.unregister('remove-bench')).toBe(true);
    expect(harness.getBenchmark('remove-bench')).toBeUndefined();
    expect(harness.unregister('nonexistent')).toBe(false);
  });

  it('registers all benchmarks at once', () => {
    const harness = new BenchmarkHarness();
    harness.registerAll([
      { id: 'b1', name: 'B1', category: 'cat1', fn: async () => 1, slo: { name: 'latency', target: 100, unit: 'ms', comparison: 'lt' } },
      { id: 'b2', name: 'B2', category: 'cat2', fn: async () => 2, slo: { name: 'latency', target: 200, unit: 'ms', comparison: 'lt' } },
    ]);

    expect(harness.getAllBenchmarks()).toHaveLength(2);
  });

  it('gets categories', () => {
    const harness = new BenchmarkHarness();
    harness.registerAll([
      { id: 'b1', name: 'B1', category: 'navigation', fn: async () => 1, slo: { name: 'latency', target: 100, unit: 'ms', comparison: 'lt' } },
      { id: 'b2', name: 'B2', category: 'routing', fn: async () => 2, slo: { name: 'latency', target: 200, unit: 'ms', comparison: 'lt' } },
      { id: 'b3', name: 'B3', category: 'navigation', fn: async () => 3, slo: { name: 'latency', target: 300, unit: 'ms', comparison: 'lt' } },
    ]);

    const cats = harness.getCategories();
    expect(cats).toContain('navigation');
    expect(cats).toContain('routing');
    expect(cats).toHaveLength(2);
  });

  it('runs a single benchmark', async () => {
    const harness = new BenchmarkHarness({ defaultIterations: 3 });
    harness.register({
      id: 'run-one',
      name: 'Run One',
      category: 'unit',
      fn: () => {
        // Simulate some work - return a latency value
        const start = performance.now();
        let x = 0;
        for (let i = 0; i < 1000; i++) x += i;
        return performance.now() - start;
      },
      slo: { name: 'latency', target: 1000, unit: 'ms', comparison: 'lt' },
    });

    const result = await harness.runOne('run-one');
    expect(result).toBeDefined();
    expect(result.id).toBe('run-one');
    expect(result.status).toBe('passed');
    expect(result.stats.mean).toBeGreaterThanOrEqual(0);
  });

  it('runs all benchmarks as a suite', async () => {
    const harness = new BenchmarkHarness({ defaultIterations: 2 });
    harness.registerAll([
      { id: 's1', name: 'S1', category: 'test', fn: () => 1, slo: { name: 'value', target: 5000, unit: 'ms', comparison: 'lt' } },
      { id: 's2', name: 'S2', category: 'test', fn: () => 2, slo: { name: 'value', target: 5000, unit: 'ms', comparison: 'lt' } },
    ]);

    const suite = await harness.runAll();
    expect(suite).toBeDefined();
    expect(suite.results).toHaveLength(2);
    expect(suite.totalDurationMs).toBeGreaterThanOrEqual(0);
  });

  it('sets and retrieves baselines', async () => {
    const harness = new BenchmarkHarness({ defaultIterations: 2 });
    harness.register({
      id: 'baseline-bench',
      name: 'Baseline',
      category: 'unit',
      fn: () => 10,
      slo: { name: 'latency', target: 5000, unit: 'ms', comparison: 'lt' },
    });

    const result = await harness.runOne('baseline-bench');
    const baseline = harness.setBaselineFromResult(result, 'v1.0', 'abc123');
    expect(baseline).toBeDefined();
    expect(baseline.label).toBe('v1.0');
    expect(baseline.commitHash).toBe('abc123');

    const retrieved = harness.getBaseline('baseline-bench');
    expect(retrieved).toBeDefined();
    expect(retrieved!.label).toBe('v1.0');
  });

  it('clears baselines', async () => {
    const harness = new BenchmarkHarness({ defaultIterations: 2 });
    harness.register({
      id: 'clear-bench',
      name: 'Clear',
      category: 'unit',
      fn: () => 1,
      slo: { name: 'latency', target: 5000, unit: 'ms', comparison: 'lt' },
    });

    const result = await harness.runOne('clear-bench');
    harness.setBaselineFromResult(result, 'v1');
    expect(harness.getBaseline('clear-bench')).toBeDefined();

    harness.clearBaselines();
    expect(harness.getBaseline('clear-bench')).toBeUndefined();
  });

  it('createGANEBenchmarks returns predefined benchmarks', () => {
    const benchmarks = createGANEBenchmarks();
    expect(Array.isArray(benchmarks)).toBe(true);
    expect(benchmarks.length).toBeGreaterThan(0);
    benchmarks.forEach((b: any) => {
      expect(b.id).toBeDefined();
      expect(b.name).toBeDefined();
      expect(b.category).toBeDefined();
      expect(typeof b.fn).toBe('function');
      expect(b.slo).toBeDefined();
      expect(b.slo.name).toBeDefined();
      expect(typeof b.slo.target).toBe('number');
    });
  });

  it('detects regressions against baselines', async () => {
    const harness = new BenchmarkHarness({ defaultIterations: 2 });
    harness.register({
      id: 'regress-bench',
      name: 'Regression',
      category: 'unit',
      fn: () => 10,
      slo: { name: 'latency', target: 5000, unit: 'ms', comparison: 'lt' },
    });

    const result = await harness.runOne('regress-bench');
    harness.setBaselineFromResult(result, 'v1');

    // Run again - should detect no regression since same benchmark
    const suite = await harness.runAll();
    expect(suite.regressions).toBeDefined();
    expect(Array.isArray(suite.regressions)).toBe(true);
  });

  it('runs benchmarks by category', async () => {
    const harness = new BenchmarkHarness({ defaultIterations: 2 });
    harness.registerAll([
      { id: 'cat-a-1', name: 'A1', category: 'catA', fn: () => 1, slo: { name: 'latency', target: 5000, unit: 'ms', comparison: 'lt' } },
      { id: 'cat-a-2', name: 'A2', category: 'catA', fn: () => 2, slo: { name: 'latency', target: 5000, unit: 'ms', comparison: 'lt' } },
      { id: 'cat-b-1', name: 'B1', category: 'catB', fn: () => 3, slo: { name: 'latency', target: 5000, unit: 'ms', comparison: 'lt' } },
    ]);

    const results = await harness.runCategory('catA');
    expect(results).toHaveLength(2);
    results.forEach((r: any) => {
      expect(r.id.startsWith('cat-a')).toBe(true);
    });
  });
});
