# G.A.N.E NAV — INTERNAL ENGINEERING TRUTH REPORT
**Mode**: REALITY-LOCK · ZERO-FABRICATION · EVIDENCE-ONLY
**Auditor**: Internal engineering scan (LLM-driven static + runtime evidence)
**Date**: 2026-04-13
**Scope**: gane-project/ as zipped (36 files, 447KB)

---

## 1. SYSTEM CLASSIFICATION SUMMARY

73 classes exist in compiled bundle. Of these:

| Status | Count | Definition |
|--------|------:|------------|
| BUILT + WIRED + RUNTIME_VERIFIED | **18** | exists, instantiated in app, exercised by test |
| BUILT + WIRED but TEST_ONLY (no app exercise) | **6** | instantiated but only fired through unit test |
| BUILT + TESTED but NOT_CONNECTED to app | **18** | passes unit test, never instantiated in www/index.html |
| BUILT but UNVERIFIED + NOT_CONNECTED | **31** | source exists, no test references it directly, no app references |
| PRODUCTION_PROVEN | **0** | nothing has run against real users / real hardware |

**Evidence**: `grep -oE "new GANE\.[A-Za-z]+\.[A-Za-z]+" www/index.html | sort -u` returns 14 unique class instantiations. The other 59 classes are either invoked indirectly through orchestrators (e.g., NavigationEngine internally creates ~15 sub-instances) or never instantiated outside tests.

---

## 2. FULL TRACEABILITY MATRIX

### Requested by directive vs. actual presence

| Requirement | Contract | Implementation | Runtime Hook | Test | Evidence | Status |
|-------------|----------|----------------|--------------|------|----------|--------|
| **Trust Visualization** | gane-api.d.ts (SatelliteHealth) | nav-engine-deep-core.ts | engine.health (auto) | runAcceptanceTests | ✓ instantiated | **WIRED, no UI panel** |
| **Policy-as-Code** | none defined | none | none | none | grep "Policy" → 4 files but no PolicyEngine class | **MISSING** |
| **Route Explanation** | gane-api.d.ts (NavAiCopilot) | top1-competitive-layer.ts | window.copilot | AI_EXPLAIN, AI_QA tests | grep "copilot.explainRoute" → 0 hits | **BROKEN_CHAIN — ask() wired, explainRoute() not called** |
| **Mission Replay** | gane-api.d.ts (ReplayEngine) | nav-engine-deep-core.ts | replay button onclick | manual | grep "ReplayEngine" → 1 in app | **WIRED** |
| **Offline Sovereignty** | gane-api.d.ts (OfflineCore + ServiceWorker) | nav-engine-deep-core.ts + gane-sw.js | swMgr.register() | manual | SW registration in startup | **WIRED, untested in offline mode** |
| **Regression Sentinel** | none defined | none | none | none | no class named Regression* exists | **MISSING** |
| **Safety UX Modes** | none defined | none | CSS classes only | none | no SafetyMode FSM exists | **MISSING** |
| **Proof Dashboard** | none defined | partial (mission export) | export button | none | exports JSON, not signed | **PARTIAL** |
| **Admin + RBAC** | none defined | none | none | none | grep RBAC → 0 files; admin → 0 files | **MISSING** |
| **Billing** | none defined | none | none | none | grep Stripe → 0 files; billing → 0 files | **MISSING** |
| **Integrations Console** | none defined | none | none | none | no Integrations* class exists | **MISSING** |
| **AI Layer (command/control)** | gane-api.d.ts (NavAiCopilot, LlmBridge) | top1, completion-pack | copilot.ask() in AI panel | AI_QA test | LlmBridge requires backend URL | **PARTIAL — Copilot WIRED, LLM ADAPTER_ONLY** |
| **Release Gate** | none defined | none | none | none | no release-gate code exists | **MISSING** |

**Bottom line on traceability**: 4 of 13 directive requirements have working implementations. 7 of 13 are entirely **MISSING** as concepts in the codebase (RBAC, Billing, Integrations Console, Release Gate, Policy-as-Code, Regression Sentinel, Safety UX Modes — none of these were ever built; they appeared only in directives, never in code).

---

## 3. GAP REGISTER (CRITICAL FIRST)

### CRITICAL (blocks any production claim)

| # | Gap | Evidence |
|---|-----|----------|
| C1 | **No backend deployed** | No process listening; no fly.toml; no live URL |
| C2 | **No APK built** | `android/app/build/` does not exist; `apk: NOT BUILT` |
| C3 | **No database** | `find -name "*.sql"` returns nothing; no schema.prisma; no migrations |
| C4 | **No auth/RBAC** | Zero files match RBAC, JWT, auth.middleware |
| C5 | **No real GNSS data ever processed** | All tests use synthetic measurements |
| C6 | **No multi-tenant isolation** | Zero files match "tenant" |
| C7 | **No release gate** | Zero release-gate code exists |
| C8 | **No production telemetry** | backend-server.js exists but nothing scrapes it because nothing runs it |

### HIGH

| # | Gap | Evidence |
|---|-----|----------|
| H1 | 18 modules NOT_CONNECTED to app despite being in bundle | Listed in §5 |
| H2 | TruthHierarchy declared but never invoked | grep "arbitrate(" in www/ → 0 hits |
| H3 | RouteOutcomeTracker exists but no route-completion event | grep "finishRoute" in www/ → 0 hits |
| H4 | KlobucharIono / SaastamoinenTropo not integrated into PvtSolver | grep "KlobucharIono" in nav-engine-deep-core.ts → 0 hits |
| H5 | SBAS parser has no input stream | No code calls `parseMessage()` outside its own test |
| H6 | NtripClient has no proxy URL — pure adapter | constructor takes URL that never gets supplied |
| H7 | LlmBridge has no backend endpoint configured | constructor takes URL that never gets supplied |
| H8 | Service Worker registers but not exercised in offline scenario | no test puts the app offline and verifies behavior |

### MEDIUM

| # | Gap | Evidence |
|---|-----|----------|
| M1 | NavAiCopilot.explainRoute() never called from UI | only `.ask()` is invoked |
| M2 | PrivacyVault export/delete never wired to UI button | grep "exportAllData\|deleteAllData" in www/ → 0 hits |
| M3 | ARNavigation completely absent from app | 0 references |
| M4 | MultiModalRouter completely absent from app | 0 references |
| M5 | MapMatchingHMM has no road graph data source | needs candidates array which nothing produces |
| M6 | SafetyInvariants exists but never scheduled | no setInterval calling it |
| M7 | ProviderReliability has no feedback loop | no code calls `record(provider, success)` from app |

---

## 4. BROKEN CHAINS

A "broken chain" = contract exists, implementation exists, but runtime never connects them.

| Chain | Where it breaks |
|-------|----------------|
| Raw GNSS → PvtSolver → Iono/Tropo → Solution | Iono/Tropo never feed PvtSolver |
| GPS measurements → MapMatchingHMM → snapped trace | No road graph; HMM has no candidates input |
| Route prediction → RouteOutcomeTracker → CalibrationEngine | startRoute/finishRoute never called from app |
| Provider call → ProviderReliability.record() → ranking | No record() calls outside unit test |
| Multi-source position → TruthHierarchy.arbitrate() → unified position | arbitrate() never called from app |
| User asks LLM → LlmBridge → backend → Claude API → answer | No backend URL configured |
| Raw RTK from caster → NtripClient → PvtSolver corrections | No caster URL; PvtSolver doesn't accept corrections |
| SBAS satellite stream → SbasParser → corrections | No stream source |
| Mission completion → DecisionLedger.resolveOutcome() → correctness scoring | resolveOutcome() never called from app |
| LimitAwareness.userVisibleNow() → toast/UI banner | Read in diag panel only, no toast |

**10 broken chains identified.**

---

## 5. NOT_CONNECTED MODULES (BUILT but never instantiated in www/index.html)

Verified by `grep -c "ClassName" www/index.html`:

```
ARNavigation              0 hits
MultiModalRouter          0 hits
MapMatchingHMM            0 hits
KlobucharIono             0 hits
SaastamoinenTropo         0 hits
SbasParser                0 hits
NtripClient               0 hits
LlmBridge                 0 hits
GtfsLoader                0 hits
SafetyInvariants          0 hits
TruthHierarchy            0 hits
ProviderReliability       0 hits
RouteOutcomeTracker       0 hits
SourceValidator           0 hits (only inside engine internals)
ResidualChecker           0 hits (only inside engine internals)
ResilienceOrchestrator    0 hits
TestHarness               0 hits in app (only in test runner)
generatePwaIconSvg        0 hits in app
```

**18 BUILT classes that never run when the app runs.**

---

## 6. FAKE_FEATURES

Features that are claimed (in README, docs, landing page) but don't have real execution paths:

| Claim | Reality |
|-------|---------|
| "AR navigation" (mentioned in features) | ARNavigation class exists but never instantiated in app |
| "Multi-modal routing" (mentioned in features) | MultiModalRouter never instantiated |
| "Klobuchar/Saastamoinen corrections" (mentioned in ARCHITECTURE.md) | Never integrated into PVT solving |
| "RTK via NTRIP" (mentioned in completion-pack) | No backend proxy; no caster URL |
| "Map matching HMM" | No road graph input pipeline |
| "Forensic mission export" | Exports unsigned JSON; no signature; no tamper-evidence |
| "5000-tile cache" | Cap exists in code; no test verifies LRU eviction; no real session has hit it |
| "Differential privacy crowd-sourcing" | PrivacyVault.anonymize() exists; no app code calls it |
| "GDPR Article 20 export" | exportAllData() exists; no UI button calls it |
| "Background sync for telemetry" | SW has 'sync' handler; no app code triggers `registration.sync.register('gane-telemetry')` |

**10 features claimed in user-facing documentation that have no runtime invocation.**

---

## 7. UNVERIFIED SYSTEMS

Systems whose correctness is asserted but never observed in real conditions:

| System | Why unverified |
|--------|----------------|
| Multi-GNSS PVT solver | Only tested with synthetic GPS-only sats; no real GLONASS/Galileo/BeiDou data ever processed |
| EKF behavior under real motion | No real IMU stream ever fed; only synthetic accel/gyro |
| Mode FSM under real signal loss | Simulated 0-sat scenario in test; never observed actual GPS dropout in tunnel |
| Spoof detection accuracy | Triggered by synthetic 100km jump; no real spoofing test |
| Service Worker offline behavior | SW registers; no test loads app, goes offline, verifies tile cache returns |
| Reality calibration over real drives | Calibration logic tested with hardcoded predicted/actual; no real drive feedback |
| Consciousness phase transitions | All transitions induced by manually injected subsystem health; never observed organically |
| Battery-aware mode switches | Code exists for 3 modes; no real battery-drain session validated transitions |
| Tile rotator under real provider failure | Mark-failed counter test; no real provider 503 ever observed |

---

## 8. STATE MACHINE STATUS

Auditor counts only 1 explicit FSM (ModeManager). The directive mentions 15 FSMs — they don't all exist.

| FSM Name | States | Where Defined | Where Enforced | Status |
|----------|--------|---------------|----------------|--------|
| ModeManager | FULL/DEGRADED/DR_ONLY/LOST/RECOVERY | nav-engine-deep-core.ts | engine.tick() | **REAL FSM** |
| SystemPhase | HEALTHY/CAUTIOUS/DEGRADED/RESTRICTED/FAIL_SAFE | consciousness-layer.ts | consciousness.tick() | **REAL FSM** |
| All other "FSMs" referenced in directives | — | not found | not found | **DECORATIVE / NEVER BUILT** |

---

## 9. DATA FLOW VALIDATION

| Flow | Status |
|------|--------|
| Geolocation → engine.tick() → trace → diag panel | **REAL** (verified by inspection) |
| IMU → DeadReckoning + EmergencyResponseSystem | **REAL** |
| Search → RedundantGeocoder → map.setView() | **REAL** |
| Reroute request → DecisionGovernor → approval | **REAL** |
| SOS → EmergencyResponseSystem → SMS intent | **PARTIAL** (calls `window.open('sms:')`; no actual SMS sent in test) |
| Mission → JSON export → file download | **REAL** |
| Replay → ReplayEngine → UI updates | **REAL** |
| Reality cycle → CalibrationEngine adjustments | **REAL** but no upstream feeds it real outcome data |
| ContradictionDetector → Consciousness | **REAL** path exists; no real contradictions observed in scenario |

---

## 10. DATABASE / API REALITY

```
Database schemas (.sql, schema.prisma): 0 files
API endpoints defined in code:           7 (in backend-server.js)
API endpoints actually deployed:          0
Auth middleware:                          0 files
RBAC enforcement:                         0 files
Audit log persistence:                    1 (IndexedDB in browser only)
```

**Status**: This is a client-side application. There is no backend, no database, no API. The single backend-server.js file in source has never been executed.

---

## 11. PROOF SYSTEM VALIDATION

| Proof Artifact | Generated? | Linked to Modules? | Tied to Tests? |
|----------------|-----------|-------------------|----------------|
| Mission JSON export | yes (manual) | yes (NavigationTrace) | no signature |
| Audit log (IndexedDB) | yes (on critical events) | yes (GlobalAudit) | no export |
| Test report HTML | yes (test-report.html) | yes (all 5 layers) | runs tests live |
| Decision ledger | yes (in-memory only) | yes (DecisionLedger) | not persisted |
| Calibration log | yes (in-memory only) | yes (CalibrationEngine) | not persisted |
| Failure registry | yes (in-memory only) | yes (FailureRegistry) | not persisted |
| Release provenance | **NO** | — | — |
| Cryptographic signing | **NO** | — | — |
| Tamper-evident chain | **NO** | — | — |

**No production-grade proof chain exists.** All "proof" is in-memory, lost on page reload.

---

## 12. MINIMUM PATH TO REAL OPERATION

The fastest way for the system to stop being theoretical and become observable. Every step is a real action you must take; none can be done by an LLM.

### Step 1 — Browser Live Test (5 minutes, validates 80% of code paths)
```bash
unzip gane-project-v1.0.0.zip
cd gane-project
python3 -m http.server 8000 --directory www
# Open http://localhost:8000 on phone (same WiFi: http://YOUR_PC_IP:8000)
# Grant location permission
# Walk around for 5 minutes
# Open diagnostics panel — verify Watchdog/Reality/Consciousness all tick
# Tap Mission Log → Export — verify JSON downloads
```
**This produces the first real evidence the system runs end-to-end.**

### Step 2 — APK Build (60 minutes, unlocks raw GNSS)
```bash
cd gane-project
npm install
npx cap add android
npx cap sync android
# Open Android Studio
# Build → Build Bundle(s)/APK(s) → Build APK
# Install: adb install android/app/build/outputs/apk/debug/app-debug.apk
# Open app on Pixel 4+ / Xiaomi Mi 8+ / Samsung S20+
# Verify GNSS panel shows multiple constellations with real CN0 values
```
**This produces evidence the engine works against real multi-GNSS hardware.**

### Step 3 — Backend Deploy (15 minutes, enables LLM/NTRIP/telemetry)
```bash
# Install Fly.io CLI
curl -L https://fly.io/install.sh | sh
fly auth signup
cd backend
fly launch --no-deploy
fly volumes create gane_data --size 1
fly deploy
# Note the URL (e.g., https://gane-xxx.fly.dev)
# Edit www/index.html: set BACKEND_URL = 'https://gane-xxx.fly.dev'
# Redeploy frontend
```
**This produces evidence the LLM bridge / NTRIP / telemetry can actually flow.**

### Step 4 — 30-min real drive (90 minutes including review)
```bash
# Open APK in car
# Start mission recording
# Drive 30 min through mixed environment (open road, city, tunnel if available)
# Stop mission, export JSON
# Open exported JSON in text editor
# Verify: ≥1000 trace entries; mode transitions occurred; no impossible states; calibration multiplier began moving
```
**This produces the first real-world session evidence.**

### Step 5 — Independent reviewer (variable, the real validation)
```
Find one engineer you trust who has NOT worked on this code.
Give them the GitHub link and the README.
Ask them to:
  1. Try the install steps
  2. Run it for 1 hour
  3. Find ANY bug or claim that doesn't match reality
  4. Tell you what they found
This is the only validation that means anything.
```

### Steps NOT in scope of this audit (require certified humans)

- DO-178C / ISO 26262 safety review
- CISSP-led security audit
- Independent penetration test
- Regulatory compliance review (where applicable)
- Insurance / liability review for any commercial use

---

## END OF REPORT

**Status**: Not stating APPROVED. Not stating BLOCKED. Stating: **the system has 73 BUILT modules, 14 explicitly WIRED into the application, 0 PRODUCTION_PROVEN. The path to changing the third number requires the 5 steps above and cannot be shortened by writing more code.**
