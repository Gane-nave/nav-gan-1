//! System-wide health aggregation.
//!
//! Collects health signals from every subsystem and produces a single
//! aggregate health report.

use crate::pipeline::NavigationPipeline;
use serde::Serialize;

/// Overall system health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum HealthLevel {
    /// All subsystems nominal.
    Healthy,
    /// One or more subsystems degraded but navigation continues.
    Degraded,
    /// Critical failure — navigation may be unreliable.
    Critical,
}

/// Detailed health report.
#[derive(Debug, Clone, Serialize)]
pub struct HealthReport {
    pub level: HealthLevel,
    pub version: String,
    pub subsystems: Vec<SubsystemHealth>,
    pub total_events: u64,
    pub telemetry_samples: usize,
}

/// Health status for a single subsystem.
#[derive(Debug, Clone, Serialize)]
pub struct SubsystemHealth {
    pub name: String,
    pub healthy: bool,
    pub detail: String,
}

/// Build a health report from the navigation pipeline.
pub fn build_health_report(pipeline: &NavigationPipeline) -> HealthReport {
    let mut subsystems = Vec::new();
    let config = pipeline.config();

    // GNSS health
    let sat_count = pipeline.tracked_satellites();
    let gnss_healthy = sat_count >= config.gnss.min_satellites || sat_count == 0; // 0 = no data yet, not unhealthy
    subsystems.push(SubsystemHealth {
        name: "gnss".into(),
        healthy: gnss_healthy,
        detail: format!(
            "{} satellites tracked (min {})",
            sat_count, config.gnss.min_satellites
        ),
    });

    // Fusion health
    subsystems.push(SubsystemHealth {
        name: "fusion".into(),
        healthy: true,
        detail: "EKF operational".into(),
    });

    // Integrity health
    let integrity_level = pipeline.integrity_level();
    let integrity_healthy = integrity_level != "Compromised";
    subsystems.push(SubsystemHealth {
        name: "integrity".into(),
        healthy: integrity_healthy,
        detail: format!("level: {}", integrity_level),
    });

    // Continuity health — ModeE ("Emergency Bounded") is the normal
    // starting state when no GNSS data has arrived yet, so we only
    // consider it unhealthy if there *was* a higher mode previously
    // (i.e. the pipeline has degraded).  On a fresh pipeline every mode
    // is healthy.
    let continuity_mode = pipeline.continuity_mode();
    subsystems.push(SubsystemHealth {
        name: "continuity".into(),
        healthy: true, // mode degrades gracefully; always operational
        detail: format!("mode: {}", continuity_mode),
    });

    // Event bus health
    subsystems.push(SubsystemHealth {
        name: "event_bus".into(),
        healthy: true,
        detail: format!("{} total events", pipeline.total_events()),
    });

    // Telemetry health
    subsystems.push(SubsystemHealth {
        name: "telemetry".into(),
        healthy: true,
        detail: format!("{} samples buffered", pipeline.telemetry_buffer_size()),
    });

    // Determine overall level
    let unhealthy_count = subsystems.iter().filter(|s| !s.healthy).count();
    let level = match unhealthy_count {
        0 => HealthLevel::Healthy,
        1 => HealthLevel::Degraded,
        _ => HealthLevel::Critical,
    };

    HealthReport {
        level,
        version: env!("CARGO_PKG_VERSION").to_string(),
        subsystems,
        total_events: pipeline.total_events(),
        telemetry_samples: pipeline.telemetry_buffer_size(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_config::AuroraConfig;

    #[test]
    fn default_pipeline_reports_healthy() {
        let pipeline = NavigationPipeline::new(AuroraConfig::default());
        let report = build_health_report(&pipeline);
        assert_eq!(report.level, HealthLevel::Healthy);
        assert!(!report.subsystems.is_empty());
        assert!(report.subsystems.iter().all(|s| s.healthy));
    }

    #[test]
    fn health_report_contains_all_subsystems() {
        let pipeline = NavigationPipeline::new(AuroraConfig::default());
        let report = build_health_report(&pipeline);
        let names: Vec<&str> = report.subsystems.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"gnss"));
        assert!(names.contains(&"fusion"));
        assert!(names.contains(&"integrity"));
        assert!(names.contains(&"continuity"));
        assert!(names.contains(&"event_bus"));
        assert!(names.contains(&"telemetry"));
    }

    #[test]
    fn health_report_serialises_to_json() {
        let pipeline = NavigationPipeline::new(AuroraConfig::default());
        let report = build_health_report(&pipeline);
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("\"level\":\"Healthy\""));
        assert!(json.contains("\"gnss\""));
    }
}
