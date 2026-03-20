# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 34 crates. Testing is done entirely via cargo commands — there is no UI, browser, or external service to test against.

## Devin Secrets Needed
None — this is a pure Rust workspace with no external dependencies or credentials.

## Build Pipeline
Run these commands in order from the repo root (`/home/ubuntu/repos/aurora-nav`):

```bash
cargo build --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
cargo test --workspace
```

All four must pass with zero errors and zero warnings. As of Phase 9, expect 588+ tests.

## Per-Crate Test Counts
Verify individual crate test counts with:
```bash
cargo test -p <crate-name> 2>&1 | grep "^test result:"
```

Key crates and minimum expected test counts:
- aurora-city: 51+ (Phase 9)
- aurora-twin: 46+ (Phase 9)
- aurora-fleet: 42+ (Phase 7)
- aurora-emergency: 41+ (Phase 7)
- aurora-resilience: 44+ (Phase 8)
- aurora-edge: 42+ (Phase 8)
- aurora-satellite: 44+ (Phase 8)
- aurora-api: 4+ (Phase 1)

Note: Integration tests (in `tests/` directories) show as separate test result lines. Sum all lines for total.

## Adversarial Test Pattern
For each bug fix, write an integration test in `crates/<crate>/tests/adversarial_<name>.rs` that:
1. Sets up the exact scenario that triggered the bug
2. Asserts the **correct** behavior with a message explaining what the old broken behavior would produce
3. Would FAIL if the fix were reverted

Run individual adversarial tests:
```bash
cargo test -p <crate> --test adversarial_<name> -- --nocapture
```

## API Server Testing
The API server uses Axum and can be tested via `tower::oneshot` (in-process, no HTTP server needed):
```bash
cargo test -p aurora-api -- --nocapture
```

Expected endpoints:
- `/health` → 200, `{"status": "operational", "version": "0.1.0"}`
- `/position` → 503 (no GNSS fix available)
- `/integrity` → 200, `{"level": "NoSolution", ...}`
- `/status` → 200, valid JSON with counters

## Common Pitfalls

### Struct Field Mismatches
When writing integration tests, always check the actual struct definition in `aurora-core/src/infrastructure.rs` (or relevant module). Field names may differ from what you expect. For example:
- `TrafficSignalState` uses `latitude_deg`/`longitude_deg`/`altitude_m` (not `lat`/`lon`/`alt`)
- `time_to_change_s` is `Option<f64>` (not `f64`)
- `updated_at` uses `chrono::DateTime<Utc>` (not `Instant`)

### Method Signatures
Always verify method signatures before writing tests. Common mistakes:
- `SignalController::update_signal` takes 1 argument (`TrafficSignalState`), not 2
- `StorageBudget::new` takes specific parameters — check the constructor

### Devin Review Comments
After pushing to a PR, check for Devin Review comments within a few minutes. These often catch real bugs (e.g., `classify_status` ignoring `critical_threshold`, `load_model` double-counting memory). Fix clear bugs immediately, not just stylistic suggestions.

### No CI Workflow
This repo does not have GitHub Actions CI. All verification must be done locally. Always run the full build pipeline before committing.

## Testing Workflow
1. Run full build pipeline (build/clippy/fmt/test)
2. Verify per-crate test counts for changed crates
3. Run adversarial tests individually for each bug fix
4. Run API server tests
5. Commit and push
6. Check for Devin Review comments on the PR
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
