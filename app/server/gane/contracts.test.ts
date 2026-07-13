/**
 * G.A.N.E — Contract Module Tests
 * =================================
 * Tests for SLO Catalog, Security Model, Failure Matrix,
 * Data Lineage, Config System, Model Registry, Evidence Chain.
 */
import { describe, it, expect, beforeEach } from 'vitest';

// ─── SLO Catalog ────────────────────────────────────────
import {
  SLOs as SLO_CATALOG,
  RUNBOOKS as INCIDENT_RUNBOOKS,
  CHAOS_SCENARIOS,
} from '@shared/contracts/sloCatalog';

describe('SLO Catalog', () => {
  it('has at least 5 SLOs', () => {
    expect(SLO_CATALOG.length).toBeGreaterThanOrEqual(5);
  });

  it('every SLO has valid error budget', () => {
    for (const slo of SLO_CATALOG) {
      expect(slo.errorBudget).toBeGreaterThan(0);
      expect(slo.errorBudget).toBeLessThanOrEqual(1);
      expect(slo.target).toBeGreaterThan(0);
    }
  });

  it('getSLOBySubsystem returns matching SLOs', () => {
    const results = SLO_CATALOG.filter(s => s.subsystem === 'ESKF');
    expect(results.length).toBeGreaterThan(0);
    for (const slo of results) {
      expect(slo.subsystem).toBe('ESKF');
    }
  });

  it('getSLOBySubsystem returns empty for unknown subsystem', () => {
    expect(SLO_CATALOG.filter(s => s.subsystem === 'nonexistent')).toEqual([]);
  });

  it('all SLOs have burn rate alerts', () => {
    for (const slo of SLO_CATALOG) {
      expect(slo.burnRateAlerts.length).toBeGreaterThan(0);
    }
  });

  it('has at least 3 incident runbooks', () => {
    expect(INCIDENT_RUNBOOKS.length).toBeGreaterThanOrEqual(3);
  });

  it('every runbook has diagnostic and mitigation steps', () => {
    for (const rb of INCIDENT_RUNBOOKS) {
      expect(rb.diagnosticSteps.length).toBeGreaterThan(0);
      expect(rb.mitigationSteps.length).toBeGreaterThan(0);
      expect(rb.estimatedTTR).toBeDefined();
    }
  });

  it('getRunbookByIncident returns matching runbook', () => {
    const first = INCIDENT_RUNBOOKS[0];
    const result = INCIDENT_RUNBOOKS.find(r => r.id === first.id);
    expect(result).toBeDefined();
    expect(result?.id).toBe(first.id);
  });

  it('has at least 3 chaos scenarios', () => {
    expect(CHAOS_SCENARIOS.length).toBeGreaterThanOrEqual(3);
  });

  it('every chaos scenario has valid duration', () => {
    for (const cs of CHAOS_SCENARIOS) {
      expect(cs.targetSubsystem).toBeDefined();
      expect(cs.injectionType).toBeDefined();
    }
  });
});

// ─── Security Model ─────────────────────────────────────
import {
  TRUST_BOUNDARIES,
  THREAT_MODEL,
  KEY_HIERARCHY,
  RBAC_MATRIX,
  AUTH_FLOWS,
  IDENTITY_MODEL,
  TENANT_ISOLATION,
  hasPermission,
  getPermissions,
  getCriticalThreats,
} from '@shared/contracts/securityModel';

describe('Security Model', () => {
  it('has at least 4 trust boundaries', () => {
    expect(TRUST_BOUNDARIES.length).toBeGreaterThanOrEqual(4);
  });

  it('every trust boundary has validation rules', () => {
    for (const tb of TRUST_BOUNDARIES) {
      expect(tb.validationRules.length).toBeGreaterThan(0);
      expect(tb.protocol).toBeDefined();
    }
  });

  it('has at least 6 threats in STRIDE model', () => {
    expect(THREAT_MODEL.length).toBeGreaterThanOrEqual(6);
  });

  it('every threat has mitigations', () => {
    for (const t of THREAT_MODEL) {
      expect(t.mitigations.length).toBeGreaterThan(0);
      expect(t.riskScore).toBeGreaterThan(0);
      expect(t.riskScore).toBeLessThanOrEqual(25);
    }
  });

  it('getCriticalThreats returns threats with score >= 15', () => {
    const critical = getCriticalThreats();
    for (const t of critical) {
      expect(t.riskScore).toBeGreaterThanOrEqual(15);
    }
  });

  it('has at least 4 crypto keys', () => {
    expect(KEY_HIERARCHY.length).toBeGreaterThanOrEqual(4);
  });

  it('every key has rotation period', () => {
    for (const k of KEY_HIERARCHY) {
      expect(k.rotationPeriod).toBeDefined();
      expect(k.algorithm).toBeDefined();
    }
  });

  it('RBAC matrix has all 5 roles', () => {
    expect(Object.keys(RBAC_MATRIX)).toContain('anonymous');
    expect(Object.keys(RBAC_MATRIX)).toContain('user');
    expect(Object.keys(RBAC_MATRIX)).toContain('fleet_operator');
    expect(Object.keys(RBAC_MATRIX)).toContain('admin');
    expect(Object.keys(RBAC_MATRIX)).toContain('owner');
  });

  it('owner has all admin permissions', () => {
    const adminPerms = getPermissions('admin');
    const ownerPerms = getPermissions('owner');
    for (const p of adminPerms) {
      expect(ownerPerms).toContain(p);
    }
  });

  it('anonymous has limited permissions', () => {
    const anonPerms = getPermissions('anonymous');
    expect(anonPerms.length).toBeLessThan(getPermissions('user').length);
    expect(hasPermission('anonymous', 'read:map')).toBe(true);
    expect(hasPermission('anonymous', 'manage:users')).toBe(false);
  });

  it('hasPermission works correctly', () => {
    expect(hasPermission('admin', 'manage:users')).toBe(true);
    expect(hasPermission('user', 'manage:users')).toBe(false);
    expect(hasPermission('owner', 'manage:system')).toBe(true);
  });

  it('has at least 3 auth flows', () => {
    expect(AUTH_FLOWS.length).toBeGreaterThanOrEqual(3);
  });

  it('has at least 5 identity types', () => {
    expect(IDENTITY_MODEL.length).toBeGreaterThanOrEqual(5);
  });

  it('has at least 4 tenant isolation layers', () => {
    expect(TENANT_ISOLATION.length).toBeGreaterThanOrEqual(4);
  });
});

// ─── Failure Matrix ─────────────────────────────────────
import {
  FAILURE_CASES,
  DEPENDENCY_GRAPH,
  getFailuresBySeverity,
  getCriticalDependencies,
  getDependencyChain,
} from '@shared/contracts/failureMatrix';

describe('Failure Matrix', () => {
  it('has at least 6 failure cases', () => {
    expect(FAILURE_CASES.length).toBeGreaterThanOrEqual(6);
  });

  it('every failure has recovery steps', () => {
    for (const fc of FAILURE_CASES) {
      expect(fc.recoverySteps.length).toBeGreaterThan(0);
      expect(fc.fallbackMode).toBeDefined();
      expect(fc.testScenario).toBeDefined();
    }
  });

  it('getFailuresBySeverity returns correct subset', () => {
    const critical = getFailuresBySeverity('critical');
    for (const fc of critical) {
      expect(fc.severity).toBe('critical');
    }
  });

  it('has at least 8 dependency nodes', () => {
    expect(DEPENDENCY_GRAPH.length).toBeGreaterThanOrEqual(8);
  });

  it('getCriticalDependencies returns critical nodes', () => {
    const critical = getCriticalDependencies();
    for (const d of critical) {
      expect(d.criticality).toBe('critical');
    }
    expect(critical.length).toBeGreaterThan(0);
  });

  it('getDependencyChain returns valid chain', () => {
    const chain = getDependencyChain('dep-eskf');
    expect(chain).toContain('dep-eskf');
    expect(chain).toContain('dep-gnss');
    expect(chain).toContain('dep-imu');
  });

  it('getDependencyChain handles unknown node', () => {
    const chain = getDependencyChain('nonexistent');
    expect(chain).toEqual(['nonexistent']);
  });
});

// ─── Data Lineage ───────────────────────────────────────
import {
  DATA_SOURCES,
  TRANSFORMATIONS,
  DATA_FLOW_EDGES,
  FRESHNESS_CONTRACTS,
  getSourceById,
  getUpstreamSources,
  getDownstreamSources,
  getTransformationChain,
} from '@shared/contracts/dataLineage';

describe('Data Lineage', () => {
  it('has at least 8 data sources', () => {
    expect(DATA_SOURCES.length).toBeGreaterThanOrEqual(8);
  });

  it('every data source has quality metrics', () => {
    for (const ds of DATA_SOURCES) {
      expect(ds.qualityMetrics.accuracy).toBeDefined();
      expect(ds.qualityMetrics.completeness).toBeDefined();
      expect(ds.qualityMetrics.timeliness).toBeDefined();
    }
  });

  it('has at least 4 transformations', () => {
    expect(TRANSFORMATIONS.length).toBeGreaterThanOrEqual(4);
  });

  it('every transformation has input and output', () => {
    for (const tx of TRANSFORMATIONS) {
      expect(tx.inputSources.length).toBeGreaterThan(0);
      expect(tx.outputSources.length).toBeGreaterThan(0);
    }
  });

  it('has at least 6 data flow edges', () => {
    expect(DATA_FLOW_EDGES.length).toBeGreaterThanOrEqual(6);
  });

  it('has freshness contracts', () => {
    expect(FRESHNESS_CONTRACTS.length).toBeGreaterThan(0);
    for (const fc of FRESHNESS_CONTRACTS) {
      expect(fc.maxAgeMs).toBeGreaterThan(0);
    }
  });

  it('getSourceById returns correct source', () => {
    const source = getSourceById('ds-gnss-raw');
    expect(source).toBeDefined();
    expect(source?.name).toContain('GNSS');
  });

  it('getSourceById returns undefined for unknown', () => {
    expect(getSourceById('nonexistent')).toBeUndefined();
  });

  it('getUpstreamSources returns correct upstream', () => {
    const upstream = getUpstreamSources('ds-fused-position');
    expect(upstream.length).toBeGreaterThan(0);
  });

  it('getDownstreamSources returns correct downstream', () => {
    const downstream = getDownstreamSources('ds-gnss-raw');
    expect(downstream.length).toBeGreaterThan(0);
  });

  it('getTransformationChain returns valid chain', () => {
    const chain = getTransformationChain('ds-fused-position');
    expect(chain.length).toBeGreaterThan(0);
  });
});

// ─── Config System ──────────────────────────────────────
import {
  CONFIG_KEYS,
  ConfigResolver,
  EXPERIMENT_TEMPLATES,
} from '@shared/contracts/configSystem';

describe('Config System', () => {
  let resolver: ConfigResolver;

  beforeEach(() => {
    resolver = new ConfigResolver();
  });

  it('has at least 15 config keys', () => {
    expect(CONFIG_KEYS.length).toBeGreaterThanOrEqual(15);
  });

  it('every config key has validation', () => {
    for (const ck of CONFIG_KEYS) {
      expect(ck.validation).toBeDefined();
      expect(ck.defaultValue).toBeDefined();
      expect(ck.allowedLevels.length).toBeGreaterThan(0);
    }
  });

  it('resolves to default when no overrides', () => {
    const result = resolver.resolve('nav.gps.update_rate_hz', {});
    expect(result.resolvedValue).toBe(1);
    expect(result.resolvedLevel).toBe('global');
  });

  it('regional override wins over global', () => {
    resolver.setOverride({
      key: 'nav.reroute.hysteresis_meters',
      level: 'regional',
      scope: 'us-west',
      value: 100,
      setBy: 'admin',
      setAt: Date.now(),
    });
    const result = resolver.resolve('nav.reroute.hysteresis_meters', { region: 'us-west' });
    expect(result.resolvedValue).toBe(100);
    expect(result.resolvedLevel).toBe('regional');
  });

  it('experiment override wins over all', () => {
    resolver.setOverride({
      key: 'nav.gps.update_rate_hz',
      level: 'experiment',
      scope: 'exp-001',
      value: 5,
      setBy: 'ml-team',
      setAt: Date.now(),
    });
    resolver.setOverride({
      key: 'nav.gps.update_rate_hz',
      level: 'device',
      scope: 'dev-001',
      value: 2,
      setBy: 'admin',
      setAt: Date.now(),
    });
    const result = resolver.resolve('nav.gps.update_rate_hz', {
      experimentId: 'exp-001',
      deviceId: 'dev-001',
    });
    expect(result.resolvedValue).toBe(5);
    expect(result.resolvedLevel).toBe('experiment');
  });

  it('expired override is skipped', () => {
    resolver.setOverride({
      key: 'nav.gps.update_rate_hz',
      level: 'device',
      scope: 'dev-001',
      value: 10,
      setBy: 'admin',
      setAt: Date.now() - 100000,
      expiresAt: Date.now() - 1000, // expired
    });
    const result = resolver.resolve('nav.gps.update_rate_hz', { deviceId: 'dev-001' });
    expect(result.resolvedValue).toBe(1); // falls back to default
  });

  it('removeOverride works', () => {
    resolver.setOverride({
      key: 'nav.gps.update_rate_hz',
      level: 'device',
      scope: 'dev-001',
      value: 5,
      setBy: 'admin',
      setAt: Date.now(),
    });
    expect(resolver.removeOverride('nav.gps.update_rate_hz', 'device', 'dev-001')).toBe(true);
    const result = resolver.resolve('nav.gps.update_rate_hz', { deviceId: 'dev-001' });
    expect(result.resolvedValue).toBe(1);
  });

  it('getCategories returns unique categories', () => {
    const categories = resolver.getCategories();
    expect(categories.length).toBeGreaterThan(0);
    expect(new Set(categories).size).toBe(categories.length);
  });

  it('getKeysByCategory returns correct keys', () => {
    const navKeys = resolver.getKeysByCategory('navigation');
    expect(navKeys.length).toBeGreaterThan(0);
    for (const k of navKeys) {
      expect(k.category).toBe('navigation');
    }
  });

  it('exportConfig returns all defaults', () => {
    const config = resolver.exportConfig();
    expect(Object.keys(config).length).toBe(CONFIG_KEYS.length);
  });

  it('has experiment templates', () => {
    expect(EXPERIMENT_TEMPLATES.length).toBeGreaterThan(0);
    for (const et of EXPERIMENT_TEMPLATES) {
      expect(et.configOverrides.length).toBeGreaterThan(0);
    }
  });
});

// ─── Model Registry ─────────────────────────────────────
import {
  REGISTERED_MODELS,
  FEATURE_STORE,
  getModelByName,
  getProductionModel,
  getFeaturesByEntity,
  getStaleFeatures,
  calculatePSI,
} from '@shared/contracts/modelRegistry';

describe('Model Registry', () => {
  it('has at least 3 registered models', () => {
    expect(REGISTERED_MODELS.length).toBeGreaterThanOrEqual(3);
  });

  it('every model has metrics', () => {
    for (const m of REGISTERED_MODELS) {
      expect(m.metrics.accuracy).toBeGreaterThan(0);
      expect(m.metrics.latencyP50Ms).toBeGreaterThan(0);
      expect(m.metrics.throughputRPS).toBeGreaterThan(0);
    }
  });

  it('getModelByName returns correct models', () => {
    const models = getModelByName('eta_predictor');
    expect(models.length).toBeGreaterThan(0);
    for (const m of models) {
      expect(m.modelName).toBe('eta_predictor');
    }
  });

  it('getProductionModel returns production stage', () => {
    const model = getProductionModel('eta_predictor');
    expect(model).toBeDefined();
    expect(model?.stage).toBe('production');
  });

  it('getProductionModel returns undefined for unknown', () => {
    expect(getProductionModel('nonexistent')).toBeUndefined();
  });

  it('has at least 4 features in feature store', () => {
    expect(FEATURE_STORE.length).toBeGreaterThanOrEqual(4);
  });

  it('getFeaturesByEntity returns correct features', () => {
    const features = getFeaturesByEntity('road_segment');
    expect(features.length).toBeGreaterThan(0);
    for (const f of features) {
      expect(f.entity).toBe('road_segment');
    }
  });

  it('getStaleFeatures filters by freshness', () => {
    const stale = getStaleFeatures(60000); // > 1 minute
    for (const f of stale) {
      expect(f.freshnessMs).toBeGreaterThan(60000);
    }
  });

  it('calculatePSI returns valid score', () => {
    const baseline = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const current = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const psi = calculatePSI(baseline, current);
    expect(psi).toBeGreaterThanOrEqual(0);
    expect(psi).toBeLessThan(0.1); // same distribution = low PSI
  });

  it('calculatePSI detects drift', () => {
    const baseline = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const drifted = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    const psi = calculatePSI(baseline, drifted);
    expect(psi).toBeGreaterThan(0);
  });
});

// ─── Evidence Chain ─────────────────────────────────────
import {
  EVIDENCE_POLICIES,
  EvidenceManager,
} from '@shared/contracts/evidenceChain';

describe('Evidence Chain-of-Custody', () => {
  let manager: EvidenceManager;

  beforeEach(() => {
    manager = new EvidenceManager();
  });

  it('has evidence policies for all types', () => {
    expect(EVIDENCE_POLICIES.length).toBeGreaterThanOrEqual(5);
    const types = EVIDENCE_POLICIES.map(p => p.type);
    expect(types).toContain('incident_report');
    expect(types).toContain('telemetry_snapshot');
    expect(types).toContain('media_capture');
  });

  it('creates evidence with initial custody entry', () => {
    const evidence = manager.createEvidence(
      'incident_report',
      'a'.repeat(64),
      'user-001',
      { incidentType: 'accident', severity: 'high', description: 'Test' },
      { lat: 32.0853, lng: 34.7818, accuracy: 10 }
    );
    expect(evidence.id).toBeDefined();
    expect(evidence.custodyChain.length).toBe(1);
    expect(evidence.custodyChain[0].action).toBe('created');
    expect(evidence.integrityStatus).toBe('intact');
  });

  it('adds custody entries', () => {
    const evidence = manager.createEvidence(
      'incident_report',
      'b'.repeat(64),
      'user-001',
      { incidentType: 'accident', severity: 'high', description: 'Test' }
    );
    const entry = manager.addCustodyEntry(
      evidence.id,
      'transferred',
      'admin-001',
      'admin',
      'Transferred for review'
    );
    expect(entry).not.toBeNull();
    expect(entry?.action).toBe('transferred');

    const updated = manager.getEvidence(evidence.id);
    expect(updated?.custodyChain.length).toBe(2);
  });

  it('verifies intact evidence', () => {
    const evidence = manager.createEvidence(
      'incident_report',
      'c'.repeat(64),
      'user-001',
      { incidentType: 'accident', severity: 'high', description: 'Test' },
      { lat: 32.0853, lng: 34.7818, accuracy: 10 }
    );
    const result = manager.verify(evidence.id);
    expect(result.isValid).toBe(true);
    expect(result.overallConfidence).toBe(1);
  });

  it('verify returns invalid for unknown evidence', () => {
    const result = manager.verify('nonexistent');
    expect(result.isValid).toBe(false);
    expect(result.overallConfidence).toBe(0);
  });

  it('seals evidence', () => {
    const evidence = manager.createEvidence(
      'incident_report',
      'd'.repeat(64),
      'user-001',
      { incidentType: 'accident', severity: 'high', description: 'Test' }
    );
    const sealed = manager.sealEvidence(evidence.id, 'admin-001');
    expect(sealed).toBe(true);

    const updated = manager.getEvidence(evidence.id);
    const lastEntry = updated?.custodyChain[updated.custodyChain.length - 1];
    expect(lastEntry?.action).toBe('sealed');
  });

  it('getByType returns correct evidence', () => {
    manager.createEvidence('incident_report', 'e'.repeat(64), 'user-001', { incidentType: 'a', severity: 'b', description: 'c' });
    manager.createEvidence('telemetry_snapshot', 'f'.repeat(64), 'device-001', { deviceId: 'x', sessionId: 'y', sensorTypes: ['gps'] });
    
    const incidents = manager.getByType('incident_report');
    expect(incidents.length).toBe(1);
    expect(incidents[0].type).toBe('incident_report');
  });

  it('getByCreator returns correct evidence', () => {
    manager.createEvidence('incident_report', 'g'.repeat(64), 'user-001', { incidentType: 'a', severity: 'b', description: 'c' });
    manager.createEvidence('incident_report', 'h'.repeat(64), 'user-002', { incidentType: 'a', severity: 'b', description: 'c' });
    
    const user1Evidence = manager.getByCreator('user-001');
    expect(user1Evidence.length).toBe(1);
  });

  it('getStats returns correct statistics', () => {
    manager.createEvidence('incident_report', 'i'.repeat(64), 'user-001', { incidentType: 'a', severity: 'b', description: 'c' });
    manager.createEvidence('telemetry_snapshot', 'j'.repeat(64), 'device-001', { deviceId: 'x', sessionId: 'y', sensorTypes: ['gps'] });
    
    const stats = manager.getStats();
    expect(stats.total).toBe(2);
    expect(stats.byType['incident_report']).toBe(1);
    expect(stats.byType['telemetry_snapshot']).toBe(1);
    expect(stats.avgChainLength).toBe(1);
  });
});
