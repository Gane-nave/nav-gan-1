# Testing G.A.N.E NAV

## Overview
G.A.N.E NAV is a Rust workspace with many crates (see workspace members in `Cargo.toml`). Testing is done entirely via `cargo` commands — there is no UI, browser, or external service to test against.

## Devin Secrets Needed
None.

## CI and Local Verification
This repository has GitHub Actions CI at `.github/workflows/ci.yml`. To minimize “works locally but fails CI”, run the same checks locally before pushing.

## Build / Test Pipeline (mirrors CI)
Run these commands from the repo root:

```bash
cargo build --all-targets
cargo check --workspace
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
cargo test --workspace
cargo bench --no-run
cargo doc --no-deps --workspace
```

## Test Count Verification
CI enforces a minimum total test count (currently 870+) for `cargo test --workspace` (`.github/workflows/ci.yml:44-51`). To compute the total locally:

```bash
cargo test --workspace 2>&1 | grep "^test result:" | awk '{sum += $4} END {print "Total tests:", sum}'
```

## Per-Crate / Targeted Testing

```bash
cargo test -p <crate-name>                                # run tests for one crate
cargo test -p <crate-name> --test <test-file-stem>         # run one integration test file
cargo test -p <crate-name> <test_name_substring>           # run tests matching a name
```

To see a crate’s `test result:` lines (unit tests + each integration test binary):

```bash
cargo test -p <crate-name> 2>&1 | grep "^test result:"
```

Note: integration tests (in `tests/`) show up as separate `test result:` lines. Sum all relevant lines if you need totals.

## Adversarial Test Pattern
For each bug fix, add a regression integration test under `crates/<crate>/tests/` (many crates use the `adversarial_phase*.rs` naming pattern).

Guidelines:
1. Set up the exact pre-fix scenario
2. Assert the correct behavior and document what the old broken behavior did
3. Ensure the test would fail if the fix were reverted

Run an adversarial test file:

```bash
cargo test -p <crate> --test adversarial_phase11 -- --nocapture
```

Optional (rare): for a standalone scratch test, you can compile with `rustc`, but prefer `cargo test` so features/deps match CI.

## API Server Testing
The API server (`gane-api`) uses Axum and is tested in-process via `tower::oneshot` (no live HTTP server needed).

```bash
cargo test -p gane-api -- --nocapture
```

Key endpoints:
- `/health` → 200, JSON with fields `status`, `version`, `uptime_s`, `event_count`, `telemetry_buffer_size` (`crates/gane-api/src/routes.rs:17-24`). Example shape:
  `{"status":"operational","version":"0.1.0","uptime_s":0.0,"event_count":0,"telemetry_buffer_size":0}`
- `/position` → 503 when no GNSS fix is available
- `/integrity` → 200 with an integrity `level` string
- `/status` → 200 with system counters

## Common Pitfalls

### Struct Field Mismatches
Always check the actual struct definition before writing tests.

Example: `TrafficSignalState` has `position: GeoPosition` (not separate `latitude_deg` / `longitude_deg` / `altitude_m`) and `time_to_change_s: Option<f64>` (`crates/gane-core/src/infrastructure.rs:69-78`).

### Method Signatures
Verify method signatures in source before writing tests (don’t guess constructor params or argument order).

### Devin Review Comments
After pushing to a PR, check Devin Review comments. These often catch real correctness issues; fix clear bugs immediately, not just stylistic suggestions.

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
- Don’t assert specific distribution percentages — just assert both instances appear and higher weight > lower weight
- The safety guard fix uses a local `iterations` counter per pick() call instead of persistent index

## Common Compilation Issues
- Enum variant names: check actual source before writing tests
- API parameter order: always check function signatures in source
- Import paths: crates export from submodules (e.g., `gane_marketplace::listing::MarketplaceStore`)
- Structs with `Box<dyn Trait>` cannot auto-derive `Debug`
- New Rust/clippy versions can add new lints — fix CI failures rather than silencing lints

## Bug Fix Verification Checklist
When verifying bug fixes, always:
1. Read the fixed code lines to confirm the change is present
2. Run the adversarial/regression test that guards against regression
3. Verify the test would fail with the old (broken) behavior (e.g., by temporarily reverting locally)
