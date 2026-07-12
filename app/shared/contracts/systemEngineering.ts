/**
 * G.A.N.E — System Engineering Contracts
 * 
 * Covers:
 * - Dependency Lock Matrix (what depends on what, and what must never depend on what)
 * - Version Compatibility Matrix (cross-service version compatibility rules)
 * - Commercial Architecture (subscription tiers, API billing, fleet pricing)
 * - Unit Economics Model (cost per user, per route, per API call)
 * - Distribution Engineering (Android Auto, CarPlay, OEM, SDK)
 * - Migration Strategy (wrap → replace → validate → cutover)
 * - Legacy Bridge Layer (adapter pattern for legacy system integration)
 * - Repo Strategy (monorepo governance, package ownership, CI rules)
 * - Code Generation Pipeline (schema → code → test → docs)
 */

// ═══════════════════════════════════════════════════════════
// DEPENDENCY LOCK MATRIX
// ═══════════════════════════════════════════════════════════

export interface DependencyRule {
  id: string;
  source: string;
  target: string;
  relationship: 'depends_on' | 'must_not_depend_on' | 'optional_dependency' | 'event_driven_only';
  rationale: string;
  enforcement: 'compile_time' | 'ci_check' | 'architecture_review';
}

export const DEPENDENCY_LOCK_MATRIX: DependencyRule[] = [
  // Core must NEVER depend on optional features
  { id: 'DEP-001', source: 'positioning', target: 'ai_intelligence', relationship: 'must_not_depend_on', rationale: 'L0.5: Core positioning must work without AI', enforcement: 'compile_time' },
  { id: 'DEP-002', source: 'guidance', target: 'internet', relationship: 'must_not_depend_on', rationale: 'L0.6: Guidance must work offline', enforcement: 'compile_time' },
  { id: 'DEP-003', source: 'failover', target: 'cloud', relationship: 'must_not_depend_on', rationale: 'L0.1: Failover must work without cloud', enforcement: 'compile_time' },
  { id: 'DEP-004', source: 'alerting', target: 'single_channel', relationship: 'must_not_depend_on', rationale: 'Alerts must have multi-channel fallback', enforcement: 'ci_check' },
  { id: 'DEP-005', source: 'safety_ux', target: 'personalization', relationship: 'must_not_depend_on', rationale: 'Safety UX must work without personalization', enforcement: 'compile_time' },
  { id: 'DEP-006', source: 'emergency_mode', target: 'payment_state', relationship: 'must_not_depend_on', rationale: 'Emergency mode must never be gated by payment', enforcement: 'compile_time' },
  { id: 'DEP-007', source: 'core_maps', target: 'live_imagery', relationship: 'must_not_depend_on', rationale: 'Core maps must work with cached data', enforcement: 'compile_time' },
  { id: 'DEP-008', source: 'integrity_scoring', target: 'user_reports_only', relationship: 'must_not_depend_on', rationale: 'Integrity must use multi-source validation', enforcement: 'ci_check' },
  { id: 'DEP-009', source: 'offline_mode', target: 'recent_sync', relationship: 'must_not_depend_on', rationale: 'Offline must work even with stale data', enforcement: 'compile_time' },
  { id: 'DEP-010', source: 'routing', target: 'deterministic_fallback', relationship: 'depends_on', rationale: 'Routing must always have deterministic fallback path', enforcement: 'compile_time' },

  // Allowed dependencies (event-driven)
  { id: 'DEP-011', source: 'routing', target: 'positioning', relationship: 'depends_on', rationale: 'Routing needs current position', enforcement: 'compile_time' },
  { id: 'DEP-012', source: 'guidance', target: 'routing', relationship: 'depends_on', rationale: 'Guidance follows computed route', enforcement: 'compile_time' },
  { id: 'DEP-013', source: 'fusion', target: 'positioning', relationship: 'depends_on', rationale: 'Fusion feeds positioning', enforcement: 'compile_time' },
  { id: 'DEP-014', source: 'telemetry', target: 'all_services', relationship: 'event_driven_only', rationale: 'Telemetry consumes events, never called directly', enforcement: 'ci_check' },
  { id: 'DEP-015', source: 'replay', target: 'telemetry', relationship: 'depends_on', rationale: 'Replay reads telemetry events', enforcement: 'compile_time' },
  { id: 'DEP-016', source: 'ai_traffic', target: 'telemetry', relationship: 'event_driven_only', rationale: 'AI consumes telemetry events for predictions', enforcement: 'ci_check' },
  { id: 'DEP-017', source: 'alerting', target: 'routing', relationship: 'event_driven_only', rationale: 'Alerts triggered by routing events', enforcement: 'ci_check' },
  { id: 'DEP-018', source: 'payments', target: 'alerting', relationship: 'event_driven_only', rationale: 'Payment events trigger alerts', enforcement: 'ci_check' },
];

// ═══════════════════════════════════════════════════════════
// VERSION COMPATIBILITY MATRIX
// ═══════════════════════════════════════════════════════════

export interface VersionCompatibility {
  service: string;
  currentVersion: string;
  minClientVersion: string;
  maxClientVersion: string;
  breakingChanges: BreakingChange[];
  deprecations: Deprecation[];
}

export interface BreakingChange {
  version: string;
  description: string;
  migrationPath: string;
  deadline: string;
}

export interface Deprecation {
  feature: string;
  deprecatedIn: string;
  removedIn: string;
  replacement: string;
}

export const VERSION_COMPATIBILITY: VersionCompatibility[] = [
  {
    service: 'positioning-api',
    currentVersion: '2.0.0',
    minClientVersion: '1.8.0',
    maxClientVersion: '2.x.x',
    breakingChanges: [
      { version: '2.0.0', description: 'PositionSample now includes covariance matrix', migrationPath: 'Add covariance field handling', deadline: '2026-06-01' },
    ],
    deprecations: [
      { feature: 'single_gnss_mode', deprecatedIn: '1.9.0', removedIn: '3.0.0', replacement: 'multi_constellation_mode' },
    ],
  },
  {
    service: 'routing-api',
    currentVersion: '3.1.0',
    minClientVersion: '2.5.0',
    maxClientVersion: '3.x.x',
    breakingChanges: [
      { version: '3.0.0', description: 'Route cost_breakdown now includes cognitive_load', migrationPath: 'Add cognitive_load field', deadline: '2026-07-01' },
    ],
    deprecations: [
      { feature: 'simple_cost_model', deprecatedIn: '2.8.0', removedIn: '4.0.0', replacement: 'multi_objective_cost' },
    ],
  },
  {
    service: 'event-schema',
    currentVersion: '1.5.0',
    minClientVersion: '1.0.0',
    maxClientVersion: '1.x.x',
    breakingChanges: [],
    deprecations: [
      { feature: 'legacy_event_format', deprecatedIn: '1.3.0', removedIn: '2.0.0', replacement: 'EventEnvelope protobuf' },
    ],
  },
  {
    service: 'telemetry-api',
    currentVersion: '1.2.0',
    minClientVersion: '1.0.0',
    maxClientVersion: '1.x.x',
    breakingChanges: [],
    deprecations: [],
  },
  {
    service: 'map-tiles-api',
    currentVersion: '2.3.0',
    minClientVersion: '2.0.0',
    maxClientVersion: '2.x.x',
    breakingChanges: [
      { version: '2.0.0', description: 'Tile format changed from PNG to vector PBF', migrationPath: 'Update tile renderer to PBF', deadline: '2026-05-01' },
    ],
    deprecations: [],
  },
];

// ═══════════════════════════════════════════════════════════
// COMMERCIAL ARCHITECTURE
// ═══════════════════════════════════════════════════════════

export interface SubscriptionTier {
  id: string;
  name: string;
  price_monthly_usd: number;
  features: string[];
  limits: Record<string, number>;
  sla: string;
}

export const SUBSCRIPTION_TIERS: SubscriptionTier[] = [
  {
    id: 'free',
    name: 'Explorer',
    price_monthly_usd: 0,
    features: [
      'Basic navigation',
      'Standard routing (single route)',
      'Community incident reports',
      'Ad-supported',
    ],
    limits: {
      routes_per_day: 10,
      offline_maps_gb: 1,
      api_calls_per_month: 0,
      fleet_vehicles: 0,
    },
    sla: 'Best effort',
  },
  {
    id: 'pro',
    name: 'Navigator Pro',
    price_monthly_usd: 9.99,
    features: [
      'Multi-route alternatives',
      'Real-time traffic prediction',
      'Offline maps (unlimited)',
      'AI copilot',
      'No ads',
      'Priority incident validation',
      'ETA sharing',
    ],
    limits: {
      routes_per_day: -1,
      offline_maps_gb: 50,
      api_calls_per_month: 0,
      fleet_vehicles: 0,
    },
    sla: '99.9% uptime',
  },
  {
    id: 'fleet',
    name: 'Fleet Commander',
    price_monthly_usd: 49.99,
    features: [
      'All Pro features',
      'Fleet management dashboard',
      'Driver scoring',
      'Route optimization (multi-vehicle)',
      'Geofencing',
      'API access (10K calls/month)',
      'Priority support',
      'Custom branding',
    ],
    limits: {
      routes_per_day: -1,
      offline_maps_gb: 200,
      api_calls_per_month: 10_000,
      fleet_vehicles: 50,
    },
    sla: '99.95% uptime',
  },
  {
    id: 'enterprise',
    name: 'Enterprise',
    price_monthly_usd: -1, // Custom pricing
    features: [
      'All Fleet features',
      'Unlimited API access',
      'Dedicated infrastructure',
      'Custom SLA',
      'White-label option',
      'On-premise deployment option',
      'Dedicated support engineer',
      'Custom AI model training',
      'Data export & analytics',
    ],
    limits: {
      routes_per_day: -1,
      offline_maps_gb: -1,
      api_calls_per_month: -1,
      fleet_vehicles: -1,
    },
    sla: 'Custom (up to 99.99%)',
  },
];

export interface APIBillingModel {
  endpoint: string;
  freeQuota: number;
  pricePerCall_usd: number;
  bulkDiscount: { threshold: number; discount_pct: number }[];
}

export const API_BILLING: APIBillingModel[] = [
  {
    endpoint: '/api/v1/route',
    freeQuota: 1000,
    pricePerCall_usd: 0.005,
    bulkDiscount: [
      { threshold: 10_000, discount_pct: 10 },
      { threshold: 100_000, discount_pct: 25 },
      { threshold: 1_000_000, discount_pct: 40 },
    ],
  },
  {
    endpoint: '/api/v1/position',
    freeQuota: 5000,
    pricePerCall_usd: 0.001,
    bulkDiscount: [
      { threshold: 50_000, discount_pct: 15 },
      { threshold: 500_000, discount_pct: 30 },
    ],
  },
  {
    endpoint: '/api/v1/geocode',
    freeQuota: 2000,
    pricePerCall_usd: 0.003,
    bulkDiscount: [
      { threshold: 20_000, discount_pct: 10 },
      { threshold: 200_000, discount_pct: 25 },
    ],
  },
  {
    endpoint: '/api/v1/traffic',
    freeQuota: 500,
    pricePerCall_usd: 0.01,
    bulkDiscount: [
      { threshold: 5_000, discount_pct: 10 },
      { threshold: 50_000, discount_pct: 20 },
    ],
  },
];

// ═══════════════════════════════════════════════════════════
// UNIT ECONOMICS MODEL
// ═══════════════════════════════════════════════════════════

export interface CostComponent {
  category: string;
  item: string;
  costPerUnit_usd: number;
  unit: string;
  scaleFactor: string;
  notes: string;
}

export const UNIT_ECONOMICS: CostComponent[] = [
  { category: 'compute', item: 'Position computation', costPerUnit_usd: 0.000002, unit: 'per position fix', scaleFactor: '~100 fixes/min/user', notes: 'Edge compute reduces cloud cost' },
  { category: 'compute', item: 'Route computation', costPerUnit_usd: 0.0005, unit: 'per route', scaleFactor: '~5 routes/session', notes: 'Graph pre-computation amortizes cost' },
  { category: 'compute', item: 'AI inference (traffic prediction)', costPerUnit_usd: 0.001, unit: 'per prediction', scaleFactor: '~1 prediction/min/active user', notes: 'Batch inference reduces per-unit cost' },
  { category: 'compute', item: 'AI inference (copilot)', costPerUnit_usd: 0.003, unit: 'per query', scaleFactor: '~2 queries/session', notes: 'LLM cost dominates' },
  { category: 'storage', item: 'Telemetry storage', costPerUnit_usd: 0.00001, unit: 'per event', scaleFactor: '~1000 events/session', notes: 'TTL-based retention reduces long-term cost' },
  { category: 'storage', item: 'Map tile storage', costPerUnit_usd: 0.023, unit: 'per GB/month', scaleFactor: '~500GB global tiles', notes: 'CDN caching reduces origin reads' },
  { category: 'storage', item: 'Offline map package delivery', costPerUnit_usd: 0.09, unit: 'per GB transferred', scaleFactor: '~2GB per city package', notes: 'Delta updates reduce transfer' },
  { category: 'network', item: 'CDN bandwidth', costPerUnit_usd: 0.08, unit: 'per GB', scaleFactor: 'Varies by region', notes: 'Edge caching reduces origin bandwidth' },
  { category: 'network', item: 'Real-time data push', costPerUnit_usd: 0.0001, unit: 'per message', scaleFactor: '~10 messages/min/user', notes: 'WebSocket reduces overhead vs polling' },
  { category: 'third_party', item: 'Map data licensing', costPerUnit_usd: 0.0, unit: 'per month', scaleFactor: 'Fixed cost', notes: 'OSM is free; proprietary varies' },
  { category: 'third_party', item: 'Weather data API', costPerUnit_usd: 0.0005, unit: 'per request', scaleFactor: '~1 request/5min/active user', notes: 'Caching reduces calls' },
  { category: 'support', item: 'Customer support', costPerUnit_usd: 0.50, unit: 'per ticket', scaleFactor: '~0.02 tickets/user/month', notes: 'AI-first support reduces human cost' },
];

export interface RevenueMetric {
  metric: string;
  value: number;
  unit: string;
  notes: string;
}

export const REVENUE_TARGETS: RevenueMetric[] = [
  { metric: 'ARPU (free)', value: 0.50, unit: 'USD/month', notes: 'Ad revenue only' },
  { metric: 'ARPU (pro)', value: 9.99, unit: 'USD/month', notes: 'Subscription' },
  { metric: 'ARPU (fleet)', value: 49.99, unit: 'USD/month', notes: 'Subscription + API' },
  { metric: 'Free-to-Pro conversion', value: 5, unit: 'percent', notes: 'Target conversion rate' },
  { metric: 'Pro churn', value: 3, unit: 'percent/month', notes: 'Target monthly churn' },
  { metric: 'LTV (pro)', value: 200, unit: 'USD', notes: 'Lifetime value estimate' },
  { metric: 'CAC target', value: 15, unit: 'USD', notes: 'Customer acquisition cost' },
  { metric: 'LTV/CAC ratio', value: 13.3, unit: 'ratio', notes: 'Target >3x' },
  { metric: 'Gross margin target', value: 70, unit: 'percent', notes: 'After infrastructure costs' },
];

// ═══════════════════════════════════════════════════════════
// DISTRIBUTION ENGINEERING
// ═══════════════════════════════════════════════════════════

export interface DistributionChannel {
  id: string;
  platform: string;
  integrationMethod: string;
  constraints: string[];
  requirements: string[];
  timeline: string;
}

export const DISTRIBUTION_CHANNELS: DistributionChannel[] = [
  {
    id: 'DIST-001',
    platform: 'Android Auto',
    integrationMethod: 'Android Auto Navigation Template',
    constraints: [
      'Limited UI templates (fixed layouts)',
      'Must pass Google review',
      'Voice-first interaction required',
      'No custom rendering on car display',
    ],
    requirements: [
      'Navigation notification with TurnByTurn',
      'Voice command integration',
      'Background location permission',
      'Automotive OS compatibility',
    ],
    timeline: 'Phase 2',
  },
  {
    id: 'DIST-002',
    platform: 'Apple CarPlay',
    integrationMethod: 'CarPlay Navigation Framework (CPNavigationSession)',
    constraints: [
      'Apple-mandated UI components only',
      'Must use MapKit for car display',
      'Strict review process',
      'No background audio mixing',
    ],
    requirements: [
      'CarPlay entitlement from Apple',
      'CPNavigationSession implementation',
      'Siri integration for voice',
      'Dashboard widget support',
    ],
    timeline: 'Phase 2',
  },
  {
    id: 'DIST-003',
    platform: 'OEM Head Unit (Embedded)',
    integrationMethod: 'Native SDK integration or AOSP fork',
    constraints: [
      'Fixed hardware (no updates)',
      'Limited RAM/storage',
      'Custom display resolutions',
      'CAN bus integration required',
      'Long certification cycle (6-12 months)',
    ],
    requirements: [
      'Lightweight SDK (<50MB)',
      'CAN bus adapter layer',
      'Custom rendering pipeline',
      'OTA update mechanism',
      'Crash reporting integration',
    ],
    timeline: 'Phase 3',
  },
  {
    id: 'DIST-004',
    platform: 'SDK / API Platform',
    integrationMethod: 'REST API + Native SDKs (iOS/Android/Web)',
    constraints: [
      'Rate limiting required',
      'API versioning mandatory',
      'SDK size budget (<10MB)',
    ],
    requirements: [
      'API documentation (OpenAPI 3.0)',
      'SDK for iOS, Android, JavaScript',
      'Authentication (API keys + OAuth)',
      'Usage dashboard',
      'Sandbox environment',
    ],
    timeline: 'Phase 2',
  },
  {
    id: 'DIST-005',
    platform: 'Web Application',
    integrationMethod: 'Progressive Web App (PWA)',
    constraints: [
      'Browser GNSS accuracy varies',
      'No background location in most browsers',
      'Limited sensor access',
    ],
    requirements: [
      'Service worker for offline',
      'Web GNSS API integration',
      'Responsive design (mobile-first)',
      'Push notification support',
    ],
    timeline: 'Phase 1',
  },
];

// ═══════════════════════════════════════════════════════════
// MIGRATION STRATEGY
// ═══════════════════════════════════════════════════════════

export type MigrationPhase = 'wrap' | 'shadow' | 'validate' | 'cutover' | 'cleanup';

export interface MigrationStep {
  phase: MigrationPhase;
  order: number;
  name: string;
  description: string;
  rollbackPlan: string;
  validationCriteria: string[];
  duration_estimate: string;
}

export const MIGRATION_STRATEGY: MigrationStep[] = [
  {
    phase: 'wrap',
    order: 1,
    name: 'Wrap Legacy Module',
    description: 'Create adapter layer around existing module. New contract interface wraps old implementation. No behavior change.',
    rollbackPlan: 'Remove adapter, revert to direct calls',
    validationCriteria: [
      'All existing tests pass through adapter',
      'No performance regression (latency within 5%)',
      'Telemetry emitted from adapter layer',
    ],
    duration_estimate: '1-2 weeks per module',
  },
  {
    phase: 'shadow',
    order: 2,
    name: 'Shadow New Implementation',
    description: 'Run new implementation in parallel (shadow mode). Compare outputs against legacy. Log discrepancies.',
    rollbackPlan: 'Disable shadow mode, continue with legacy',
    validationCriteria: [
      'Shadow mode produces identical outputs for >99% of inputs',
      'Discrepancies logged and analyzed',
      'No impact on production latency',
    ],
    duration_estimate: '2-4 weeks per module',
  },
  {
    phase: 'validate',
    order: 3,
    name: 'Validate with Canary',
    description: 'Route 5% of traffic to new implementation. Monitor all SLIs. Compare against legacy cohort.',
    rollbackPlan: 'Route 100% back to legacy within 1 minute',
    validationCriteria: [
      'SLIs within tolerance vs legacy cohort',
      'No increase in error rate',
      'User satisfaction metrics stable',
      'Canary running for minimum 1 week',
    ],
    duration_estimate: '1-2 weeks',
  },
  {
    phase: 'cutover',
    order: 4,
    name: 'Progressive Cutover',
    description: 'Gradually increase traffic to new implementation: 5% → 25% → 50% → 100%. Monitor at each step.',
    rollbackPlan: 'Instant rollback to any previous traffic split',
    validationCriteria: [
      'Each traffic increase stable for 24 hours',
      'All SLOs met at each level',
      'No regression in any metric',
    ],
    duration_estimate: '1-2 weeks',
  },
  {
    phase: 'cleanup',
    order: 5,
    name: 'Remove Legacy',
    description: 'Remove legacy implementation and adapter layer. Update documentation. Archive legacy code.',
    rollbackPlan: 'Restore from version control if needed',
    validationCriteria: [
      'All references to legacy removed',
      'No dead code remaining',
      'Documentation updated',
      'Legacy code archived in separate branch',
    ],
    duration_estimate: '1 week',
  },
];

// ═══════════════════════════════════════════════════════════
// LEGACY BRIDGE LAYER
// ═══════════════════════════════════════════════════════════

export interface LegacyBridge {
  id: string;
  legacySystem: string;
  adapterPattern: 'facade' | 'adapter' | 'proxy' | 'translator' | 'anti_corruption_layer';
  inputFormat: string;
  outputFormat: string;
  transformationRules: string[];
  fallbackBehavior: string;
}

export const LEGACY_BRIDGES: LegacyBridge[] = [
  {
    id: 'LB-001',
    legacySystem: 'Legacy GPS-only positioning',
    adapterPattern: 'anti_corruption_layer',
    inputFormat: 'NMEA sentences',
    outputFormat: 'PositionSample (protobuf)',
    transformationRules: [
      'Parse NMEA GGA/RMC sentences',
      'Convert lat/lon from DDMM.MMMM to decimal degrees',
      'Map fix quality to confidence score',
      'Add default covariance matrix',
    ],
    fallbackBehavior: 'Pass through raw NMEA if parsing fails',
  },
  {
    id: 'LB-002',
    legacySystem: 'Legacy REST routing API',
    adapterPattern: 'adapter',
    inputFormat: 'JSON (legacy schema)',
    outputFormat: 'Route (canonical schema)',
    transformationRules: [
      'Map legacy waypoint format to GeoPoint',
      'Convert time_seconds to eta_seconds',
      'Add default cost_breakdown with zero values',
      'Generate route_id from hash of origin+destination',
    ],
    fallbackBehavior: 'Return error with legacy response attached for debugging',
  },
  {
    id: 'LB-003',
    legacySystem: 'Legacy map tile server',
    adapterPattern: 'proxy',
    inputFormat: 'TMS tile coordinates',
    outputFormat: 'Vector PBF tiles',
    transformationRules: [
      'Convert TMS y-coordinate to standard z/x/y',
      'Rasterize vector tiles on-the-fly if client requires PNG',
      'Add cache headers',
    ],
    fallbackBehavior: 'Serve cached tile or blank tile with error flag',
  },
];

// ═══════════════════════════════════════════════════════════
// REPO STRATEGY
// ═══════════════════════════════════════════════════════════

export interface RepoPackage {
  path: string;
  name: string;
  owner: string;
  type: 'app' | 'service' | 'package' | 'infrastructure' | 'tool' | 'docs';
  dependencies: string[];
  ciRules: string[];
}

export const REPO_STRUCTURE: RepoPackage[] = [
  // Apps
  { path: '/apps/mobile', name: 'Mobile App', owner: 'mobile-team', type: 'app', dependencies: ['contracts', 'schemas', 'ui-core', 'geo-core'], ciRules: ['lint', 'unit', 'integration', 'e2e'] },
  { path: '/apps/web', name: 'Web App', owner: 'web-team', type: 'app', dependencies: ['contracts', 'schemas', 'ui-core'], ciRules: ['lint', 'unit', 'integration'] },
  { path: '/apps/car', name: 'Car App', owner: 'automotive-team', type: 'app', dependencies: ['contracts', 'schemas', 'ui-core', 'geo-core'], ciRules: ['lint', 'unit', 'integration', 'automotive-compliance'] },
  { path: '/apps/desktop', name: 'Desktop App', owner: 'desktop-team', type: 'app', dependencies: ['contracts', 'schemas', 'ui-core'], ciRules: ['lint', 'unit', 'integration'] },

  // Services
  { path: '/services/api-gateway', name: 'API Gateway', owner: 'platform-team', type: 'service', dependencies: ['contracts', 'shared-config'], ciRules: ['lint', 'unit', 'contract', 'load'] },
  { path: '/services/positioning', name: 'Positioning Service', owner: 'core-team', type: 'service', dependencies: ['contracts', 'schemas', 'math-core', 'geo-core'], ciRules: ['lint', 'unit', 'contract', 'simulation', 'replay', 'load', 'chaos'] },
  { path: '/services/fusion', name: 'Fusion Service', owner: 'core-team', type: 'service', dependencies: ['contracts', 'schemas', 'math-core'], ciRules: ['lint', 'unit', 'contract', 'simulation', 'replay'] },
  { path: '/services/routing', name: 'Routing Service', owner: 'core-team', type: 'service', dependencies: ['contracts', 'schemas', 'routing-core', 'geo-core'], ciRules: ['lint', 'unit', 'contract', 'load', 'chaos'] },
  { path: '/services/telemetry', name: 'Telemetry Service', owner: 'platform-team', type: 'service', dependencies: ['contracts', 'schemas'], ciRules: ['lint', 'unit', 'contract', 'load'] },
  { path: '/services/alerts', name: 'Alert Service', owner: 'platform-team', type: 'service', dependencies: ['contracts'], ciRules: ['lint', 'unit', 'contract', 'chaos'] },
  { path: '/services/payments', name: 'Payment Service', owner: 'commerce-team', type: 'service', dependencies: ['contracts'], ciRules: ['lint', 'unit', 'contract', 'security'] },
  { path: '/services/ai-traffic', name: 'AI Traffic Service', owner: 'ai-team', type: 'service', dependencies: ['contracts', 'schemas', 'math-core'], ciRules: ['lint', 'unit', 'contract', 'simulation'] },
  { path: '/services/copilot', name: 'Copilot Service', owner: 'ai-team', type: 'service', dependencies: ['contracts'], ciRules: ['lint', 'unit', 'contract'] },
  { path: '/services/replay', name: 'Replay Service', owner: 'platform-team', type: 'service', dependencies: ['contracts', 'schemas'], ciRules: ['lint', 'unit', 'contract'] },

  // Packages
  { path: '/packages/contracts', name: 'Contracts', owner: 'architecture-team', type: 'package', dependencies: [], ciRules: ['lint', 'unit', 'contract', 'backward-compat'] },
  { path: '/packages/schemas', name: 'Schemas', owner: 'architecture-team', type: 'package', dependencies: ['contracts'], ciRules: ['lint', 'unit', 'backward-compat'] },
  { path: '/packages/math-core', name: 'Math Core', owner: 'core-team', type: 'package', dependencies: [], ciRules: ['lint', 'unit', 'benchmark'] },
  { path: '/packages/geo-core', name: 'Geo Core', owner: 'core-team', type: 'package', dependencies: ['math-core'], ciRules: ['lint', 'unit', 'benchmark'] },
  { path: '/packages/routing-core', name: 'Routing Core', owner: 'core-team', type: 'package', dependencies: ['math-core', 'geo-core'], ciRules: ['lint', 'unit', 'benchmark'] },
  { path: '/packages/ui-core', name: 'UI Core', owner: 'design-team', type: 'package', dependencies: [], ciRules: ['lint', 'unit', 'visual-regression'] },
  { path: '/packages/shared-errors', name: 'Shared Errors', owner: 'architecture-team', type: 'package', dependencies: ['contracts'], ciRules: ['lint', 'unit'] },
  { path: '/packages/shared-config', name: 'Shared Config', owner: 'platform-team', type: 'package', dependencies: [], ciRules: ['lint', 'unit'] },

  // Infrastructure
  { path: '/infrastructure/terraform', name: 'Terraform', owner: 'infra-team', type: 'infrastructure', dependencies: [], ciRules: ['lint', 'plan', 'security-scan'] },
  { path: '/infrastructure/k8s', name: 'Kubernetes', owner: 'infra-team', type: 'infrastructure', dependencies: [], ciRules: ['lint', 'dry-run'] },
  { path: '/infrastructure/monitoring', name: 'Monitoring', owner: 'sre-team', type: 'infrastructure', dependencies: [], ciRules: ['lint', 'config-validate'] },

  // Tools
  { path: '/tools/simulators', name: 'Simulators', owner: 'qa-team', type: 'tool', dependencies: ['contracts', 'schemas'], ciRules: ['lint', 'unit'] },
  { path: '/tools/replayers', name: 'Replayers', owner: 'qa-team', type: 'tool', dependencies: ['contracts', 'schemas'], ciRules: ['lint', 'unit'] },
  { path: '/tools/generators', name: 'Code Generators', owner: 'architecture-team', type: 'tool', dependencies: ['contracts', 'schemas'], ciRules: ['lint', 'unit'] },

  // Docs
  { path: '/docs', name: 'Documentation', owner: 'architecture-team', type: 'docs', dependencies: [], ciRules: ['lint', 'link-check'] },
];

// ═══════════════════════════════════════════════════════════
// CODE GENERATION PIPELINE
// ═══════════════════════════════════════════════════════════

export interface CodeGenStage {
  order: number;
  name: string;
  input: string;
  output: string;
  tool: string;
  validation: string;
}

export const CODE_GENERATION_PIPELINE: CodeGenStage[] = [
  { order: 1, name: 'Schema Definition', input: 'Protobuf / JSON Schema files', output: 'Canonical schema source', tool: 'protoc / ajv', validation: 'Schema lint + backward compat check' },
  { order: 2, name: 'Type Generation', input: 'Canonical schemas', output: 'TypeScript types + Rust structs + Swift codables', tool: 'protoc-gen-ts / quicktype', validation: 'Compile check in all target languages' },
  { order: 3, name: 'Validator Generation', input: 'JSON Schemas', output: 'Runtime validators (Zod schemas)', tool: 'json-schema-to-zod', validation: 'Validate against test corpus' },
  { order: 4, name: 'API Client Generation', input: 'OpenAPI 3.0 spec', output: 'SDK clients (TS, Swift, Kotlin)', tool: 'openapi-generator', validation: 'Integration test against mock server' },
  { order: 5, name: 'Test Scaffold Generation', input: 'Contract definitions', output: 'Test stubs with assertions', tool: 'Custom generator', validation: 'Tests compile and run (may fail initially)' },
  { order: 6, name: 'Documentation Generation', input: 'Schemas + contracts + code comments', output: 'API docs + architecture docs', tool: 'typedoc / redoc', validation: 'Link check + completeness audit' },
];

// ═══════════════════════════════════════════════════════════
// OEM / HEAD-UNIT CONSTRAINTS
// ═══════════════════════════════════════════════════════════

export interface OEMConstraint {
  category: string;
  constraint: string;
  impact: string;
  mitigation: string;
}

export const OEM_CONSTRAINTS: OEMConstraint[] = [
  { category: 'hardware', constraint: 'Fixed RAM (2-4GB shared with OS)', impact: 'Memory budget <200MB for navigation', mitigation: 'Aggressive memory pooling, tile eviction, lazy loading' },
  { category: 'hardware', constraint: 'Low-power SoC (ARM Cortex-A53 class)', impact: 'Limited compute for AI inference', mitigation: 'Pre-computed models, edge-optimized inference (TFLite)' },
  { category: 'display', constraint: 'Fixed resolution (800x480 to 1920x720)', impact: 'Must support multiple aspect ratios', mitigation: 'Responsive layout with breakpoints, vector rendering' },
  { category: 'input', constraint: 'Resistive touch or rotary knob only', impact: 'No multi-touch gestures', mitigation: 'Single-touch UI, knob navigation support, voice-first' },
  { category: 'connectivity', constraint: 'Shared cellular modem with telematics', impact: 'Bandwidth limited, latency variable', mitigation: 'Aggressive caching, delta updates, offline-first' },
  { category: 'update', constraint: 'OTA updates require OEM approval (weeks)', impact: 'Cannot push hotfixes quickly', mitigation: 'Feature flags, server-side config, A/B testing' },
  { category: 'certification', constraint: 'Automotive SPICE / ISO 26262 compliance', impact: 'Code review and testing overhead', mitigation: 'Automated compliance checks in CI, traceability matrix' },
  { category: 'lifecycle', constraint: 'Vehicle lifecycle 10-15 years', impact: 'Must support old software versions long-term', mitigation: 'Strict backward compatibility, version compatibility matrix' },
];

// ═══════════════════════════════════════════════════════════
// HUMAN FACTORS VALIDATION
// ═══════════════════════════════════════════════════════════

export interface HumanFactorsCriterion {
  id: string;
  category: 'glance_time' | 'cognitive_load' | 'distraction' | 'accessibility' | 'safety';
  criterion: string;
  threshold: string;
  testMethod: string;
  standard: string;
}

export const HUMAN_FACTORS_CRITERIA: HumanFactorsCriterion[] = [
  { id: 'HF-001', category: 'glance_time', criterion: 'Single glance duration', threshold: '<2 seconds', testMethod: 'Eye tracking study', standard: 'NHTSA Visual-Manual Guidelines' },
  { id: 'HF-002', category: 'glance_time', criterion: 'Total eyes-off-road time per task', threshold: '<12 seconds', testMethod: 'Eye tracking study', standard: 'NHTSA Visual-Manual Guidelines' },
  { id: 'HF-003', category: 'cognitive_load', criterion: 'Maximum simultaneous information elements', threshold: '<=5 elements', testMethod: 'Cognitive walkthrough', standard: "Miller's Law (7±2)" },
  { id: 'HF-004', category: 'distraction', criterion: 'Task completion without looking at screen', threshold: 'Voice-only for critical tasks', testMethod: 'Driving simulator', standard: 'ISO 15005' },
  { id: 'HF-005', category: 'accessibility', criterion: 'Color contrast ratio', threshold: '>=4.5:1 (AA)', testMethod: 'Automated contrast checker', standard: 'WCAG 2.1 AA' },
  { id: 'HF-006', category: 'accessibility', criterion: 'Touch target size', threshold: '>=44x44 CSS pixels', testMethod: 'Layout inspection', standard: 'WCAG 2.1 AA' },
  { id: 'HF-007', category: 'safety', criterion: 'Critical alert visibility', threshold: 'Visible within 0.5 seconds', testMethod: 'Reaction time study', standard: 'ISO 15006' },
  { id: 'HF-008', category: 'safety', criterion: 'Night mode automatic activation', threshold: 'Within 30 seconds of ambient light change', testMethod: 'Sensor response test', standard: 'Internal standard' },
];

// ═══════════════════════════════════════════════════════════
// LEGAL LIABILITY FRAMEWORK
// ═══════════════════════════════════════════════════════════

export interface LegalRequirement {
  id: string;
  jurisdiction: 'global' | 'eu' | 'us' | 'israel' | 'apac';
  regulation: string;
  requirement: string;
  implementation: string;
  auditFrequency: string;
}

export const LEGAL_FRAMEWORK: LegalRequirement[] = [
  { id: 'LEG-001', jurisdiction: 'eu', regulation: 'GDPR', requirement: 'Right to erasure (Article 17)', implementation: 'Data deletion pipeline with 30-day SLA', auditFrequency: 'Quarterly' },
  { id: 'LEG-002', jurisdiction: 'eu', regulation: 'GDPR', requirement: 'Data portability (Article 20)', implementation: 'Export API in machine-readable format', auditFrequency: 'Quarterly' },
  { id: 'LEG-003', jurisdiction: 'eu', regulation: 'GDPR', requirement: 'Privacy by design (Article 25)', implementation: 'Data minimization, anonymization pipeline', auditFrequency: 'Quarterly' },
  { id: 'LEG-004', jurisdiction: 'eu', regulation: 'ePrivacy Directive', requirement: 'Location data consent', implementation: 'Explicit opt-in for location tracking beyond navigation', auditFrequency: 'Annually' },
  { id: 'LEG-005', jurisdiction: 'us', regulation: 'CCPA/CPRA', requirement: 'Right to know and delete', implementation: 'Privacy dashboard with data access and deletion', auditFrequency: 'Annually' },
  { id: 'LEG-006', jurisdiction: 'global', regulation: 'PCI DSS', requirement: 'Payment card data security', implementation: 'No card data stored; tokenization via payment processor', auditFrequency: 'Annually' },
  { id: 'LEG-007', jurisdiction: 'global', regulation: 'ISO 27001', requirement: 'Information security management', implementation: 'ISMS with documented controls', auditFrequency: 'Annually' },
  { id: 'LEG-008', jurisdiction: 'global', regulation: 'Liability limitation', requirement: 'Navigation accuracy disclaimer', implementation: 'Terms of service + in-app disclaimer for safety-critical decisions', auditFrequency: 'Annually' },
  { id: 'LEG-009', jurisdiction: 'israel', regulation: 'Privacy Protection Law', requirement: 'Database registration with PPA', implementation: 'Register location database with Privacy Protection Authority', auditFrequency: 'Annually' },
  { id: 'LEG-010', jurisdiction: 'global', regulation: 'Audit trail', requirement: 'Immutable audit logs for all admin actions', implementation: 'Append-only audit log with cryptographic chaining', auditFrequency: 'Continuous' },
];

// ═══════════════════════════════════════════════════════════
// EMERGENCY MODE CERTIFICATION
// ═══════════════════════════════════════════════════════════

export interface EmergencyCertification {
  id: string;
  capability: string;
  requirement: string;
  testScenario: string;
  passCriteria: string;
  certificationBody: string;
}

export const EMERGENCY_CERTIFICATIONS: EmergencyCertification[] = [
  { id: 'EMC-001', capability: 'Emergency positioning', requirement: 'Position fix within 50m under total GNSS loss', testScenario: 'Disable all GNSS, verify IMU + cell tower positioning', passCriteria: 'Position error <50m for 10 minutes', certificationBody: 'Internal + third-party audit' },
  { id: 'EMC-002', capability: 'Emergency routing', requirement: 'Route to nearest hospital/police within 3 seconds', testScenario: 'Trigger SOS, measure time to route computation', passCriteria: 'Route ready <3 seconds, includes nearest 3 facilities', certificationBody: 'Internal' },
  { id: 'EMC-003', capability: 'Emergency alerting', requirement: 'Alert dispatch within 200ms of SOS trigger', testScenario: 'Trigger SOS, measure time to alert sent', passCriteria: 'Alert dispatched <200ms, delivered <5 seconds', certificationBody: 'Internal' },
  { id: 'EMC-004', capability: 'Offline emergency', requirement: 'Full emergency mode works without network', testScenario: 'Airplane mode + SOS trigger', passCriteria: 'Route computed from cached data, alert queued for send', certificationBody: 'Internal' },
  { id: 'EMC-005', capability: 'Emergency override', requirement: 'Emergency mode bypasses all non-safety features', testScenario: 'Verify payment gates, personalization, ads all disabled in emergency', passCriteria: 'Zero non-essential features active during emergency', certificationBody: 'Internal' },
];

// ═══════════════════════════════════════════════════════════
// CROSS-PLATFORM RENDERING CONTRACTS
// ═══════════════════════════════════════════════════════════

export interface RenderingContract {
  platform: string;
  renderEngine: string;
  maxFrameTime_ms: number;
  maxMemory_mb: number;
  tileFormat: string;
  fontSystem: string;
  constraints: string[];
}

export const RENDERING_CONTRACTS: RenderingContract[] = [
  { platform: 'mobile_android', renderEngine: 'Skia / Canvas', maxFrameTime_ms: 16, maxMemory_mb: 200, tileFormat: 'Vector PBF', fontSystem: 'System fonts + bundled', constraints: ['GPU may be shared', 'Thermal throttling possible'] },
  { platform: 'mobile_ios', renderEngine: 'Metal / Core Graphics', maxFrameTime_ms: 16, maxMemory_mb: 200, tileFormat: 'Vector PBF', fontSystem: 'System fonts + bundled', constraints: ['Metal required for 3D', 'Memory pressure warnings'] },
  { platform: 'web', renderEngine: 'WebGL / Canvas 2D', maxFrameTime_ms: 16, maxMemory_mb: 300, tileFormat: 'Vector PBF + raster fallback', fontSystem: 'Web fonts (Google Fonts CDN)', constraints: ['Browser compatibility varies', 'No GPU compute'] },
  { platform: 'car_head_unit', renderEngine: 'OpenGL ES 2.0', maxFrameTime_ms: 33, maxMemory_mb: 100, tileFormat: 'Pre-rendered raster', fontSystem: 'Bundled bitmap fonts', constraints: ['Very limited GPU', 'Fixed resolution', 'No shader complexity'] },
  { platform: 'hud_projection', renderEngine: 'Minimal vector', maxFrameTime_ms: 8, maxMemory_mb: 50, tileFormat: 'Simplified vector', fontSystem: 'High-contrast bundled', constraints: ['Monochrome or limited color', 'Extreme simplicity required'] },
];

// ═══════════════════════════════════════════════════════════
// RENDERING PERFORMANCE BUDGETS
// ═══════════════════════════════════════════════════════════

export interface PerformanceBudget {
  metric: string;
  mobile: string;
  web: string;
  car: string;
  hud: string;
}

export const RENDERING_BUDGETS: PerformanceBudget[] = [
  { metric: 'Frame time', mobile: '<16ms', web: '<16ms', car: '<33ms', hud: '<8ms' },
  { metric: 'First contentful paint', mobile: '<1.5s', web: '<2s', car: '<3s', hud: 'N/A' },
  { metric: 'Tile load time', mobile: '<200ms', web: '<300ms', car: '<500ms', hud: 'N/A' },
  { metric: 'Route render time', mobile: '<100ms', web: '<150ms', car: '<200ms', hud: '<50ms' },
  { metric: 'Animation smoothness', mobile: '60fps', web: '60fps', car: '30fps', hud: '120fps' },
  { metric: 'Memory ceiling', mobile: '200MB', web: '300MB', car: '100MB', hud: '50MB' },
  { metric: 'Bundle size', mobile: '<15MB', web: '<3MB', car: '<50MB', hud: '<5MB' },
];

// ═══════════════════════════════════════════════════════════
// DESIGN TOKEN SYSTEM CATALOG
// ═══════════════════════════════════════════════════════════

export interface DesignToken {
  category: string;
  token: string;
  lightValue: string;
  darkValue: string;
  usage: string;
}

export const DESIGN_TOKENS: DesignToken[] = [
  // Colors
  { category: 'color', token: '--color-primary', lightValue: '#0066FF', darkValue: '#4499FF', usage: 'Primary actions, links, active states' },
  { category: 'color', token: '--color-secondary', lightValue: '#6B7280', darkValue: '#9CA3AF', usage: 'Secondary text, borders' },
  { category: 'color', token: '--color-success', lightValue: '#10B981', darkValue: '#34D399', usage: 'Success states, valid positions' },
  { category: 'color', token: '--color-warning', lightValue: '#F59E0B', darkValue: '#FBBF24', usage: 'Warnings, degraded states' },
  { category: 'color', token: '--color-danger', lightValue: '#EF4444', darkValue: '#F87171', usage: 'Errors, critical alerts, SOS' },
  { category: 'color', token: '--color-route', lightValue: '#3B82F6', darkValue: '#60A5FA', usage: 'Active route line' },
  { category: 'color', token: '--color-route-alt', lightValue: '#94A3B8', darkValue: '#64748B', usage: 'Alternative route lines' },
  { category: 'color', token: '--color-traffic-free', lightValue: '#22C55E', darkValue: '#4ADE80', usage: 'Free-flowing traffic' },
  { category: 'color', token: '--color-traffic-slow', lightValue: '#EAB308', darkValue: '#FACC15', usage: 'Slow traffic' },
  { category: 'color', token: '--color-traffic-jam', lightValue: '#EF4444', darkValue: '#F87171', usage: 'Traffic jam' },
  // Typography
  { category: 'typography', token: '--font-family-primary', lightValue: 'Inter, system-ui, sans-serif', darkValue: 'Inter, system-ui, sans-serif', usage: 'Body text, UI elements' },
  { category: 'typography', token: '--font-family-mono', lightValue: 'JetBrains Mono, monospace', darkValue: 'JetBrains Mono, monospace', usage: 'Code, coordinates, technical data' },
  { category: 'typography', token: '--font-size-hud', lightValue: '24px', darkValue: '24px', usage: 'HUD speed, ETA display' },
  { category: 'typography', token: '--font-size-body', lightValue: '14px', darkValue: '14px', usage: 'Body text' },
  { category: 'typography', token: '--font-size-caption', lightValue: '12px', darkValue: '12px', usage: 'Captions, metadata' },
  // Spacing
  { category: 'spacing', token: '--space-xs', lightValue: '4px', darkValue: '4px', usage: 'Tight spacing' },
  { category: 'spacing', token: '--space-sm', lightValue: '8px', darkValue: '8px', usage: 'Small gaps' },
  { category: 'spacing', token: '--space-md', lightValue: '16px', darkValue: '16px', usage: 'Standard spacing' },
  { category: 'spacing', token: '--space-lg', lightValue: '24px', darkValue: '24px', usage: 'Section spacing' },
  { category: 'spacing', token: '--space-xl', lightValue: '32px', darkValue: '32px', usage: 'Large gaps' },
  // Shadows
  { category: 'shadow', token: '--shadow-card', lightValue: '0 1px 3px rgba(0,0,0,0.12)', darkValue: '0 1px 3px rgba(0,0,0,0.4)', usage: 'Card elevation' },
  { category: 'shadow', token: '--shadow-panel', lightValue: '0 4px 12px rgba(0,0,0,0.08)', darkValue: '0 4px 12px rgba(0,0,0,0.3)', usage: 'Panel elevation' },
  { category: 'shadow', token: '--shadow-modal', lightValue: '0 8px 24px rgba(0,0,0,0.15)', darkValue: '0 8px 24px rgba(0,0,0,0.5)', usage: 'Modal overlay' },
  // Borders
  { category: 'border', token: '--radius-sm', lightValue: '4px', darkValue: '4px', usage: 'Small elements (badges, chips)' },
  { category: 'border', token: '--radius-md', lightValue: '8px', darkValue: '8px', usage: 'Cards, inputs' },
  { category: 'border', token: '--radius-lg', lightValue: '12px', darkValue: '12px', usage: 'Panels, modals' },
  { category: 'border', token: '--radius-full', lightValue: '9999px', darkValue: '9999px', usage: 'Circular elements' },
  // Animation
  { category: 'animation', token: '--duration-fast', lightValue: '150ms', darkValue: '150ms', usage: 'Hover, focus states' },
  { category: 'animation', token: '--duration-normal', lightValue: '300ms', darkValue: '300ms', usage: 'Transitions, panel open/close' },
  { category: 'animation', token: '--duration-slow', lightValue: '500ms', darkValue: '500ms', usage: 'Page transitions' },
  { category: 'animation', token: '--easing-default', lightValue: 'cubic-bezier(0.4, 0, 0.2, 1)', darkValue: 'cubic-bezier(0.4, 0, 0.2, 1)', usage: 'Standard easing' },
];

// ═══════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════

export function getDependencyViolations(source: string): DependencyRule[] {
  return DEPENDENCY_LOCK_MATRIX.filter(d => d.source === source && d.relationship === 'must_not_depend_on');
}

export function getRepoPackagesByType(type: RepoPackage['type']): RepoPackage[] {
  return REPO_STRUCTURE.filter(p => p.type === type);
}

export function getTierByPrice(maxPrice: number): SubscriptionTier[] {
  return SUBSCRIPTION_TIERS.filter(t => t.price_monthly_usd >= 0 && t.price_monthly_usd <= maxPrice);
}

export function getConstraintsByCategory(category: string): OEMConstraint[] {
  return OEM_CONSTRAINTS.filter(c => c.category === category);
}

export function getLegalByJurisdiction(jurisdiction: LegalRequirement['jurisdiction']): LegalRequirement[] {
  return LEGAL_FRAMEWORK.filter(l => l.jurisdiction === jurisdiction);
}

export function getMigrationByPhase(phase: MigrationPhase): MigrationStep | undefined {
  return MIGRATION_STRATEGY.find(s => s.phase === phase);
}
