# G.A.N.E NAV — PHASE 1 TECHNICAL DESIGN DOCUMENT (TDD v1.0)

**Scope:** the four critical-path P1 items from the Master Blueprint, designed to code level.
**Status basis:** WASM feasibility **proven in this session** — `aurora-{core,routing,map,gnss,fusion,integrity}` compile cleanly to `wasm32-unknown-unknown` after two one-line fixes (dead `tokio` dep removed from `aurora-events`; `getrandom/js` added for wasm32). Native build unaffected (47/47 core tests pass, clippy clean).

---

## TD-1 — WASM BRIDGE (`gane-wasm`): ONE ENGINE, CLIENT + SERVER

### 1.1 Goal
The TS app currently re-implements navigation math in JavaScript (15-state ESKF, PDR, routing helpers). The Rust engine implements it independently (9-state EKF, WLS PVT, Dijkstra). TD-1 makes the Rust engine the **only** implementation, consumed natively on the server and as WASM in the browser/Capacitor app.

### 1.2 New crate
```
crates/gane-wasm/
├── Cargo.toml       # crate-type = ["cdylib", "rlib"]; wasm-bindgen, serde-wasm-bindgen
└── src/lib.rs       # thin #[wasm_bindgen] facade — NO logic here
```

```toml
[dependencies]
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
aurora-core = { path = "../aurora-core" }        # → gane-core after rename
aurora-gnss = { path = "../aurora-gnss" }
aurora-fusion = { path = "../aurora-fusion" }
aurora-integrity = { path = "../aurora-integrity" }
aurora-map = { path = "../aurora-map" }
aurora-routing = { path = "../aurora-routing" }
```

### 1.3 Exported API surface (v1 — deliberately minimal)
```rust
#[wasm_bindgen]
pub struct GaneEngine { pipeline: CorePipeline }   // fusion+integrity+continuity, no I/O

#[wasm_bindgen]
impl GaneEngine {
    pub fn new(config_json: &str) -> Result<GaneEngine, JsValue>;
    /// GNSS epoch: satellites + pseudoranges + CN0 (JSON per contracts/positioning schema)
    pub fn feed_gnss(&mut self, epoch_json: &str) -> Result<(), JsValue>;
    /// IMU sample at sensor rate
    pub fn feed_imu(&mut self, accel: &[f64], gyro: &[f64], dt_s: f64);
    /// → FusedPosition { lat, lon, alt, speed, heading, cov, mode, integrity_flags }
    pub fn position(&self) -> JsValue;
    /// Load a serialized road-graph tile (see TD-2 format)
    pub fn load_graph_tile(&mut self, tile: &[u8]) -> Result<(), JsValue>;
    /// → RouteResult { polyline, eta_s, cost_breakdown, explanation }
    pub fn route(&mut self, request_json: &str) -> Result<JsValue, JsValue>;
    pub fn integrity_status(&self) -> JsValue;     // RAIM/spoof/jam flags
}
```
Boundary rule: **JSON strings / byte slices only** at the FFI edge (schemas from `contracts/`); no shared mutable state; one engine instance per worker.

### 1.4 Build & packaging
- `wasm-pack build crates/gane-wasm --target web --release` → `@gane/engine` npm package (checked into app/ via workspace, not registry, until publishing is decided).
- App integration: engine runs in a **Web Worker** (keeps 60 FPS budget); main thread communicates via structured-clone messages mirroring the FFI API.
- Server keeps using the native crates directly — same code, zero drift.
- Fallback: if WASM unavailable (old WebView), app calls the existing tRPC endpoints (`position`, `route`) served by the native engine.

### 1.5 Acceptance (Definition of Done)
- Golden-trace harness (TD-3.4) shows **bit-identical route geometry and ≤1e-9 position delta** between native and WASM for the same input log.
- Route computation in WASM ≤ 50 ms for a 10 km urban route on the reference device profile.
- App's `eskf.ts`, `osmRouter.ts` marked deprecated with migration notes.

---

## TD-2 — OSM IMPORT PIPELINE: REAL ROADS, REAL RESTRICTIONS

### 2.1 Source & cadence
- Geofabrik extract `israel-and-palestine-latest.osm.pbf` (~120 MB) — weekly pull; delta updates via `delta_updates` table mechanism already in the app schema.

### 2.2 New crate `crates/gane-osm-import` (CLI, native-only)
Pipeline stages:
1. **Parse** (`osmpbf` crate): stream ways with `highway=*`; collect referenced nodes.
2. **Classify**: map `highway` class → speed defaults; parse `oneway`, `access`, `maxspeed`.
3. **Restrictions** (the vehicle-routing unlock): parse `maxheight`, `maxwidth`, `maxweight`, `maxaxleload`, `hazmat`, `hgv`, `tunnel`, `bridge` into a compact `SegmentRestrictions` bitset+values struct on every edge.
4. **Graph build**: intersection nodes → graph vertices; way segments → directed edges with length (haversine), class, speed, restrictions. Feed into existing `aurora-map::RoadGraphIndex`.
5. **Turn restrictions**: `type=restriction` relations → banned maneuvers table (phase 1.5 if time-boxed).
6. **Tile & serialize**: partition by H3 res-6 cells (aligning with crowd-intelligence bucketing); serialize per-tile with `bincode` + zstd; emit `manifest.json { tile_id, version, sha256, bbox }`.

### 2.3 Storage & loading
- Server: tiles on disk/S3, lazy-loaded LRU by bbox of active requests.
- Client: tiles fetched via existing offline-region mechanism (service worker cache + IndexedDB), loaded into WASM via `load_graph_tile`.

### 2.4 Acceptance
- Israel extract imports in < 10 min; graph covers ≥ 99% of `highway` ways.
- Golden test: route Tel Aviv→Jerusalem returns plausible geometry (snapshot test), and a 4.2 m-height truck profile **avoids a mapped ≤4.0 m underpass** while a car takes it.

---

## TD-3 — CANONICAL ESKF: ONE FILTER TO RULE BOTH SIDES

### 3.1 Decision
Adopt a single **15-state error-state Kalman filter** in Rust (`aurora-fusion`), superseding both the current Rust 9-state EKF and the TS 15-state `eskf.ts`. The TS implementation (Solà-cited, already correct in structure) serves as the **reference spec** for the port; the Rust crate provides the production implementation for native + WASM.

### 3.2 State vector
```
δx = [ δp(3) | δv(3) | δθ(3) | δb_a(3) | δb_g(3) ]   (ENU error state)
Nominal: p, v, q (quaternion), b_a, b_g
```
- Propagation: IMU strapdown on nominal state; F/Q per Solà eqs. (discrete, dt from sensor clock).
- Updates: GNSS position/velocity (from WLS PVT with per-epoch covariance from DOP·URA), heading (magnetometer/course-over-ground gate), barometric altitude, zero-velocity (ZUPT) when stationary detector fires, odometry when available.
- Injection & reset after each update; covariance symmetrization (already practiced in current EKF — keep).
- Integrity hooks: innovation chi-squared gate per measurement (feeds `aurora-integrity` flags; rejected measurements logged with reason — "no silent failure").

### 3.3 Migration steps
1. Implement `Eskf15` alongside existing `ekf.rs` (feature-flag `eskf15`).
2. Port the TS unit tests (uncertainty growth, convergence, angle wrap) + add bias-observability and ZUPT tests.
3. Golden-trace harness (3.4) to compare old/new/TS.
4. Switch `FusionEngine` default to `Eskf15`; deprecate 9-state after one release.

### 3.4 Golden-trace harness (new crate `gane-replay-verify`)
- Input: JSONL logs of raw epochs (GNSS/IMU/odo) — format matches `raw_telemetry` rows so field logs replay directly (the replayEngine + trip_events already give us capture).
- Output: per-epoch fused state from (a) native Rust, (b) WASM, (c) legacy TS (one-off).
- Assertions: native↔WASM bit-identical; new-vs-TS within documented tolerance; regression snapshots stored under `tests/golden/`.
- This harness doubles as the **forensic replay** engine required by the evidence-chain contract.

---

## TD-4 — VEHICLE-ENVELOPE ROUTING: THE P1 PRODUCT UNLOCK

### 4.1 Model (mirrors app's `vehicleProfiles.ts` — single schema in `contracts/`)
```rust
pub struct VehicleEnvelope {
    pub class: VehicleClass,          // 11 types (car…military) — codegen from contracts
    pub height_m: f32, pub width_m: f32, pub length_m: f32,
    pub weight_t: f32, pub axle_load_t: f32, pub axles: u8,
    pub hazmat: bool, pub max_grade_pct: f32, pub turning_radius_m: f32,
}
```

### 4.2 Integration into `aurora-routing`
- **Hard constraints = edge filter**, not cost penalty: an edge whose `SegmentRestrictions` violates the envelope is *not expanded* (correctness > tuning; prevents "least-bad illegal route").
- **Soft preferences = cost terms**: grade, narrow-road discomfort, tunnel avoidance for hazmat where legal but discouraged.
- API: `RoutePlanRequest { origin, destination, envelope: Option<VehicleEnvelope>, cost_profile }`. `None` ⇒ current behavior (backward compatible).
- Emergency-mode bias (blueprint §9): `cost_profile=emergency` relaxes soft terms, never hard legal/physical constraints.

### 4.3 Explanation output (contracts `routeExplanations`)
Every result carries: constraints applied, edges excluded count by reason (`height:12, weight:3`), alternatives rejected & why, confidence. UI surfaces this in the route panel (P2 wiring).

### 4.4 Acceptance
- Golden cases on the imported Israel graph (TD-2.4).
- Property test: for random envelopes, returned route never traverses a violating edge.
- Latency budget: constrained route ≤ 1.2× unconstrained compute time.

---

## SEQUENCING & CI

```
Week 1:  gane-wasm crate + Worker integration (TD-1)      [feasibility already proven]
Week 2:  gane-osm-import MVP → Israel graph (TD-2)
Week 3:  Eskf15 + golden harness (TD-3)                   [parallel with TD-2]
Week 4:  VehicleEnvelope routing + explanations (TD-4)
Week 5:  integration: app consumes @gane/engine for position+route; deprecations
```
CI additions: wasm32 check job for the six core crates (now green); golden-trace regression job; route-latency benchmark with budget assertion; graph-import smoke on a small PBF fixture.

**Risk register:** PBF parse memory on CI (mitigate: streaming + fixture extract); WebView WASM support floor (mitigate: tRPC fallback); ESKF tuning drift vs field logs (mitigate: golden traces from real drives before switch-over); rename `aurora-*`→`gane-*` merge conflicts (do it as the *first* commit of Phase 1, scripted, in one atomic PR).
