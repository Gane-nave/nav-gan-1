//! End-to-end integration tests for the full AURORA NAV navigation pipeline.
//!
//! These tests exercise cross-crate interactions that unit tests within
//! individual crates cannot cover.

use aurora_app::health::{build_health_report, HealthLevel};
use aurora_app::pipeline::NavigationPipeline;
use aurora_config::{AuroraConfig, ConfigBuilder};

// ---------------------------------------------------------------------------
// Pipeline lifecycle
// ---------------------------------------------------------------------------

#[test]
fn pipeline_initialises_all_subsystems() {
    let pipeline = NavigationPipeline::new(AuroraConfig::default());
    assert!(pipeline.is_healthy());
    assert_eq!(pipeline.tracked_satellites(), 0);
    assert!(pipeline.current_position().is_none());
    assert_eq!(pipeline.total_events(), 0);
    assert_eq!(pipeline.integrity_level(), "NoSolution");
}

#[test]
fn pipeline_config_propagates_to_subsystems() {
    let cfg = ConfigBuilder::new()
        .telemetry_buffer(500)
        .api_port(9999)
        .min_satellites(5)
        .build_unchecked();

    let pipeline = NavigationPipeline::new(cfg);
    assert_eq!(pipeline.config().api.port, 9999);
    assert_eq!(pipeline.config().gnss.min_satellites, 5);
    assert_eq!(pipeline.config().telemetry.buffer_capacity, 500);
}

// ---------------------------------------------------------------------------
// Health reporting
// ---------------------------------------------------------------------------

#[test]
fn health_report_reflects_pipeline_state() {
    let pipeline = NavigationPipeline::new(AuroraConfig::default());
    let report = build_health_report(&pipeline);

    assert_eq!(report.level, HealthLevel::Healthy);
    assert!(!report.version.is_empty());
    assert!(report.subsystems.len() >= 6);

    // All subsystems should be healthy on a fresh pipeline
    for sub in &report.subsystems {
        assert!(sub.healthy, "subsystem {} should be healthy", sub.name);
    }
}

#[test]
fn health_report_serialises_cleanly() {
    let pipeline = NavigationPipeline::new(AuroraConfig::default());
    let report = build_health_report(&pipeline);
    let json = serde_json::to_string_pretty(&report).unwrap();

    assert!(json.contains("\"level\""));
    assert!(json.contains("\"version\""));
    assert!(json.contains("\"subsystems\""));
    assert!(json.contains("\"gnss\""));
    assert!(json.contains("\"fusion\""));
    assert!(json.contains("\"integrity\""));
}

// ---------------------------------------------------------------------------
// Subsystem status
// ---------------------------------------------------------------------------

#[test]
fn subsystem_status_reflects_config_toggles() {
    let cfg = ConfigBuilder::new()
        .fleet_enabled(true)
        .satellite_enabled(true)
        .twin_enabled(true)
        .vehicle_enabled(true)
        .payments_enabled(true)
        .build_unchecked();

    let pipeline = NavigationPipeline::new(cfg);
    let status = pipeline.subsystem_status();

    assert!(status.fleet_enabled, "fleet should be enabled");
    assert!(status.satellite_enabled, "satellite should be enabled");
    assert!(status.twin_enabled, "twin should be enabled");
    assert!(status.vehicle_enabled, "vehicle should be enabled");
    assert!(status.payments_enabled, "payments should be enabled");
}

#[test]
fn subsystem_status_default_disabled_features() {
    let pipeline = NavigationPipeline::new(AuroraConfig::default());
    let status = pipeline.subsystem_status();

    // These should be off by default
    assert!(!status.fleet_enabled, "fleet should be disabled by default");
    assert!(
        !status.satellite_enabled,
        "satellite should be disabled by default"
    );
    assert!(!status.twin_enabled, "twin should be disabled by default");
    assert!(
        !status.vehicle_enabled,
        "vehicle should be disabled by default"
    );
    assert!(
        !status.payments_enabled,
        "payments should be disabled by default"
    );

    // These should be on by default
    assert!(status.gnss_enabled, "gnss should be enabled by default");
    assert!(
        status.integrity_enabled,
        "integrity should be enabled by default"
    );
    assert!(
        status.routing_enabled,
        "routing should be enabled by default"
    );
    assert!(
        status.traffic_enabled,
        "traffic should be enabled by default"
    );
    assert!(
        status.offline_enabled,
        "offline should be enabled by default"
    );
    assert!(status.edge_enabled, "edge should be enabled by default");
}
