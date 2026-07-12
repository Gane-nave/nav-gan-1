# G.A.N.E NAV — Testing & Build

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
| gane-core | 6 |
| gane-events | 3 |
| gane-gnss | 9 |
| gane-corrections | 3 |
| gane-sensors | 5 |
| gane-fusion | 7 |
| gane-integrity | 5 |
| gane-continuity | 3 |
| gane-telemetry | 2 |
| gane-api | 4 |
| gane-map | 12 |
| gane-routing | 11 |
| gane-lane | 7 |
| gane-offline | 9 |
| gane-risk | 9 |
| gane-confidence | 6 |
| gane-probabilistic | 4 |
| gane-evidence | 10 |
| gane-trust | 11 |
| gane-anti-manipulation | 10 |
| gane-traffic | 36 |
| gane-stability | 34 |
| gane-ux | 32 |
| gane-micro-nav | 18 |
| gane-parking | 14 |
| gane-charging | 21 |

## API Server

- `gane-api` is a **library crate** (no binary target)
- Use `cargo test --package gane-api` to test endpoints
- Tests cover: `/health`, `/position`, `/integrity`, `/status`, `/telemetry`, `/constellation`
- `/position` returns 503 when no GNSS fix (correct behavior)

## Running Per-Crate Tests

```bash
cargo test --package gane-ux          # Run tests for a specific crate
cargo test --package gane-ux -- --nocapture  # With stdout output
cargo test --package gane-ux adversarial     # Filter by test name
```

## Rust Version

- Rust 1.83.0 (stable)
- Workspace root: `/home/ubuntu/repos/gane-nav`
- GitHub repo: `amjad2161/Trade`
