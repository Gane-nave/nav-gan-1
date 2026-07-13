//! System health checks — monitors subsystem status and reports overall system health.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Health status of a subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HealthStatus {
    /// Subsystem is fully operational.
    Healthy,
    /// Subsystem is working but with reduced capability.
    Degraded,
    /// Subsystem is experiencing issues.
    Unhealthy,
    /// Subsystem is not responding.
    Down,
}

/// A health check result for a single subsystem.
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Subsystem name.
    pub subsystem: String,
    /// Current status.
    pub status: HealthStatus,
    /// Response latency.
    pub latency: Duration,
    /// Optional diagnostic message.
    pub message: Option<String>,
    /// Timestamp of last check.
    pub checked_at: Instant,
}

/// Configuration for health monitoring.
#[derive(Debug, Clone)]
pub struct HealthConfig {
    /// Maximum acceptable latency before marking as degraded.
    pub degraded_latency_threshold: Duration,
    /// Maximum acceptable latency before marking as unhealthy.
    pub unhealthy_latency_threshold: Duration,
    /// Check interval.
    pub check_interval: Duration,
    /// Number of consecutive failures before marking as down.
    pub down_after_failures: u32,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            degraded_latency_threshold: Duration::from_millis(500),
            unhealthy_latency_threshold: Duration::from_secs(2),
            check_interval: Duration::from_secs(30),
            down_after_failures: 3,
        }
    }
}

/// Health monitor — tracks subsystem health status.
pub struct HealthMonitor {
    config: HealthConfig,
    subsystems: HashMap<String, SubsystemState>,
}

struct SubsystemState {
    status: HealthStatus,
    last_check: Option<Instant>,
    consecutive_failures: u32,
    total_checks: u64,
    total_failures: u64,
}

impl HealthMonitor {
    /// Create a new health monitor.
    pub fn new(config: HealthConfig) -> Self {
        Self {
            config,
            subsystems: HashMap::new(),
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(HealthConfig::default())
    }

    /// Register a subsystem for monitoring.
    pub fn register(&mut self, name: &str) {
        self.subsystems
            .entry(name.to_string())
            .or_insert(SubsystemState {
                status: HealthStatus::Healthy,
                last_check: None,
                consecutive_failures: 0,
                total_checks: 0,
                total_failures: 0,
            });
    }

    /// Record a health check result.
    pub fn record_check(&mut self, name: &str, latency: Duration, success: bool) -> HealthStatus {
        let config = self.config.clone();
        let state = self
            .subsystems
            .entry(name.to_string())
            .or_insert(SubsystemState {
                status: HealthStatus::Healthy,
                last_check: None,
                consecutive_failures: 0,
                total_checks: 0,
                total_failures: 0,
            });

        state.total_checks += 1;
        state.last_check = Some(Instant::now());

        if !success {
            state.consecutive_failures += 1;
            state.total_failures += 1;

            state.status = if state.consecutive_failures >= config.down_after_failures {
                HealthStatus::Down
            } else {
                HealthStatus::Unhealthy
            };
        } else {
            state.consecutive_failures = 0;

            state.status = if latency >= config.unhealthy_latency_threshold {
                HealthStatus::Unhealthy
            } else if latency >= config.degraded_latency_threshold {
                HealthStatus::Degraded
            } else {
                HealthStatus::Healthy
            };
        }

        state.status
    }

    /// Get status of a specific subsystem.
    pub fn status(&self, name: &str) -> Option<HealthStatus> {
        self.subsystems.get(name).map(|s| s.status)
    }

    /// Get overall system health (worst status across all subsystems).
    pub fn overall_health(&self) -> HealthStatus {
        self.subsystems
            .values()
            .map(|s| s.status)
            .max()
            .unwrap_or(HealthStatus::Healthy)
    }

    /// Get the number of monitored subsystems.
    pub fn subsystem_count(&self) -> usize {
        self.subsystems.len()
    }

    /// Get failure rate for a subsystem.
    pub fn failure_rate(&self, name: &str) -> Option<f64> {
        self.subsystems.get(name).map(|s| {
            if s.total_checks == 0 {
                0.0
            } else {
                s.total_failures as f64 / s.total_checks as f64
            }
        })
    }

    /// Get all subsystems with a given status.
    pub fn by_status(&self, status: HealthStatus) -> Vec<String> {
        self.subsystems
            .iter()
            .filter(|(_, s)| s.status == status)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Get summary: (healthy, degraded, unhealthy, down).
    pub fn summary(&self) -> (usize, usize, usize, usize) {
        let mut healthy = 0;
        let mut degraded = 0;
        let mut unhealthy = 0;
        let mut down = 0;
        for s in self.subsystems.values() {
            match s.status {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Degraded => degraded += 1,
                HealthStatus::Unhealthy => unhealthy += 1,
                HealthStatus::Down => down += 1,
            }
        }
        (healthy, degraded, unhealthy, down)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_subsystem() {
        let mut monitor = HealthMonitor::with_defaults();
        monitor.register("gnss");
        monitor.register("imu");
        assert_eq!(monitor.subsystem_count(), 2);
    }

    #[test]
    fn test_healthy_check() {
        let mut monitor = HealthMonitor::with_defaults();
        let status = monitor.record_check("gnss", Duration::from_millis(50), true);
        assert_eq!(status, HealthStatus::Healthy);
    }

    #[test]
    fn test_degraded_on_slow_response() {
        let mut monitor = HealthMonitor::with_defaults();
        let status = monitor.record_check("gnss", Duration::from_millis(600), true);
        assert_eq!(status, HealthStatus::Degraded);
    }

    #[test]
    fn test_unhealthy_on_very_slow_response() {
        let mut monitor = HealthMonitor::with_defaults();
        let status = monitor.record_check("gnss", Duration::from_secs(3), true);
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_unhealthy_on_failure() {
        let mut monitor = HealthMonitor::with_defaults();
        let status = monitor.record_check("gnss", Duration::ZERO, false);
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_down_after_consecutive_failures() {
        let mut monitor = HealthMonitor::with_defaults();
        monitor.record_check("gnss", Duration::ZERO, false);
        monitor.record_check("gnss", Duration::ZERO, false);
        let status = monitor.record_check("gnss", Duration::ZERO, false);
        assert_eq!(status, HealthStatus::Down);
    }

    #[test]
    fn test_recovery_resets_consecutive_failures() {
        let mut monitor = HealthMonitor::with_defaults();
        monitor.record_check("gnss", Duration::ZERO, false);
        monitor.record_check("gnss", Duration::ZERO, false);
        // Recover before hitting 3 consecutive
        let status = monitor.record_check("gnss", Duration::from_millis(10), true);
        assert_eq!(status, HealthStatus::Healthy);
        // Next failure should not be Down (counter was reset)
        let status = monitor.record_check("gnss", Duration::ZERO, false);
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_overall_health() {
        let mut monitor = HealthMonitor::with_defaults();
        monitor.record_check("gnss", Duration::from_millis(10), true);
        monitor.record_check("imu", Duration::from_millis(600), true);
        assert_eq!(monitor.overall_health(), HealthStatus::Degraded);
    }

    #[test]
    fn test_overall_health_empty() {
        let monitor = HealthMonitor::with_defaults();
        assert_eq!(monitor.overall_health(), HealthStatus::Healthy);
    }

    #[test]
    fn test_failure_rate() {
        let mut monitor = HealthMonitor::with_defaults();
        monitor.record_check("gnss", Duration::from_millis(10), true);
        monitor.record_check("gnss", Duration::ZERO, false);
        let rate = monitor.failure_rate("gnss").unwrap();
        assert!((rate - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_by_status() {
        let mut monitor = HealthMonitor::with_defaults();
        monitor.record_check("gnss", Duration::from_millis(10), true);
        monitor.record_check("imu", Duration::ZERO, false);
        let healthy = monitor.by_status(HealthStatus::Healthy);
        assert_eq!(healthy.len(), 1);
        assert!(healthy.contains(&"gnss".to_string()));
    }

    #[test]
    fn test_summary() {
        let mut monitor = HealthMonitor::with_defaults();
        monitor.record_check("gnss", Duration::from_millis(10), true);
        monitor.record_check("imu", Duration::from_millis(600), true);
        monitor.record_check("lidar", Duration::ZERO, false);
        let (healthy, degraded, unhealthy, down) = monitor.summary();
        assert_eq!(healthy, 1);
        assert_eq!(degraded, 1);
        assert_eq!(unhealthy, 1);
        assert_eq!(down, 0);
    }

    #[test]
    fn test_status_ordering() {
        assert!(HealthStatus::Healthy < HealthStatus::Degraded);
        assert!(HealthStatus::Degraded < HealthStatus::Unhealthy);
        assert!(HealthStatus::Unhealthy < HealthStatus::Down);
    }
}
