# Contributing to G.A.N.E

## Before opening a PR

Every new module must include:
1. TypeScript source with exported class in `src/`.
2. At least one acceptance test in its layer's `run*Tests()`.
3. Type declaration entry in `src/gane-api.d.ts`.
4. If it adds data shapes, entry in `src/schemas.json`.
5. If it adds UI strings, entries in `www/gane-i18n.js` for all 3 languages.

## Running tests

```bash
npm test                  # Node runner
npm run test:browser      # Visual dashboard
```

All 34 baseline tests must continue to pass.

## Layer discipline

The 6-layer hierarchy is strict:
- Higher layers may import from lower.
- Lower layers must NEVER import from higher.
- If you need lower-layer awareness of a higher-layer concept, use a callback/listener pattern.

Review `docs/ARCHITECTURE.md` before adding cross-layer dependencies.

## Code style

- TypeScript with `strict: false` (we chose this for integration ease).
- No runtime dependencies. Keep bundle small.
- Comment the "why", not the "what".
- No dead code. No commented-out code.

## Commit messages

`<layer>(<module>): <change>` — e.g. `core(PvtSolver): add GLONASS frequency handling`.
