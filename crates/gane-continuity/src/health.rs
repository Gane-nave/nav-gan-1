//! Health state machine for pipeline watchdogs.

use chrono::{DateTime, Utc};
use gane_core::types::ContinuityMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Subsystem identifier for health monitoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subsystem {
    GnssReceiver,
    CorrectionEngine,
    ImuSensor,
    WheelOdometry,
    FusionEngine,
    IntegrityEngine,
    MapMatching,
    VisualOdometry,
    Barometer,
    CommunicationLink,
}

/// Health status of a subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Subsystem operating normally.
    Healthy,
    /// Subsystem showing degraded performance.
    Degraded,
    /// Subsystem has failed.
    Failed,
    /// Subsystem status unknown (no heartbeat).
    Unknown,
}

/// Health entry for a single subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemHealth {
    pub subsystem: Subsystem,
    pub status: HealthStatus,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub consecutive_failures: u32,
    pub details: Option<String>,
}

/// Health state machine that monitors all pipeline subsystems
/// and determines the maximum achievable continuity mode.
pub struct HealthStateMachine {
    subsystems: HashMap<Subsystem, SubsystemHealth>,
    /// Maximum time without heartbeat before status changes to Unknown (seconds).
    heartbeat_timeout_s: f64,
    /// Maximum consecutive failures before status changes to Failed.
    max_consecutive_failures: u32,
}

impl HealthStateMachine {
    pub fn new() -> Self {
        let mut subsystems = HashMap::new();
        for sub in &[
            Subsystem::GnssReceiver,
            Subsystem::CorrectionEngine,
            Subsystem::ImuSensor,
            Subsystem::WheelOdometry,
            Subsystem::FusionEngine,
            Subsystem::IntegrityEngine,
            Subsystem::MapMatching,
            Subsystem::VisualOdometry,
            Subsystem::Barometer,
            Subsystem::CommunicationLink,
        ] {
            subsystems.insert(
                *sub,
                SubsystemHealth {
                    subsystem: *sub,
                    status: HealthStatus::Unknown,
                    last_heartbeat: None,
                    consecutive_failures: 0,
                    details: None,
                },
            );
        }

        Self {
            subsystems,
            heartbeat_timeout_s: 5.0,
            max_consecutive_failures: 3,
        }
    }

    /// Report a healthy heartbeat from a subsystem.
    pub fn heartbeat(&mut self, subsystem: Subsystem) {
        if let Some(entry) = self.subsystems.get_mut(&subsystem) {
            entry.status = HealthStatus::Healthy;
            entry.last_heartbeat = Some(Utc::now());
            entry.consecutive_failures = 0;
        }
    }

    /// Report a failure from a subsystem.
    pub fn report_failure(&mut self, subsystem: Subsystem, details: String) {
        if let Some(entry) = self.subsystems.get_mut(&subsystem) {
            entry.consecutive_failures += 1;
            entry.details = Some(details);

            if entry.consecutive_failures >= self.max_consecutive_failures {
                entry.status = HealthStatus::Failed;
            } else {
                entry.status = HealthStatus::Degraded;
            }
        }
    }

    /// Check for stale heartbeats and update statuses.
    pub fn check_timeouts(&mut self) {
        let now = Utc::now();
        for entry in self.subsystems.values_mut() {
            if let Some(last) = entry.last_heartbeat {
                let age = (now - last).num_milliseconds() as f64 / 1000.0;
                if age > self.heartbeat_timeout_s && entry.status == HealthStatus::Healthy {
                    entry.status = HealthStatus::Degraded;
                }
                if age > self.heartbeat_timeout_s * 3.0 {
                    entry.status = HealthStatus::Unknown;
                }
            }
        }
    }

    /// Determine the maximum achievable continuity mode based on subsystem health.
    pub fn max_achievable_mode(&self) -> ContinuityMode {
        let gnss_ok = self.is_healthy_or_degraded(Subsystem::GnssReceiver);
        let corrections_ok = self.is_healthy(Subsystem::CorrectionEngine);
        let imu_ok = self.is_healthy_or_degraded(Subsystem::ImuSensor);
        let odometry_ok = self.is_healthy_or_degraded(Subsystem::WheelOdometry);
        let map_ok = self.is_healthy_or_degraded(Subsystem::MapMatching);

        if gnss_ok && corrections_ok {
            ContinuityMode::ModeA
        } else if gnss_ok {
            ContinuityMode::ModeB
        } else if imu_ok && (gnss_ok || self.is_degraded(Subsystem::GnssReceiver)) {
            ContinuityMode::ModeC
        } else if imu_ok || odometry_ok || map_ok {
            ContinuityMode::ModeD
        } else {
            ContinuityMode::ModeE
        }
    }

    /// Get the health status of a specific subsystem.
    pub fn status(&self, subsystem: Subsystem) -> HealthStatus {
        self.subsystems
            .get(&subsystem)
            .map(|s| s.status)
            .unwrap_or(HealthStatus::Unknown)
    }

    /// Get all subsystem health entries.
    pub fn all_health(&self) -> Vec<&SubsystemHealth> {
        self.subsystems.values().collect()
    }

    fn is_healthy(&self, subsystem: Subsystem) -> bool {
        self.status(subsystem) == HealthStatus::Healthy
    }

    fn is_degraded(&self, subsystem: Subsystem) -> bool {
        self.status(subsystem) == HealthStatus::Degraded
    }

    fn is_healthy_or_degraded(&self, subsystem: Subsystem) -> bool {
        matches!(
            self.status(subsystem),
            HealthStatus::Healthy | HealthStatus::Degraded
        )
    }
}

impl Default for HealthStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_unknown() {
        let hsm = HealthStateMachine::new();
        assert_eq!(hsm.status(Subsystem::GnssReceiver), HealthStatus::Unknown);
    }

    #[test]
    fn heartbeat_sets_healthy() {
        let mut hsm = HealthStateMachine::new();
        hsm.heartbeat(Subsystem::GnssReceiver);
        assert_eq!(hsm.status(Subsystem::GnssReceiver), HealthStatus::Healthy);
    }

    #[test]
    fn failures_cause_degradation_then_failure() {
        let mut hsm = HealthStateMachine::new();
        hsm.heartbeat(Subsystem::ImuSensor);

        hsm.report_failure(Subsystem::ImuSensor, "bias drift".into());
        assert_eq!(hsm.status(Subsystem::ImuSensor), HealthStatus::Degraded);

        hsm.report_failure(Subsystem::ImuSensor, "bias drift".into());
        hsm.report_failure(Subsystem::ImuSensor, "bias drift".into());
        assert_eq!(hsm.status(Subsystem::ImuSensor), HealthStatus::Failed);
    }

    #[test]
    fn mode_a_when_gnss_and_corrections_healthy() {
        let mut hsm = HealthStateMachine::new();
        hsm.heartbeat(Subsystem::GnssReceiver);
        hsm.heartbeat(Subsystem::CorrectionEngine);
        assert_eq!(hsm.max_achievable_mode(), ContinuityMode::ModeA);
    }

    #[test]
    fn mode_b_when_gnss_only() {
        let mut hsm = HealthStateMachine::new();
        hsm.heartbeat(Subsystem::GnssReceiver);
        assert_eq!(hsm.max_achievable_mode(), ContinuityMode::ModeB);
    }

    #[test]
    fn mode_d_when_only_imu() {
        let mut hsm = HealthStateMachine::new();
        hsm.heartbeat(Subsystem::ImuSensor);
        assert_eq!(hsm.max_achievable_mode(), ContinuityMode::ModeD);
    }

    #[test]
    fn mode_e_when_nothing_healthy() {
        let hsm = HealthStateMachine::new();
        assert_eq!(hsm.max_achievable_mode(), ContinuityMode::ModeE);
    }
}
