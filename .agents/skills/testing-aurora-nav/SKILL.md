# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 10 crates implementing a multi-GNSS navigation pipeline.

## Crate Structure
| Crate | Tests | Purpose |
|---|---|---|
| aurora-core | 0 | Data model (no logic to test) |
| aurora-events | 3 | Event bus pub/sub |
| aurora-gnss | 5 | GNSS receiver, quality scoring |
| aurora-corrections | 0 | Correction source stubs |
| aurora-sensors | 2 | IMU processing |
| aurora-fusion | 6 | EKF, coordinate transforms |
| aurora-integrity | 8 | Anomaly detection, trust scoring |
| aurora-continuity | 12 | Mode switching, health FSM |
| aurora-telemetry | 3 | Ring-buffer recorder, audit |
| aurora-api | 4 | REST API endpoints |
| **Total** | **43** | |

## Build & Test Commands
```bash
cargo build          # Build all crates (should be zero warnings)
cargo clippy         # Lint (may have style warnings — not errors)
cargo test           # Run all 43 unit tests
```

## Running the API Server
The `aurora-api` crate is a library, not a binary. To run the server:
1. Create a temporary `src/bin/test_server.rs` in `crates/aurora-api/`:
```rust
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9876));
    aurora_api::run_server(addr).await.unwrap();
}
```
2. Run with `cargo run --bin test_server`
3. Clean up the bin directory after testing

## API Endpoints (default state)
| Endpoint | Status | Notes |
|---|---|---|
| GET /health | 200 | `status: "operational"`, `version: "0.1.0"` |
| GET /position | 503 | No GNSS fix = SERVICE_UNAVAILABLE |
| GET /integrity | 200 | `level: "NoSolution"`, `continuity_mode: "E: Emergency Bounded"` |
| GET /status | 200 | All counters at 0 in fresh state |
| GET /telemetry | 200 | Empty samples array |
| GET /constellation | 200 | Empty JSON array |

## Known Issues
- `cargo clippy` produces ~3 style warnings (if_same_then_else, manual_clamp, comparison_chain) — these are not bugs
- No CI workflow is configured yet (no `.github/workflows`)
- No binary entry point for `aurora-api` — must create temporary bin to run server
- The repo may be hosted under a different name on GitHub (e.g. `Trade` instead of `aurora-nav`) due to GitHub App permission limitations for repo creation

## Coordinate Transform Verification
To verify the EKF math, test the `geodetic_to_enu` and `enu_to_geodetic` functions in `aurora-fusion/src/engine.rs`:
- Use a known origin (e.g. Tel Aviv: 32.0853°N, 34.7818°E)
- Move ~1km north (32.0943°N) — expect ENU north ~1000m, east ~0m
- Round-trip error should be < 0.0001°

## Devin Secrets Needed
- GITHUB_PAT: GitHub Personal Access Token with `repo` scope — needed if creating new repos (the GitHub App integration cannot create repos)
# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 83+ crates. Testing is done via `cargo test`, `cargo clippy`, and adversarial integration tests.

## Build Pipeline
```bash
cargo build --all-targets                    # compile all crates
cargo clippy --all-targets -- -D warnings    # lint with zero warnings
cargo fmt --check                            # formatting check
cargo test                                   # run all tests
```
All four commands must pass with zero errors and zero warnings.

## Test Count Verification
Use this to verify total test counts:
```bash
cargo test 2>&1 | rg "^test result:" | awk '{s+=$4; f+=$6} END {print "Total passed:", s, "Failed:", f}'
```
As of Phase 43-45: **2,234 tests, 0 failures**

## Per-Crate Testing
```bash
cargo test -p aurora-marketplace               # run tests for one crate
cargo test -p aurora-vehicle --test adversarial_phase11  # run specific test file
cargo test -p aurora-developer -- rotate_key   # run tests matching name
```

## API Server Testing
The API server (aurora-api) uses tower::oneshot for endpoint tests. No need to start a live server.
Endpoints: `/health` (200), `/position` (503 when no fix), `/integrity` (200), `/status` (200)

## Adversarial Test Patterns
Adversarial tests are in `crates/<crate>/tests/adversarial_phase*.rs`. They test:
1. Full lifecycle flows
2. Bug regression guards
3. Formula verification
4. State machine correctness

For standalone adversarial tests, compile with:
```bash
rustc --edition 2021 test_file.rs -L target/debug/deps \
  --extern crate_name=$(ls target/debug/libcrate_name*.rlib | head -1) \
  -o test_binary && ./test_binary
```

## API Conventions to Watch
- `RateLimiter::new(capacity: u64, refill_per_second: u64)` — both args are `u64`, not `f64`
- `RateLimiter::try_acquire(&mut self, key_id: &EntityId)` — requires a key ID
- `ApiKeyManager::rotate_key(&mut self, key_id: &EntityId)` — takes only key_id
- `AuroraClient::disconnect(&mut self)` — returns `()`, not `Result`
- `LruCache::new(max_entries: usize)` — takes only max_entries, no max_bytes
- `LruCache::insert(key: String, value: V, byte_size: usize)` — key must be String, not &str
- `CircuitBreaker::new(name: &str, config: CircuitConfig)` — requires both name and config
- `CircuitBreaker::record_success(&mut self)` — takes NO parameters (no timestamp)
- `HealthChecker::new(config: HealthCheckConfig)` — requires config parameter
- `HealthChecker::register_target(&mut self, target_id: &str)` — takes only target_id
- `BatchJob::record_progress(processed: u64, failed: u64)` — second param is failed count, NOT timestamp

## WeightedRoundRobin Algorithm Notes
The algorithm uses a counter-based approach where distribution is inherently skewed:
- High-weight instances get picked much more frequently than low-weight ones
- With weights A=3, B=1: A may get ~99.98% of picks, B ~0.02%
- Don't assert specific distribution percentages — just assert both instances appear and higher weight > lower weight
- The safety guard fix uses a local `iterations` counter per pick() call instead of persistent index

## Common Compilation Issues
- Enum variant names: Check actual source before writing tests
- API parameter order: Always check function signatures in source
- Import paths: Crates export from submodules (e.g., `aurora_marketplace::listing::MarketplaceStore`)
- Structs with `Box<dyn Trait>` cannot auto-derive `Debug`
- Rust 1.94 adds new clippy lints (e.g., `unnecessary_map_or`) — may need fixes for CI

## Bug Fix Verification
When verifying bug fixes, always:
1. Read the fixed code lines to confirm the change is present
2. Run the adversarial test that guards against regression
3. Verify the test would fail with the old (broken) code

## Devin Secrets Needed
No secrets required for testing — all tests run locally via cargo.
# Testing AURORA NAV

## Quick Start
```bash
cd /home/ubuntu/repos/aurora-nav
cargo build --all-targets
cargo clippy --all-targets
cargo test
cargo fmt --check
```

## Test Counts (as of Phase 12)
- Total: 871+ tests, 0 failures, 0 clippy warnings
- aurora-config: 26 tests
- aurora-app: 16 tests
- aurora-integration-tests: 29 tests (4 test files)
- Full suite runs in ~10 seconds

## API Server Testing
The API server can be tested two ways:
1. **Unit tests via tower::oneshot** (preferred): `cargo test -p aurora-integration-tests --test e2e_api`
2. **Live server**: `cargo run --bin aurora-nav -- --port 9876` then curl endpoints

Endpoints:
- `/health` → 200, `{"status": "operational"}`
- `/position` → 503 (no GNSS fix available)
- `/integrity` → 200, `{"level": "NoSolution"}`
- `/status` → 200, valid JSON counters
- `/telemetry` → 200, empty samples
- `/constellation` → 200, `[]`

## CLI Binary Testing
```bash
# Dump default config
cargo run --bin aurora-nav -- --dump-config

# Print version
cargo run --bin aurora-nav -- --version

# Print subsystem status as JSON
cargo run --bin aurora-nav -- --status

# Test validation rejection
cargo run --bin aurora-nav -- --port 0
# Expected: Error: Validation("api.port must be > 0"), exit code 1
```

## Adversarial Testing Patterns

### GNSS Health State Tracking
The `NavigationPipeline` tracks whether GNSS data has ever been received via `has_received_gnss` AtomicBool. To test:
1. Fresh pipeline → `has_received_gnss_data() == false` → health Healthy (sat_count=0 is OK)
2. Call `mark_gnss_received()` → `has_received_gnss_data() == true`
3. Health with sat_count=0 after receiving data → Degraded (not Healthy)

### Config Validation
- All config section structs use `#[serde(default)]` — partial TOML fills missing values from defaults
- `ConfigBuilder::build()` validates; `build_unchecked()` skips validation (testing only)
- CLI overrides are re-validated after application
- Environment variable overrides use `AURORA_*` prefix

### Cross-Crate Integration Tests
Run specific test files:
```bash
cargo test -p aurora-integration-tests --test e2e_config
cargo test -p aurora-integration-tests --test e2e_api
cargo test -p aurora-integration-tests --test e2e_cross_crate
cargo test -p aurora-integration-tests --test e2e_pipeline
```

## Common Issues
- If `cargo test` fails with serde errors on partial TOML, check that `#[serde(default)]` is on the struct
- If health check reports Degraded on a fresh pipeline, check `has_received_gnss` flag logic
- The binary might not start a persistent server in early phases — `--status` and `--dump-config` are the safe testing paths
- Some adversarial tests compile against `target/debug/deps` — make sure `cargo build` runs first

## Devin Secrets Needed
No secrets required — this is a pure Rust workspace tested locally.
