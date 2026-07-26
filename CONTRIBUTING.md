# Contributing to G.A.N.E NAV

## Repository layout

| Path | What lives here |
|---|---|
| `crates/` | Rust navigation engine — 264 crates (core, GNSS, fusion, integrity, routing, map, API) |
| `crates/gane-wasm/` | The same engine compiled to WebAssembly for the browser and Capacitor |
| `crates/gane-osm-import/` | Overpass JSON → restriction-aware road graph, plus the geo-routing CLI |
| `app/` | TypeScript product application (React PWA client + Express/tRPC server) |
| `mobile/` | Android delivery channel (Capacitor + raw-GNSS plugin) |
| `docs/` | Technical designs |

## Setup

```bash
# Rust engine
rustup target add wasm32-unknown-unknown        # needed for the WASM gate
cargo test --workspace                          # 5,067 tests

# Web app
cd app && corepack enable && pnpm install
pnpm check                                      # tsc --noEmit
pnpm test                                       # vitest
pnpm build                                       # production build
```

## The gates your PR must pass

CI runs nine jobs; all must be green:

| Gate | Command it runs |
|---|---|
| Build & Check | `cargo check --workspace --all-targets` |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` |
| Test | `cargo test --workspace` + a minimum-test-count gate |
| Format | `cargo fmt --all --check` |
| Benchmarks Compile | benches must compile |
| Documentation | `RUSTDOCFLAGS=-D warnings cargo doc` |
| WASM Core Check | the engine must compile for `wasm32-unknown-unknown` |
| App | `pnpm check` + `pnpm test` in `app/` |
| Mobile | mobile backend tests |

**Actions must be pinned to full commit SHAs** — this repository's policy rejects
tag references (`@v4`), including transitively inside composite actions.

## Rebuilding the WASM engine

The browser engine is built from `crates/gane-wasm`. CI rebuilds it on every
deploy, so a stale committed artifact can never ship — but to test locally:

```bash
cargo build --target wasm32-unknown-unknown --release -p gane-wasm
wasm-bindgen --target web --out-dir app/client/public/engine \
  target/wasm32-unknown-unknown/release/gane_wasm.wasm
```

Keep `wasm-bindgen-cli` at the version pinned in `Cargo.lock` (mismatched
versions produce glue that fails at runtime).

## Adding a map region

Regions are data, not code:

```bash
# 1. Import an OSM extract into a routable graph
cargo run --release -p gane-osm-import -- import <region-name> <overpass.json> \
  app/client/public/engine/<region>-graph.json

# 2. Register it
#    app/client/public/engine/regions.json → add { id, name, graph, center, zoom }
```

The map reads the registry at load; no component changes are needed.

## Conventions

- **Commits**: Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`), imperative mood.
- **Comments**: explain constraints the code cannot show. Do not narrate what the
  next line does.
- **i18n**: every user-visible string goes through `t()` in
  `app/client/src/lib/i18n.ts`. Add the key to the `TranslationKey` union and to
  `en`, `he`, and `ar`; the remaining languages inherit English via `...en`.
- **Accessibility**: interactive surfaces need keyboard parity with pointer input,
  an accessible name, and `aria-live` for asynchronous results. The map is the
  reference implementation (`OpenNavigationMap.tsx`).
- **Tests**: new engine behavior needs a Rust test; new UI behavior needs a vitest
  or Playwright spec. The test-count gate will fail a PR that removes coverage.

## Reporting issues

Include: what you expected, what happened, the platform (browser + OS, or
`cargo --version`), and whether the live site or a local build reproduces it.
