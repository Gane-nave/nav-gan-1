# Testing AURORA NAV

## Overview
AURORA NAV is a Rust workspace with 14 crates. Testing involves a 4-step build pipeline and optional API server verification.

## Build Pipeline (Required)
Run all 4 checks in order from the repo root:
```bash
cargo build --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
cargo fmt --all -- --check
```

### Expected Results
- Build: zero warnings, exit code 0
- Clippy: zero warnings (treated as errors via `-D warnings`)
- Test: 65+ tests across 14 crates, all passing
- Fmt: no diffs

### Test Count by Crate
| Crate | Tests |
|---|---|
| aurora-api | 4 |
| aurora-continuity | 12 |
| aurora-events | 3 |
| aurora-fusion | 6 |
| aurora-gnss | 5 |
| aurora-integrity | 8 |
| aurora-lane | 7 |
| aurora-map | 12 |
| aurora-offline | 9 |
| aurora-routing | 11 |
| aurora-sensors | 2 |
| aurora-telemetry | 3 |

## API Server Testing
The `aurora-api` crate is a library — it does NOT have a binary target by default. To test the live server:

1. Create a temporary `main.rs` in `crates/aurora-api/src/main.rs`:
```rust
use std::net::SocketAddr;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 9877));
    aurora_api::run_server(addr).await
}
```
2. Run: `cargo run -p aurora-api`
3. Test endpoints:
   - `GET /health` → 200, `{"status":"operational","version":"0.1.0",...}`
   - `GET /position` → 503 (no GNSS fix)
   - `GET /integrity` → 200, `{"level":"NoSolution","continuity_mode":"E: Emergency Bounded",...}`
   - `GET /status` → 200, JSON with counters
   - `GET /telemetry` → 200, `{"samples":[],...}`
   - `GET /constellation` → 200, `[]`
4. **Delete the temporary main.rs after testing** — do not commit it.

## Per-Crate Detailed Testing
For detailed output on specific crates:
```bash
cargo test -p aurora-map -- --nocapture
cargo test -p aurora-routing -- --nocapture
cargo test -p aurora-lane -- --nocapture
cargo test -p aurora-offline -- --nocapture
```

## Common Issues
- If lane detector tests fail with `assertion failed: result.is_some()`, check that test positions are within ~1-2m of lane geometry points (0.00001° lat ≈ 1.1m). Positions farther than `lane_width * 2` (default 7m) will return confidence below threshold.
- If clippy fails on `unused variable` or `manual assign`, these are style issues — fix with underscore prefix or `+=` operator.
- The workspace uses Rust 1.83.0+. Ensure `rustup` has the correct toolchain.

## CI
GitHub Actions workflow at `.github/workflows/ci.yml` runs: check, clippy, test, fmt. Note: the repo may not have Actions enabled — verify at GitHub Settings > Actions.

## Devin Secrets Needed
None — this is a pure Rust project with no external service dependencies for testing.
