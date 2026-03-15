# Testing AURORA NAV

## Overview
AURORA NAV is a multi-crate Rust workspace for a global navigation system. Testing involves building all crates, running unit tests, verifying API endpoints, and running adversarial tests against bug fixes.

## Prerequisites
- Rust toolchain (1.83.0+)
- Cargo workspace at repo root

## Build Pipeline
Run these commands in order from the repo root:
```bash
cargo build          # Compile all crates
cargo clippy --all-targets  # Lint check (zero warnings expected)
cargo test           # Run all unit tests
cargo fmt --check    # Format check
```

## Per-Crate Test Counts
To verify test counts per crate:
```bash
cargo test -p <crate-name> 2>&1 | rg "^test result:"
```
To get total count across all crates:
```bash
cargo test 2>&1 | rg "^test result:" | awk '{sum += $4; fail += $6} END {print "Passed:", sum, "Failed:", fail}'
```

## API Server Testing
The `aurora-api` crate is a **library crate** (no `main.rs`). It cannot be run with `cargo run`. Instead:
- Use the built-in endpoint tests: `cargo test -p aurora-api`
- These use `tower::ServiceExt::oneshot` to test endpoints without starting a real server
- Endpoints: `/health`, `/position`, `/integrity`, `/status`, `/telemetry`, `/constellation`

## Adversarial Bug Fix Testing
When testing bug fixes, run the specific regression test with `--nocapture` to see output:
```bash
cargo test -p <crate> <test_name> -- --nocapture
```
Key adversarial patterns:
- **Idempotency tests**: Call the same operation twice, verify counters don't double-count
- **Threshold classification tests**: Test values at boundaries (above target, between target and threshold, below threshold)
- **Priority/scoring tests**: Verify that higher-priority items rank above lower-priority ones regardless of distance

## Common Issues
- `aurora-api` has no binary target — don't try `cargo run -p aurora-api`
- Copy-paste errors in bug fix commits can introduce duplicate lines that shadow variables — always verify the diff after committing fixes
- Clippy warnings like `if_same_then_else` often indicate real logic bugs (duplicate branches)

## Workspace Structure
As of Phase 7, the workspace has 30 crates across 8 phases. New crates are added as workspace members in the root `Cargo.toml`.
