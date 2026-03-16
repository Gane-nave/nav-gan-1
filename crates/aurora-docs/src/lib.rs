//! # AURORA NAV / GMIN - Global Mobility Intelligence Network
//!
//! A comprehensive, production-grade navigation system built in Rust.
//!
//! ## Architecture Overview
//!
//! AURORA NAV is organised as a modular Rust workspace with 43+ crates,
//! each responsible for a well-defined domain within the navigation stack.
//! The system follows these core principles:
//!
//! 1. **Single Source of Truth** - one canonical data model ([`aurora_core`])
//! 2. **Event-Driven Architecture** - all subsystems communicate via typed events ([`aurora_events`])
//! 3. **Offline-First** - full operation without network connectivity ([`aurora_offline`], [`aurora_resilience`])
//! 4. **Edge-First Processing** - ML inference and data reduction at the edge ([`aurora_edge`])
//! 5. **Satellite Fallback** - LEO/GEO mesh communication for extreme scenarios ([`aurora_satellite`])
//! 6. **Probabilistic Navigation** - confidence-weighted routing and risk assessment
//! 7. **Network Stability** - global optimisation over local greedy choices ([`aurora_stability`])
//!
//! ## Crate Map
//!
//! ### Layer 1 - Foundation
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_core`] | Canonical data model with 15+ domain modules |
//! | [`aurora_events`] | Typed event bus with pub/sub, envelope metadata, and idempotency |
//! | [`aurora_config`] | Unified TOML/env configuration with validation and builder API |
//!
//! ### Layer 2 - GNSS and Positioning
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_gnss`] | Multi-GNSS receiver (GPS/Galileo/GLONASS/BeiDou), PVT solver, quality scoring |
//! | [`aurora_corrections`] | SBAS, PPP, RTK, NRTK correction sources |
//! | [`aurora_sensors`] | IMU/barometer/magnetometer/wheel-tick ingestion |
//! | [`aurora_fusion`] | 9-D Extended Kalman Filter for position/velocity/heading fusion |
//! | [`aurora_integrity`] | Multi-layer integrity monitoring (RAIM, cross-constellation, temporal) |
//! | [`aurora_continuity`] | Mode switching with graceful degradation |
//!
//! ### Layer 3 - Map and Routing
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_map`] | Road graph spatial index, Mercator tile management, map matching |
//! | [`aurora_routing`] | Dijkstra pathfinding with pluggable cost functions, turn-by-turn navigator |
//! | [`aurora_lane`] | Lane detection, lane-change advisor with urgency levels |
//! | [`aurora_offline`] | Offline region cache, deterministic sync engine |
//!
//! ### Layer 4 - Intelligence
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_risk`] | Multi-factor risk scoring per route segment |
//! | [`aurora_confidence`] | Position confidence metrics and DOP-based quality indicators |
//! | [`aurora_probabilistic`] | Stochastic route selection, Monte Carlo risk simulation |
//! | [`aurora_traffic`] | Flow controller, traffic forecaster, herd suppressor |
//! | [`aurora_stability`] | Network stability index, oscillation suppression |
//!
//! ### Layer 5 - Trust and Evidence
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_evidence`] | Evidence capture (photo, video, sensor snapshot, telemetry) |
//! | [`aurora_trust`] | Trust scoring with decay, multi-dimensional weighting |
//! | [`aurora_anti_manipulation`] | Spoofing detection, cross-validation, jamming mitigation |
//!
//! ### Layer 6 - User Experience
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_ux`] | Driving mode manager, cognitive-load reduction, theme engine |
//! | [`aurora_micro_nav`] | Gate-level arrival, indoor BFS pathfinding, outdoor/indoor handoff |
//! | [`aurora_parking`] | Parking probability (EMA occupancy), legality, reservations |
//! | [`aurora_charging`] | Energy consumption prediction, 5 route-scoring strategies |
//!
//! ### Layer 7 - Fleet and Emergency
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_fleet`] | Task lifecycle, 4-strategy assignment, multi-stop dispatch, SLA monitoring |
//! | [`aurora_emergency`] | Emergency corridor routing, mass incident management, hospital navigation |
//!
//! ### Layer 8 - Resilience and Communication
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_resilience`] | TTL/dedup queue, checksums, graceful degradation policies |
//! | [`aurora_edge`] | Multi-stage processing pipeline, ML model management, data reduction |
//! | [`aurora_satellite`] | CRC32 packet protocol, mesh routing, store-and-forward, link budgets |
//!
//! ### Layer 9 - Smart City and Digital Twin
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_city`] | SPaT signal controller, zone management, infrastructure health |
//! | [`aurora_twin`] | Digital twin registry, discrete-time simulation, scenario sweeps |
//!
//! ### Layer 10 - Developer Platform
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_sdk`] | Client SDK for external integrations |
//! | [`aurora_developer`] | API key management, rate limiting, webhook delivery |
//! | [`aurora_marketplace`] | Plugin listing, review, installation |
//! | [`aurora_payments`] | Billing, subscriptions, invoices, payment processing |
//! | [`aurora_vehicle`] | OEM adapters, OBD-II protocol, vehicle profiles |
//!
//! ### Layer 11 - Application
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`aurora_app`] | Main application with pipeline wiring and health aggregation |
//! | [`aurora_telemetry`] | Ring-buffer recorder, audit log, replay engine |
//! | `aurora_api` | Axum REST API server with CORS and tracing |
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
//! use aurora_config::ConfigBuilder;
//! use aurora_app::pipeline::NavigationPipeline;
//! use aurora_app::health::build_health_report;
//!
//! // Build configuration with builder pattern
//! let (config, _warnings) = ConfigBuilder::new()
//!     .api_port(3000)
//!     .min_satellites(4)
//!     .instance_name("my-aurora-node")
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
//! use aurora_config::AuroraConfig;
//!
//! let config = AuroraConfig::default();
//! assert_eq!(config.api.port, 3000);
//! assert_eq!(config.gnss.min_satellites, 4);
//! ```
//!
//! ## Event System
//!
//! ```rust
//! use aurora_events::bus::EventBus;
//! use aurora_events::envelope::EventEnvelope;
//! use aurora_events::event_type::EventType;
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
//! use aurora_fusion::ekf::NavigationEkf;
//!
//! let mut ekf = NavigationEkf::new();
//! ekf.predict(1.0);
//! ekf.update_position(100.0, 200.0, 50.0, 5.0);
//! ekf.update_velocity(10.0, 5.0, 0.0, 1.0);
//!
//! let pos = ekf.position_enu();
//! let uncertainty = ekf.position_uncertainty_m();
//! ```

/// Crate count in the AURORA NAV workspace.
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
        let config = aurora_config::AuroraConfig::default();
        assert_eq!(config.api.port, 3000);
        assert_eq!(config.gnss.min_satellites, 4);
    }

    #[test]
    fn event_bus_example() {
        use aurora_events::bus::EventBus;
        use aurora_events::envelope::EventEnvelope;
        use aurora_events::event_type::EventType;

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
        use aurora_fusion::ekf::NavigationEkf;

        let mut ekf = NavigationEkf::new();
        ekf.predict(1.0);
        ekf.update_position(100.0, 200.0, 50.0, 5.0);
        let pos = ekf.position_enu();
        assert!(pos.x.abs() > 0.0 || pos.y.abs() > 0.0);
    }

    #[test]
    fn pipeline_health_example() {
        use aurora_app::health::build_health_report;
        use aurora_app::pipeline::NavigationPipeline;

        let pipeline = NavigationPipeline::new(aurora_config::AuroraConfig::default());
        let report = build_health_report(&pipeline);
        assert!(!report.subsystems.is_empty());
    }
}
