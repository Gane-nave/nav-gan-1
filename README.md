# AURORA NAV / GMIN

**Global Mobility Intelligence Network** -- A production-grade navigation system built in Rust with 210+ modular crates covering GNSS positioning, sensor fusion, routing, traffic, V2X communication, indoor positioning, AR navigation, and more.

## Architecture Overview

AURORA NAV is organized as a Cargo workspace with modular crates in `crates/`:

```
aurora-nav/
  crates/
    aurora-core/          # Core types, coordinates, errors
    aurora-gnss/          # Multi-constellation GNSS receiver
    aurora-fusion/        # Extended Kalman Filter sensor fusion
    aurora-integrity/     # RAIM, protection levels, jamming/spoofing detection
    aurora-routing/       # A* / Dijkstra route planning
    aurora-map/           # Map data, tiles, matching
    aurora-lane/          # Lane-level guidance
    aurora-traffic/       # Real-time traffic flow
    aurora-v2x/           # Vehicle-to-Everything (DSRC/C-V2X)
    aurora-indoor/        # BLE beacon trilateration, magnetic fingerprinting
    aurora-ar-nav/        # Augmented Reality overlay, lane projection
    aurora-api/           # REST API server (Axum)
    aurora-web/           # Interactive web dashboard (Leaflet.js)
    aurora-app/           # Application orchestration
    aurora-orchestrator/  # Pipeline orchestrator
    ... (200+ more crates)
```

## Key Features

### Positioning and Navigation
- **Multi-GNSS**: GPS, Galileo, GLONASS, BeiDou with quality scoring
- **EKF Fusion**: Extended Kalman Filter combining GNSS, IMU, barometer, odometry
- **Tunnel Mode**: Dead-reckoning with automatic GNSS handoff
- **RTK/PPP**: Centimetre-level corrections with RAIM integrity
- **Indoor Positioning**: BLE beacon trilateration, WiFi RTT, magnetic fingerprinting
- **Dead Reckoning**: Inertial navigation for GNSS-denied environments

### Routing and Traffic
- **Multi-modal Routing**: Car, bicycle, pedestrian, public transit
- **Real-time Traffic**: Congestion heatmaps, incident detection, predictive routing
- **Lane Guidance**: Turn-by-turn with lane-level precision
- **Risk Engine**: Probabilistic route scoring with confidence intervals
- **ETA Prediction**: ML-enhanced arrival time estimation

### V2X Communication
- **DSRC / C-V2X**: Dual-mode vehicle-to-everything communication
- **BSM**: Basic Safety Message exchange with nearby vehicles
- **SPaT**: Signal Phase and Timing from smart intersections
- **GLOSA**: Green-Light Optimal Speed Advisory
- **Collision Detection**: Time-to-collision (TTC) based warnings

### AR Navigation
- **3D Projection**: Pinhole camera model for AR waypoint overlay
- **Lane Projection**: Straight and curved lane guidance visualization
- **Distance Fading**: Opacity and scale based on depth
- **Depth Sorting**: Back-to-front rendering order

### Infrastructure
- **210+ Crates**: Modular, independently testable components
- **4,600+ Tests**: Comprehensive unit, integration, and adversarial tests
- **REST API**: Axum-based server with health checks, CORS, tracing
- **Web Dashboard**: Interactive Leaflet.js map with real-time telemetry
- **Docker Ready**: Multi-stage build with health checks
- **Observability**: Structured logging, distributed tracing, histograms

## Quick Start

### Prerequisites
- Rust 1.70+ (tested on 1.94.0)
- Cargo (included with Rust)

### Build
```bash
cargo build --workspace
```

### Run Tests
```bash
cargo test --workspace
```

### Run API Server
```bash
cargo run -p aurora-api
```
The server starts on `http://localhost:3000` with:
- `GET /` -- Web dashboard
- `GET /api/dashboard` -- JSON telemetry data
- `GET /health` -- Health check

### Lint
```bash
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Crate Categories

| Category | Crates | Description |
|----------|--------|-------------|
| Core | aurora-core, aurora-events, aurora-config | Foundation types, events, configuration |
| GNSS/PNT | aurora-gnss, aurora-multi-gnss, aurora-ekf, aurora-tunnel | Positioning, navigation, timing |
| Sensors | aurora-sensors, aurora-fusion, aurora-dead-reckoning | IMU, barometer, odometry fusion |
| Integrity | aurora-integrity, aurora-continuity, aurora-anti-manipulation | RAIM, jamming/spoofing detection |
| Routing | aurora-routing, aurora-risk, aurora-probabilistic | Route planning, risk scoring |
| Traffic | aurora-traffic, aurora-stability, aurora-crowd-speed | Real-time traffic management |
| V2X | aurora-v2x | Vehicle-to-Everything communication |
| Indoor | aurora-indoor | Indoor positioning systems |
| AR | aurora-ar-nav, aurora-ar | Augmented Reality navigation |
| Maps | aurora-map, aurora-lane, aurora-offline, aurora-tiles | Map data, lane guidance, offline maps |
| API/Web | aurora-api, aurora-web | REST API server, web dashboard |
| Fleet | aurora-fleet, aurora-emergency | Fleet management, emergency routing |
| Smart City | aurora-city, aurora-twin | Smart city integration, digital twins |
| Infrastructure | aurora-cache, aurora-pipeline, aurora-mesh | Data structures, concurrency, networking |
| Observability | aurora-telemetry, aurora-metrics, aurora-tracing-dist | Logging, metrics, distributed tracing |
| Security | aurora-auth, aurora-security, aurora-compliance | Authentication, encryption, compliance |

## License

MIT
