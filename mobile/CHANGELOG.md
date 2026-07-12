# Changelog

All notable changes to G.A.N.E Navigator are documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). SemVer.

## [1.0.0] — 2026-04-13

Initial public release. Production-grade navigation system.

### Added
- **Core layer (21 modules)**: MeasurementIngestion, SatelliteHealth, PvtSolver, 15-state EKF (StateEngine/CovarianceMatrix/UpdateCycle), 5-state FSM (ModeManager/FailoverChain/RecoveryValidator), integrity (ResidualChecker/FaultDetector/SourceValidator), sensor adapters, PhysicalWorldModel, NavigationTrace, ReplayEngine, OfflineCore.
- **Top-1 layer (6)**: NavAiCopilot, PredictiveEngine, EmergencyResponseSystem, PrivacyVault, ARNavigation, MultiModalRouter.
- **Resilience layer (13)**: ServiceWorkerManager, Watchdog, CircuitBreaker, 5-tier RoutingFallbackChain, DeadReckoning, RedundantGeocoder, SelfHealingController, TileProviderRotator, BatteryAwareMode, NetworkAdaptive, ResilienceOrchestrator.
- **Completion layer (12)**: KlobucharIono, SaastamoinenTropo, SbasParser, NtripClient, MapMatchingHMM (Viterbi), SafetyInvariants (8 rules), TestHarness, LlmBridge, GtfsLoader, PWA manifest.
- **Reality layer (11)**: GroundTruthCollector, CalibrationEngine, ProviderReliability, ContradictionDetector, ImpossibleStateDetector, FailureRegistry, DecisionLedger, TruthOverride, RouteOutcomeTracker, RealityValidationOrchestrator.
- **Consciousness layer (10)**: UnifiedSystemState, DecisionGovernor, SelfDistrust, FailSafeAuthority, TruthHierarchy, StabilityController, LimitAwareness, GlobalAudit, ConsciousnessOrchestrator.
- Capacitor Android project with GnssPlugin.java exposing raw multi-GNSS.
- PWA manifest + production service worker.
- i18n for English, Hebrew (RTL), Arabic (RTL).
- Landing page, GitHub Actions CI, deploy scripts.
- Complete TypeScript SDK declarations (`gane-api.d.ts`).
- JSON Schemas for all data contracts (`schemas.json`).
- 34 acceptance tests. All passing.

### Known Limitations
- Not certified for safety-critical use.
- APK requires Android Studio build (not pre-built).
- Multi-GNSS requires devices with raw GNSS support.

[1.0.0]: https://github.com/USER/gane-navigator/releases/tag/v1.0.0
