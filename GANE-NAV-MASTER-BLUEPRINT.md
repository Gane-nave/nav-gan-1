# G.A.N.E NAV — UNIFIED MASTER ENGINEERING BLUEPRINT v1.0

**Canonical name:** G.A.N.E NAV (Global Autonomous Navigation Ecosystem)
**Date:** 2026-07-12
**Fact model:** `[VERIFIED]` = confirmed by direct inspection/execution in this audit · `[INFERRED]` = justified conclusion from verified data · `[TARGET]` = intended capability, not yet implemented · `[GAP]` = missing/contradictory/unverified

This document supersedes all prior names (AURORA NAV, GMIN, gane-project, Trade, aurora-*). Every module, crate, package, and document converges on the single namespace **G.A.N.E NAV**.

---

## SECTION 0 — CANONICAL PROJECT DEFINITION

G.A.N.E NAV is a **mission-critical, trust-aware, multi-source navigation and orchestration platform**: a Rust high-integrity positioning/routing engine (multi-GNSS fusion, RAIM, spoof/jam detection, cost-based routing) fused with a TypeScript product application (Hebrew-first RTL PWA, vehicle-profile routing, fleet/C4ISR operations, real-time collaboration, payments) and an Android delivery channel (Capacitor raw-GNSS plugin + TWA packaging). It is **not** a consumer map app; its operational class is *fail-operational decision engine for mobility under uncertainty* — it treats GNSS as one signal among many, validates continuously, degrades gracefully, and never fails silently.

---

## SECTION 1 — SOURCE RECONSTRUCTION & PROVENANCE MAP

| Source | Status | Content |
|---|---|---|
| Rust workspace (identical in 4 repos: Gane, nav-gan-1, Amjad-gane, Gane--by-DEVIN) | [VERIFIED — built & executed] | 2,452 crates; ~198 real; runnable `aurora-nav` binary, 16 REST endpoints, Leaflet dashboard |
| gmin-spec (TS full-stack app, 418 files) | [VERIFIED — code-level analysis] | React 19 PWA + Express/tRPC + MySQL/Drizzle + 22 contract modules |
| gane-project v1.0.0 (Node + Capacitor) | [VERIFIED — verification log 28/28 + source inspected] | Missions REST backend, JSONL persistence, GnssPlugin.java, TWA guide |
| G.A.N (2).pdf + USER_REQUIREMENTS_COMPREHENSIVE.md | [VERIFIED] | Product vision, P0/P1/P2 bug & feature list |
| Trade repo (zip) | [VERIFIED — file listing] | React "quantum" UI, Firebase-backed, VehicleProfilePanel |
| navigation2 | [VERIFIED] | Untouched upstream ROS2 Nav2 clone; zero project linkage |
| Gemini/Manus/Devin conversation corpus (pasted) | [VERIFIED as intent] | Multi-GNSS directive, Polymorphic UI spec, Traffic-Pipeline-first operating model, integrations matrix, master-spec template |
| VERIFICATION-LOG-2.txt | [GAP] | Corrupted (all NUL bytes) — unrecoverable |
| Production deployment, API keys, WhatsApp/Gmail/Drive credentials | [GAP] | No evidence anywhere in the corpus |

---

## SECTION 2 — CHRONOLOGICAL PROJECT HISTORY

1. **Genesis** — `AURORA NAV Phase 1`: real navigation pipeline (GNSS→Fusion→Integrity→Continuity), CI, clippy/fmt gates. [VERIFIED]
2. **Phases 2–5 (PRs #1–#7)** — genuine engines added: map/routing/lane/offline; risk/confidence/probabilistic; evidence/trust/anti-manipulation; traffic flow/stability. [VERIFIED]
3. **Phases ~1176–2615 (automated)** — "crate factory" (Devin/Copilot): 18/commit then 72/commit boilerplate crates → 2,254 placeholder crates, 300 duplicate workspace entries; commit totals overstate reality (claims 2,606 crates/19,509 tests vs 2,452/18,497 on disk). [VERIFIED]
4. **Parallel track** — gmin-spec TS app built to the product vision (Hebrew UI, vehicle profiles, contracts layer, 33-table schema). [VERIFIED]
5. **Delivery track** — gane-project v1.0.0 verified 28/28 (missions API, Android GNSS plugin), APK blocked on production URL. [VERIFIED]
6. **This audit** — full-system review; Rust core compiled, binary executed, `/health` operational. [VERIFIED]

**Unresolved debt created along the way:** 4 identical repo copies; ~19 MB of zips committed into git (one zip recursively contains the repo and further zips); no bridge between Rust engine and TS product. [VERIFIED]

---

## SECTION 3 — PRODUCT INTENT & SYSTEM PURPOSE

- **Primary objective:** continuous, high-integrity navigation with *no single point of navigation failure*. [VERIFIED intent]
- **Secondary:** fleet/emergency orchestration (C4ISR/EOC), real-time collaboration, monetization (Stripe, ₪), forensic replay/evidence chain. [VERIFIED intent]
- **Users:** professional drivers, fleet operators, EMS, security/military operations. **Region focus:** Israel/Palestine; Hebrew primary, Arabic/English, full RTL. [VERIFIED]
- **Failure-sensitive scenarios:** tunnel/urban canyon, jamming/spoofing, network loss, backend failure, sensor degradation. [VERIFIED intent; partially implemented]

---

## SECTION 4 — UNIFIED NAMING & CONSOLIDATION DIRECTIVE

1. **Single canonical monorepo** (recommended name: `gane-nav`). Archive `Gane`, `Amjad-gane`, `Gane--by-DEVIN`, `Gane---the-last-project` as read-only mirrors. [TARGET — mechanical]
2. **Namespace unification:** `aurora-*` crates → `gane-*` (workspace-wide rename: directory, `Cargo.toml` package names, `use` paths — a scripted, mechanical migration). TS packages already carry `gane`; keep. App ID stays `com.gane.nav`. [TARGET — mechanical, low risk]
3. **De-duplication:** delete the 2,254 template crates from the workspace (preserve on an `archive/generated-phases` branch); fix 300 duplicate member lines; keep the ~198 real crates. [TARGET — P0]
4. **Zip eviction:** remove all `.zip` artifacts from git history/tree → GitHub Releases. Delete corrupted log. [TARGET — P0]
5. **Truthful metrics:** README/CI badges state real numbers (~198 crates, ~4,507 meaningful tests, 88 adversarial test files). [TARGET — P0]

---

## SECTION 5 — TARGET MONOREPO STRUCTURE

```
gane-nav/
├── .github/workflows/        # CI: build, clippy -D warnings, nextest, coverage, e2e
├── docs/                     # This blueprint, ADRs, C4 diagrams (from gmin-spec/docs)
├── contracts/                # SINGLE SOURCE OF TRUTH (from gmin-spec/shared/contracts)
│   └── *.ts + codegen → Rust types + OpenAPI (apiEnvelope, sloCatalog, stateMachines,
│       eventCatalog, failureMatrix, securityModel, formalVerification, evidenceChain…)
├── engine/                   # Rust workspace (~198 real crates, renamed gane-*)
│   ├── crates/gane-core|gnss|fusion|integrity|routing|map|traffic|v2x|indoor|ar-nav|
│   │   corrections|sensors|continuity|telemetry|api|web|app|orchestrator|…
│   └── Cargo.toml            # deduped members + [profile.release] lto/strip
├── app/                      # TS product (from gmin-spec): client/ server/ drizzle/ e2e/
├── mobile/                   # Capacitor Android (GnssPlugin.java raw GNSS) + TWA config
├── infra/                    # Dockerfiles, docker-compose, monitoring (Prometheus/Grafana),
│   └── k6/                   # load tests (exists in gmin-spec)
└── scripts/                  # rename migration, OSM import, verify.sh (from gane-project)
```

---

## SECTION 6 — COMPLETE FEATURE INVENTORY (STATUS-TAGGED)

| Feature | Status | Where |
|---|---|---|
| Multi-GNSS acquisition (GPS/GLONASS/Galileo/BeiDou) | Partially implemented [VERIFIED] | engine `gane-gnss` (constellation manager, WLS PVT, CN0 weighting); app `multiConstellation.ts`; **real RF/raw-measurement ingestion [GAP]** |
| EKF/ESKF sensor fusion | Implemented (two independent impls) [VERIFIED] | Rust 9-state EKF; TS 15-state ESKF (Solà) — **must converge to one canonical filter** [TARGET] |
| RAIM / spoof / jam detection | Implemented (CN0-heuristics) [VERIFIED] | `gane-integrity`; advanced RF fingerprinting [TARGET] |
| Dead reckoning / tunnel recovery | Implemented in TS engines; Rust continuity manager [VERIFIED] | pdr.ts, tunnelRecovery.ts, positionFallbackChain.ts |
| Corrections (RTK/PPP/SBAS/NTRIP) | Specified only [TARGET] | `gane-corrections` crate is scaffolding-level |
| Routing (Dijkstra, cost functions) | Implemented [VERIFIED] | `gane-routing`; **vehicle-envelope constraints (height/weight/hazmat) [TARGET — P1]** |
| Vehicle profiles (11 types) | Implemented in UI+data [VERIFIED] | `lib/vehicleProfiles.ts`, VehicleSelectionPanel; **not yet wired into route cost function [GAP]** |
| Traffic pipeline (segment speeds, 5s loop) | Implemented [VERIFIED] | app `trafficPipeline.ts` (537 LOC, grid segmentation ~111 m, congestion model, WS emission) — matches the Manus TASK 1 spec almost exactly |
| Crowd intelligence (percentile speeds, MAD outliers) | Implemented, in-memory [VERIFIED] | `crowdIntelligence.ts` — needs persistence [TARGET] |
| Reroute engine (<500 ms) | Implemented client-side [VERIFIED]; latency budget unproven [GAP] | `rerouteEngine.ts` |
| ETA engine | Implemented [VERIFIED] | `etaEngine.ts` |
| Incident system (report→validate→affect routing) | Implemented [VERIFIED] | `incidentEngine.ts` (514), anomalyRouter (multi-source confirmation) |
| Geofencing | Implemented [VERIFIED] | `geofenceEngine.ts` (point-in-polygon, enter/exit/dwell) |
| Offline maps / PWA / service worker | Implemented [VERIFIED] | sw.js 4-cache strategy, IndexedDB offlineStore/offlineSync |
| Real-time collaboration | Implemented [VERIFIED] | collaborationRouter (1,080 LOC) + WS + invites |
| Fleet / C4ISR / EOC / dispatcher | Implemented (UI + DB + routers) [VERIFIED] | fleets/vehicles/missions tables, OPS panels; **Ghost-tailing live-stream [TARGET]** |
| Payments (Stripe, ILS) | Implemented, unconfigured [VERIFIED] | stripe.ts + webhook signature verification; needs keys [GAP] |
| Evidence chain / forensic replay | Implemented as contracts + engines [VERIFIED] | evidenceChain.ts, replayEngine.ts, trip_events audit trail |
| i18n 40+ languages, Hebrew/Arabic RTL | Implemented [VERIFIED] | lib/i18n.ts (670 LOC) |
| Observability (OTel, Prometheus, Sentry) | Implemented, endpoints unconfigured [VERIFIED] | tracing.ts, metricsCollector, monitoring/ |
| WhatsApp integration | [TARGET] — no code, no credentials [GAP] | requires WhatsApp Business API account |
| Gmail / Google Drive push | [TARGET] — integration_channels table exists; watch/Pub-Sub client [GAP] |
| APK (Bubblewrap TWA) | Blocked [GAP: production URL] | full guide exists (G.A.pdf), `com.gane.nav` |
| NeRF photorealistic digital twin | [TARGET — augmentation layer ONLY, never base map] | webgpuNerf.ts scaffold exists |
| AR navigation overlays | Implemented as engine scaffold [VERIFIED]; camera-pose calibration [TARGET] | `gane-ar-nav` (pinhole projection), arHud.ts |
| V2X (BSM/SPaT/GLOSA/TTC) | Implemented as simulation-grade engines [VERIFIED]; hardware integration [TARGET] |

---

## SECTION 7 — LAYERED ARCHITECTURE (CANONICAL STACK)

```
L0 Mission Definition        → missions/trips (app DB)                     [VERIFIED]
L1 Identity/Roles/Trust      → OAuth+JWT, RBAC (user/admin/dispatcher/…)   [VERIFIED, keys GAP]
L2 Device/Vehicle Registry   → devices, vehicles, vehicle profiles         [VERIFIED]
L3 Position Ingestion        → GnssPlugin (Android raw GNSS) / gpsd NMEA   [PARTIAL: plugin exists, engine feed GAP]
L4 Multi-GNSS Fusion         → gane-gnss WLS PVT + constellation scoring   [VERIFIED]
L5 INS/DR Fusion             → canonical ESKF (merge Rust EKF + TS ESKF)   [TARGET P1]
L6 Map Matching              → gane-map matcher + osmRouter                [VERIFIED, needs real map data]
L7 Road Graph + Restrictions → OSM import + vehicle envelopes              [TARGET P1 — the key unlock]
L8 Routing/ETA/Reroute       → gane-routing + etaEngine + rerouteEngine    [VERIFIED core]
L9 Guidance UX               → NavigationHUD, voice, polymorphic modes     [VERIFIED core]
L10 Offline                  → SW caches + IndexedDB + delta_updates       [VERIFIED]
L11 Fleet/Dispatch           → fleet routers + OPS panels                  [VERIFIED]
L12 Telemetry/Event Sourcing → raw_telemetry (hot/warm/cold/archive) + eventCatalog [VERIFIED]
L13 Alerts/Escalation        → notificationRouter + alertWebhook + mute controls [VERIFIED; mute UX = P0]
L14 Payments                 → Stripe + payment_events (ILS)               [VERIFIED, keys GAP]
L15 Integrations             → integration_channels (in_app/gmail/drive/sms/push/webhook) [schema VERIFIED, connectors TARGET]
L16 Security/Compliance      → securityModel (6 trust zones, STRIDE), evidenceChain [VERIFIED as contracts]
L17 Observability/SRE        → OTel + Prometheus + Grafana + SLO catalog   [VERIFIED, endpoints GAP]
L18 DR/BCP                   → failureMatrix (detection→fallback→recovery) [VERIFIED as contracts]
L19 Analytics/ML             → mlPrediction, modelRegistry (canary/shadow/drift) [contracts VERIFIED, training TARGET]
L20 Governance               → schemaRegistry, release gates, formalVerification (ISO 26262/DO-178C matrices) [VERIFIED as contracts]
```

Rule for every layer: defined inputs/outputs, failure mode, observability hook, and a contract in `contracts/` enforced by conformance tests in CI. [TARGET — the contracts and tests already exist; make them a mandatory PR gate]

---

## SECTION 8 — POSITIONING STACK (GNSS + SENSORS)

**Verified current:** constellation manager with per-satellite quality scoring; WLS PVT (ECEF, pseudorange, CN0 weights, DOP); 9-state EKF (Rust); 15-state ESKF, PDR, urban-canyon and tunnel-recovery engines (TS); CN0-history jamming/spoofing/multipath detector; continuity modes incl. `EMERGENCY_BOUNDED`; Android raw-GNSS plugin (Java). 

**Target-state (build order):**
1. **Real signal ingestion** — Android `GnssMeasurement` → engine feed; gpsd/NMEA adapter for embedded. [P1]
2. **Canonical fusion** — one 15-state ESKF (position/velocity/attitude/biases), Rust implementation compiled to native + **WASM** so client and server share the exact filter. [P1]
3. **Dual-mode operation** — fused all-constellation solution + single-constellation fallback + automatic mode switching (already modeled by ContinuityManager). [P1]
4. **Corrections** — NTRIP client → RTK/PPP/SBAS, correction aging, cached fallback. [P2]
5. **Integrity hardening** — residual chi-squared tests (mathModels.ts already defines them), satellite exclusion, cross-check vs map matching + INS. [P2]
6. NavIC/QZSS, dual-antenna heading, visual odometry. [P3]

**Engineering target:** *No single point of navigation failure* — every source loss has a defined degraded mode and a defined re-entry validation path (re-acquisition → re-validation → fused re-entry).

---

## SECTION 9 — ROUTING, GUIDANCE & DECISION ENGINE

- Graph: `gane-map` RoadGraphIndex ← **OSM import pipeline [P1]** (Israel extract first; tiles versioned via delta_updates).
- Cost model: existing pluggable `CostFn` (distance/time/no-tolls/avoid-tunnels) **extended with vehicle envelope** (height/width/weight/axles/hazmat/turning radius from vehicleProfiles) and **trust/risk weighting** (gane-risk + trust scores). [P1]
- Reroute triggers: deviation (exists), incident-confirmed (exists), congestion delta (traffic pipeline exists) — bind all three to one RoutingFSM with a **500 ms reroute budget measured in CI benchmarks**. [P1]
- ETA: segment-speed based (exists) + learned correction [P3].
- Explanation engine: every route change emits `why` (trigger, alternatives rejected, confidence) — schema exists in contracts (routeExplanations); wire to UI. [P2]

---

## SECTION 10 — POLYMORPHIC UI SYSTEM (SPATIAL CONSCIOUSNESS LAYER)

**State machine (authoritative):** `PASSIVE / NAVIGATION_ACTIVE / ALERT / EMERGENCY / COURIER / DISPATCHER / DEGRADED / OFFLINE` — implement as an actual FSM in `engine/fsm` (fsm.ts exists; formalize transitions + guards + telemetry emission per stateMachines.ts contract). [P1]

**Render stack (strict order):** base map (vector/raster, offline-capable) → 3D/NeRF **augmentation only, auto-disabled offline/low-GPU** → navigation overlays → contextual alerts → interaction layer. Adaptive contrast engine mandatory; glassmorphism falls back to flat HUD in glare/night/emergency. [TARGET, partial assets exist: HolographicEffects, QuantumBoot, NavigationHUD]

**Hard performance budgets:** 60 FPS; UI response ≤100 ms; navigation update ≤50 ms; AR alignment error ≤1 m; auto layer-degradation when FPS drops (priority: navigation > alerts > visuals). [TARGET — enforce via Playwright + frame-timing tests]

**Mode notes (mapped to existing panels):** Emergency = black background/high-contrast red route/glove targets (EOCPanel exists); Courier = multi-stop VRP sidebar + curb-side + dimensional warnings (Transport/Parking panels exist); Dispatcher = clustering + entity expansion + AI command console (CommandCenter/C4ISR panels exist; ghost-tailing needs streaming pipeline [TARGET]).

**P0 UX debts from requirements doc:** empty buttons/labels, notification spam (add rate-limit + mute-per-category using notification_preferences table), theme consistency — close and verify on-device. [P0]

---

## SECTION 11 — INTEGRATIONS MATRIX

| Integration | Purpose | Status | Missing |
|---|---|---|---|
| MySQL | system of record | code VERIFIED | `DATABASE_URL` [GAP] |
| OAuth | identity | code VERIFIED | server URL, APP_ID, JWT secret [GAP] |
| Stripe | payments, ILS catalog | code VERIFIED (checkout+webhook sig) | live keys + webhook secret [GAP] |
| Redis | pub/sub, scaled rate-limit | code VERIFIED (fallback in-memory) | `REDIS_URL` [GAP] |
| Maps/traffic providers (Mapbox/HERE/TomTom + Google proxy) | routing/tiles/traffic | multi-provider code VERIFIED | API keys [GAP] |
| S3 | offline packages, media | SDK wired | credentials [GAP] |
| Sentry / OTLP | errors/tracing | wired | DSN/endpoint [GAP] |
| Gmail push (watch+Pub/Sub, 7-day renew) | ops notifications | [TARGET] | connector + Google Cloud project [GAP] |
| Google Drive (watch/changes webhook) | evidence/report sync | [TARGET] | connector [GAP] |
| WhatsApp Business API | alerts to operators | [TARGET] | approved business number, provider, tokens [GAP] |
| Main-server sync | client↔server state | VERIFIED (tRPC + WS + sync queue) | — |

Every connector follows one pattern: canonical event in → `integration_channels` routing → delivery with retry policy + dead-letter + signed webhooks (apiEnvelope contract). [TARGET for new connectors; pattern exists]

---

## SECTION 12 — EVENT MODEL, TELEMETRY & DATA

- **Event catalog:** 12 domains / ~35 typed payloads with routing table (priority, persistence, PII) — exists in contracts; enforce in CI. [VERIFIED]
- **Mandatory envelope fields:** event_id, correlation_id, trip_id, route_id, mission_id, device_id, vehicle_id, user_id, ts_device, ts_server, location+accuracy, source_provider, confidence, policy_version, app_version, payload_hash, retry_count, outcome, latency, signature. [VERIFIED in contracts; audit actual emit sites — P2]
- **Data model:** 33 tables exist (users, devices, trips, routes, waypoints, alerts, payment_events, trip_events, geofences, raw_telemetry with hot/warm/cold/archive retention, map_anomalies, fleets/vehicles/missions, delta_updates, admin_actions, feature_flags, collaboration cluster, notifications, POIs). Additions: `trust_records`, `policy_rules`, `offline_packages`, `sync_jobs` as first-class tables. [P2]
- **Replay:** replayEngine + trip_events + evidence chain → frame-by-frame forensic reconstruction. Persist crowd/moderation in-memory stores to disk. [P1/P2]

---

## SECTION 13 — PERFORMANCE & SLO CATALOG (BINDING)

Already formalized in `contracts/sloCatalog.ts` — adopt as-is and measure: position availability 99.9%; position-fix p99 <100 ms; route computation p95 <500 ms; reroute ≤500 ms; ETA within 10% @85%; incident detection p95 <30 s; telemetry ingest >10k events/s; traffic freshness <60 s; API error rate <0.1%; offline sync durability >99.9%; map render >30 FPS (UI target 60); V2X p99 <100 ms. Each SLO has error budget, burn-rate alert, runbook. **CI gate: k6 + benches must pass budgets before release.** [VERIFIED spec / TARGET enforcement]

---

## SECTION 14 — SECURITY, PRIVACY, GOVERNANCE

Adopt existing contracts as enforced policy: 6 trust zones with per-boundary TLS1.3/JWT-RS256/param-queries/V2X-PKI/MFA rules; STRIDE threat register + red-team catalog (CVSS) → convert to CI security checks + periodic review; evidence chain (SHA-256/384/512, signatures, custody) for incidents; RBAC matrix (viewer/operator/admin/sre/super_admin); secrets only via env/secret-manager (never in git — **current zips must be purged from history for hygiene** [P0]); GDPR/data-minimization per privacy classification on every event. Formal-verification matrices (ISO 26262 ASIL / DO-178C DAL / IEC 61508 SIL) remain the certification roadmap for safety-critical subsystems. [VERIFIED as spec / TARGET as enforcement]

---

## SECTION 15 — RELIABILITY & DR

Failure matrix exists (GNSS blackout, partition, DB failure, ML failure, memory pressure, third-party degradation, security incident — each with detection latency, fallback, recovery steps/time). Bind it: chaos tests (chaosTesting.ts exists) run the matrix in CI; graceful-degradation pattern (already implemented for every external dependency) is the standard; backpressure/retry/retention live in reliability.ts (657 LOC). Rule: **no failure may break navigation output** — degraded modes end at `EMERGENCY_BOUNDED`, never blank. [VERIFIED core / TARGET drills]

---

## SECTION 16 — MARKET CAPABILITY MATRIX

- **Commodity (don't rebuild):** base maps/tiles, commercial routing/traffic APIs, GPS-only nav apps.
- **Fragmented across vendors:** multi-GNSS integrity, indoor/outdoor continuity, fleet+dispatch+medical semantics, signed telemetry replay.
- **Genuinely differentiated (this project's moat):** one platform combining GNSS-integrity-first navigation + trust engine + evidence chain + Hebrew-first RTL emergency/fleet UX + offline sovereignty + executable engineering contracts (12k lines of enforced SLO/security/FSM specs). The 4,507 real tests + 88 adversarial suites are a credibility asset — market them, not inflated counts.

---

## SECTION 17 — GAP ANALYSIS (IMPLEMENTATION vs TARGET)

| Capability | Evidence | Status | Next action |
|---|---|---|---|
| Rust engine ↔ TS product bridge | none | [GAP — CRITICAL] | compile gane-{core,gnss,fusion,integrity,routing} to WASM; expose per apiEnvelope |
| Real map data (OSM) in engine | in-memory graph only | [GAP — CRITICAL] | OSM Israel import → RoadGraph + restrictions |
| Real GNSS feed | plugin exists, unwired | [GAP] | GnssPlugin → ESKF ingestion path |
| Vehicle-aware routing | profiles exist, cost fn ignores them | [GAP] | envelope constraints into CostFn |
| Production deployment | none | [GAP] | deploy app/ to stable URL; then Bubblewrap APK |
| 9 external service configs | code ready | [GAP] | provision DB/Redis/Stripe/maps/S3/Sentry/OTLP/OAuth |
| Repo consolidation & de-bloat | 4 identical repos + 2,254 stub crates | [GAP] | Section 4 directive |
| WhatsApp/Gmail/Drive connectors | schema only | [TARGET] | build on integration_channels pattern |
| NeRF/AR/ghost-tailing | scaffolds | [TARGET P3] | only after core is live |

---

## SECTION 18 — EXECUTION BACKLOG (PRIORITIZED)

**P0 — Foundation truth (week 1):**
1. Canonical repo + archive mirrors. *Done when:* one repo, others read-only.
2. Purge zips (git history rewrite) + delete corrupt log. *Done when:* clean history, artifacts in Releases.
3. Quarantine 2,254 stub crates; dedupe Cargo.toml; add `[profile.release]` (thin LTO, strip). *Done when:* `cargo check --workspace` on ~198 crates, CI < 15 min.
4. Truthful README/badges; CI gate switched to coverage (cargo-llvm-cov) + nextest; CI covers all active branches.
5. Close P0 UX: labels/buttons audit, notification mute + rate-limit, theme tokens unification. *Done when:* Playwright suite green on the five P0 items.

**P1 — One product (weeks 2–5):**
6. `aurora-*`→`gane-*` rename migration (scripted).
7. WASM build of engine core + TS binding; replace duplicate TS filter with canonical ESKF. *Done when:* same fix/route output client & server on golden traces.
8. OSM import → engine RoadGraph; vehicle-envelope cost constraints. *Done when:* truck test-route avoids low bridge on real Israel data.
9. GnssPlugin raw measurements → fusion pipeline on-device. *Done when:* live drive log shows fused fixes with integrity flags.
10. Provision the 9 integrations; deploy to production URL; Bubblewrap APK to internal track. 
11. RoutingFSM + reroute latency benchmark (≤500 ms) in CI.

**P2 — Hardening (weeks 6–10):**
12. Contracts as CI gates (conformance + SLO k6 budgets + chaos matrix).
13. Persistence for crowd/moderation/rate-limit (Redis/DB); trust_records + policy_rules tables; route-explanation UI.
14. NTRIP/RTK corrections client; residual-test integrity upgrade.
15. Gmail/Drive connectors on integration_channels; WhatsApp pending business-account [GAP].

**P3 — Differentiators:**
16. Learned ETA correction + drift-monitored models (modelRegistry).
17. AR camera-pose calibration; NeRF augmentation layer (LOD, auto-disable).
18. Dispatcher ghost-tailing streaming pipeline (permissioned, compressed).
19. navigation2/ROS2: adopt only if a robotics/AV line is funded; otherwise drop the fork. [DECISION REQUIRED]

---

## SECTION 19 — OPERATING MODEL (HOW WORK PROCEEDS)

One target per cycle → build minimal real version → attach metrics immediately (latency/accuracy/throughput/failure-rate) → run real data (or realistic synthetic) → log everything → verify continuous operation, correct degradation, recovery → only then advance. **Decision rule:** improves routing decisions/latency/reliability → do it; cosmetic-only → defer. **Build order per feature:** engine → pipeline → control → observability → API → UI. **Definition of Done everywhere:** executed in a real flow, affects output, can fail, failure handled, emits telemetry. No completion claims without an execution artifact (test log, benchmark, replay trace).

---

## SECTION 20 — OPEN CONTRADICTIONS & UNKNOWNS

- Two positioning filters (Rust 9-state vs TS 15-state) — resolved by P1 item 7. [GAP→plan]
- Conversation stack claims (PostgreSQL/PostGIS/Redis/tRPC) vs actual app DB (MySQL/Drizzle) — **decision:** keep MySQL now; PostGIS optional at OSM-import layer only. [DECISION]
- Commit-message totals overstate on-disk reality (2,606 vs 2,452) — treat commit messages as non-authoritative. [VERIFIED]
- navigation2 fork purpose undefined. [DECISION REQUIRED]
- No WhatsApp business account, no production host, no provider keys anywhere in corpus. [GAP — owner action]
- "Gane---the-last-project" empty repo intent (fresh canonical home?) — candidate location for the consolidated monorepo. [DECISION]

---

## FINAL DEFINITION

G.A.N.E NAV = one repository, one namespace, one contracts layer, one fusion engine shared client/server, real map data, real signals, enforced SLOs — a navigation platform whose every capability is **functionally real, verifiably executing, and progressively expanding**; never visually complete without being operationally true.

---

## APPENDIX A — COMPLETE ASSET INVENTORY (EVERYTHING FOUND, EVERYWHERE)

| Asset | Location | Disposition |
|---|---|---|
| Rust workspace (identical ×4) | Gane / nav-gan-1 / Amjad-gane / Gane--by-DEVIN | nav-gan-1 = canonical; others archive |
| ~198 real crates (EKF, PVT, RAIM, Dijkstra, traffic, V2X, indoor, AR, API, web, app) | `crates/` | keep → rename `gane-*` |
| 2,254 template crates (5-bool structs, Phase 1–2615) | `crates/` | quarantine to archive branch |
| gmin-spec app (418 files: client 216, server 74, contracts 22 modules, drizzle 33 tables, e2e, k6, monitoring, C4 docs) | `gmin-spec-complete*.zip` (×5 copies) | becomes `app/` + `contracts/` + `infra/` |
| gane-project v1.0.0 (Node backend 28/28-verified, www PWA bundle, Capacitor android, GnssPlugin.java, VERIFY.sh, TWA configs) | `gane-project-v1.0.0*.zip` | becomes `mobile/` + `scripts/verify` |
| Trade frontend (React quantum UI, Firebase, VehicleProfilePanel, NavigationFSM.ts, TrustHUD) | `gh-repo-clone-amjad2161-Trade-main.zip` | harvest components; Firebase path deprecated in favor of app/ stack |
| Phase0 workspace snapshots (12,745 files, incl. nested zips) | `*aurora-nav-phase0.zip` ×2 | delete from git → Release artifact |
| AEGIS/Brainiac scaffold (Python "open_sovereign", GodAgent, SatLink…) | mislabeled `anthropics-skills` zip | out of scope — separate prototype; archive |
| G.A.pdf (APK/TWA guide, `com.gane.nav`, #0A0F1C) | Gane--by-DEVIN | move to `docs/` |
| G.A.N (2).pdf + USER_REQUIREMENTS_COMPREHENSIVE.md (vision + P0/P1/P2) | Gane--by-DEVIN + spec | move to `docs/`; traceability in Appendix F |
| VERIFICATION-LOG.txt (28/28) | Gane--by-DEVIN | move to `docs/evidence/` |
| VERIFICATION-LOG-2.txt | Gane--by-DEVIN | corrupted (all NUL) — delete |
| navigation2 (upstream Nav2 v1.4.0, zero modifications) | separate repo | keep only if robotics track funded [DECISION] |
| Gane---the-last-project (empty README repo) | separate repo | candidate home for consolidated monorepo [DECISION] |
| .agents skills (testing-aurora-nav), CI workflows, SECURITY.md (Amjad-gane only) | various | merge into canonical repo |

## APPENDIX B — UX PERFECTION CATALOG (20 PRINCIPLES → MECHANISM)

1. Zero Friction → one-action navigation start; auto-start via habit model (trips history) 
2. Invisible Interaction → PASSIVE state renders map+marker only; UI appears on context triggers 
3. Context Totality → inputs: position, time, habits, driving state, traffic, weather → UI profile selection 
4. Perceptual Stability → camera-behavior controller; no jitter (fused-position smoothing, fixed scale policy) 
5. Temporal Precision → instruction timing = f(speed, complexity, human reaction window); GuidanceFSM 
6. Optimal Information Hierarchy → max 3 concurrent visual signals; priority renderer 
7. Deep Personalization → driver profile (style, confidence, route prefs, decision history) 
8. Absolute Consistency → same state ⇒ same response; FSM-driven UI forbids ad-hoc branches 
9. Zero-Latency Perception → ≤100 ms UI budget; optimistic rendering 
10. Effortless Control → voice (VoiceCommandSystem exists), gestures, minimal touch 
11. Preemptive UI → predictive prefetch of next maneuver/panel 
12. Error Prevention → invalid actions unselectable; confusion detection → simplify mode 
13. Extreme-Condition Adaptation → night/sun/rain/cognitive-load → contrast + density switch 
14. Instant Trust → single source of truth (backend state), no UI without backend truth 
15. Continuity → phone↔car↔return; state sync via WS + offline queue 
16. Functional Minimalism → every element must justify criticality 
17. Complexity Hiding → engine complexity never surfaces; explanations on demand only 
18. Physiological Adaptation → brightness/audio adapt to ambient light/noise; fatigue-aware palette 
19. Full Environment Integration → Android Auto/CarPlay/HUD [TARGET P3] 
20. User Independence → system operates for the user; zero learning curve as acceptance test

## APPENDIX C — VISUAL LANGUAGE CATALOG (20 PRINCIPLES → RENDER MECHANISM)

1. Rigid perceptual hierarchy — one dominant element (next maneuver) 
2. State-based visual system — each FSM state owns a style profile 
3. Shape-over-text encoding — arrows/line-weight/direction; text last resort 
4. Motion as data — animation only to convey direction/intent 
5. Zero visual noise — no flicker/jumps/sudden scale changes 
6. Adaptive contrast — computed real-time (sun/night/rain) — MANDATORY engine 
7. Functional colors only — blue=navigation, red=danger/emergency only, green=confirmed flow, yellow=warning 
8. Perceptual depth — layering for bridges/underpasses; 3D only where it prevents confusion 
9. Attention steering — dim irrelevant regions; temporary highlight 
10. Critical micro-interactions — immediate feedback, no logical jumps 
11. Speed-aware scale — high speed ⇒ fewer details 
12. Adaptive density — information load per driving state 
13. Geometric consistency — one representation per road/arrow class 
14. Edge clarity — high sharpness, no smears 
15. Temporal layering — sequenced appearance, never all at once 
16. Cognitive-load prevention — screen understood without "reading" 
17. Peripheral-vision optimization — critical info visible off-axis (color/motion) 
18. Anti-overlap system — collision-free label/element placement 
19. Smart visual redundancy — critical info in color+shape+motion 
20. One visual language — single design-token system across all panels (fixes P0 color inconsistency)

## APPENDIX D — STRATEGIC MOAT & POWER LAYERS (CONDENSED, MAPPED)

Proven core advantage: integrity-first navigation + evidence chain + Hebrew-first emergency UX (Sections 8, 14, 16). Objective benchmarks: ETA error (median/p95), off-route rate, TTFF, reacquisition, jitter — add to CI bench suite [P2]. Distribution: PWA→TWA now; Android Auto/CarPlay/OEM/SDK [P3]. Revenue: B2C premium (offline/lane/AR), B2B API, fleet, marketplace — pricing model already sketched in systemEngineering.ts. Trust/privacy: on-device processing default, anonymization, retention controls (Section 14). Habit lock, default capture, city/authority partnerships, contract lock-in, narrative control — business-track items outside engineering scope, tracked as program risks. IP: fusion/anti-spoof/ETA methods are patentable candidates once the canonical ESKF + integrity upgrades land [P2+].

## APPENDIX E — INTEGRATION EXECUTION PREREQUISITES (OWNER-ACTION CHECKLIST)

All are [GAP: owner must provision] — no credential exists anywhere in the corpus:
1. **Main server** — host/domain, TLS, deploy target (closes APK blocker) 
2. **WhatsApp** — approved Business number, BSP provider, API token, webhook URL, send/receive permissions 
3. **Mail** — provider, SMTP/IMAP hosts+ports, app password, TLS 
4. **Alert logic** — event→alert rules (message, mail, payment failure, charge request, document, status) 
5. **Payments** — Stripe live keys + webhook secret; currency ILS (catalog exists) 
6. **Automation rules** — who is alerted, channel, SLA, retry count, failure path 
7. **Security** — secret manager, IP allowlist, webhook signature keys, role permissions 
8. **Database** — managed MySQL URL; backup policy 
9. **Monitoring** — Sentry DSN, OTLP endpoint, Grafana host, heartbeat checks 
10. **Sandboxes** — Stripe test, WhatsApp sandbox, test mailbox, edge-case test user 
11. **Process map** — input→processing→alert→payment link→confirmation→update→audit (schema exists: trip_events + payment_events + integration_channels) 
12. **Access document** — who holds passwords, DNS, mail domain, WhatsApp business account

## APPENDIX F — REQUIREMENTS TRACEABILITY (VISION DOC → BACKLOG)

| Requirement (USER_REQUIREMENTS / PDFs) | Blueprint item |
|---|---|
| P0 empty buttons/labels | Backlog #5 |
| P0 vehicle type selection | exists in code; wiring = Backlog #8 |
| P0 color consistency | Appendix C #20 + Backlog #5 |
| P0 notification spam/mute | Backlog #5 (notification_preferences) |
| P1 vehicle-specific routing | Backlog #8 |
| P1 category selectors | Backlog #5 audit scope |
| P1 responsive + offline-region + PWA-install testing | Backlog #5/#10 e2e |
| P2 APK generation | Backlog #10 (blocked by Appendix E #1) |
| P2 live traffic layer | trafficPipeline exists → provider keys (Appendix E) |
| P2 panel simplification + descriptions | Appendix B #6/#16/#17 |
| Multi-GNSS "never blocked, never interrupted" | Section 8 + Backlog #7/#9/#11 |
| WhatsApp/Gmail/Drive/payment alerts | Section 11 + Backlog #15 + Appendix E |
| Hebrew-first RTL everywhere | i18n exists [VERIFIED]; translation of docs → P2 |
| NeRF/AR/digital twin/ghost tailing | Backlog #17/#18 (augmentation only, post-core) |
