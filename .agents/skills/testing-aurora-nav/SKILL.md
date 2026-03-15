# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 30+ crates implementing a global navigation system. Testing is done via cargo commands — there is no UI or frontend.

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
cargo test -p aurora-resilience  # 44 tests
cargo test -p aurora-edge        # 42 tests
cargo test -p aurora-satellite    # 44 tests
cargo test -p aurora-fleet        # 42 tests
# etc.
```

## API Server Testing
The API server (`aurora-api`) has 4 built-in tower::oneshot tests:
- `/health` → 200, `status: "operational"`
- `/position` → 503 (no GNSS fix expected)
- `/integrity` → 200 with `continuity_mode` field
- `/status` → 200 with JSON counters

Run: `cargo test -p aurora-api`

To test the live server: `cargo run -p aurora-api` then curl `http://localhost:9876/health`

## Adversarial Testing Patterns
When Devin Review finds bugs, write adversarial tests targeting the exact trigger sequence. Common bug categories:

### Memory Accounting Bugs (inference.rs)
- Test the load→update→reload cycle to catch double-counting
- Test register→update→load for never-loaded models (loaded_at guard)
- Test unload of never-loaded models (should not subtract memory)
- Key invariant: `loaded_at.is_some()` indicates memory was counted

### Threshold Logic Bugs (sla.rs)
- Test boundary values at exactly the threshold (e.g., value == warning_threshold)
- Verify three-tier classification: Met ≥ target, Warning ≥ warning_threshold, Critical < warning_threshold
- Check both higher-is-better and lower-is-better metric branches

### State Management Bugs (policy.rs)
- Test overlapping constraints: apply two constraints that shed the same tier, clear one, verify features stay shed
- Test constraint idempotency: applying same constraint twice should shed 0 on second call

### Priority Eviction Bugs (store_forward.rs)
- Test strict priority ordering: higher priority evicts lower, same priority is rejected
- Use tight buffer sizes to force eviction (buffer_size < sum of message sizes)

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
