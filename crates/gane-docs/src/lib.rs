//! # G.A.N.E NAV - Global Autonomous Navigation Ecosystem
//!
//! A comprehensive, production-grade navigation system built in Rust.
//!
//! ## Architecture Overview
//!
//! G.A.N.E NAV is organised as a modular Rust workspace with 43+ crates,
//! each responsible for a well-defined domain within the navigation stack.
//! The system follows these core principles:
//!
//! 1. **Single Source of Truth** - one canonical data model ([`gane_core`])
//! 2. **Event-Driven Architecture** - all subsystems communicate via typed events ([`gane_events`])
//! 3. **Offline-First** - full operation without network connectivity ([`gane_offline`], [`gane_resilience`])
//! 4. **Edge-First Processing** - ML inference and data reduction at the edge ([`gane_edge`])
//! 5. **Satellite Fallback** - LEO/GEO mesh communication for extreme scenarios ([`gane_satellite`])
//! 6. **Probabilistic Navigation** - confidence-weighted routing and risk assessment
//! 7. **Network Stability** - global optimisation over local greedy choices ([`gane_stability`])
//!
//! ## Crate Map
//!
//! ### Layer 1 - Foundation
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_core`] | Canonical data model with 15+ domain modules |
//! | [`gane_events`] | Typed event bus with pub/sub, envelope metadata, and idempotency |
//! | [`gane_config`] | Unified TOML/env configuration with validation and builder API |
//!
//! ### Layer 2 - GNSS and Positioning
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_gnss`] | Multi-GNSS receiver (GPS/Galileo/GLONASS/BeiDou), PVT solver, quality scoring |
//! | [`gane_corrections`] | SBAS, PPP, RTK, NRTK correction sources |
//! | [`gane_sensors`] | IMU/barometer/magnetometer/wheel-tick ingestion |
//! | [`gane_fusion`] | 9-D Extended Kalman Filter for position/velocity/heading fusion |
//! | [`gane_integrity`] | Multi-layer integrity monitoring (RAIM, cross-constellation, temporal) |
//! | [`gane_continuity`] | Mode switching with graceful degradation |
//!
//! ### Layer 3 - Map and Routing
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_map`] | Road graph spatial index, Mercator tile management, map matching |
//! | [`gane_routing`] | Dijkstra pathfinding with pluggable cost functions, turn-by-turn navigator |
//! | [`gane_lane`] | Lane detection, lane-change advisor with urgency levels |
//! | [`gane_offline`] | Offline region cache, deterministic sync engine |
//!
//! ### Layer 4 - Intelligence
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_risk`] | Multi-factor risk scoring per route segment |
//! | [`gane_confidence`] | Position confidence metrics and DOP-based quality indicators |
//! | [`gane_probabilistic`] | Stochastic route selection, Monte Carlo risk simulation |
//! | [`gane_traffic`] | Flow controller, traffic forecaster, herd suppressor |
//! | [`gane_stability`] | Network stability index, oscillation suppression |
//!
//! ### Layer 5 - Trust and Evidence
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_evidence`] | Evidence capture (photo, video, sensor snapshot, telemetry) |
//! | [`gane_trust`] | Trust scoring with decay, multi-dimensional weighting |
//! | [`gane_anti_manipulation`] | Spoofing detection, cross-validation, jamming mitigation |
//!
//! ### Layer 6 - User Experience
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_ux`] | Driving mode manager, cognitive-load reduction, theme engine |
//! | [`gane_micro_nav`] | Gate-level arrival, indoor BFS pathfinding, outdoor/indoor handoff |
//! | [`gane_parking`] | Parking probability (EMA occupancy), legality, reservations |
//! | [`gane_charging`] | Energy consumption prediction, 5 route-scoring strategies |
//!
//! ### Layer 7 - Fleet and Emergency
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_fleet`] | Task lifecycle, 4-strategy assignment, multi-stop dispatch, SLA monitoring |
//! | [`gane_emergency`] | Emergency corridor routing, mass incident management, hospital navigation |
//!
//! ### Layer 8 - Resilience and Communication
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_resilience`] | TTL/dedup queue, checksums, graceful degradation policies |
//! | [`gane_edge`] | Multi-stage processing pipeline, ML model management, data reduction |
//! | [`gane_satellite`] | CRC32 packet protocol, mesh routing, store-and-forward, link budgets |
//!
//! ### Layer 9 - Smart City and Digital Twin
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_city`] | SPaT signal controller, zone management, infrastructure health |
//! | [`gane_twin`] | Digital twin registry, discrete-time simulation, scenario sweeps |
//!
//! ### Layer 10 - Developer Platform
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_sdk`] | Client SDK for external integrations |
//! | [`gane_developer`] | API key management, rate limiting, webhook delivery |
//! | [`gane_marketplace`] | Plugin listing, review, installation |
//! | [`gane_payments`] | Billing, subscriptions, invoices, payment processing |
//! | [`gane_vehicle`] | OEM adapters, OBD-II protocol, vehicle profiles |
//!
//! ### Layer 11 - Application
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`gane_app`] | Main application with pipeline wiring and health aggregation |
//! | [`gane_telemetry`] | Ring-buffer recorder, audit log, replay engine |
//! | `gane_api` | Axum REST API server with CORS and tracing |
//!
//! ## Data Flow
//!
//! ```text
//! GNSS Satellites --> GnssReceiver --> QualityScorer --> PvtSolver
//!                                                          |
//! IMU/Barometer/Wheel --> SensorFusion --> NavigationEkf <--+
//!                                              |
//!                          IntegrityMonitor <---+
//!                          ContinuityManager <--+
//!                          RoadGraphIndex <------+
//!                               |
//!                          Dijkstra --> Navigator --> UX Manager --> REST API
//! ```
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gane_config::ConfigBuilder;
//! use gane_app::pipeline::NavigationPipeline;
//! use gane_app::health::build_health_report;
//!
//! // Build configuration with builder pattern
//! let (config, _warnings) = ConfigBuilder::new()
//!     .api_port(3000)
//!     .min_satellites(4)
//!     .instance_name("my-gane-node")
//!     .build()
//!     .expect("valid configuration");
//!
//! // Create the navigation pipeline
//! let pipeline = NavigationPipeline::new(config);
//!
//! // Check system health
//! let report = build_health_report(&pipeline);
//! println!("System health: {:?}", report.level);
//! ```
//!
//! ## Configuration
//!
//! ```rust
//! use gane_config::AuroraConfig;
//!
//! let config = AuroraConfig::default();
//! assert_eq!(config.api.port, 3000);
//! assert_eq!(config.gnss.min_satellites, 4);
//! ```
//!
//! ## Event System
//!
//! ```rust
//! use gane_events::bus::EventBus;
//! use gane_events::envelope::EventEnvelope;
//! use gane_events::event_type::EventType;
//! use uuid::Uuid;
//!
//! let bus = EventBus::new();
//! let event = EventEnvelope::new(
//!     "gnss",
//!     "Measurement",
//!     Uuid::new_v4(),
//!     EventType::GnssMeasurementReceived,
//!     serde_json::json!({"satellite_count": 12}),
//! );
//! bus.publish(&event);
//! assert_eq!(bus.total_events(), 1);
//! ```
//!
//! ## EKF Fusion
//!
//! ```rust
//! use gane_fusion::ekf::NavigationEkf;
//!
//! let mut ekf = NavigationEkf::new();
//! ekf.predict(1.0);
//! ekf.update_position(100.0, 200.0, 50.0, 5.0);
//! ekf.update_velocity(10.0, 5.0, 0.0, 1.0);
//!
//! let pos = ekf.position_enu();
//! let uncertainty = ekf.position_uncertainty_m();
//! ```

/// Crate count in the G.A.N.E NAV workspace.
pub const WORKSPACE_CRATE_COUNT: usize = 43;

/// Minimum test count across all workspace crates.
pub const MINIMUM_TEST_COUNT: usize = 870;

/// System version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_constants() {
        assert_eq!(WORKSPACE_CRATE_COUNT, 43);
        assert_eq!(MINIMUM_TEST_COUNT, 870);
        // VERSION is a compile-time constant from CARGO_PKG_VERSION
        let v: &str = VERSION;
        assert!(v.contains('.'));
    }

    #[test]
    fn config_default_example() {
        let config = gane_config::AuroraConfig::default();
        assert_eq!(config.api.port, 3000);
        assert_eq!(config.gnss.min_satellites, 4);
    }

    #[test]
    fn event_bus_example() {
        use gane_events::bus::EventBus;
        use gane_events::envelope::EventEnvelope;
        use gane_events::event_type::EventType;

        let bus = EventBus::new();
        let event = EventEnvelope::new(
            "docs",
            "Test",
            uuid::Uuid::new_v4(),
            EventType::PositionUpdate,
            serde_json::json!({}),
        );
        bus.publish(&event);
        assert_eq!(bus.total_events(), 1);
    }

    #[test]
    fn ekf_fusion_example() {
        use gane_fusion::ekf::NavigationEkf;

        let mut ekf = NavigationEkf::new();
        ekf.predict(1.0);
        ekf.update_position(100.0, 200.0, 50.0, 5.0);
        let pos = ekf.position_enu();
        assert!(pos.x.abs() > 0.0 || pos.y.abs() > 0.0);
    }

    #[test]
    fn pipeline_health_example() {
        use gane_app::health::build_health_report;
        use gane_app::pipeline::NavigationPipeline;

        let pipeline = NavigationPipeline::new(gane_config::AuroraConfig::default());
        let report = build_health_report(&pipeline);
        assert!(!report.subsystems.is_empty());
    }
}
