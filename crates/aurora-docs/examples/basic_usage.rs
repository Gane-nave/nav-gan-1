//! Basic usage example for AURORA NAV.
//!
//! Demonstrates creating a navigation pipeline, checking health,
//! and inspecting subsystem status.

use aurora_app::health::{build_health_report, HealthLevel};
use aurora_app::pipeline::NavigationPipeline;
use aurora_config::AuroraConfig;

fn main() {
    // Use default configuration
    let config = AuroraConfig::default();
    println!("AURORA NAV v{}", env!("CARGO_PKG_VERSION"));
    println!("API port: {}", config.api.port);
    println!("Min satellites: {}", config.gnss.min_satellites);

    // Create the navigation pipeline
    let pipeline = NavigationPipeline::new(config);

    // Build and display health report
    let report = build_health_report(&pipeline);
    println!("\nSystem Health: {:?}", report.level);
    println!("Version: {}", report.version);
    println!("Total events: {}", report.total_events);
    println!("Telemetry samples: {}", report.telemetry_samples);

    println!("\nSubsystems:");
    for sub in &report.subsystems {
        let status = if sub.healthy { "OK" } else { "DEGRADED" };
        println!("  [{}] {} - {}", status, sub.name, sub.detail);
    }

    // Verify overall health
    assert_eq!(report.level, HealthLevel::Healthy);
    println!("\nAll subsystems operational.");
}
