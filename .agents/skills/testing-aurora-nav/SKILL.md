# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 36+ crates. Testing is done via `cargo test`, `cargo clippy`, and adversarial integration tests.

## Build Pipeline
```bash
cargo build --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
cargo test --workspace
```
All four commands must pass with zero errors and zero warnings.

## Test Count Verification
Use this to verify per-crate test counts:
```bash
cargo test --workspace 2>&1 | rg "^test result:" | awk '{sum += $4} END {print "Total tests:", sum}'
cargo test -p <crate-name> 2>&1 | rg "^test result:"
```

## API Conventions to Watch
- `RateLimiter::new(capacity: u64, refill_per_second: u64)` — both args are `u64`, not `f64`
- `RateLimiter::try_acquire(&mut self, key_id: &EntityId)` — requires a key ID, returns `Result<RateLimitResult, RateLimitResult>`, not `bool`
- `ApiKeyManager::rotate_key(&mut self, key_id: &EntityId)` — takes only key_id, no second arg
- `AuroraClient::disconnect(&mut self)` — returns `()`, not `Result`
- `AuroraClient::connect(&mut self)` — returns `Result<(), ClientError>`

## Webhook Dual-Counter Design
The `Webhook` struct has two failure counters:
- `failure_count` — consecutive failures, reset to 0 on success (used for auto-disable)
- `total_failure_count` — lifetime total, never reset (used by `health()` for accurate stats)

When testing `health()`, verify that `success_rate` uses `total_failure_count`, not `failure_count`.

## Adversarial Test Patterns
For bug fix verification, write integration tests in `crates/<crate>/tests/adversarial_phase<N>.rs` that:
1. Set up the exact precondition that triggers the bug
2. Assert the correct behavior (not just "doesn't crash")
3. Include comments explaining what the old broken behavior was

## API Server Testing
The API server uses Axum with `tower::ServiceExt::oneshot` for testing:
```bash
cargo test -p aurora-api -- --nocapture
```
Endpoints: `/health` (200), `/position` (503 — no GNSS fix), `/integrity` (200), `/status` (200)

## Common Issues
- Structs containing `Box<dyn Trait>` cannot auto-derive `Debug` — need manual `impl fmt::Debug`
- Unused imports flagged by `clippy -D warnings` — check after adding new modules
- `cargo fmt` may reformat newly created files — always run before committing

## Devin Secrets Needed
No secrets required for testing — all tests run locally via cargo.
# Testing AURORA NAV

Rust workspace with 39 crates. All testing is done via shell commands.

## Build Pipeline (run in order)

```bash
cargo build                                    # compile all crates
cargo clippy --all-targets -- -D warnings      # lint with zero warnings
cargo fmt --check                              # formatting check
cargo test                                     # run all tests
```

## Expected Test Counts (as of Phase 11)

Total: 808 tests, 0 failures

Key per-crate counts:
- aurora-marketplace: 37 unit + 1 adversarial = 38
- aurora-payments: 41 unit + 1 adversarial = 42
- aurora-vehicle: 40 unit + 6 adversarial = 46
- aurora-sdk: 41 unit + 3 adversarial = 44
- aurora-developer: 53 unit + 3 adversarial = 56
- aurora-city: 52
- aurora-twin: 46
- aurora-fleet: 42
- aurora-emergency: 41
- aurora-resilience: 45
- aurora-edge: 43
- aurora-satellite: 44
- aurora-api: 4 (tower::oneshot endpoint tests)

## Per-Crate Testing

```bash
cargo test -p aurora-marketplace               # run tests for one crate
cargo test -p aurora-vehicle --test adversarial_phase11  # run specific test file
cargo test -p aurora-developer -- rotate_key   # run tests matching name
```

## API Server Testing

The API server (aurora-api) uses tower::oneshot for endpoint tests. No need to start a live server — tests run in-process.

Endpoints: `/health` (200), `/position` (503 when no fix), `/integrity` (200), `/status` (200)

## Adversarial Test Patterns

Adversarial tests are in `crates/<crate>/tests/adversarial_phase*.rs`. They test:
1. Full lifecycle flows (publish→approve→search→version for marketplace)
2. Bug regression guards (map_axes axis=0, rotate_key expired)
3. Formula verification (OBD-II decode with known byte inputs)
4. State machine correctness (client connect() → Failed on plugin error)

## Common Compilation Issues

- Enum variant names: Check actual source for variant names (e.g., `Category::Traffic` not `Category::TrafficAndRouting`, `ListingStatus::Published` not `ListingStatus::Approved`)
- API parameter order: Always check function signatures in source before writing test calls (e.g., `add_version` takes version, changelog, stability, min_sdk, size_bytes, checksum)
- Import paths: Phase 11 crates export from submodules (e.g., `aurora_marketplace::listing::MarketplaceStore`)

## Bug Fix Verification

When verifying bug fixes, always:
1. Read the fixed code lines to confirm the change is present
2. Run the adversarial test that guards against regression
3. Verify the test would fail with the old (broken) code

## Devin Secrets Needed

No secrets required for testing — all tests run locally via cargo.
