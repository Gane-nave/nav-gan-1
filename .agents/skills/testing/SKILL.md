# AURORA NAV — Testing & Build

## Build & Lint Pipeline

```bash
cargo build                    # Build all 26 crates
cargo clippy -- -D warnings    # Strict lint — zero warnings required
cargo fmt --check              # Formatting check
cargo test                     # Run all tests (270+ tests)
```

## Per-Crate Test Counts (Phase 1-6)

| Crate | Tests |
|---|---|
| aurora-core | 6 |
| aurora-events | 3 |
| aurora-gnss | 9 |
| aurora-corrections | 3 |
| aurora-sensors | 5 |
| aurora-fusion | 7 |
| aurora-integrity | 5 |
| aurora-continuity | 3 |
| aurora-telemetry | 2 |
| aurora-api | 4 |
| aurora-map | 12 |
| aurora-routing | 11 |
| aurora-lane | 7 |
| aurora-offline | 9 |
| aurora-risk | 9 |
| aurora-confidence | 6 |
| aurora-probabilistic | 4 |
| aurora-evidence | 10 |
| aurora-trust | 11 |
| aurora-anti-manipulation | 10 |
| aurora-traffic | 36 |
| aurora-stability | 34 |
| aurora-ux | 32 |
| aurora-micro-nav | 18 |
| aurora-parking | 14 |
| aurora-charging | 21 |

## API Server

- `aurora-api` is a **library crate** (no binary target)
- Use `cargo test --package aurora-api` to test endpoints
- Tests cover: `/health`, `/position`, `/integrity`, `/status`, `/telemetry`, `/constellation`
- `/position` returns 503 when no GNSS fix (correct behavior)

## Running Per-Crate Tests

```bash
cargo test --package aurora-ux          # Run tests for a specific crate
cargo test --package aurora-ux -- --nocapture  # With stdout output
cargo test --package aurora-ux adversarial     # Filter by test name
```

## Rust Version

- Rust 1.83.0 (stable)
- Workspace root: `/home/ubuntu/repos/aurora-nav`
- GitHub repo: `amjad2161/Trade`
