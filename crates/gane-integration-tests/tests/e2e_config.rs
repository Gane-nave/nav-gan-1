//! End-to-end configuration integration tests.
//!
//! Proves that configuration flows correctly from TOML → AuroraConfig →
//! pipeline subsystems.

use gane_config::{AuroraConfig, ConfigBuilder};

#[test]
fn config_round_trip_toml() {
    let original = AuroraConfig::default();
    let toml_str = toml::to_string_pretty(&original).unwrap();
    let parsed: AuroraConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.api.port, original.api.port);
    assert_eq!(parsed.gnss.min_satellites, original.gnss.min_satellites);
    assert_eq!(parsed.gnss.enable_gps, original.gnss.enable_gps);
    assert_eq!(parsed.system.worker_threads, original.system.worker_threads);
    assert_eq!(
        parsed.telemetry.buffer_capacity,
        original.telemetry.buffer_capacity
    );
    assert_eq!(parsed.fleet.enabled, original.fleet.enabled);
    assert_eq!(parsed.satellite.enabled, original.satellite.enabled);
}

#[test]
fn config_partial_toml_preserves_defaults() {
    let toml_str = r#"
[api]
port = 8080

[gnss]
min_satellites = 6
enable_gps = false
"#;
    let cfg: AuroraConfig = toml::from_str(toml_str).unwrap();

    // Overridden values
    assert_eq!(cfg.api.port, 8080);
    assert_eq!(cfg.gnss.min_satellites, 6);
    assert!(!cfg.gnss.enable_gps);

    // Default values preserved
    assert!(cfg.gnss.enable_galileo);
    assert!(cfg.gnss.enable_glonass);
    assert!(cfg.gnss.enable_beidou);
    assert_eq!(cfg.system.worker_threads, 4);
    assert_eq!(cfg.telemetry.buffer_capacity, 50_000);
    assert!(cfg.integrity.enable_jamming_detection);
}

#[test]
fn config_builder_produces_valid_config() {
    let (cfg, result) = ConfigBuilder::new()
        .instance_name("test-instance")
        .api_port(7070)
        .min_satellites(5)
        .log_level("debug")
        .worker_threads(2)
        .fleet_enabled(true)
        .satellite_enabled(true)
        .twin_enabled(true)
        .risk_aversion(0.8)
        .max_alternatives(5)
        .build()
        .unwrap();

    assert!(result.warnings.is_empty());
    assert_eq!(cfg.system.instance_name, "test-instance");
    assert_eq!(cfg.api.port, 7070);
    assert_eq!(cfg.gnss.min_satellites, 5);
    assert_eq!(cfg.system.log_level, "debug");
    assert_eq!(cfg.system.worker_threads, 2);
    assert!(cfg.fleet.enabled);
    assert!(cfg.satellite.enabled);
    assert!(cfg.twin.enabled);
    assert!((cfg.routing.risk_aversion - 0.8).abs() < f64::EPSILON);
    assert_eq!(cfg.routing.max_alternatives, 5);
}

#[test]
fn config_validation_catches_invalid_gnss() {
    let result = ConfigBuilder::new().min_satellites(2).build();
    assert!(result.is_err());
    assert!(
        result.unwrap_err().to_string().contains("min_satellites"),
        "error should mention min_satellites"
    );
}

#[test]
fn config_validation_catches_zero_port() {
    let result = ConfigBuilder::new().api_port(0).build();
    assert!(result.is_err());
    assert!(
        result.unwrap_err().to_string().contains("port"),
        "error should mention port"
    );
}

#[test]
fn config_all_23_sections_present() {
    let cfg = AuroraConfig::default();
    let toml_str = toml::to_string_pretty(&cfg).unwrap();

    // Verify all 23 config sections are present in TOML output
    let sections = [
        "[system]",
        "[gnss]",
        "[corrections]",
        "[fusion]",
        "[integrity]",
        "[continuity]",
        "[routing]",
        "[map]",
        "[traffic]",
        "[ux]",
        "[fleet]",
        "[emergency]",
        "[offline]",
        "[edge]",
        "[satellite]",
        "[city]",
        "[twin]",
        "[developer]",
        "[marketplace]",
        "[payments]",
        "[vehicle]",
        "[api]",
        "[telemetry]",
    ];

    for section in &sections {
        assert!(
            toml_str.contains(section),
            "TOML output must contain section {}",
            section
        );
    }
}
