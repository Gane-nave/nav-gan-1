# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 14 crates implementing a global navigation system. Testing is done via cargo commands — there is no UI or frontend.

Workspace crates: `aurora-core`, `aurora-events`, `aurora-gnss`, `aurora-corrections`, `aurora-sensors`, `aurora-fusion`, `aurora-integrity`, `aurora-continuity`, `aurora-telemetry`, `aurora-api`, `aurora-map`, `aurora-routing`, `aurora-lane`, `aurora-offline`.

## Prerequisites
- Rust toolchain (1.83.0+)
- No external services or credentials needed — all tests are in-memory

## Build Pipeline
Run these commands in order from the repo root:
```bash
cargo build --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo fmt --check
```

## Per-Crate Testing
To verify specific crates:
```bash
cargo test -p aurora-core
cargo test -p aurora-gnss
cargo test -p aurora-fusion
cargo test -p aurora-integrity
cargo test -p aurora-continuity
cargo test -p aurora-offline
cargo test -p aurora-api
# etc.
```

## API Server Testing
The API server (`aurora-api`) has 4 built-in `ServiceExt::oneshot` tests (in `crates/aurora-api/src/server.rs`):
- `/health` → 200, `status: "operational"`
- `/position` → 503 (no GNSS fix expected)
- `/integrity` → 200 with `continuity_mode` field
- `/status` → 200 with JSON counters

Run: `cargo test -p aurora-api`

To test the live server: `cargo run -p aurora-api` then curl `http://localhost:3000/health`

## Adversarial Testing Patterns
When Devin Review finds bugs, write adversarial tests targeting the exact trigger sequence. Common bug categories:

### Offline Cache & Sync Bugs (aurora-offline)
- Test region lifecycle: create → mark ready → query coverage → delete
- Test `is_covered` boundary conditions (position exactly on region edge)
- Test `missing_tiles` returns all tiles for a newly created region
- Test sync engine FIFO ordering: operations must be processed in enqueue order
- Test offline-to-online transition: operations queued while offline should flush when `set_online(true)`

### Integrity & Trust Bugs (aurora-integrity)
- Test integrity level transitions under source score changes
- Test trust manager score tracking across multiple sources
- Test fault detection with conflicting source data

### Continuity Mode Bugs (aurora-continuity)
- Test mode transitions: Full GNSS (A) → Degraded (C) → Dead Reckoning (D) → Emergency (E)
- Test recovery logic: verify re-entry conditions when GNSS signal returns
- Test `is_recovery_pending` state machine correctness

### Policy Data Model Bugs (aurora-core/src/policy.rs)
- Test `ModelVersion` display formatting (especially short hash edge case where hash length < 8)
- Test `AlertCondition` with all `ComparisonOperator` variants
- Test `Policy` serialization/deserialization round-trips

## Test Count Verification
After any changes, verify total test count hasn't decreased. Count with:
```bash
cargo test --workspace 2>&1 | grep 'test result:' | grep -v '0 passed'
```

## Common Issues
- Clippy may flag `if_same_then_else`, `manual_clamp`, `comparison_chain` — these are style warnings, not bugs, but `-D warnings` will fail on them
- Type ambiguity errors in floating-point math: add explicit `f64` type annotations
- Unused field warnings: prefix with underscore (e.g., `_field_name`) for intentionally unused fields

## Devin Secrets Needed
None — all tests are self-contained with no external dependencies.
