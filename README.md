# G.A.N.E NAV — Global Autonomous Navigation Ecosystem

**Mission-critical, trust-aware navigation platform.** GNSS is treated as one signal among many: the system validates continuously (RAIM, spoof/jam detection), fuses multiple sensors, degrades gracefully, and never fails silently. Hebrew-first RTL product UX for professional drivers, fleets, and emergency services.

> Canonical engineering documents: [`SYSTEM-REVIEW.md`](./SYSTEM-REVIEW.md) · [`GANE-NAV-MASTER-BLUEPRINT.md`](./GANE-NAV-MASTER-BLUEPRINT.md) · [`docs/PHASE1-TECHNICAL-DESIGN.md`](./docs/PHASE1-TECHNICAL-DESIGN.md)

## Monorepo layout

```
crates/        Rust navigation engine workspace — 264 crates, all real or connected
               ├─ core engine: aurora-core, -gnss (WLS PVT), -fusion (canonical 15-state
               │  ESKF), -integrity (RAIM/spoof/jam), -routing (Dijkstra + vehicle-envelope
               │  constraints), -map, -traffic, -v2x, -indoor, -ar-nav,
               │  -api (Axum, 16 endpoints), -web (embedded dashboard), -app (binary)
               ├─ gane-wasm: the same engine compiled to WebAssembly (browser/Capacitor)
               ├─ gane-osm-import: Overpass JSON -> restriction-aware road graphs + geo CLI
               └─ gane-replay-verify: golden-trace harness (native/WASM bit-identity)
app/           TypeScript product application (React 19 PWA + Express/tRPC + MySQL/Drizzle)
               58 panels, 49 client engines, 22 executable contract modules, 33-table schema,
               40+ languages with full RTL, 11-type vehicle profile system
mobile/        Android delivery channel (Capacitor + raw-GNSS plugin + TWA packaging,
               app id com.gane.nav) with a zero-dependency verified missions backend
docs/          Technical designs and engineering documentation
```

## Honest metrics (verified by execution, 2026-07)

- Engine: **264 crates, every one real or dependency-connected** — the ~2,190
  generated placeholder crates were removed (they live in git history only).
- **5,066 engine tests — all green** (incl. 88 adversarial test files and
  cross-crate e2e suites); full workspace compile in ~20 s.
- The engine **builds, runs, and serves**: `cargo run -p aurora-app` → REST API on :3000
  (`/health`, `/position`, `/integrity`, `/metrics`, OpenAPI + Swagger) + live dashboard.
- The navigation core **compiles to WASM** (`gane-wasm`, ~630 KB release artifact) — one
  engine for server, browser, and Android.
- Vehicle-envelope routing enforced as hard constraints: an illegal route for the given
  vehicle (height/weight/hazmat/road class) is never returned.
- `app/` runs standalone in degraded demo mode; full capability requires the external
  integrations listed in the Blueprint §11 (DB, OAuth, Stripe, Redis, map keys, S3,
  Sentry, OTLP).
- `mobile/backend`: 15/15 integration tests, JSONL persistence with tombstone audit trail.

## Quick start

```bash
# Engine binary (REST API + dashboard on :3000)
cargo run -p aurora-app

# Engine status / health probe
cargo run -p aurora-app -- --status

# Core engine tests
cargo test -p aurora-core -p aurora-gnss -p aurora-fusion -p aurora-integrity \
           -p aurora-routing -p aurora-map -p gane-wasm

# WASM engine artifact
cargo build --target wasm32-unknown-unknown --release -p gane-wasm

# Product app (degraded demo mode without env config)
cd app && pnpm install && pnpm dev

# Mobile backend verification
cd mobile/backend && node test.js

# Docker (engine)
docker compose up --build
```

## CI

Seven gates on every PR: Build & Check, Clippy (`-D warnings`), full test suite with
count gate, Format, Benchmarks Compile, Documentation (`RUSTDOCFLAGS=-D warnings`),
and **WASM Core Check** (the navigation core must always compile for
`wasm32-unknown-unknown`). All actions are pinned to full commit SHAs per org policy.

## License

MIT
