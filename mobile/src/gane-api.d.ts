/**
 * G.A.N.E SDK — TypeScript Declarations
 * Import this alongside gane-core-bundle.js for full type safety.
 *
 * Usage:
 *   <script src="gane-core-bundle.js"></script>
 *   /// <reference path="gane-api.d.ts" />
 *   const engine = new GANE.Core.NavigationEngine();
 *
 * Version: 1.0.0
 * Stability contract: SemVer. Breaking changes require major version bump.
 */

declare namespace GANE {
  // ═══════════════════════════════════════════════════════════════════
  // SHARED DOMAIN TYPES
  // ═══════════════════════════════════════════════════════════════════
  type ConstellationId = 'GPS'|'GLONASS'|'Galileo'|'BeiDou'|'QZSS'|'NavIC'|'SBAS';
  type NavMode = 'FULL'|'DEGRADED'|'DR_ONLY'|'LOST'|'RECOVERY';
  type EnvClass = 'OPEN'|'URBAN'|'CANYON'|'TUNNEL'|'INDOOR'|'UNKNOWN';
  type Health = 'OK'|'DEGRADED'|'FAIL'|'UNKNOWN';
  type SystemPhase = 'HEALTHY'|'CAUTIOUS'|'DEGRADED'|'RESTRICTED'|'FAIL_SAFE';
  type TruthSource = 'GNSS'|'FUSION'|'MAP'|'INS'|'AI'|'USER'|'POLICY';
  type RiskLevel = 'LOW'|'MEDIUM'|'HIGH'|'CRITICAL';

  interface ECEF { x:number; y:number; z:number }
  interface LLA  { lat:number; lon:number; alt:number }
  interface Vel3 { vx:number; vy:number; vz:number }

  interface RawMeasurement {
    svid:number; constellation:ConstellationId;
    pseudorange:number; carrierPhase?:number; doppler?:number;
    cn0:number; elevation?:number; azimuth?:number;
    carrierFreq?:number; usedInFix:boolean; hasEphemeris?:boolean; t:number;
  }

  interface PvtSolution {
    pos:ECEF; lla:LLA; clockBias:number;
    HDOP:number; VDOP:number; PDOP:number; GDOP:number;
    numUsed:number; confidence:number; residuals:number[]; t:number;
  }

  interface ImuSample   { ax:number; ay:number; az:number; gx:number; gy:number; gz:number; t:number; health:Health }
  interface NavTraceEntry {
    t:number; mode:NavMode; numSats:number; pdop:number;
    confidence:number; trustSum:number; integrity:IntegrityEvent[]; env:EnvClass;
  }
  interface IntegrityEvent {
    type:'JUMP'|'CN0_FLOOR'|'SINGLE_CONSTELLATION'|'RESIDUAL_OUTLIER'|'SV_EXCLUDED'|'SV_REVALIDATED'|'IMU_DIVERGENCE'|'MAP_MISMATCH';
    severity:'INFO'|'WARN'|'CRIT'; data:any; t:number;
  }

  // ═══════════════════════════════════════════════════════════════════
  // CORE LAYER (21 modules) — positioning, fusion, continuity, integrity
  // ═══════════════════════════════════════════════════════════════════
  namespace Core {
    class MeasurementIngestion {
      ingest(raw:any[], source:'android-native'|'web-geolocation'|'replay'): RawMeasurement[];
      latest(): RawMeasurement[];
      statistics(): {accepted:number; rejected:number; stale:number; bufferSize:number};
    }

    class SatelliteHealth {
      score(meas:RawMeasurement[]): Map<string, number>;
      constellationHealth(meas:RawMeasurement[]): Record<string, Health>;
      trustOf(constellation:ConstellationId, svid:number): number;
      exclusionList(threshold?:number): Array<{constellation:ConstellationId; svid:number; trust:number}>;
    }

    class PvtSolver {
      solve(input:{sats:Array<RawMeasurement & ECEF>; initial:[number,number,number,number]}): PvtSolution | null;
    }

    class CovarianceMatrix {
      constructor(n:number, initial?:number);
      n:number;
      get(i:number, j:number): number;
      set(i:number, j:number, v:number): void;
      inflate(i:number, amount:number): void;
      shrink(i:number, factor:number): void;
      trace3(): number;
      bounded(maxVar:number): void;
      snapshot(): number[];
    }

    class StateEngine {
      x: Float64Array;
      P: CovarianceMatrix;
      cycles:number; lastUpdateT:number;
      predict(dt:number): void;
      position(): [number, number, number];
      velocity(): [number, number, number];
    }

    class UpdateCycle {
      constructor(state:StateEngine);
      applyMeasurement(z:number[], idx:number[], R:number[], trustWeight:number): Array<{accepted:boolean; chi:number}>;
    }

    class ModeManager {
      mode:NavMode; modeEnteredAt:number;
      transitions: Array<{from:NavMode; to:NavMode; reason:string; t:number; metrics:any}>;
      evaluate(ctx:{numSats:number; pdop:number; spoofScore:number; imuHealthy:boolean; mapAvail:boolean; driftM:number}): NavMode;
      timeInMode(): number;
    }

    class FailoverChain { select(ctx:any): NavMode; }
    class RecoveryValidator { tick(ctx:{numSats:number; pdop:number; spoofScore:number}): boolean; reset(): void; }
    class ResidualChecker { analyze(residuals:number[], sigma?:number): {passed:boolean; outlierIdx:number[]; chiSquared:number}; }

    class FaultDetector {
      events: IntegrityEvent[];
      detect(sats:RawMeasurement[], currentPos:LLA|null, imuAccel:number|null, mapDist:number|null): IntegrityEvent[];
      getSpoofScore(): number;
    }

    class SourceValidator { validate(gnssPos:LLA|null, insPos:LLA|null, mapPos:LLA|null): {trusted:string[]; suspicious:string[]}; }
    class PhysicalWorldModel {
      classify(ctx:{numSats:number; meanCN0:number; elevationMean:number; speed:number}): EnvClass;
      expectedNoise(env:EnvClass): {pseudorange:number; heading:number};
    }

    class NavigationTrace {
      append(e:NavTraceEntry): void;
      timeline(fromT?:number): NavTraceEntry[];
      modeHistory(): Array<{t:number; mode:NavMode}>;
      confidenceTimeline(): Array<{t:number; c:number}>;
      integrityTimeline(): IntegrityEvent[];
      export(): {version:string; entries:NavTraceEntry[]; exportedAt:number};
    }

    class ReplayEngine {
      constructor(trace:NavTraceEntry[], cb:(e:NavTraceEntry)=>void);
      playbackRate:number;
      play(fromT?:number): void;
      pause(): void;
      seek(t:number): void;
    }

    class OfflineCore {
      persist(state:any): void;
      restore(): any;
      rememberHazard(h:{lla:LLA; type:string}): void;
      hazardsNear(lla:LLA, radiusM?:number): Array<{lla:LLA; type:string; t:number}>;
      syncReconciliation(serverState:any): any[];
    }

    class WebSensorAdapter { name:string; isAvailable():Promise<boolean>; start(cb:(s:any)=>void):Promise<void>; stop():Promise<void>; health():Health; }
    class ReplaySensorAdapter { constructor(data:any[]); name:string; isAvailable():Promise<boolean>; start(cb:(s:any)=>void):Promise<void>; stop():Promise<void>; health():Health; }
    class SensorHealthMonitor { record(sensor:string):void; status(sensor:string, expectedHz:number):Health; }

    class NavigationEngine {
      ingestion:MeasurementIngestion;
      health:SatelliteHealth;
      solver:PvtSolver;
      state:StateEngine;
      updater:UpdateCycle;
      mode:ModeManager;
      failover:FailoverChain;
      recovery:RecoveryValidator;
      residuals:ResidualChecker;
      faults:FaultDetector;
      validator:SourceValidator;
      sensorHealth:SensorHealthMonitor;
      world:PhysicalWorldModel;
      trace:NavigationTrace;
      offline:OfflineCore;
      tick(rawMeas:RawMeasurement[], imu:ImuSample|null, pos:LLA|null): NavTraceEntry;
      snapshot(): {mode:NavMode; modeTransitions:number; ingestionStats:any; trust:number[]; spoofScore:number; traceLength:number; stateCycles:number; P_trace:number};
    }

    function runAcceptanceTests(): string[];
  }

  // ═══════════════════════════════════════════════════════════════════
  // TOP-1 LAYER (6 modules) — AI, ML, AR, Privacy, Emergency
  // ═══════════════════════════════════════════════════════════════════
  namespace Top1 {
    class NavAiCopilot {
      observe(ctx:any): void;
      explainRoute(chosen:any, alternatives:any[], userPrefs:any): string;
      explainPositionQuality(trust:number, env:string, spoofScore:number): string;
      ask(question:string): string;
      getDecisionLog(): any[];
    }

    class PredictiveEngine {
      record(sample:{speed:number; density:number}): void;
      predictCongestion(horizonMin?:number): {probability:number; confidence:number; trend:'improving'|'stable'|'worsening'};
      predictEtaDrift(currentEta:number): {correctedEta:number; drift:number; reason:string};
    }

    class EmergencyResponseSystem {
      addContact(c:{name:string; phone:string; priority:number}): void;
      detectCrash(imuMag:number, speedDelta:number): {detected:boolean; severity:'minor'|'major'|'severe'|null};
      triggerEmergency(location:{lat:number; lon:number}, severity:string, autoCall?:boolean): Promise<{sent:number; beaconActive:boolean}>;
      startBeacon(location:{lat:number; lon:number}): void;
      stopBeacon(): void;
      isBeaconActive(): boolean;
    }

    class PrivacyVault {
      anonymize(lat:number, lon:number, precision?:'city'|'neighborhood'|'street'): {lat:number; lon:number};
      sessionId(): string;
      exportAllData(): Promise<any>;
      deleteAllData(): Promise<void>;
    }

    class ARNavigation {
      isSupported(): Promise<boolean>;
      startAR(onFrame:(pose:any)=>void): Promise<boolean>;
      stopAR(): Promise<void>;
      projectRoute(routeLLA:Array<[number,number,number]>, userLLA:[number,number,number]): Array<{x:number;y:number;z:number}>;
    }

    class MultiModalRouter {
      planMultiModal(origin:any, dest:any, prefs:{maxWalkMin:number; allowTransit:boolean; allowBike:boolean}): Promise<{legs:any[]; totalDuration:number; totalCO2:number}>;
    }

    function runTop1Tests(): string[];
  }

  // ═══════════════════════════════════════════════════════════════════
  // RESILIENCE LAYER (13 modules) — never-fail navigation
  // ═══════════════════════════════════════════════════════════════════
  namespace Resilience {
    const SERVICE_WORKER_CODE: string;
    class ServiceWorkerManager {
      register(): Promise<boolean>;
      cacheRegion(bounds:{n:number;s:number;e:number;w:number}, zoomMin?:number, zoomMax?:number): Promise<{cached:number; failed:number}>;
    }

    class Watchdog {
      constructor(onFail:(subsystem:string)=>void);
      register(subsystem:string, timeoutMs:number): void;
      pet(subsystem:string): void;
      start(): void;
      stop(): void;
      status(): Array<{subsystem:string; lastPetMs:number; timeoutMs:number; healthy:boolean}>;
    }

    class CircuitBreaker {
      call<T>(key:string, fn:()=>Promise<T>): Promise<T|null>;
      reset(key:string): void;
      status(): Record<string, {fails:number; lastFail:number; openUntil:number; open:boolean}>;
    }

    class RoutingFallbackChain {
      route(from:[number,number], to:[number,number]): Promise<{coords:number[][]; source:string}|null>;
    }

    class DeadReckoning {
      init(lat:number, lon:number): void;
      updateFromImu(ax:number, ay:number, az:number, gz:number, dt:number): void;
      propagate(dt:number): {lat:number; lon:number; confidence:number}|null;
      reset(lat:number, lon:number): void;
    }

    class RedundantGeocoder {
      search(query:string): Promise<Array<{lat:number; lon:number; name:string; source:string}>>;
    }

    class SelfHealingController {
      healSubsystem(name:string, healFn:()=>Promise<boolean>): Promise<boolean>;
      stats(): {total:number; recovered:number; rate:number};
    }

    class TileProviderRotator {
      current:number;
      getUrl(): string;
      markFailed(): void;
      markSuccess(): void;
    }

    class BatteryAwareMode {
      init(onChange:(mode:'normal'|'saver'|'critical')=>void): Promise<void>;
      getRecommendations(): {pollIntervalMs:number; maxTilesPreload:number; animationsEnabled:boolean; radarEnabled:boolean};
    }

    class NetworkAdaptive {
      getQuality(): 'offline'|'slow-2g'|'2g'|'3g'|'4g'|'unknown';
      recommendations(): {tileMaxZoom:number; poiEnabled:boolean; weatherEnabled:boolean; routingTimeout:number};
    }

    class ResilienceOrchestrator {
      init(): Promise<void>;
      status(): any;
    }

    function runResilienceTests(): string[];
  }

  // ═══════════════════════════════════════════════════════════════════
  // COMPLETION LAYER (12 modules) — corrections, safety, tests
  // ═══════════════════════════════════════════════════════════════════
  namespace Completion {
    class KlobucharIono {
      setCoefficients(a:number[], b:number[]): void;
      computeDelay(userLat:number, userLon:number, satElev:number, satAz:number, gpsTimeSec:number): number;
    }
    class SaastamoinenTropo {
      computeDelay(elevDeg:number, altM:number, tempK?:number, pressureHpa?:number, humidPct?:number): number;
    }
    class SbasParser {
      parseMessage(bits:Uint8Array): {type:number; prn:number; crc:boolean; payload:any}|null;
    }
    class NtripClient {
      constructor(cb:(rtcm:Uint8Array)=>void);
      connect(proxyUrl:string, mountpoint:string, username?:string, password?:string): void;
      sendPosition(lat:number, lon:number, alt:number): void;
      disconnect(): void;
    }
    class MapMatchingHMM {
      match(gpsTrace:Array<{lat:number;lon:number}>, candidates:Array<Array<{lat:number;lon:number;roadId:string}>>): Array<{roadId:string; lat:number; lon:number}>;
    }
    class SafetyInvariants {
      check(state:any): string[];
      report(): {total:number; recent:any[]};
      clear(): void;
    }
    class TestHarness {
      test(name:string, fn:()=>boolean|Promise<boolean>): void;
      runAll(): Promise<{pass:number; fail:number; results:string[]}>;
    }
    class LlmBridge {
      constructor(endpointUrl:string);
      ask(question:string, context:any): Promise<string>;
    }
    class GtfsLoader {
      loadFromFeed(feedUrl:string): Promise<boolean>;
      stopsNear(lat:number, lon:number, radiusM?:number): any[];
      routesCount(): number;
      stopsCount(): number;
    }
    const PWA_MANIFEST: any;
    function generatePwaIconSvg(size?:number): string;
    function runCompletionTests(): Promise<{pass:number; fail:number; results:string[]}>;
  }

  // ═══════════════════════════════════════════════════════════════════
  // REALITY LAYER (11 modules) — self-correcting intelligence
  // ═══════════════════════════════════════════════════════════════════
  namespace Reality {
    class GroundTruthCollector {
      record(type:'ETA'|'PATH'|'TRUST'|'CONGESTION'|'HAZARD'|'MODE_DURATION', predictedValue:any, confidence:number, context:any): string;
      resolve(id:string, actualValue:any): any;
      statistics(type?:string): {n:number; meanError:number; p95Error:number; bias:number};
      recentResolved(n?:number): any[];
    }
    class CalibrationEngine {
      calibrators: Record<string, {multiplier:number; confidence:number; samples:number; rmse:number}>;
      update(category:string, predicted:number, actual:number): void;
      apply(category:string, rawValue:number): number;
      reset(category:string): void;
      snapshot(): any;
    }
    class ProviderReliability {
      record(provider:string, success:boolean, latencyMs?:number): void;
      rank(): Array<{provider:string; score:number}>;
      best(providers:string[]): string|null;
    }
    class ContradictionDetector {
      check(s:{positioning?:any; fusion?:any; routing?:any; ui?:any; proof?:any}): string[];
      recent(n?:number): any[];
    }
    class ImpossibleStateDetector { check(state:any): string[]; }
    class FailureRegistry {
      capture(type:string, context:any): string;
      recurring(minCount?:number): any[];
      all(): any[];
      clear(): void;
    }
    class DecisionLedger {
      record(type:string, chosen:any, alternatives:any[], confidence:number, factors:Record<string,number>): string;
      resolveOutcome(id:string, actualOutcome:any, correctness:number): void;
      accuracyByType(type:string): {n:number; meanCorrectness:number; highConfErrors:number};
      explain(id:string): string|null;
    }
    class TruthOverride {
      constructor(callbacks:{forceDegraded?:()=>void; forceTrustReset?:()=>void; suppressReroute?:()=>void; forceRecalibration?:()=>void});
      evaluate(evidence:{calibrationRmse?:number; contradictions?:number; recurringFailures?:number; highConfErrors?:number}): string[];
      recent(n?:number): any[];
    }
    class RouteOutcomeTracker {
      startRoute(routeId:string, predicted:Array<[number,number]>, predictedEtaSec:number): void;
      appendPosition(routeId:string, lat:number, lon:number): void;
      finishRoute(routeId:string): {pathDeviation:number; etaError:number; actualEta:number}|null;
    }
    class RealityValidationOrchestrator {
      truth:GroundTruthCollector;
      calibration:CalibrationEngine;
      providers:ProviderReliability;
      contradictions:ContradictionDetector;
      impossible:ImpossibleStateDetector;
      failures:FailureRegistry;
      decisions:DecisionLedger;
      routes:RouteOutcomeTracker;
      override:TruthOverride;
      constructor(callbacks?:any);
      cycle(layerState:any): {actions:string[]; contradictions:string[]; impossible:string[]};
      selfReport(): any;
    }
    function runRealityTests(): string[];
  }

  // ═══════════════════════════════════════════════════════════════════
  // CONSCIOUSNESS LAYER (10 modules) — system that governs itself
  // ═══════════════════════════════════════════════════════════════════
  namespace Consciousness {
    class UnifiedSystemState {
      subsystems: Map<string, any>;
      phase: SystemPhase;
      register(name:string, deps?:string[]): void;
      update(name:string, health:Health, readiness:number): void;
      synthesize(): SystemPhase;
      readinessScore(): number;
      cascadeRisk(): string[];
      report(): {phase:SystemPhase; readiness:number; subsystems:any[]; cascadeRisks:string[]};
    }
    class DecisionGovernor {
      propose(decision:{type:string; payload:any; risk:RiskLevel; impact:string[]; confidence:number; requester:string}, systemPhase:SystemPhase): {approved:boolean; reason?:string; id:string};
      approveManually(id:string): boolean;
      rollback(id:string): any|null;
      stats(): {pending:number; approved:number; rejected:number; recentRejections:any[]};
    }
    class SelfDistrust {
      confidence:number;
      observe(event:'CONTRADICTION'|'IMPOSSIBLE_STATE'|'OVERRIDE_FIRED'|'HIGH_CONF_ERROR'|'HEALTHY_CYCLE'|'CALIBRATION_DRIFT', magnitude?:number): void;
      shouldSuppress(actionRisk:RiskLevel): boolean;
      forceDegrade(): boolean;
      state(): {confidence:number; label:string; recentDrops:any[]};
    }
    class FailSafeAuthority {
      engaged:boolean; reason:string;
      engage(reason:string, scope?:string[]): void;
      disengage(): void;
      isBlocked(action:string): boolean;
      state(): any;
    }
    class TruthHierarchy {
      hierarchy: Record<string, Array<{source:TruthSource; weight:number}>>;
      arbitrate(domain:string, candidates:Array<{source:TruthSource; value:any; confidence:number}>): {value:any; source:TruthSource; confidence:number; arbitrated:boolean};
      adjustWeight(domain:string, source:TruthSource, newWeight:number): void;
    }
    class StabilityController {
      observe(metric:string, value:any): void;
      isOscillating(metric:string, threshold?:number): boolean;
      dampen(metric:string, newValue:any): any;
      jitterScore(metric:string): number;
      report(): Array<{metric:string; jitter:number; oscillating:boolean}>;
    }
    class LimitAwareness {
      check(ctx:{accuracy?:number; altitude?:number; cn0?:number; numSats?:number; mode?:string; dataAge?:number}): any[];
      userVisibleNow(): string[];
    }
    class GlobalAudit {
      record(actor:string, action:string, before:any, after:any, reason:string, critical?:boolean): void;
      query(filter:{actor?:string; action?:string; critical?:boolean; since?:number}): any[];
      export(): string;
      stats(): {total:number; critical:number; uniqueActors:number};
    }
    class ConsciousnessOrchestrator {
      state:UnifiedSystemState;
      governor:DecisionGovernor;
      distrust:SelfDistrust;
      failsafe:FailSafeAuthority;
      hierarchy:TruthHierarchy;
      stability:StabilityController;
      limits:LimitAwareness;
      audit:GlobalAudit;
      tick(input:{subsystems:any[]; contradictions:any[]; impossibleStates:any[]; overrides:any[]; sensors:any}): {phase:SystemPhase; confidence:number; actions:string[]; exposures:string[]};
      proposeAction(action:string, risk:RiskLevel, payload:any, requester:string): {approved:boolean; reason?:string; id?:string};
      selfReport(): any;
    }
    function runConsciousnessTests(): string[];
  }
}

// Global augmentation for browser scripts
declare global {
  interface Window { GANE: typeof GANE }
  const GANE: typeof GANE;
}

export = GANE;
