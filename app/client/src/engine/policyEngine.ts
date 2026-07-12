/**
 * Policy Engine — Routing Rules DSL, Emergency Overrides, Compliance Constraints
 * Spec reference: P2 Architecture Hardening
 *
 * A declarative rule engine that evaluates routing policies against trip context.
 * Supports:
 *   - DSL-based rule definitions (conditions → actions)
 *   - Priority-based conflict resolution
 *   - Emergency override system (SOS bypasses all normal rules)
 *   - Compliance constraints (speed limits, restricted zones, vehicle class)
 *   - Hot-reload (update rules without restart)
 *   - Audit trail for every evaluation
 */

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

export type ComparisonOp = 'eq' | 'neq' | 'gt' | 'gte' | 'lt' | 'lte' | 'in' | 'notIn' | 'contains' | 'between' | 'matches';
export type LogicalOp = 'and' | 'or' | 'not';
export type RuleAction = 'allow' | 'deny' | 'modify' | 'reroute' | 'alert' | 'throttle' | 'log';
export type RulePriority = 'emergency' | 'critical' | 'high' | 'medium' | 'low' | 'default';
export type VehicleClass = 'car' | 'truck' | 'bus' | 'motorcycle' | 'emergency' | 'bicycle' | 'pedestrian' | 'hazmat';
export type ComplianceRegion = 'EU' | 'US' | 'IL' | 'JP' | 'IN' | 'CN' | 'GLOBAL';

/** A single condition in the DSL */
export interface Condition {
  field: string;           // dot-notation path: "vehicle.class", "route.tollCost", "time.hour"
  op: ComparisonOp;
  value: unknown;          // compared against the field
  negate?: boolean;
}

/** Compound condition (AND/OR/NOT groups) */
export interface ConditionGroup {
  logic: LogicalOp;
  conditions: Array<Condition | ConditionGroup>;
}

/** Action to take when rule matches */
export interface PolicyAction {
  type: RuleAction;
  params?: Record<string, unknown>;
  message?: string;        // human-readable explanation
  messageHe?: string;      // Hebrew translation
}

/** A single policy rule */
export interface PolicyRule {
  id: string;
  name: string;
  nameHe?: string;
  description?: string;
  priority: RulePriority;
  enabled: boolean;
  conditions: ConditionGroup;
  actions: PolicyAction[];
  tags?: string[];
  region?: ComplianceRegion;
  vehicleClasses?: VehicleClass[];
  validFrom?: number;      // UTC timestamp
  validUntil?: number;     // UTC timestamp
  version: number;
  createdAt: number;
  updatedAt: number;
}

/** Context passed to the engine for evaluation */
export interface PolicyContext {
  vehicle: {
    class: VehicleClass;
    weight?: number;        // kg
    height?: number;        // meters
    length?: number;        // meters
    fuelType?: string;
    emissionClass?: string;
    hasHazmat?: boolean;
  };
  route: {
    origin: { lat: number; lon: number };
    destination: { lat: number; lon: number };
    distanceKm?: number;
    durationMin?: number;
    tollCost?: number;
    hasTunnels?: boolean;
    hasBridges?: boolean;
    hasUnpavedRoads?: boolean;
    maxGradient?: number;   // percent
    zones?: string[];       // zone IDs the route passes through
  };
  driver: {
    id?: string;
    licenseClass?: string;
    experienceYears?: number;
    score?: number;         // 0-100 driver score
    violations?: number;
  };
  time: {
    hour: number;           // 0-23
    dayOfWeek: number;      // 0=Sunday
    isHoliday?: boolean;
    isRushHour?: boolean;
  };
  environment: {
    weather?: string;
    visibility?: string;
    temperature?: number;
    roadCondition?: string;
  };
  emergency?: {
    active: boolean;
    type?: string;
    level?: number;         // 1-5
  };
  custom?: Record<string, unknown>;
}

/** Result of evaluating a single rule */
export interface RuleEvaluation {
  ruleId: string;
  ruleName: string;
  matched: boolean;
  actions: PolicyAction[];
  priority: RulePriority;
  timestamp: number;
  durationMs: number;
}

/** Result of evaluating all rules */
export interface PolicyEvalResult {
  allowed: boolean;
  appliedRules: RuleEvaluation[];
  deniedBy?: string;       // rule ID that denied
  modifications: Record<string, unknown>;
  alerts: string[];
  evaluationMs: number;
  ruleCount: number;
  matchCount: number;
}

/** Audit log entry */
export interface PolicyAuditEntry {
  id: string;
  timestamp: number;
  contextHash: string;
  result: PolicyEvalResult;
  emergencyOverride: boolean;
}

/** Engine configuration */
export interface PolicyEngineConfig {
  maxRules: number;
  auditLogSize: number;
  enableEmergencyOverride: boolean;
  defaultAction: RuleAction;
  evaluationTimeoutMs: number;
  enableCaching: boolean;
  cacheMaxAge: number;
}

// ═══════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════

const PRIORITY_ORDER: Record<RulePriority, number> = {
  emergency: 0,
  critical: 1,
  high: 2,
  medium: 3,
  low: 4,
  default: 5,
};

const DEFAULTS: PolicyEngineConfig = {
  maxRules: 10000,
  auditLogSize: 1000,
  enableEmergencyOverride: true,
  defaultAction: 'allow',
  evaluationTimeoutMs: 50,
  enableCaching: true,
  cacheMaxAge: 30_000,
};

// ═══════════════════════════════════════════════════════════
// BUILT-IN COMPLIANCE RULES
// ═══════════════════════════════════════════════════════════

function createComplianceRules(): PolicyRule[] {
  const now = Date.now();
  return [
    // Speed limit enforcement
    {
      id: 'compliance-speed-urban',
      name: 'Urban Speed Limit',
      nameHe: 'מגבלת מהירות עירונית',
      description: 'Enforce 50 km/h in urban zones',
      priority: 'high',
      enabled: true,
      conditions: {
        logic: 'and',
        conditions: [
          { field: 'route.zones', op: 'contains', value: 'urban' },
        ],
      },
      actions: [
        { type: 'modify', params: { maxSpeedKmh: 50 }, message: 'Urban zone: max 50 km/h', messageHe: 'אזור עירוני: מקסימום 50 קמ"ש' },
      ],
      region: 'GLOBAL',
      version: 1,
      createdAt: now,
      updatedAt: now,
    },
    // Hazmat tunnel restriction
    {
      id: 'compliance-hazmat-tunnel',
      name: 'Hazmat Tunnel Ban',
      nameHe: 'איסור חומ"ס במנהרות',
      description: 'Hazmat vehicles cannot use tunnels',
      priority: 'critical',
      enabled: true,
      conditions: {
        logic: 'and',
        conditions: [
          { field: 'vehicle.hasHazmat', op: 'eq', value: true },
          { field: 'route.hasTunnels', op: 'eq', value: true },
        ],
      },
      actions: [
        { type: 'deny', message: 'Hazmat vehicles prohibited in tunnels', messageHe: 'רכבי חומ"ס אסורים במנהרות' },
        { type: 'reroute', params: { avoidTunnels: true }, message: 'Rerouting to avoid tunnels' },
      ],
      vehicleClasses: ['hazmat', 'truck'],
      region: 'GLOBAL',
      version: 1,
      createdAt: now,
      updatedAt: now,
    },
    // Truck height restriction
    {
      id: 'compliance-truck-height',
      name: 'Vehicle Height Restriction',
      nameHe: 'הגבלת גובה רכב',
      description: 'Vehicles over 4m cannot use low-clearance routes',
      priority: 'high',
      enabled: true,
      conditions: {
        logic: 'and',
        conditions: [
          { field: 'vehicle.height', op: 'gt', value: 4.0 },
          { field: 'route.hasTunnels', op: 'eq', value: true },
        ],
      },
      actions: [
        { type: 'deny', message: 'Vehicle too tall for this route', messageHe: 'הרכב גבוה מדי למסלול זה' },
        { type: 'reroute', params: { avoidLowClearance: true } },
      ],
      vehicleClasses: ['truck', 'bus'],
      region: 'GLOBAL',
      version: 1,
      createdAt: now,
      updatedAt: now,
    },
    // Night driving restriction for new drivers
    {
      id: 'compliance-night-new-driver',
      name: 'Night Driving Restriction',
      nameHe: 'הגבלת נהיגת לילה',
      description: 'New drivers (< 2 years) restricted between 00:00-05:00',
      priority: 'medium',
      enabled: true,
      conditions: {
        logic: 'and',
        conditions: [
          { field: 'driver.experienceYears', op: 'lt', value: 2 },
          { field: 'time.hour', op: 'between', value: [0, 5] },
        ],
      },
      actions: [
        { type: 'alert', message: 'Night driving restriction for new drivers', messageHe: 'הגבלת נהיגת לילה לנהגים חדשים' },
        { type: 'log', params: { category: 'compliance' } },
      ],
      region: 'IL',
      version: 1,
      createdAt: now,
      updatedAt: now,
    },
    // Emergency vehicle priority
    {
      id: 'compliance-emergency-priority',
      name: 'Emergency Vehicle Priority',
      nameHe: 'עדיפות רכב חירום',
      description: 'Emergency vehicles bypass all routing restrictions',
      priority: 'emergency',
      enabled: true,
      conditions: {
        logic: 'and',
        conditions: [
          { field: 'vehicle.class', op: 'eq', value: 'emergency' },
          { field: 'emergency.active', op: 'eq', value: true },
        ],
      },
      actions: [
        { type: 'allow', message: 'Emergency override active', messageHe: 'עקיפת חירום פעילה' },
        { type: 'modify', params: { ignoreTraffic: true, ignoreTolls: true, ignoreRestrictions: true } },
      ],
      region: 'GLOBAL',
      version: 1,
      createdAt: now,
      updatedAt: now,
    },
    // Weather-based speed reduction
    {
      id: 'compliance-weather-speed',
      name: 'Weather Speed Reduction',
      nameHe: 'הפחתת מהירות מזג אוויר',
      description: 'Reduce speed limits in adverse weather',
      priority: 'high',
      enabled: true,
      conditions: {
        logic: 'or',
        conditions: [
          { field: 'environment.weather', op: 'in', value: ['rain', 'snow', 'ice', 'fog'] },
          { field: 'environment.visibility', op: 'eq', value: 'poor' },
        ],
      },
      actions: [
        { type: 'modify', params: { speedReductionPercent: 20 }, message: 'Speed reduced due to weather', messageHe: 'מהירות מופחתת בגלל מזג אוויר' },
        { type: 'alert', message: 'Adverse weather conditions detected' },
      ],
      region: 'GLOBAL',
      version: 1,
      createdAt: now,
      updatedAt: now,
    },
  ];
}

// ═══════════════════════════════════════════════════════════
// DSL PARSER
// ═══════════════════════════════════════════════════════════

/** Parse a DSL string into a PolicyRule */
export function parseDSL(dsl: string): PolicyRule {
  const lines = dsl.trim().split('\n').map(l => l.trim()).filter(l => l && !l.startsWith('#'));
  const rule: Partial<PolicyRule> = {
    id: `rule-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
    enabled: true,
    priority: 'medium',
    version: 1,
    createdAt: Date.now(),
    updatedAt: Date.now(),
    actions: [],
  };

  const conditions: Condition[] = [];
  let logic: LogicalOp = 'and';

  for (const line of lines) {
    // RULE name
    if (line.startsWith('RULE ')) {
      rule.name = line.slice(5).replace(/['"]/g, '');
      continue;
    }
    // PRIORITY level
    if (line.startsWith('PRIORITY ')) {
      rule.priority = line.slice(9).toLowerCase() as RulePriority;
      continue;
    }
    // REGION code
    if (line.startsWith('REGION ')) {
      rule.region = line.slice(7).toUpperCase() as ComplianceRegion;
      continue;
    }
    // VEHICLE classes
    if (line.startsWith('VEHICLE ')) {
      rule.vehicleClasses = line.slice(8).split(',').map(v => v.trim() as VehicleClass);
      continue;
    }
    // TAGS
    if (line.startsWith('TAGS ')) {
      rule.tags = line.slice(5).split(',').map(t => t.trim());
      continue;
    }
    // LOGIC operator
    if (line === 'ANY' || line === 'OR') { logic = 'or'; continue; }
    if (line === 'ALL' || line === 'AND') { logic = 'and'; continue; }
    // WHEN condition
    if (line.startsWith('WHEN ')) {
      const cond = parseCondition(line.slice(5));
      if (cond) conditions.push(cond);
      continue;
    }
    // THEN action
    if (line.startsWith('THEN ')) {
      const action = parseAction(line.slice(5));
      if (action) (rule.actions as PolicyAction[]).push(action);
      continue;
    }
  }

  rule.conditions = { logic, conditions };
  return rule as PolicyRule;
}

function parseCondition(text: string): Condition | null {
  // Pattern: field OP value
  const ops: Array<{ token: string; op: ComparisonOp }> = [
    { token: ' NOT IN ', op: 'notIn' },
    { token: ' BETWEEN ', op: 'between' },
    { token: ' CONTAINS ', op: 'contains' },
    { token: ' MATCHES ', op: 'matches' },
    { token: ' IN ', op: 'in' },
    { token: ' >= ', op: 'gte' },
    { token: ' <= ', op: 'lte' },
    { token: ' != ', op: 'neq' },
    { token: ' > ', op: 'gt' },
    { token: ' < ', op: 'lt' },
    { token: ' = ', op: 'eq' },
  ];

  for (const { token, op } of ops) {
    const idx = text.toUpperCase().indexOf(token);
    if (idx >= 0) {
      const field = text.slice(0, idx).trim();
      const rawValue = text.slice(idx + token.length).trim();
      return { field, op, value: parseValue(rawValue) };
    }
  }
  return null;
}

function parseValue(raw: string): unknown {
  // Boolean
  if (raw === 'true') return true;
  if (raw === 'false') return false;
  // Number
  if (/^-?\d+(\.\d+)?$/.test(raw)) return parseFloat(raw);
  // Array: [a, b, c]
  if (raw.startsWith('[') && raw.endsWith(']')) {
    return raw.slice(1, -1).split(',').map(v => parseValue(v.trim()));
  }
  // String (strip quotes)
  if ((raw.startsWith('"') && raw.endsWith('"')) || (raw.startsWith("'") && raw.endsWith("'"))) {
    return raw.slice(1, -1);
  }
  return raw;
}

function parseAction(text: string): PolicyAction | null {
  const upper = text.toUpperCase();
  if (upper.startsWith('ALLOW')) return { type: 'allow', message: text.slice(6).trim() || undefined };
  if (upper.startsWith('DENY')) return { type: 'deny', message: text.slice(5).trim() || undefined };
  if (upper.startsWith('ALERT')) return { type: 'alert', message: text.slice(6).trim() || undefined };
  if (upper.startsWith('LOG')) return { type: 'log', message: text.slice(4).trim() || undefined };
  if (upper.startsWith('THROTTLE')) return { type: 'throttle', message: text.slice(9).trim() || undefined };
  if (upper.startsWith('REROUTE')) {
    const params: Record<string, unknown> = {};
    const paramStr = text.slice(8).trim();
    if (paramStr) {
      paramStr.split(',').forEach(p => {
        const [k, v] = p.split('=').map(s => s.trim());
        if (k && v) params[k] = parseValue(v);
      });
    }
    return { type: 'reroute', params };
  }
  if (upper.startsWith('MODIFY')) {
    const params: Record<string, unknown> = {};
    const paramStr = text.slice(7).trim();
    if (paramStr) {
      paramStr.split(',').forEach(p => {
        const [k, v] = p.split('=').map(s => s.trim());
        if (k && v) params[k] = parseValue(v);
      });
    }
    return { type: 'modify', params };
  }
  return null;
}

// ═══════════════════════════════════════════════════════════
// CONDITION EVALUATOR
// ═══════════════════════════════════════════════════════════

function getNestedValue(obj: Record<string, unknown>, path: string): unknown {
  const parts = path.split('.');
  let current: unknown = obj;
  for (const part of parts) {
    if (current == null || typeof current !== 'object') return undefined;
    current = (current as Record<string, unknown>)[part];
  }
  return current;
}

function evaluateCondition(condition: Condition, context: Record<string, unknown>): boolean {
  const fieldValue = getNestedValue(context, condition.field);
  const target = condition.value;
  let result: boolean;

  switch (condition.op) {
    case 'eq':
      result = fieldValue === target;
      break;
    case 'neq':
      result = fieldValue !== target;
      break;
    case 'gt':
      result = typeof fieldValue === 'number' && typeof target === 'number' && fieldValue > target;
      break;
    case 'gte':
      result = typeof fieldValue === 'number' && typeof target === 'number' && fieldValue >= target;
      break;
    case 'lt':
      result = typeof fieldValue === 'number' && typeof target === 'number' && fieldValue < target;
      break;
    case 'lte':
      result = typeof fieldValue === 'number' && typeof target === 'number' && fieldValue <= target;
      break;
    case 'in':
      result = Array.isArray(target) && target.includes(fieldValue);
      break;
    case 'notIn':
      result = Array.isArray(target) && !target.includes(fieldValue);
      break;
    case 'contains':
      if (Array.isArray(fieldValue)) {
        result = fieldValue.includes(target);
      } else if (typeof fieldValue === 'string' && typeof target === 'string') {
        result = fieldValue.includes(target);
      } else {
        result = false;
      }
      break;
    case 'between':
      if (typeof fieldValue === 'number' && Array.isArray(target) && target.length === 2) {
        result = fieldValue >= (target[0] as number) && fieldValue <= (target[1] as number);
      } else {
        result = false;
      }
      break;
    case 'matches':
      if (typeof fieldValue === 'string' && typeof target === 'string') {
        try {
          result = new RegExp(target).test(fieldValue);
        } catch {
          result = false;
        }
      } else {
        result = false;
      }
      break;
    default:
      result = false;
  }

  return condition.negate ? !result : result;
}

function evaluateConditionGroup(group: ConditionGroup, context: Record<string, unknown>): boolean {
  if (group.logic === 'not') {
    // NOT: negate the first condition/group
    if (group.conditions.length === 0) return true;
    const first = group.conditions[0];
    if ('logic' in first) {
      return !evaluateConditionGroup(first, context);
    }
    return !evaluateCondition(first, context);
  }

  const evaluator = group.logic === 'or'
    ? (items: Array<Condition | ConditionGroup>) => items.some(c =>
        'logic' in c ? evaluateConditionGroup(c, context) : evaluateCondition(c, context))
    : (items: Array<Condition | ConditionGroup>) => items.every(c =>
        'logic' in c ? evaluateConditionGroup(c, context) : evaluateCondition(c, context));

  return evaluator(group.conditions);
}

// ═══════════════════════════════════════════════════════════
// POLICY ENGINE
// ═══════════════════════════════════════════════════════════

export class PolicyEngine {
  private config: PolicyEngineConfig;
  private rules: Map<string, PolicyRule> = new Map();
  private auditLog: PolicyAuditEntry[] = [];
  private cache: Map<string, { result: PolicyEvalResult; expiry: number }> = new Map();
  private listeners = new Map<string, Set<(data: unknown) => void>>();
  private emergencyOverrideActive = false;
  private emergencyOverrideReason = '';

  constructor(config: Partial<PolicyEngineConfig> = {}) {
    this.config = { ...DEFAULTS, ...config };
    // Load built-in compliance rules
    for (const rule of createComplianceRules()) {
      this.rules.set(rule.id, rule);
    }
  }

  // ── Rule Management ──────────────────────────────────

  addRule(rule: PolicyRule): void {
    if (this.rules.size >= this.config.maxRules) {
      throw new Error(`Maximum rule count (${this.config.maxRules}) reached`);
    }
    rule.updatedAt = Date.now();
    this.rules.set(rule.id, rule);
    this.clearCache();
    this.emit('rule:added', { ruleId: rule.id });
  }

  addRuleFromDSL(dsl: string): PolicyRule {
    const rule = parseDSL(dsl);
    this.addRule(rule);
    return rule;
  }

  removeRule(ruleId: string): boolean {
    const removed = this.rules.delete(ruleId);
    if (removed) {
      this.clearCache();
      this.emit('rule:removed', { ruleId });
    }
    return removed;
  }

  updateRule(ruleId: string, updates: Partial<PolicyRule>): boolean {
    const existing = this.rules.get(ruleId);
    if (!existing) return false;
    const updated = { ...existing, ...updates, id: ruleId, updatedAt: Date.now(), version: existing.version + 1 };
    this.rules.set(ruleId, updated);
    this.clearCache();
    this.emit('rule:updated', { ruleId });
    return true;
  }

  enableRule(ruleId: string): boolean {
    return this.updateRule(ruleId, { enabled: true });
  }

  disableRule(ruleId: string): boolean {
    return this.updateRule(ruleId, { enabled: false });
  }

  getRule(ruleId: string): PolicyRule | undefined {
    return this.rules.get(ruleId);
  }

  getAllRules(): PolicyRule[] {
    return Array.from(this.rules.values());
  }

  getRulesByPriority(priority: RulePriority): PolicyRule[] {
    return Array.from(this.rules.values()).filter(r => r.priority === priority);
  }

  getRulesByTag(tag: string): PolicyRule[] {
    return Array.from(this.rules.values()).filter(r => r.tags?.includes(tag));
  }

  getRulesByRegion(region: ComplianceRegion): PolicyRule[] {
    return Array.from(this.rules.values()).filter(r => r.region === region || r.region === 'GLOBAL');
  }

  /** Hot-reload: replace all rules atomically */
  hotReload(rules: PolicyRule[]): void {
    this.rules.clear();
    for (const rule of rules) {
      this.rules.set(rule.id, rule);
    }
    this.clearCache();
    this.emit('rules:reloaded', { count: rules.length });
  }

  // ── Emergency Override ───────────────────────────────

  activateEmergencyOverride(reason: string): void {
    this.emergencyOverrideActive = true;
    this.emergencyOverrideReason = reason;
    this.clearCache();
    this.emit('emergency:activated', { reason });
  }

  deactivateEmergencyOverride(): void {
    this.emergencyOverrideActive = false;
    this.emergencyOverrideReason = '';
    this.clearCache();
    this.emit('emergency:deactivated', {});
  }

  isEmergencyOverrideActive(): boolean {
    return this.emergencyOverrideActive;
  }

  // ── Evaluation ───────────────────────────────────────

  evaluate(context: PolicyContext): PolicyEvalResult {
    const start = performance.now();

    // Emergency override: bypass all rules
    if (this.config.enableEmergencyOverride && this.emergencyOverrideActive) {
      const result: PolicyEvalResult = {
        allowed: true,
        appliedRules: [],
        modifications: { emergencyOverride: true, reason: this.emergencyOverrideReason },
        alerts: [`Emergency override active: ${this.emergencyOverrideReason}`],
        evaluationMs: performance.now() - start,
        ruleCount: this.rules.size,
        matchCount: 0,
      };
      this.addAuditEntry(context, result, true);
      return result;
    }

    // Check cache
    if (this.config.enableCaching) {
      const cacheKey = this.computeCacheKey(context);
      const cached = this.cache.get(cacheKey);
      if (cached && cached.expiry > Date.now()) {
        return cached.result;
      }
    }

    // Get active rules, sorted by priority
    const now = Date.now();
    const activeRules = Array.from(this.rules.values())
      .filter(r => {
        if (!r.enabled) return false;
        if (r.validFrom && now < r.validFrom) return false;
        if (r.validUntil && now > r.validUntil) return false;
        if (r.vehicleClasses && r.vehicleClasses.length > 0 && !r.vehicleClasses.includes(context.vehicle.class)) return false;
        return true;
      })
      .sort((a, b) => PRIORITY_ORDER[a.priority] - PRIORITY_ORDER[b.priority]);

    const contextObj = context as unknown as Record<string, unknown>;
    const appliedRules: RuleEvaluation[] = [];
    let allowed = this.config.defaultAction === 'allow';
    let deniedBy: string | undefined;
    const modifications: Record<string, unknown> = {};
    const alerts: string[] = [];

    for (const rule of activeRules) {
      const ruleStart = performance.now();
      const matched = evaluateConditionGroup(rule.conditions, contextObj);
      const ruleMs = performance.now() - ruleStart;

      const evaluation: RuleEvaluation = {
        ruleId: rule.id,
        ruleName: rule.name,
        matched,
        actions: matched ? rule.actions : [],
        priority: rule.priority,
        timestamp: Date.now(),
        durationMs: ruleMs,
      };
      appliedRules.push(evaluation);

      if (!matched) continue;

      // Process actions
      for (const action of rule.actions) {
        switch (action.type) {
          case 'deny':
            allowed = false;
            deniedBy = rule.id;
            if (action.message) alerts.push(action.message);
            break;
          case 'allow':
            allowed = true;
            break;
          case 'modify':
            if (action.params) Object.assign(modifications, action.params);
            break;
          case 'reroute':
            modifications._reroute = action.params || {};
            if (action.message) alerts.push(action.message);
            break;
          case 'alert':
            if (action.message) alerts.push(action.message);
            break;
          case 'throttle':
            modifications._throttle = action.params || {};
            break;
          case 'log':
            // Logged in audit trail
            break;
        }
      }
    }

    const result: PolicyEvalResult = {
      allowed,
      appliedRules,
      deniedBy,
      modifications,
      alerts,
      evaluationMs: performance.now() - start,
      ruleCount: activeRules.length,
      matchCount: appliedRules.filter(r => r.matched).length,
    };

    // Cache result
    if (this.config.enableCaching) {
      const cacheKey = this.computeCacheKey(context);
      this.cache.set(cacheKey, { result, expiry: Date.now() + this.config.cacheMaxAge });
    }

    this.addAuditEntry(context, result, false);
    this.emit('evaluation:complete', result);

    return result;
  }

  /** Evaluate a single rule against context (for testing/debugging) */
  evaluateRule(ruleId: string, context: PolicyContext): RuleEvaluation | null {
    const rule = this.rules.get(ruleId);
    if (!rule) return null;

    const start = performance.now();
    const contextObj = context as unknown as Record<string, unknown>;
    const matched = evaluateConditionGroup(rule.conditions, contextObj);

    return {
      ruleId: rule.id,
      ruleName: rule.name,
      matched,
      actions: matched ? rule.actions : [],
      priority: rule.priority,
      timestamp: Date.now(),
      durationMs: performance.now() - start,
    };
  }

  // ── Audit ────────────────────────────────────────────

  getAuditLog(): PolicyAuditEntry[] {
    return [...this.auditLog];
  }

  clearAuditLog(): void {
    this.auditLog = [];
  }

  // ── Statistics ───────────────────────────────────────

  getStats(): {
    totalRules: number;
    enabledRules: number;
    disabledRules: number;
    byPriority: Record<string, number>;
    byRegion: Record<string, number>;
    auditEntries: number;
    cacheSize: number;
    emergencyOverride: boolean;
  } {
    const rules = Array.from(this.rules.values());
    const byPriority: Record<string, number> = {};
    const byRegion: Record<string, number> = {};

    for (const rule of rules) {
      byPriority[rule.priority] = (byPriority[rule.priority] || 0) + 1;
      const region = rule.region || 'GLOBAL';
      byRegion[region] = (byRegion[region] || 0) + 1;
    }

    return {
      totalRules: rules.length,
      enabledRules: rules.filter(r => r.enabled).length,
      disabledRules: rules.filter(r => !r.enabled).length,
      byPriority,
      byRegion,
      auditEntries: this.auditLog.length,
      cacheSize: this.cache.size,
      emergencyOverride: this.emergencyOverrideActive,
    };
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
    this.rules.clear();
    this.auditLog = [];
    this.cache.clear();
    this.listeners.clear();
    this.emergencyOverrideActive = false;
  }

  // ── Private ──────────────────────────────────────────

  private emit(event: string, data: unknown): void {
    const handlers = this.listeners.get(event);
    if (handlers) {
      for (const handler of Array.from(handlers)) {
        try { handler(data); } catch { /* swallow */ }
      }
    }
  }

  private addAuditEntry(context: PolicyContext, result: PolicyEvalResult, emergencyOverride: boolean): void {
    const entry: PolicyAuditEntry = {
      id: `audit-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      timestamp: Date.now(),
      contextHash: this.hashContext(context),
      result,
      emergencyOverride,
    };
    this.auditLog.push(entry);
    if (this.auditLog.length > this.config.auditLogSize) {
      this.auditLog = this.auditLog.slice(-this.config.auditLogSize);
    }
  }

  private hashContext(context: PolicyContext): string {
    const key = `${context.vehicle.class}:${context.route.origin.lat},${context.route.origin.lon}:${context.time.hour}`;
    let hash = 0;
    for (let i = 0; i < key.length; i++) {
      hash = ((hash << 5) - hash + key.charCodeAt(i)) | 0;
    }
    return Math.abs(hash).toString(36);
  }

  private computeCacheKey(context: PolicyContext): string {
    return this.hashContext(context) + ':' + this.rules.size;
  }

  private clearCache(): void {
    this.cache.clear();
  }
}

// ═══════════════════════════════════════════════════════════
// SINGLETON
// ═══════════════════════════════════════════════════════════

let _instance: PolicyEngine | null = null;

export function getPolicyEngine(config?: Partial<PolicyEngineConfig>): PolicyEngine {
  if (!_instance) {
    _instance = new PolicyEngine(config);
  }
  return _instance;
}

export function resetPolicyEngine(): void {
  _instance?.destroy();
  _instance = null;
}
