# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 80+ crates. All testing is done via shell commands.

## Build Pipeline (run in order)

```bash
cargo build                                    # compile all crates
cargo clippy --all-targets -- -D warnings      # lint with zero warnings
cargo fmt --check                              # formatting check
cargo test                                     # run all tests
```

## Test Count Verification
```bash
cargo test 2>&1 | rg "^test result:" | awk '{sum += $4} END {print "Total tests:", sum}'
cargo test -p <crate-name> 2>&1 | rg "^test result:"
```

## Per-Crate Testing

```bash
cargo test -p aurora-marketplace               # run tests for one crate
cargo test -p aurora-vehicle --test adversarial_phase11  # run specific test file
cargo test -p aurora-developer -- rotate_key   # run tests matching name
```

## API Server Testing

The API server (aurora-api) uses tower::oneshot for endpoint tests. No need to start a live server — tests run in-process.

Endpoints: `/health` (200), `/position` (503 when no fix), `/integrity` (200), `/status` (200)

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

Adversarial tests are in `crates/<crate>/tests/adversarial_phase*.rs`. They test:
1. Full lifecycle flows (publish→approve→search→version for marketplace)
2. Bug regression guards (map_axes axis=0, rotate_key expired)
3. Formula verification (OBD-II decode with known byte inputs)
4. State machine correctness (client connect() → Failed on plugin error)
- **Idempotency tests**: Call the same operation twice, verify counters don't double-count
- **Threshold classification tests**: Test values at boundaries (above target, between target and threshold, below threshold)
- **Priority/scoring tests**: Verify that higher-priority items rank above lower-priority ones regardless of distance
- **Graph algorithm tests**: Test diamond DAGs (A→B, A→C, B→D, C→D) to verify no false positive cycles; test self-loops (A→A) and disconnected components with cycles
- **State machine lifecycle tests**: Verify multi-step transitions (Active → Deactivating → Destroying adds exactly 2 history entries)
- **Priority queue FIFO tests**: Enqueue items with same priority at different times, verify FIFO ordering within same priority level
- **Scheduled trigger tests**: Verify Once triggers auto-deactivate after firing, Interval triggers respect cooldown, zero-interval never fires

## Bug Fix Verification

When verifying bug fixes, always:
1. Read the fixed code lines to confirm the change is present
2. Run the adversarial test that guards against regression
3. Verify the test would fail with the old (broken) code

## Common Issues
- Structs containing `Box<dyn Trait>` cannot auto-derive `Debug` — need manual `impl fmt::Debug`
- Unused imports flagged by `clippy -D warnings` — check after adding new modules
- `cargo fmt` may reformat newly created files — always run before committing
- Enum variant names: Check actual source for variant names (e.g., `Category::Traffic` not `Category::TrafficAndRouting`)
- API parameter order: Always check function signatures in source before writing test calls
- `aurora-api` has no binary target — don't try `cargo run -p aurora-api`
- Circular dependency detection: Simple visited-set DFS gives false positives on diamond DAGs — use ancestor-tracking DFS (3-color: white/gray/black) instead
- `let _ = expr` silently discards errors — always check if the result should be propagated

## Devin Secrets Needed

No secrets required for testing — all tests run locally via cargo.
