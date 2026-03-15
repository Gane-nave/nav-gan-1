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
