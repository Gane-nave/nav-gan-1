# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 207+ crates. All crates are pure Rust libraries with no external runtime dependencies.

## Running Tests

### Full workspace test suite
```bash
cargo test --workspace
```
Expected: 4,600+ tests, 0 failures.

### Individual crate tests
```bash
cargo test -p aurora-v2x
cargo test -p aurora-indoor
cargo test -p aurora-ar-nav
cargo test -p aurora-web
```

### Lint checks (matching CI)
```bash
RUSTFLAGS="-D warnings" cargo build --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace
```

## Writing Adversarial Tests

For standalone adversarial test binaries that test across crate boundaries:

```bash
# First build the workspace
cargo build --workspace

# Compile standalone test file
rustc --edition 2021 /tmp/test.rs \
  -L target/debug/deps \
  --extern aurora_v2x=$(ls target/debug/libaurora_v2x*.rlib | head -1) \
  --extern aurora_indoor=$(ls target/debug/libaurora_indoor*.rlib | head -1) \
  --extern aurora_ar_nav=$(ls target/debug/libaurora_ar_nav*.rlib | head -1) \
  -o /tmp/test_bin

# Run
/tmp/test_bin
```

## Key Edge Cases to Test

### V2X (aurora-v2x)
- Haversine distance at same point (should be 0)
- TTC with zero speed (closing_speed clamps to 0.01)
- GLOSA with short time_to_change (<0.5s green, <1.0s red → None)
- Yellow/FlashingRed/Unknown signals → GLOSA returns None
- Prune on empty vehicle list (no panic)

### Indoor (aurora-indoor)
- Co-located beacons (weighted average still works)
- RSSI equal to tx_power (distance = 1.0m exactly)
- Magnetic fingerprint exact match (confidence = 1.0)
- Floor transition boundaries: <5 Pa = None, 5-15 Pa = Escalator, >15 Pa = Stairs, >40 Pa = Elevator
- Negative pressure deltas (abs() is used)
- Unregistered beacon IDs are silently filtered

### AR Nav (aurora-ar-nav)
- Projection at z<=0.1 returns None (behind camera)
- Projection at z=0.11 works (just past boundary)
- Fade at max_distance clamps opacity to 0.1
- Sort by depth with equal z values (no panic)
- Empty active engine renders empty vec

## CI Notes
- CI workflow is at `.github/workflows/ci.yml`
- CI uses `dtolnay/rust-toolchain@stable` which might differ from local Rust version
- CI logs have historically returned 404 (infrastructure/proxy issue)
- All CI checks are not marked as required — PR is mergeable even if they fail
- Always verify locally with the same CI flags before pushing

## Devin Secrets Needed
None — this is a pure Rust workspace with no external service dependencies.
