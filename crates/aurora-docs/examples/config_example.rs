//! Configuration example for AURORA NAV.
//!
//! Demonstrates building configuration with the fluent builder API,
//! including validation, constellation toggles, and feature flags.

use aurora_config::ConfigBuilder;

fn main() {
    // Build a custom configuration
    let (config, warnings) = ConfigBuilder::new()
        .instance_name("aurora-prod-01")
        .log_level("info")
        .worker_threads(4)
        .api_port(8080)
        .api_host("0.0.0.0")
        .min_satellites(5)
        .elevation_mask_deg(10.0)
        .enable_gps(true)
        .enable_galileo(true)
        .enable_glonass(true)
        .enable_beidou(false)
        .telemetry_buffer(5000)
        .fleet_enabled(true)
        .satellite_enabled(false)
        .twin_enabled(true)
        .vehicle_enabled(true)
        .payments_enabled(false)
        .process_noise_scale(1.2)
        .risk_aversion(0.6)
        .max_alternatives(3)
        .build()
        .expect("configuration should be valid");

    println!("Configuration built successfully");
    println!("  Instance: {}", config.system.instance_name);
    println!("  API: {}:{}", config.api.host, config.api.port);
    println!("  Min satellites: {}", config.gnss.min_satellites);
    println!("  GPS: {}", config.gnss.enable_gps);
    println!("  Galileo: {}", config.gnss.enable_galileo);
    println!("  GLONASS: {}", config.gnss.enable_glonass);
    println!("  BeiDou: {}", config.gnss.enable_beidou);
    println!("  Fleet: {}", config.fleet.enabled);
    println!("  Twin: {}", config.twin.enabled);
    println!("  Risk aversion: {}", config.routing.risk_aversion);

    if !warnings.warnings.is_empty() {
        println!("\nWarnings:");
        for w in &warnings.warnings {
            println!("  - {}", w);
        }
    } else {
        println!("\nNo validation warnings.");
    }

    // Default configuration
    let default_config = aurora_config::AuroraConfig::default();
    println!("\nDefault config port: {}", default_config.api.port);

    // Serialise to JSON
    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("\nConfig JSON (first 200 chars):");
    println!("{}", &json[..json.len().min(200)]);
}
