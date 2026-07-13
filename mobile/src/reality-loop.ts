// ═══════════════════════════════════════════════════════════════════════════
// G.A.N.E REALITY LOOP — The Final Layer
// Self-correcting navigation intelligence.
// Every prediction → measured against reality → fed back to calibrate models.
// ═══════════════════════════════════════════════════════════════════════════

/* ─── 1. GROUND TRUTH COLLECTOR ────────────────────────────────────
   Captures every prediction and its actual outcome. Immutable log. */
export interface Prediction {
  id:string; type:'ETA'|'PATH'|'TRUST'|'CONGESTION'|'HAZARD'|'MODE_DURATION';
  predictedValue:any; confidence:number; contextSnapshot:any; t:number;
  // Filled when outcome observed:
  actualValue?:any; outcomeT?:number; errorMetric?:number; resolved?:boolean;
}
export class GroundTruthCollector {
  private predictions = new Map<string, Prediction>();
  private resolvedLog: Prediction[] = [];
  record(type:Prediction['type'], predictedValue:any, confidence:number, context:any): string {
    const id = `${type}-${Date.now()}-${Math.random().toString(36).slice(2,6)}`;
    this.predictions.set(id, {id, type, predictedValue, confidence, contextSnapshot:context, t:Date.now(), resolved:false});
    return id;
  }
  resolve(id:string, actualValue:any): Prediction | null {
    const p = this.predictions.get(id); if (!p) return null;
    p.actualValue = actualValue; p.outcomeT = Date.now();
    p.errorMetric = this.computeError(p.type, p.predictedValue, actualValue);
    p.resolved = true;
    this.resolvedLog.push(p);
    if (this.resolvedLog.length > 2000) this.resolvedLog.shift();
    return p;
  }
  private computeError(type:string, predicted:any, actual:any): number {
    if (type === 'ETA') return Math.abs((actual as number) - (predicted as number)) / (predicted as number || 1);
    if (type === 'PATH') {
      // Fréchet-ish: mean deviation between two polylines
      const P = predicted as Array<[number,number]>; const A = actual as Array<[number,number]>;
      let sum = 0, n = 0;
      for (const a of A) {
        let min = Infinity;
        for (const p of P) { const d = Math.hypot((a[0]-p[0])*111320, (a[1]-p[1])*111320); min = Math.min(min, d); }
        sum += min; n++;
      }
      return n ? sum/n : 0;
    }
    if (type === 'TRUST' || type === 'CONGESTION') return Math.abs((actual as number) - (predicted as number));
    if (type === 'MODE_DURATION') return Math.abs((actual as number) - (predicted as number)) / 1000;
    return 0;
  }
  statistics(type?:string) {
    const relevant = type ? this.resolvedLog.filter(p => p.type === type) : this.resolvedLog;
    if (!relevant.length) return {n:0, meanError:0, p95Error:0, bias:0};
    const errors = relevant.map(p => p.errorMetric || 0).sort((a,b)=>a-b);
    const mean = errors.reduce((a,v)=>a+v,0)/errors.length;
    const p95 = errors[Math.floor(errors.length*0.95)];
    // Bias: signed mean (predicted − actual)
    const biases = relevant.filter(p => typeof p.predictedValue === 'number' && typeof p.actualValue === 'number')
      .map(p => (p.predictedValue as number) - (p.actualValue as number));
    const bias = biases.length ? biases.reduce((a,v)=>a+v,0)/biases.length : 0;
    return {n:errors.length, meanError:mean, p95Error:p95, bias};
  }
  recentResolved(n=20) { return this.resolvedLog.slice(-n); }
}

/* ─── 2. DYNAMIC CALIBRATION ENGINE ────────────────────────────────
   Adjusts internal multipliers based on observed errors. */
export class CalibrationEngine {
  // Per-category multipliers. Start at 1.0, drift toward truth.
  calibrators: Record<string, {multiplier:number; confidence:number; samples:number; rmse:number}> = {
    ETA: {multiplier:1.0, confidence:0.5, samples:0, rmse:0},
    TRUST: {multiplier:1.0, confidence:0.5, samples:0, rmse:0},
    CONGESTION: {multiplier:1.0, confidence:0.5, samples:0, rmse:0}
  };
  update(category:string, predicted:number, actual:number) {
    const c = this.calibrators[category] = this.calibrators[category] || {multiplier:1.0, confidence:0.5, samples:0, rmse:0};
    c.samples++;
    const ratio = actual / (predicted || 0.001);
    const alpha = Math.min(0.15, 1 / c.samples);
    c.multiplier = (1 - alpha) * c.multiplier + alpha * ratio;
    c.rmse = Math.sqrt((1 - alpha) * c.rmse * c.rmse + alpha * (actual-predicted) ** 2);
    c.confidence = Math.min(0.95, c.samples / 50);
  }
  apply(category:string, rawValue:number): number {
    const c = this.calibrators[category]; if (!c || c.samples < 3) return rawValue;
    return rawValue * c.multiplier;
  }
  reset(category:string) { if (this.calibrators[category]) this.calibrators[category] = {multiplier:1, confidence:0, samples:0, rmse:0}; }
  snapshot() { return JSON.parse(JSON.stringify(this.calibrators)); }
}

/* ─── 3. PROVIDER RELIABILITY CALIBRATOR ─────────────────────────── */
export class ProviderReliability {
  scores = new Map<string, {successes:number; failures:number; latencies:number[]; score:number}>();
  record(provider:string, success:boolean, latencyMs?:number) {
    const s = this.scores.get(provider) || {successes:0, failures:0, latencies:[], score:0.5};
    success ? s.successes++ : s.failures++;
    if (latencyMs && latencyMs < 30000) s.latencies.push(latencyMs);
    if (s.latencies.length > 100) s.latencies.shift();
    const total = s.successes + s.failures;
    const successRate = total ? s.successes/total : 0.5;
    const avgLatency = s.latencies.length ? s.latencies.reduce((a,v)=>a+v,0)/s.latencies.length : 1000;
    const latencyScore = Math.max(0, 1 - avgLatency/5000);
    s.score = successRate * 0.7 + latencyScore * 0.3;
    this.scores.set(provider, s);
  }
  rank(): Array<{provider:string; score:number}> {
    return [...this.scores.entries()].map(([p,s])=>({provider:p, score:s.score})).sort((a,b)=>b.score-a.score);
  }
  best(providers:string[]): string | null {
    const ranked = this.rank().filter(r => providers.includes(r.provider));
    return ranked[0]?.provider || providers[0] || null;
  }
}

/* ─── 4. CONTRADICTION DETECTOR ──────────────────────────────────
   Detects when different layers say inconsistent things. */
export interface LayerState { positioning?:any; fusion?:any; routing?:any; ui?:any; proof?:any; }
export class ContradictionDetector {
  contradictions: Array<{t:number; layers:string[]; description:string; severity:'INFO'|'WARN'|'CRIT'}> = [];
  check(s:LayerState): string[] {
    const found: any[] = []; const now = Date.now();
    // 1. Positioning says trust=HIGH but integrity says spoofing
    if (s.positioning?.trust > 0.8 && s.fusion?.spoofScore > 0.6) {
      found.push({t:now, layers:['positioning','fusion'], description:'High trust but high spoof score', severity:'CRIT'});
    }
    // 2. Routing says ETA=5min but traffic predicts 15min
    if (s.routing?.eta && s.routing?.predictedEta && Math.abs(s.routing.eta - s.routing.predictedEta) > s.routing.eta * 0.5) {
      found.push({t:now, layers:['routing','traffic'], description:'ETA vs predicted differ >50%', severity:'WARN'});
    }
    // 3. UI shows FULL mode but engine is in DR_ONLY
    if (s.ui?.displayedMode && s.fusion?.actualMode && s.ui.displayedMode !== s.fusion.actualMode) {
      found.push({t:now, layers:['ui','fusion'], description:`UI=${s.ui.displayedMode} ≠ Engine=${s.fusion.actualMode}`, severity:'CRIT'});
    }
    // 4. Proof timestamp stale vs current state
    if (s.proof?.t && now - s.proof.t > 60000) {
      found.push({t:now, layers:['proof'], description:'Proof artifact older than 60s', severity:'WARN'});
    }
    // 5. Fusion says moving but GPS says stationary
    if (s.fusion?.speed > 5 && s.positioning?.speed < 0.5) {
      found.push({t:now, layers:['fusion','positioning'], description:'Fusion velocity but GPS stationary', severity:'WARN'});
    }
    this.contradictions.push(...found);
    if (this.contradictions.length > 500) this.contradictions.splice(0, this.contradictions.length-500);
    return found.map(f => `${f.severity}:${f.description}`);
  }
  recent(n=10) { return this.contradictions.slice(-n); }
}

/* ─── 5. IMPOSSIBLE-STATE DETECTOR ──────────────────────────────── */
export class ImpossibleStateDetector {
  check(state:any): string[] {
    const out: string[] = [];
    if (state.speed > 83.33) out.push('Speed > 300 km/h (likely error)');
    if (state.altitude < -500 || state.altitude > 10000) out.push('Altitude out of Earth range');
    if (state.accuracy < 0) out.push('Negative accuracy');
    if (state.heading != null && (state.heading < 0 || state.heading > 360)) out.push('Heading out of [0,360]');
    if (state.numSats > 100) out.push('Impossible satellite count');
    if (state.uncertainty != null && !isFinite(state.uncertainty)) out.push('Non-finite uncertainty');
    // Time travel: outcome resolved before prediction made
    if (state.outcomeT && state.predictionT && state.outcomeT < state.predictionT) out.push('Time inversion');
    return out;
  }
}

/* ─── 6. FAILURE REGISTRY & CLUSTERING ──────────────────────────── */
export interface FailureCase { id:string; type:string; signature:string; context:any; t:number; count:number; }
export class FailureRegistry {
  private cases = new Map<string, FailureCase>();
  capture(type:string, context:any): string {
    const signature = this.hashSignature(type, context);
    const existing = this.cases.get(signature);
    if (existing) { existing.count++; existing.t = Date.now(); return signature; }
    const id = `F-${Date.now().toString(36)}`;
    const c:FailureCase = {id, type, signature, context, t:Date.now(), count:1};
    this.cases.set(signature, c);
    return signature;
  }
  private hashSignature(type:string, ctx:any): string {
    // Bucket context into equivalence classes for clustering
    const bucket = (v:any):any => {
      if (typeof v === 'number') return Math.round(v*10)/10;
      if (Array.isArray(v)) return v.slice(0,3).map(bucket);
      if (v && typeof v === 'object') return Object.fromEntries(Object.entries(v).slice(0,5).map(([k,x])=>[k,bucket(x)]));
      return v;
    };
    return `${type}|${JSON.stringify(bucket(ctx)).slice(0,200)}`;
  }
  recurring(minCount=3): FailureCase[] { return [...this.cases.values()].filter(c => c.count >= minCount).sort((a,b)=>b.count-a.count); }
  all() { return [...this.cases.values()].sort((a,b)=>b.t-a.t); }
  clear() { this.cases.clear(); }
}

/* ─── 7. DECISION ACCOUNTABILITY LEDGER ─────────────────────────── */
export interface Decision {
  id:string; type:string; chosen:any; alternatives:any[]; confidence:number;
  factors:Record<string,number>; t:number;
  outcome?:any; correctnessScore?:number; outcomeT?:number;
}
export class DecisionLedger {
  private ledger: Decision[] = [];
  record(type:string, chosen:any, alternatives:any[], confidence:number, factors:Record<string,number>): string {
    const id = `D-${Date.now()}-${Math.random().toString(36).slice(2,5)}`;
    this.ledger.push({id, type, chosen, alternatives, confidence, factors, t:Date.now()});
    if (this.ledger.length > 1000) this.ledger.shift();
    return id;
  }
  resolveOutcome(id:string, actualOutcome:any, correctness:number) {
    const d = this.ledger.find(x => x.id === id); if (!d) return;
    d.outcome = actualOutcome; d.correctnessScore = correctness; d.outcomeT = Date.now();
  }
  accuracyByType(type:string): {n:number; meanCorrectness:number; highConfErrors:number} {
    const relevant = this.ledger.filter(d => d.type === type && d.correctnessScore != null);
    if (!relevant.length) return {n:0, meanCorrectness:0, highConfErrors:0};
    const mean = relevant.reduce((a,d)=>a+(d.correctnessScore||0),0)/relevant.length;
    const highConfErrors = relevant.filter(d => d.confidence > 0.8 && (d.correctnessScore||0) < 0.5).length;
    return {n:relevant.length, meanCorrectness:mean, highConfErrors};
  }
  explain(id:string): string | null {
    const d = this.ledger.find(x => x.id === id); if (!d) return null;
    const factorStr = Object.entries(d.factors).map(([k,v]) => `${k}=${v.toFixed(2)}`).join(', ');
    const outcome = d.outcome != null ? ` [outcome:${d.outcome}, correctness:${(d.correctnessScore||0).toFixed(2)}]` : ' [unresolved]';
    return `${d.type} decision: chose option with confidence ${d.confidence.toFixed(2)} based on {${factorStr}}${outcome}`;
  }
}

/* ─── 8. TRUTH OVERRIDE MECHANISM ─────────────────────────────── */
export class TruthOverride {
  private overrideLog:Array<{t:number; action:string; reason:string}> = [];
  constructor(private callbacks:{
    forceDegraded?:()=>void;
    forceTrustReset?:()=>void;
    suppressReroute?:()=>void;
    forceRecalibration?:()=>void;
  }) {}
  evaluate(evidence:{
    calibrationRmse?:number;
    contradictions?:number;
    recurringFailures?:number;
    highConfErrors?:number;
  }): string[] {
    const actions: string[] = []; const now = Date.now();
    if ((evidence.calibrationRmse||0) > 0.3) {
      actions.push('FORCE_RECALIBRATION');
      this.callbacks.forceRecalibration?.();
      this.overrideLog.push({t:now, action:'FORCE_RECALIBRATION', reason:`RMSE ${evidence.calibrationRmse}`});
    }
    if ((evidence.contradictions||0) > 5) {
      actions.push('FORCE_DEGRADED');
      this.callbacks.forceDegraded?.();
      this.overrideLog.push({t:now, action:'FORCE_DEGRADED', reason:`${evidence.contradictions} contradictions`});
    }
    if ((evidence.highConfErrors||0) > 3) {
      actions.push('FORCE_TRUST_RESET');
      this.callbacks.forceTrustReset?.();
      this.overrideLog.push({t:now, action:'FORCE_TRUST_RESET', reason:`${evidence.highConfErrors} high-conf errors`});
    }
    if ((evidence.recurringFailures||0) > 2) {
      actions.push('SUPPRESS_REROUTE');
      this.callbacks.suppressReroute?.();
      this.overrideLog.push({t:now, action:'SUPPRESS_REROUTE', reason:`${evidence.recurringFailures} recurring`});
    }
    return actions;
  }
  recent(n=20) { return this.overrideLog.slice(-n); }
}

/* ─── 9. ROUTE OUTCOME TRACKER ──────────────────────────────────
   Watches user's actual path after route prediction. Measures drift,
   deviations, ETA accuracy. Feeds back to calibrator. */
export class RouteOutcomeTracker {
  private activeRoutes = new Map<string, {predicted:Array<[number,number]>; predictedEta:number; startT:number; actualPath:Array<[number,number]>}>();
  startRoute(routeId:string, predicted:Array<[number,number]>, predictedEtaSec:number) {
    this.activeRoutes.set(routeId, {predicted, predictedEta:predictedEtaSec, startT:Date.now(), actualPath:[]});
  }
  appendPosition(routeId:string, lat:number, lon:number) {
    const r = this.activeRoutes.get(routeId); if (!r) return;
    r.actualPath.push([lat, lon]);
  }
  finishRoute(routeId:string): {pathDeviation:number; etaError:number; actualEta:number}|null {
    const r = this.activeRoutes.get(routeId); if (!r) return null;
    const actualEta = (Date.now() - r.startT) / 1000;
    const etaError = Math.abs(actualEta - r.predictedEta) / r.predictedEta;
    // Mean minimum distance from actual to predicted
    let sum = 0, n = 0;
    for (const a of r.actualPath) {
      let min = Infinity;
      for (const p of r.predicted) {
        const d = Math.hypot((a[0]-p[0])*111320, (a[1]-p[1])*111320);
        min = Math.min(min, d);
      }
      sum += min; n++;
    }
    const pathDeviation = n ? sum/n : 0;
    this.activeRoutes.delete(routeId);
    return {pathDeviation, etaError, actualEta};
  }
}

/* ─── 10. REALITY VALIDATION ORCHESTRATOR ─────────────────────── */
export class RealityValidationOrchestrator {
  truth = new GroundTruthCollector();
  calibration = new CalibrationEngine();
  providers = new ProviderReliability();
  contradictions = new ContradictionDetector();
  impossible = new ImpossibleStateDetector();
  failures = new FailureRegistry();
  decisions = new DecisionLedger();
  routes = new RouteOutcomeTracker();
  override: TruthOverride;
  private cycleT = 0;
  constructor(callbacks:any = {}) { this.override = new TruthOverride(callbacks); }

  /** Main loop: run every 5s. Checks consistency, feeds calibrator. */
  cycle(layerState:any): {actions:string[]; contradictions:string[]; impossible:string[]} {
    this.cycleT++;
    const contradictions = this.contradictions.check(layerState);
    const impossible = this.impossible.check(layerState);
    // Feed calibration with latest outcomes
    for (const p of this.truth.recentResolved(10)) {
      if (typeof p.predictedValue === 'number' && typeof p.actualValue === 'number') {
        this.calibration.update(p.type, p.predictedValue, p.actualValue);
      }
    }
    // Decide override actions
    const stats = this.calibration.calibrators;
    const meanRmse = Object.values(stats).reduce((a,c)=>a+c.rmse,0) / Math.max(1, Object.keys(stats).length);
    const decisionsStats = this.decisions.accuracyByType('ROUTE');
    const actions = this.override.evaluate({
      calibrationRmse: meanRmse,
      contradictions: contradictions.length,
      recurringFailures: this.failures.recurring().length,
      highConfErrors: decisionsStats.highConfErrors
    });
    // Capture impossible states as failures
    for (const imp of impossible) this.failures.capture('IMPOSSIBLE_STATE', {msg:imp, state:layerState});
    return {actions, contradictions, impossible};
  }

  selfReport() {
    return {
      cycle: this.cycleT,
      predictions: this.truth.statistics(),
      calibration: this.calibration.snapshot(),
      providers: this.providers.rank(),
      contradictions: this.contradictions.recent(5),
      recurringFailures: this.failures.recurring(),
      recentOverrides: this.override.recent(5),
      decisions: {
        route: this.decisions.accuracyByType('ROUTE'),
        mode: this.decisions.accuracyByType('MODE_CHANGE')
      }
    };
  }
}

/* ─── ACCEPTANCE TESTS ──────────────────────────────────────── */
export function runRealityTests(): string[] {
  const results: string[] = [];

  // Truth collector
  const tc = new GroundTruthCollector();
  const id = tc.record('ETA', 600, 0.9, {route:'A'});
  const p = tc.resolve(id, 720);
  results.push(p && p.errorMetric && p.errorMetric > 0 ? '✓ GroundTruth resolves' : '✗ GroundTruth');

  // Calibration learns
  const cal = new CalibrationEngine();
  for (let i=0; i<20; i++) cal.update('ETA', 600, 720);
  const adjusted = cal.apply('ETA', 600);
  results.push(adjusted > 650 ? '✓ Calibration adjusts upward' : '✗ Calibration');

  // Contradiction detection
  const cd = new ContradictionDetector();
  const c = cd.check({positioning:{trust:0.9}, fusion:{spoofScore:0.8}, ui:{displayedMode:'FULL'}, routing:{}});
  results.push(c.length > 0 ? '✓ Contradictions detected' : '✗ Contradictions');

  // Impossible states
  const is = new ImpossibleStateDetector();
  const imp = is.check({speed:100, altitude:50000});
  results.push(imp.length === 2 ? '✓ Impossible states flagged' : '✗ Impossible');

  // Failure registry clustering
  const fr = new FailureRegistry();
  for (let i=0; i<5; i++) fr.capture('GPS_LOSS', {env:'TUNNEL', zoom:15});
  const rec = fr.recurring();
  results.push(rec.length === 1 && rec[0].count === 5 ? '✓ Failure clustering' : '✗ Failure reg');

  // Decision accountability
  const dl = new DecisionLedger();
  const did = dl.record('ROUTE', {path:'A'}, [{path:'B'}], 0.9, {distance:0.8, traffic:0.2});
  dl.resolveOutcome(did, {path:'A'}, 0.95);
  const exp = dl.explain(did);
  results.push(exp && exp.includes('correctness') ? '✓ Decision ledger' : '✗ Decisions');

  // Truth override triggers
  let overrodeDegraded = false;
  const to = new TruthOverride({forceDegraded: () => overrodeDegraded = true});
  to.evaluate({contradictions:10});
  results.push(overrodeDegraded ? '✓ Truth override fires' : '✗ Override');

  // Route outcome
  const ro = new RouteOutcomeTracker();
  ro.startRoute('r1', [[32.08,34.78],[32.09,34.79]], 600);
  ro.appendPosition('r1', 32.085, 34.785);
  const fin = ro.finishRoute('r1');
  results.push(fin && typeof fin.pathDeviation === 'number' ? '✓ Route outcome tracks' : '✗ Route outcome');

  // Full orchestrator
  const rvo = new RealityValidationOrchestrator();
  const cyc = rvo.cycle({positioning:{trust:0.95}, fusion:{spoofScore:0.7}});
  results.push(cyc.contradictions.length > 0 ? '✓ Orchestrator detects' : '✗ Orchestrator');

  return results;
}

export const RealityLoop = {
  GroundTruthCollector, CalibrationEngine, ProviderReliability,
  ContradictionDetector, ImpossibleStateDetector, FailureRegistry,
  DecisionLedger, TruthOverride, RouteOutcomeTracker,
  RealityValidationOrchestrator, runRealityTests
};
