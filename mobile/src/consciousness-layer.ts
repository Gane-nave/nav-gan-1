// ═══════════════════════════════════════════════════════════════════════════
// G.A.N.E CONSCIOUSNESS LAYER — System Governance & Self-Awareness
// Meta-layer that oversees ALL other subsystems. Not a peer — a supervisor.
// Prevents the system from lying to itself, making unsafe decisions, or
// operating beyond its limits.
// ═══════════════════════════════════════════════════════════════════════════

export type SystemPhase = 'HEALTHY'|'CAUTIOUS'|'DEGRADED'|'RESTRICTED'|'FAIL_SAFE';
export type TruthSource = 'GNSS'|'FUSION'|'MAP'|'INS'|'AI'|'USER'|'POLICY';

/* ─── 1. UNIFIED SYSTEM STATE ──────────────────────────────────────
   Single Source of Operational Truth. Synthesizes every subsystem. */
export interface SubsystemStatus {
  name:string; health:'OK'|'DEGRADED'|'FAIL'|'UNKNOWN';
  readiness:number; lastHeartbeat:number; dependencies:string[];
}
export class UnifiedSystemState {
  subsystems = new Map<string, SubsystemStatus>();
  phase: SystemPhase = 'HEALTHY';
  lastSynthesis = Date.now();
  register(name:string, deps:string[]=[]) {
    this.subsystems.set(name, {name, health:'UNKNOWN', readiness:0, lastHeartbeat:Date.now(), dependencies:deps});
  }
  update(name:string, health:SubsystemStatus['health'], readiness:number) {
    const s = this.subsystems.get(name); if (!s) return;
    s.health = health; s.readiness = readiness; s.lastHeartbeat = Date.now();
  }
  /** Compute global phase based on all subsystems */
  synthesize(): SystemPhase {
    const now = Date.now();
    const subs = [...this.subsystems.values()];
    if (!subs.length) return 'HEALTHY';
    // Stale subsystems (>30s no heartbeat) count as FAIL
    const fails = subs.filter(s => s.health === 'FAIL' || now - s.lastHeartbeat > 30000).length;
    const degraded = subs.filter(s => s.health === 'DEGRADED').length;
    const avgReadiness = subs.reduce((a,s)=>a+s.readiness,0) / subs.length;
    if (fails >= 3 || avgReadiness < 0.3) this.phase = 'FAIL_SAFE';
    else if (fails >= 1 || avgReadiness < 0.5) this.phase = 'RESTRICTED';
    else if (degraded >= 2 || avgReadiness < 0.7) this.phase = 'DEGRADED';
    else if (degraded >= 1 || avgReadiness < 0.85) this.phase = 'CAUTIOUS';
    else this.phase = 'HEALTHY';
    this.lastSynthesis = now;
    return this.phase;
  }
  readinessScore(): number {
    const subs = [...this.subsystems.values()];
    return subs.length ? subs.reduce((a,s)=>a+s.readiness,0)/subs.length : 0;
  }
  /** Detect if a subsystem's dependencies are failed (cascade risk) */
  cascadeRisk(): string[] {
    const risks: string[] = [];
    for (const s of this.subsystems.values()) {
      for (const dep of s.dependencies) {
        const d = this.subsystems.get(dep);
        if (d && (d.health === 'FAIL' || Date.now() - d.lastHeartbeat > 30000)) {
          risks.push(`${s.name} at risk — dependency ${dep} failed`);
        }
      }
    }
    return risks;
  }
  report() {
    return { phase:this.phase, readiness:this.readinessScore(), subsystems:[...this.subsystems.values()], cascadeRisks:this.cascadeRisk() };
  }
}

/* ─── 2. DECISION GOVERNOR ──────────────────────────────────────
   Every critical decision passes through here for approval. */
export interface PendingDecision {
  id:string; type:'REROUTE'|'MODE_CHANGE'|'PROVIDER_SWITCH'|'EMERGENCY'|'USER_ACTION';
  payload:any; risk:'LOW'|'MEDIUM'|'HIGH'|'CRITICAL';
  impact:string[]; confidence:number; requester:string; t:number;
}
export class DecisionGovernor {
  private pending: PendingDecision[] = [];
  private approved: PendingDecision[] = [];
  private rejected: Array<PendingDecision & {reason:string}> = [];
  private policy = {
    blockUnder:{phase:'FAIL_SAFE', types:['REROUTE','PROVIDER_SWITCH']},
    requireConfidence:{CRITICAL:0.9, HIGH:0.75, MEDIUM:0.5, LOW:0}
  };
  propose(decision:Omit<PendingDecision,'id'|'t'>, systemPhase:SystemPhase): {approved:boolean; reason?:string; id:string} {
    const id = `D-${Date.now()}-${Math.random().toString(36).slice(2,6)}`;
    const d:PendingDecision = {...decision, id, t:Date.now()};
    // Rule 1: block certain types in FAIL_SAFE
    if (systemPhase === this.policy.blockUnder.phase && this.policy.blockUnder.types.includes(d.type)) {
      this.rejected.push({...d, reason:`System in ${systemPhase} — ${d.type} blocked`});
      return {approved:false, reason:`Blocked: ${systemPhase} phase`, id};
    }
    // Rule 2: confidence must match risk level
    const required = this.policy.requireConfidence[d.risk];
    if (d.confidence < required) {
      this.rejected.push({...d, reason:`Confidence ${d.confidence} < required ${required} for ${d.risk}`});
      return {approved:false, reason:`Low confidence for ${d.risk} risk`, id};
    }
    // Rule 3: CRITICAL risk always requires explicit approval
    if (d.risk === 'CRITICAL') {
      this.pending.push(d);
      return {approved:false, reason:'CRITICAL — requires explicit approval', id};
    }
    this.approved.push(d);
    return {approved:true, id};
  }
  approveManually(id:string) {
    const idx = this.pending.findIndex(d => d.id === id);
    if (idx < 0) return false;
    this.approved.push(this.pending.splice(idx, 1)[0]);
    return true;
  }
  rollback(id:string): PendingDecision | null {
    const idx = this.approved.findIndex(d => d.id === id);
    if (idx < 0) return null;
    return this.approved.splice(idx, 1)[0];
  }
  stats() { return {pending:this.pending.length, approved:this.approved.length, rejected:this.rejected.length, recentRejections:this.rejected.slice(-5)}; }
}

/* ─── 3. SELF-DISTRUST MECHANISM ────────────────────────────────
   System's confidence in itself. Drops when evidence contradicts. */
export class SelfDistrust {
  confidence = 1.0;
  history: Array<{t:number; event:string; delta:number; newConf:number}> = [];
  private readonly DECAY = 0.99;    // per cycle
  private readonly RECOVERY = 0.01; // per healthy cycle

  observe(event:'CONTRADICTION'|'IMPOSSIBLE_STATE'|'OVERRIDE_FIRED'|'HIGH_CONF_ERROR'|'HEALTHY_CYCLE'|'CALIBRATION_DRIFT', magnitude=1) {
    let delta = 0;
    switch(event) {
      case 'CONTRADICTION':      delta = -0.05 * magnitude; break;
      case 'IMPOSSIBLE_STATE':   delta = -0.15 * magnitude; break;
      case 'OVERRIDE_FIRED':     delta = -0.10 * magnitude; break;
      case 'HIGH_CONF_ERROR':    delta = -0.20 * magnitude; break;
      case 'CALIBRATION_DRIFT':  delta = -0.08 * magnitude; break;
      case 'HEALTHY_CYCLE':      delta = +this.RECOVERY; break;
    }
    this.confidence = Math.max(0, Math.min(1, this.confidence + delta));
    this.history.push({t:Date.now(), event, delta, newConf:this.confidence});
    if (this.history.length > 500) this.history.shift();
  }
  shouldSuppress(actionRisk:'LOW'|'MEDIUM'|'HIGH'|'CRITICAL'): boolean {
    const thresholds = {LOW:0.2, MEDIUM:0.4, HIGH:0.6, CRITICAL:0.85};
    return this.confidence < thresholds[actionRisk];
  }
  forceDegrade(): boolean { return this.confidence < 0.3; }
  state() {
    const label = this.confidence > 0.85 ? 'TRUSTING' : this.confidence > 0.6 ? 'WATCHFUL' : this.confidence > 0.3 ? 'UNCERTAIN' : 'DISTRUSTFUL';
    return {confidence:this.confidence, label, recentDrops:this.history.filter(h=>h.delta<0).slice(-5)};
  }
}

/* ─── 4. FAIL-SAFE AUTHORITY ──────────────────────────────────── */
export class FailSafeAuthority {
  engaged = false;
  engagedAt = 0;
  reason = '';
  private overrides = {
    freezeRouting:false, minimalUI:false, disableNonCritical:false,
    forceLowSpeed:false, suppressAIActions:false
  };
  engage(reason:string, scope:Array<keyof typeof this.overrides> = ['freezeRouting','minimalUI','disableNonCritical','suppressAIActions']) {
    this.engaged = true; this.engagedAt = Date.now(); this.reason = reason;
    for (const k of scope) this.overrides[k] = true;
  }
  disengage() {
    this.engaged = false;
    for (const k of Object.keys(this.overrides) as Array<keyof typeof this.overrides>) this.overrides[k] = false;
  }
  isBlocked(action:string): boolean {
    if (!this.engaged) return false;
    const blocks:Record<string,keyof typeof this.overrides> = {
      reroute:'freezeRouting', changeProvider:'freezeRouting',
      showAdvancedUI:'minimalUI', enableRadar:'disableNonCritical',
      aiSuggestion:'suppressAIActions'
    };
    const flag = blocks[action]; return flag ? this.overrides[flag] : false;
  }
  state() { return {engaged:this.engaged, reason:this.reason, duration:this.engaged?Date.now()-this.engagedAt:0, overrides:{...this.overrides}}; }
}

/* ─── 5. TRUTH HIERARCHY & ARBITRATION ────────────────────────── */
export class TruthHierarchy {
  /** Priority ranking per domain. Higher = more trusted. */
  hierarchy: Record<string, Array<{source:TruthSource; weight:number}>> = {
    POSITION:      [{source:'GNSS',weight:0.5},{source:'FUSION',weight:0.3},{source:'MAP',weight:0.15},{source:'INS',weight:0.05}],
    HEADING:       [{source:'GNSS',weight:0.4},{source:'FUSION',weight:0.35},{source:'INS',weight:0.25}],
    ROUTE:         [{source:'MAP',weight:0.6},{source:'AI',weight:0.25},{source:'USER',weight:0.15}],
    HAZARD:        [{source:'POLICY',weight:0.5},{source:'MAP',weight:0.3},{source:'AI',weight:0.2}],
    MODE:          [{source:'FUSION',weight:0.6},{source:'POLICY',weight:0.3},{source:'USER',weight:0.1}]
  };
  /** When multiple sources disagree, return arbitrated value */
  arbitrate(domain:string, candidates:Array<{source:TruthSource; value:any; confidence:number}>): {value:any; source:TruthSource; confidence:number; arbitrated:boolean} {
    if (candidates.length === 0) return {value:null, source:'POLICY', confidence:0, arbitrated:false};
    if (candidates.length === 1) return {...candidates[0], arbitrated:false};
    const weights = this.hierarchy[domain] || [];
    // Weighted vote by source rank × confidence
    let bestScore = -Infinity; let winner = candidates[0];
    for (const c of candidates) {
      const rank = weights.find(w => w.source === c.source)?.weight ?? 0.1;
      const score = rank * c.confidence;
      if (score > bestScore) { bestScore = score; winner = c; }
    }
    return {...winner, arbitrated:true};
  }
  adjustWeight(domain:string, source:TruthSource, newWeight:number) {
    const arr = this.hierarchy[domain]; if (!arr) return;
    const entry = arr.find(e => e.source === source); if (entry) entry.weight = newWeight;
  }
}

/* ─── 6. STABILITY CONTROLLER ──────────────────────────────────
   Detects oscillations and dampens noise. */
export class StabilityController {
  private recent = new Map<string, Array<{v:any; t:number}>>();
  private dampening = new Map<string, number>();
  observe(metric:string, value:any) {
    const arr = this.recent.get(metric) || [];
    arr.push({v:value, t:Date.now()});
    // Keep last 20 samples
    if (arr.length > 20) arr.shift();
    this.recent.set(metric, arr);
  }
  /** Returns true if value oscillates rapidly */
  isOscillating(metric:string, threshold=5): boolean {
    const arr = this.recent.get(metric) || [];
    if (arr.length < 5) return false;
    // Count transitions (value changes)
    let changes = 0;
    for (let i=1; i<arr.length; i++) {
      if (JSON.stringify(arr[i].v) !== JSON.stringify(arr[i-1].v)) changes++;
    }
    const osc = changes >= threshold;
    if (osc) this.dampening.set(metric, (this.dampening.get(metric) || 0) + 1);
    return osc;
  }
  /** Returns dampened value — holds previous if oscillating */
  dampen(metric:string, newValue:any): any {
    const arr = this.recent.get(metric) || [];
    if (this.isOscillating(metric) && arr.length > 0) {
      // Hold for a few cycles
      const damp = this.dampening.get(metric) || 0;
      if (damp < 3) return arr[arr.length-1].v;
    }
    this.dampening.set(metric, 0);
    return newValue;
  }
  jitterScore(metric:string): number {
    const arr = this.recent.get(metric) || [];
    if (arr.length < 3) return 0;
    // For numeric: std dev / mean. For categorical: change rate.
    if (typeof arr[0].v === 'number') {
      const vals = arr.map(x => x.v as number);
      const mean = vals.reduce((a,v)=>a+v,0)/vals.length;
      const std = Math.sqrt(vals.reduce((a,v)=>a+(v-mean)**2,0)/vals.length);
      return mean ? std/Math.abs(mean) : std;
    }
    let changes = 0;
    for (let i=1; i<arr.length; i++) if (arr[i].v !== arr[i-1].v) changes++;
    return changes / arr.length;
  }
  report() { return Array.from(this.recent.keys()).map(k => ({metric:k, jitter:this.jitterScore(k), oscillating:this.isOscillating(k)})); }
}

/* ─── 7. LIMIT AWARENESS ────────────────────────────────────── */
export class LimitAwareness {
  private limits = {
    sensor:{minAccuracy:5, maxAccuracy:100, validAltitude:[-500,10000]},
    environment:{minCN0:18, minSats:4},
    model:{maxPrediction:300, validModes:['FULL','DEGRADED','DR_ONLY','LOST','RECOVERY']},
    data:{maxAge:60000}
  };
  exposures: Array<{t:number; domain:string; reason:string; userVisible:boolean}> = [];
  check(ctx:{accuracy?:number; altitude?:number; cn0?:number; numSats?:number; mode?:string; dataAge?:number}) {
    const exp: any[] = []; const now = Date.now();
    if (ctx.accuracy != null && ctx.accuracy > this.limits.sensor.maxAccuracy) {
      exp.push({t:now, domain:'SENSOR', reason:`GPS accuracy ${ctx.accuracy}m exceeds usable limit`, userVisible:true});
    }
    if (ctx.altitude != null && (ctx.altitude < this.limits.sensor.validAltitude[0] || ctx.altitude > this.limits.sensor.validAltitude[1])) {
      exp.push({t:now, domain:'SENSOR', reason:`Altitude ${ctx.altitude}m outside valid range`, userVisible:true});
    }
    if (ctx.cn0 != null && ctx.cn0 < this.limits.environment.minCN0) {
      exp.push({t:now, domain:'ENVIRONMENT', reason:`Signal quality CN0=${ctx.cn0} below usable threshold`, userVisible:true});
    }
    if (ctx.numSats != null && ctx.numSats < this.limits.environment.minSats) {
      exp.push({t:now, domain:'ENVIRONMENT', reason:`Only ${ctx.numSats} satellites (need ${this.limits.environment.minSats})`, userVisible:true});
    }
    if (ctx.dataAge != null && ctx.dataAge > this.limits.data.maxAge) {
      exp.push({t:now, domain:'DATA', reason:`Data is ${Math.round(ctx.dataAge/1000)}s old — potentially stale`, userVisible:true});
    }
    this.exposures.push(...exp);
    if (this.exposures.length > 500) this.exposures.splice(0, this.exposures.length-500);
    return exp;
  }
  userVisibleNow(): string[] {
    const recent = this.exposures.filter(e => Date.now() - e.t < 10000 && e.userVisible);
    return [...new Set(recent.map(e => e.reason))];
  }
}

/* ─── 8. GLOBAL AUDIT CONSCIOUSNESS ─────────────────────────── */
export interface AuditEntry {
  t:number; actor:string; action:string; before:any; after:any; reason:string; critical:boolean;
}
export class GlobalAudit {
  private log: AuditEntry[] = [];
  private readonly MAX = 5000;
  record(actor:string, action:string, before:any, after:any, reason:string, critical=false) {
    this.log.push({t:Date.now(), actor, action, before, after, reason, critical});
    if (this.log.length > this.MAX) this.log.shift();
    if (critical && typeof window !== 'undefined' && 'indexedDB' in window) {
      this.persistCritical();
    }
  }
  private persistCritical() {
    try {
      const req = indexedDB.open('gane-audit', 1);
      req.onupgradeneeded = e => (e.target as any).result.createObjectStore('entries', {keyPath:'t'});
      req.onsuccess = e => {
        const db = (e.target as any).result;
        const tx = db.transaction('entries', 'readwrite');
        for (const entry of this.log.filter(x=>x.critical).slice(-100)) {
          tx.objectStore('entries').put(entry);
        }
      };
    } catch {}
  }
  query(filter:{actor?:string; action?:string; critical?:boolean; since?:number}): AuditEntry[] {
    return this.log.filter(e =>
      (!filter.actor || e.actor === filter.actor) &&
      (!filter.action || e.action === filter.action) &&
      (!filter.critical || e.critical) &&
      (!filter.since || e.t >= filter.since)
    );
  }
  export(): string { return JSON.stringify({version:'1.0', exportedAt:Date.now(), entries:this.log}, null, 2); }
  stats() { return { total:this.log.length, critical:this.log.filter(e=>e.critical).length, uniqueActors:new Set(this.log.map(e=>e.actor)).size }; }
}

/* ─── 9. CONSCIOUSNESS ORCHESTRATOR — The top-level governor ──── */
export class ConsciousnessOrchestrator {
  state = new UnifiedSystemState();
  governor = new DecisionGovernor();
  distrust = new SelfDistrust();
  failsafe = new FailSafeAuthority();
  hierarchy = new TruthHierarchy();
  stability = new StabilityController();
  limits = new LimitAwareness();
  audit = new GlobalAudit();
  private cycleNum = 0;

  /** Runs every 2 seconds — the "heartbeat" of consciousness */
  tick(input:{subsystems:any[]; contradictions:any[]; impossibleStates:any[]; overrides:any[]; sensors:any}): {
    phase:SystemPhase; confidence:number; actions:string[]; exposures:string[]
  } {
    this.cycleNum++;
    // 1. Update unified state
    for (const s of input.subsystems) this.state.update(s.name, s.health, s.readiness);
    const phase = this.state.synthesize();

    // 2. Update self-distrust from evidence
    for (const _ of input.contradictions) this.distrust.observe('CONTRADICTION');
    for (const _ of input.impossibleStates) this.distrust.observe('IMPOSSIBLE_STATE', 2);
    for (const _ of input.overrides) this.distrust.observe('OVERRIDE_FIRED');
    if (input.contradictions.length === 0 && input.impossibleStates.length === 0) {
      this.distrust.observe('HEALTHY_CYCLE');
    }

    // 3. Stability tracking
    this.stability.observe('phase', phase);
    this.stability.observe('confidence', this.distrust.confidence);

    // 4. Limit awareness
    const exposures = this.limits.check(input.sensors || {});
    const userVisible = this.limits.userVisibleNow();

    // 5. Decide whether to engage fail-safe
    const actions: string[] = [];
    if (phase === 'FAIL_SAFE' && !this.failsafe.engaged) {
      this.failsafe.engage('System phase FAIL_SAFE');
      this.audit.record('CONSCIOUSNESS', 'ENGAGE_FAILSAFE', {phase:'prev'}, {phase}, 'auto-engage', true);
      actions.push('FAILSAFE_ENGAGED');
    }
    if (this.distrust.forceDegrade() && !this.failsafe.engaged) {
      this.failsafe.engage('Self-confidence below 0.3');
      actions.push('FAILSAFE_ENGAGED_LOW_CONFIDENCE');
    }
    if (phase === 'HEALTHY' && this.distrust.confidence > 0.8 && this.failsafe.engaged) {
      this.failsafe.disengage();
      this.audit.record('CONSCIOUSNESS', 'DISENGAGE_FAILSAFE', {engaged:true}, {engaged:false}, 'phase recovered', true);
      actions.push('FAILSAFE_DISENGAGED');
    }
    // 6. Stability check — if phase oscillates, suppress changes
    if (this.stability.isOscillating('phase', 3)) {
      actions.push('PHASE_OSCILLATION_DAMPENED');
      this.distrust.observe('CONTRADICTION', 0.5);
    }
    return {phase, confidence:this.distrust.confidence, actions, exposures:userVisible};
  }

  /** Gate any critical action through governor + failsafe + self-distrust */
  proposeAction(action:string, risk:'LOW'|'MEDIUM'|'HIGH'|'CRITICAL', payload:any, requester:string): {approved:boolean; reason?:string; id?:string} {
    if (this.failsafe.isBlocked(action)) {
      this.audit.record(requester, action, {}, {blocked:true}, 'failsafe blocked', risk==='CRITICAL');
      return {approved:false, reason:'Blocked by fail-safe authority'};
    }
    if (this.distrust.shouldSuppress(risk)) {
      this.audit.record(requester, action, {}, {suppressed:true}, 'low self-confidence', risk==='CRITICAL');
      return {approved:false, reason:`Suppressed — self-confidence ${(this.distrust.confidence*100).toFixed(0)}% too low for ${risk}`};
    }
    const result = this.governor.propose({type:action as any, payload, risk, impact:[], confidence:this.distrust.confidence, requester}, this.state.phase);
    this.audit.record(requester, action, {}, {approved:result.approved}, result.reason||'approved', risk==='CRITICAL');
    return result;
  }

  /** The definitive "system status" for UI display */
  selfReport() {
    return {
      cycle: this.cycleNum,
      phase: this.state.phase,
      readiness: +(this.state.readinessScore()*100).toFixed(1),
      confidence: +(this.distrust.confidence*100).toFixed(1),
      confidenceLabel: this.distrust.state().label,
      failsafeEngaged: this.failsafe.engaged,
      failsafeReason: this.failsafe.reason,
      cascadeRisks: this.state.cascadeRisk(),
      userVisibleLimits: this.limits.userVisibleNow(),
      oscillations: this.stability.report().filter(r => r.oscillating).map(r => r.metric),
      governor: this.governor.stats(),
      audit: this.audit.stats(),
      verdict: this.computeVerdict()
    };
  }

  /** The ONE sentence about system state */
  private computeVerdict(): string {
    if (this.failsafe.engaged) return `FAIL-SAFE ACTIVE: ${this.failsafe.reason}`;
    if (this.state.phase === 'FAIL_SAFE') return 'SYSTEM FAILING — minimal mode only';
    if (this.state.phase === 'RESTRICTED') return 'RESTRICTED — critical subsystem failed';
    if (this.state.phase === 'DEGRADED') return 'DEGRADED — operating with reduced confidence';
    if (this.distrust.confidence < 0.5) return 'UNCERTAIN — system distrusts its own output';
    if (this.state.phase === 'CAUTIOUS') return 'CAUTIOUS — minor anomalies detected';
    return 'HEALTHY — system trusts itself';
  }
}

/* ─── TESTS ─────────────────────────────────────────────────── */
export function runConsciousnessTests(): string[] {
  const out: string[] = [];

  // Unified state
  const uss = new UnifiedSystemState();
  uss.register('gnss'); uss.register('fusion', ['gnss']);
  uss.update('gnss', 'FAIL', 0); uss.update('fusion', 'OK', 0.9);
  const phase = uss.synthesize();
  out.push(phase === 'RESTRICTED' || phase === 'DEGRADED' ? '✓ UnifiedState escalates phase' : `✗ Phase=${phase}`);
  out.push(uss.cascadeRisk().length > 0 ? '✓ Cascade risk detected' : '✗ Cascade');

  // Governor blocks CRITICAL
  const gov = new DecisionGovernor();
  const r = gov.propose({type:'REROUTE', payload:{}, risk:'CRITICAL', impact:[], confidence:0.95, requester:'test'}, 'HEALTHY');
  out.push(!r.approved && r.reason?.includes('CRITICAL') ? '✓ Governor holds CRITICAL' : '✗ Governor');

  // Self-distrust drops on contradictions
  const sd = new SelfDistrust();
  for (let i=0;i<5;i++) sd.observe('CONTRADICTION');
  out.push(sd.confidence < 0.9 ? '✓ SelfDistrust decays' : '✗ Distrust');

  // Fail-safe engages and blocks
  const fs = new FailSafeAuthority();
  fs.engage('test');
  out.push(fs.isBlocked('reroute') ? '✓ FailSafe blocks' : '✗ FailSafe');

  // Truth hierarchy
  const th = new TruthHierarchy();
  const arb = th.arbitrate('POSITION', [
    {source:'GNSS', value:[32,34], confidence:0.9},
    {source:'INS', value:[32.1,34.1], confidence:0.95}
  ]);
  out.push(arb.source === 'GNSS' && arb.arbitrated ? '✓ Hierarchy arbitrates GNSS over INS' : `✗ Arbitrated=${arb.source}`);

  // Stability detects oscillation
  const sc = new StabilityController();
  for (let i=0;i<10;i++) sc.observe('mode', i%2 ? 'FULL' : 'DEGRADED');
  out.push(sc.isOscillating('mode', 3) ? '✓ Oscillation detected' : '✗ Stability');

  // Limits expose user
  const la = new LimitAwareness();
  const exp = la.check({accuracy:200, numSats:2});
  out.push(exp.length === 2 ? '✓ Limits detected' : `✗ Limits=${exp.length}`);

  // Audit records critical
  const aud = new GlobalAudit();
  aud.record('CORE', 'MODE_CHANGE', {mode:'FULL'}, {mode:'LOST'}, 'signal loss', true);
  out.push(aud.query({critical:true}).length === 1 ? '✓ Audit captures critical' : '✗ Audit');

  // Orchestrator full cycle
  const co = new ConsciousnessOrchestrator();
  co.state.register('gnss');
  co.state.update('gnss', 'FAIL', 0);
  const result = co.tick({subsystems:[{name:'gnss',health:'FAIL',readiness:0}], contradictions:[1,2,3], impossibleStates:[1], overrides:[], sensors:{accuracy:150}});
  out.push(result.phase !== 'HEALTHY' && result.exposures.length > 0 ? '✓ Orchestrator synthesizes' : `✗ Orchestrator phase=${result.phase}`);

  // Action gating
  const actionResult = co.proposeAction('reroute', 'HIGH', {}, 'test');
  out.push(!actionResult.approved ? '✓ Action gated under distrust' : '✗ Action gate');

  return out;
}

export const Consciousness = {
  UnifiedSystemState, DecisionGovernor, SelfDistrust, FailSafeAuthority,
  TruthHierarchy, StabilityController, LimitAwareness, GlobalAudit,
  ConsciousnessOrchestrator, runConsciousnessTests
};
