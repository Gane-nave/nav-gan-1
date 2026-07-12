# G.A.N.E Navigator

**Global Autonomous Navigation Engine** — aviation-grade navigation that runs offline, corrects itself, and governs itself.

[![Tests](https://img.shields.io/badge/tests-34%2F34-brightgreen)](./www/test-report.html)
[![Bundle Size](https://img.shields.io/badge/bundle-130KB-blue)]()
[![Modules](https://img.shields.io/badge/modules-73-purple)]()
[![Layers](https://img.shields.io/badge/layers-6-orange)]()
[![License](https://img.shields.io/badge/license-MIT-green)](./LICENSE)

## What makes G.A.N.E different

Every other navigation app trusts GNSS. G.A.N.E treats it as one signal among many, validates it continuously against reality, and refuses to act when it doesn't trust itself.

| Capability | Waze | Google Maps | Apple Maps | HERE | **G.A.N.E** |
|-----------|:----:|:-----------:|:----------:|:----:|:-----------:|
| Raw multi-GNSS (GPS+GLONASS+Galileo+BeiDou) | ✗ | ✗ | ✗ | ✗ | ✓ |
| RAIM integrity monitoring | ✗ | ✗ | ✗ | ✗ | ✓ |
| Spoof/jam detection | ✗ | ✗ | ✗ | ✗ | ✓ |
| 100% offline navigation | ✗ | partial | ✗ | partial | ✓ |
| Explainable AI routing | ✗ | ✗ | ✗ | ✗ | ✓ |
| Differential privacy | ✗ | ✗ | partial | ✗ | ✓ |
| Replay engine (forensic) | ✗ | ✗ | ✗ | ✗ | ✓ |
| Self-correcting calibration | ✗ | ✗ | ✗ | ✗ | ✓ |
| Self-governing (fail-safe authority) | ✗ | ✗ | ✗ | ✗ | ✓ |

## Quick Start

```bash
# Clone
git clone https://github.com/USER/gane-navigator.git
cd gane-navigator

# Run locally (no build needed)
npm run serve
# → Open http://localhost:8000

# Verify all tests pass
open www/test-report.html  # should show 34/34
```

Zero build step. Zero dependencies at runtime. Drop on any static host.

## Architecture

G.A.N.E is organized into six strictly-ordered layers. Higher layers govern lower ones; lower layers never depend on higher ones.

```
┌──────────────────────────────────────────────────────────┐
│ 6. CONSCIOUSNESS    10 modules   governs the system      │
│ 5. REALITY          11 modules   validates against truth │
│ 4. COMPLETION       12 modules   corrections & safety    │
│ 3. RESILIENCE       13 modules   never-fail fallbacks    │
│ 2. TOP-1             6 modules   AI/ML/AR/privacy        │
│ 1. CORE             21 modules   positioning & fusion    │
└──────────────────────────────────────────────────────────┘
             73 modules · 130KB bundle · 34 tests
```

Read the full [ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## Capabilities

**Core positioning** — Weighted least-squares PVT solver, 15-state EKF, 5-state finite state machine (FULL → DEGRADED → DR_ONLY → LOST → RECOVERY), integrity engine with χ² RAIM.

**Never fails** — 5-tier routing fallback (Valhalla → OSRM-EU → OSRM-demo → GraphHopper → straight-line). 5-provider tile rotation. Redundant geocoding (Nominatim → Photon → Pelias). Dead reckoning from IMU when GPS is dead.

**Self-corrects** — Every prediction is measured against actual outcome. ETA calibration learns dynamically. Contradiction detector flags when layers disagree. Truth override forces degraded mode when evidence demands.

**Self-governs** — Consciousness orchestrator runs every 2s. Synthesizes global system phase. Decision governor gates every critical action. Self-distrust mechanism tracks confidence; suppresses aggressive actions when uncertain. Fail-safe authority can override the entire stack.

**Honest** — Exposes its own limits to the user ("only 1 satellite, need 4"). Every critical action logged in IndexedDB. Every prediction auditable. No hidden state.

## Deployment Paths

### 1. Static Web (2 minutes)

```bash
npm run serve              # local
npm run deploy:netlify     # production
npm run deploy:cloudflare  # production
```

### 2. PWA (installable)

The app is already a valid PWA with `manifest.json` and service worker. Users click "Install" in their browser.

### 3. Android APK (raw multi-GNSS)

```bash
npm install @capacitor/android
npm run android:sync
npm run android:build
npm run android:install
```

The APK uses `GnssPlugin.java` to expose raw pseudoranges, carrier phases, and CN0 measurements from GPS/GLONASS/Galileo/BeiDou/QZSS/NavIC directly to the PvtSolver.

## Integration / SDK

G.A.N.E exposes a public API via `window.GANE`. Full TypeScript declarations in [`src/gane-api.d.ts`](./src/gane-api.d.ts).

```html
<script src="gane-core-bundle.js"></script>
<script>
  const engine = new GANE.Core.NavigationEngine();
  const consciousness = new GANE.Consciousness.ConsciousnessOrchestrator();

  engine.tick(rawMeasurements, imuSample, currentLLA);
  consciousness.tick({...});

  // Gate every critical action
  const check = consciousness.proposeAction('reroute', 'HIGH', payload, 'user');
  if (check.approved) doReroute();
</script>
```

Schemas for interchange are in [`src/schemas.json`](./src/schemas.json) (JSON Schema draft-07).

## Testing

```bash
npm test                   # Node-based test runner
npm run test:browser       # Open visual test dashboard
```

Every layer exports a `run*Tests()` function. The browser dashboard (`www/test-report.html`) runs all 34 tests and produces a visual report.

## Internationalization

Three languages ship with the bundle: English, Hebrew (RTL), Arabic (RTL).

```js
const { t, dir } = GANE_I18N.get('he');
document.documentElement.dir = dir;
button.textContent = t('emergency.sosButton'); // "חירום"
```

## Honest Limitations

- Not yet certified for safety-critical use (see LICENSE disclaimer).
- APK requires you to build it (Android Studio, ~60 min).
- Real multi-GNSS requires Android device with raw measurements (Pixel 4+, Xiaomi Mi 8+, Samsung S20+).
- AI Copilot uses template NLG by default; can route to Claude via `LlmBridge`.
- AR navigation is a scaffold; production needs THREE.js integration.

## Contributing

Every new module must include:
1. TypeScript source with exported class.
2. Acceptance test in its layer's `run*Tests()`.
3. Type declaration entry in `gane-api.d.ts`.
4. If it adds data shapes, entry in `schemas.json`.
5. If it adds UI strings, entries in `gane-i18n.js` for all 3 languages.

See [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) for design principles.

## License

MIT — see [LICENSE](./LICENSE).

Contains safety disclaimer: **not certified for aviation, autonomous vehicles, or safety-critical applications** without independent validation.

## Credits

Built by Amjad. Powered by open map data from OpenStreetMap contributors.
