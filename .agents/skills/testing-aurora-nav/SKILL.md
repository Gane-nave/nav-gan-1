# Testing Aurora NAV Rust Workspace

## Overview
Aurora NAV is a large Rust workspace with 89+ crates. Testing involves building all crates, running unit tests, and passing clippy/fmt checks.

## Build & Test Commands
```bash
# Build all targets
cargo build --all-targets

# Test specific crates (preferred over --all for speed)
cargo test -p aurora-graphdb -p aurora-streaming -p aurora-consensus

# Clippy with warnings as errors
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt --check
```

## Rust Version Compatibility
- **Local**: Rust 1.83 (stable)
- **CI**: Rust 1.94 (latest stable on GitHub Actions)

### Common CI-only Clippy Lints (Rust 1.94)
These lints exist in Rust 1.94 but not 1.83. Use `#[allow(unknown_lints, clippy::LINT_NAME)]` to suppress them on both versions:

- `clippy::manual_is_multiple_of` — `x % n != 0` should be `!x.is_multiple_of(n)` (unstable in 1.83)
- `clippy::manual_clamp` — `x.min(a).max(b)` should be `x.clamp(b, a)`
- `clippy::unnecessary_map_or` — some `.map_or()` patterns

Pattern for cross-version compatibility:
```rust
#[allow(unknown_lints, clippy::manual_is_multiple_of)]
if counter % n != 0 { ... }
```

## CI Configuration
- CI runs on `push` and `pull_request` events
- Jobs: Build & Check, Test, Clippy, Format, Benchmarks Compile, Documentation
- Documentation job may fail (pre-existing issue, not required for merge)
- All other jobs should pass

## Testing Library Crates
Since these are library crates with no UI, testing is done via:
1. `cargo test` — runs all unit tests
2. Adversarial standalone Rust programs compiled with `rustc` for integration testing
3. Individual regression tests targeting specific bug fixes

## Common Issues
- **Borrow checker with `entry()` API**: Extract values before calling `self.field.entry()` to avoid simultaneous mutable/immutable borrows
- **Dead code warnings**: Use `#[allow(dead_code)]` for struct fields used only in certain contexts
- **Format differences**: Rust 1.94 `rustfmt` may reformat `assert_eq!` macros with long string arguments differently than 1.83

## PR Workflow
- Branch naming: `devin/{timestamp}-{descriptive-slug}`
- Base branch: `devin/1773587117-aurora-nav-phase0`
- Create PR via `git_create_pr` tool
- Wait for CI with `git_pr_checks` (wait_until_complete=true)
- Fix Devin Review bugs before reporting completion

## Devin Secrets Needed
- GitHub access is handled via git-manager proxy (no additional secrets needed)
