# Testing AURORA NAV Rust Workspace

## Overview
AURORA NAV is a large Rust workspace (~77 crates) for a global navigation system. All crates are pure in-memory engines with no I/O, persistence, or network calls (except aurora-api which runs an HTTP server).

## Build & Test Commands

```bash
# Full test suite (~2000 tests)
cargo test

# Test specific crates
cargo test -p aurora-simulation -p aurora-diagnostics

# Clippy (treat warnings as errors, matching CI)
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt --check

# Auto-fix formatting
cargo fmt

# Build benchmarks (compile-only)
cargo bench --no-run

# Build docs
cargo doc --no-deps
```

## CI Pipeline
- 13 checks total (6 per toolchain matrix + Devin Review)
- Checks: Build & Check, Clippy, Test, Format, Documentation, Benchmarks Compile
- CI uses Rust stable (currently 1.94.0) which may introduce new clippy lints
- Common CI-only failures: new clippy lints not present on local older toolchain

## Adversarial Testing Patterns
When testing new crates, focus on:
1. **State machine transitions**: Test illegal transitions (e.g., play without load, double play)
2. **Boundary clamping**: Test extreme values (seek to 999999, speed to 0 or 1000)
3. **Division-by-zero guards**: Test empty collections (pass_rate with no results, average with no items)
4. **PRNG edge cases**: xorshift64 gets stuck at zero — always guard `if state == 0 { state = 1; }`
5. **Threshold boundaries**: Test values at exact boundaries (5%, 15%, 30%, 50% deviation)
6. **Dependency graphs**: Test required vs optional deps with active/inactive components
7. **Max attempt limits**: Test that limits are enforced and reset works

## Common Issues
- **Rust 1.94 clippy lints**: `unnecessary_map_or`, `manual_is_multiple_of`, `new_without_default` — these may appear on CI but not locally if local toolchain is older
- **cargo fmt differences**: CI may use a different rustfmt version — always run `cargo fmt` before committing
- **Too many arguments**: clippy flags functions with >7 args — use builder pattern instead
- **or_insert_with(Type::new)**: Use `.or_default()` instead

## Workspace Structure
- Root: `/home/ubuntu/repos/aurora-nav`
- Crates: `crates/aurora-*`
- CI: `.github/workflows/ci.yml`
- Main branch: `devin/1773587117-aurora-nav-phase0`
- GitHub repo: `amjad2161/Trade`

## Devin Secrets Needed
- None required for testing (all crates are pure Rust with no external dependencies)
