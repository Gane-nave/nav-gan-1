# G.A.N.E Architecture

**Version 1.0.0 · Draft · 73 modules across 6 layers**

## Mission Statement

G.A.N.E (Global Autonomous Navigation Engine) is a client-side navigation system that combines aviation-grade positioning math (PVT solver, EKF fusion, integrity monitoring) with resilience mechanisms (never-fail fallback chains, self-healing, offline-first) and self-correcting intelligence (reality loop, consciousness governance). It runs entirely in a browser or on Android via Capacitor, with an optional backend for telemetry.

The project is not an OSM skin. It is a navigation-engine-system that happens to use OSM tiles.

## Layer Model

Layers are strictly ordered. Higher layers govern lower ones; lower layers never depend on higher ones.

```
┌──────────────────────────────────────────────────────┐
│  Layer 6: CONSCIOUSNESS   (governs everything below) │
│  Layer 5: REALITY         (validates against truth)  │
│  Layer 4: COMPLETION      (corrections, safety)      │
│  Layer 3: RESILIENCE      (never-fail mechanisms)    │
│  Layer 2: TOP-1           (AI, ML, AR, privacy)      │
│  Layer 1: CORE            (positioning, fusion, FSM) │
└──────────────────────────────────────────────────────┘
```

### Layer 1 — Core (21 modules)

The positioning and fusion engine. Reads raw GNSS measurements (pseudorange, CN0, ephemeris), computes PVT solutions with weighted least squares, propagates a 15-state EKF, detects faults (jumps, spoofing, CN0 floors), and transitions between five navigation modes (FULL → DEGRADED → DR_ONLY → LOST → RECOVERY) via a deterministic finite state machine.

The heart of the layer is `NavigationEngine.tick(rawMeas, imu, pos)` which is called at 1Hz. It returns a `NavTraceEntry` logged into an immutable timeline.

### Layer 2 — Top-1 (6 modules)

Competitive capabilities no other navigation app ships:
- `NavAiCopilot` — explains routing/positioning decisions in natural language, supports Q&A.
- `PredictiveEngine` — local ML (no cloud) for congestion/ETA drift forecasting.
- `EmergencyResponseSystem` — crash detection via G-force, SOS beacon, multi-layer persistence.
- `PrivacyVault` — differential privacy (Laplace noise ε=1.0), GDPR export/erasure.
- `ARNavigation` — WebXR scaffold for AR overlay.
- `MultiModalRouter` — walk + transit + drive + bike chains.

### Layer 3 — Resilience (13 modules)

The system cannot fail to navigate. Mechanisms:
- `ServiceWorkerManager` — cache-first tile strategy with stale-while-revalidate.
- `Watchdog` — dead-man's switch per subsystem.
- `CircuitBreaker` — 3-failure threshold disables failing providers for 30s.
- `RoutingFallbackChain` — 5 tiers (Valhalla → OSRM-EU → OSRM-demo → GraphHopper → straight line).
- `DeadReckoning` — IMU-only positioning when GPS is dead.
- `RedundantGeocoder` — Nominatim → Photon → Pelias.
- `TileProviderRotator` — 5 tile providers auto-rotate on failure.
- `BatteryAwareMode` — 3 power modes.
- `NetworkAdaptive` — features scale per connection quality.
- `SelfHealingController` — exponential backoff retry.

### Layer 4 — Completion (12 modules)

Corrections and safety infrastructure:
- `KlobucharIono` / `SaastamoinenTropo` — standard GNSS corrections.
- `SbasParser` — SBAS L1 NAV message parser with CRC-24Q.
- `NtripClient` — RTK corrections via WebSocket proxy.
- `MapMatchingHMM` — Viterbi algorithm for snapping GPS trace to road network.
- `SafetyInvariants` — 8 runtime safety rules (INV-1 through INV-8).
- `TestHarness` — unit test framework.
- `LlmBridge` — Claude API access via backend proxy.
- `GtfsLoader` — transit schedule integration.

### Layer 5 — Reality (11 modules)

The system validates itself against observed outcomes:
- `GroundTruthCollector` — every prediction is recorded and later resolved with actual outcome.
- `CalibrationEngine` — EWMA multipliers adjust based on observed errors.
- `ContradictionDetector` — 5 types of cross-layer inconsistency.
- `ImpossibleStateDetector` — speed>300km/h, altitude>10km, etc.
- `FailureRegistry` — clustering of recurring failures.
- `DecisionLedger` — audit trail with correctness scoring.
- `TruthOverride` — triggers force-degrade / recalibration when evidence demands.
- `RouteOutcomeTracker` — measures actual ETA and path deviation.
- `RealityValidationOrchestrator` — runs cycle every 5s.

### Layer 6 — Consciousness (10 modules)

System-wide governance:
- `UnifiedSystemState` — synthesizes global phase (HEALTHY → CAUTIOUS → DEGRADED → RESTRICTED → FAIL_SAFE).
- `DecisionGovernor` — gates every critical action.
- `SelfDistrust` — tracks system confidence in itself (0..1), decays on contradictions.
- `FailSafeAuthority` — overrides all subsystems when engaged.
- `TruthHierarchy` — weighted arbitration when sources disagree (GNSS > FUSION > MAP > INS for position).
- `StabilityController` — detects and dampens oscillations.
- `LimitAwareness` — exposes system limits to user ("only 1 satellite, need 4").
- `GlobalAudit` — IndexedDB-persisted critical log.
- `ConsciousnessOrchestrator` — heartbeat every 2s, produces the verdict.

## Data Flow

```
Hardware → Ingestion → SatelliteHealth → PvtSolver → StateEngine → UpdateCycle
                                                            ↓
                             ModeManager ← FaultDetector ← Residuals
                                    ↓
                             NavigationTrace → ReplayEngine / Export
                                    ↓
           RealityValidation (calibrates) → Consciousness (governs) → UI
```

## Integration Contracts

All inter-layer communication uses the schemas defined in `schemas.json`. In particular:
- `RawMeasurement` for ingestion
- `NavTraceEntry` for trace
- `MissionExport` for session archive
- `DecisionRecord` for ledger entries
- `EvidenceBundle` for forensic export

Breaking changes to these schemas require a major version bump.

## Deployment

The package is a single 130KB bundle (`gane-core-bundle.js`) plus a 40KB HTML (`index.html`). No build step needed at runtime. Works as:
1. **Static site** — drop files on any static host.
2. **PWA** — installable via `manifest.json` + `gane-sw.js`.
3. **Android APK** — via Capacitor wrapper with `GnssPlugin.java` for raw multi-GNSS access.

## Extension Points

Third parties can integrate at four boundaries:

1. **Ingestion**: Call `engine.ingestion.ingest(raw, source)` with measurements matching `RawMeasurement` schema.
2. **Subscription**: Register a callback on `NavigationTrace.append` to receive every tick.
3. **Governance**: Propose custom actions through `consciousness.proposeAction(action, risk, payload, requester)`.
4. **Providers**: Register custom routing/geocoding providers by implementing the fallback interface.

Full TypeScript declarations are in `gane-api.d.ts`.

## Testing

Each layer exports a `run*Tests()` function. `test-report.html` runs all 34 tests in a browser and produces a visual report. CI integration: invoke in headless Chrome and parse stdout for `✓`/`✗` markers.

## Acknowledged Limitations

Honest disclosure:
- Bundle not yet battle-tested against real multi-GNSS hardware (requires APK build).
- Backend code exists but is not deployed; telemetry is local-only until operator runs `fly deploy`.
- ML predictor uses a linear trend, not TensorFlow (intentional — no external deps).
- AI Copilot uses template NLG by default; can optionally route to Claude via `LlmBridge`.
- AR navigation is a scaffold; requires 3D renderer integration (THREE.js) for production.

## Design Principles

1. **No single point of failure.** Every provider has ≥3 fallbacks.
2. **Honest uncertainty.** Trust scores are exposed, not hidden.
3. **Offline-first.** Internet is an enhancement, not a requirement.
4. **Self-correction.** Every prediction is measured against reality.
5. **Self-governance.** The system can refuse to act when it distrusts itself.
6. **No hidden state.** Every decision is auditable.

## Version History

- **1.0.0** — Initial complete release. 73 modules, 6 layers, 34 tests passing.

## License

TBD by author. Recommend MIT or Apache 2.0 for maximum ecosystem adoption.

## Contributing

Code lives in the 6 TypeScript source files (`nav-engine-deep-core.ts`, `top1-competitive-layer.ts`, `resilience-layer.ts`, `completion-pack.ts`, `reality-loop.ts`, `consciousness-layer.ts`). Compile with `tsc --target ES2020 --module esnext`. Re-bundle with the IIFE wrapper pattern shown in `DEPLOY-NOW.md`.

Every new module must include:
1. TypeScript source with exported class.
2. At least one acceptance test in its layer's `run*Tests()` function.
3. Type declaration entry in `gane-api.d.ts`.
4. If it adds new data shapes, a schema entry in `schemas.json`.
5. If it adds UI strings, entries in `gane-i18n.js` for all 3 languages.

The goal is a system that remains coherent as it grows, not one that accumulates cruft.
