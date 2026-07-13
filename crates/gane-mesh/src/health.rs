//! Health checking — periodic probing and aggregate health scoring.

use std::collections::HashMap;

/// Health check result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is healthy.
    Healthy,
    /// Service is partially healthy.
    Degraded,
    /// Service is unhealthy.
    Unhealthy,
    /// Health check timed out.
    Timeout,
}

/// A single health check result.
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Service/instance identifier.
    pub target_id: String,
    /// Health status.
    pub status: HealthStatus,
    /// Response time in milliseconds.
    pub response_ms: u64,
    /// Timestamp of the check (epoch millis).
    pub checked_at_ms: u64,
    /// Optional message.
    pub message: Option<String>,
}

/// Health checker configuration.
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Check interval in milliseconds.
    pub interval_ms: u64,
    /// Timeout for each check in milliseconds.
    pub timeout_ms: u64,
    /// Number of consecutive failures before marking unhealthy.
    pub unhealthy_threshold: u32,
    /// Number of consecutive successes before marking healthy.
    pub healthy_threshold: u32,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_ms: 10_000,
            timeout_ms: 5_000,
            unhealthy_threshold: 3,
            healthy_threshold: 2,
        }
    }
}

/// Per-target health state.
#[derive(Debug, Clone)]
struct TargetHealthState {
    /// Current health status.
    status: HealthStatus,
    /// Consecutive successes.
    consecutive_successes: u32,
    /// Consecutive failures.
    consecutive_failures: u32,
    /// Total checks performed.
    total_checks: u64,
    /// Total failures.
    total_failures: u64,
    /// Last check timestamp.
    last_check_ms: u64,
    /// Average response time (running average).
    avg_response_ms: f64,
}

/// Health checker — monitors multiple targets.
pub struct HealthChecker {
    config: HealthCheckConfig,
    targets: HashMap<String, TargetHealthState>,
}

impl HealthChecker {
    /// Create a new health checker.
    pub fn new(config: HealthCheckConfig) -> Self {
        Self {
            config,
            targets: HashMap::new(),
        }
    }

    /// Register a target to monitor.
    pub fn register_target(&mut self, target_id: &str) {
        self.targets.insert(
            target_id.to_string(),
            TargetHealthState {
                status: HealthStatus::Healthy,
                consecutive_successes: 0,
                consecutive_failures: 0,
                total_checks: 0,
                total_failures: 0,
                last_check_ms: 0,
                avg_response_ms: 0.0,
            },
        );
    }

    /// Unregister a target.
    pub fn unregister_target(&mut self, target_id: &str) -> bool {
        self.targets.remove(target_id).is_some()
    }

    /// Record a health check result.
    pub fn record_check(&mut self, result: HealthCheckResult) {
        let Some(state) = self.targets.get_mut(&result.target_id) else {
            return;
        };

        state.total_checks += 1;
        state.last_check_ms = result.checked_at_ms;

        // Update running average response time
        let n = state.total_checks as f64;
        state.avg_response_ms =
            state.avg_response_ms * ((n - 1.0) / n) + result.response_ms as f64 / n;

        match result.status {
            HealthStatus::Healthy => {
                state.consecutive_successes += 1;
                state.consecutive_failures = 0;
                if state.consecutive_successes >= self.config.healthy_threshold {
                    state.status = HealthStatus::Healthy;
                }
            }
            HealthStatus::Degraded => {
                state.consecutive_successes = 0;
                state.consecutive_failures = 0;
                state.status = HealthStatus::Degraded;
            }
            HealthStatus::Unhealthy | HealthStatus::Timeout => {
                state.consecutive_failures += 1;
                state.consecutive_successes = 0;
                state.total_failures += 1;
                if state.consecutive_failures >= self.config.unhealthy_threshold {
                    state.status = HealthStatus::Unhealthy;
                }
            }
        }
    }

    /// Get health status of a target.
    pub fn status(&self, target_id: &str) -> Option<HealthStatus> {
        self.targets.get(target_id).map(|s| s.status)
    }

    /// Get average response time for a target.
    pub fn avg_response_ms(&self, target_id: &str) -> Option<f64> {
        self.targets.get(target_id).map(|s| s.avg_response_ms)
    }

    /// Get all healthy targets.
    pub fn healthy_targets(&self) -> Vec<&str> {
        let mut targets: Vec<&str> = self
            .targets
            .iter()
            .filter(|(_, s)| s.status == HealthStatus::Healthy)
            .map(|(id, _)| id.as_str())
            .collect();
        targets.sort();
        targets
    }

    /// Get all unhealthy targets.
    pub fn unhealthy_targets(&self) -> Vec<&str> {
        let mut targets: Vec<&str> = self
            .targets
            .iter()
            .filter(|(_, s)| s.status == HealthStatus::Unhealthy)
            .map(|(id, _)| id.as_str())
            .collect();
        targets.sort();
        targets
    }

    /// Get target count.
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Get targets due for a check (last check + interval < now).
    pub fn targets_due(&self, now_ms: u64) -> Vec<String> {
        self.targets
            .iter()
            .filter(|(_, s)| now_ms.saturating_sub(s.last_check_ms) >= self.config.interval_ms)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Compute aggregate health score (0.0 to 1.0).
    pub fn aggregate_health_score(&self) -> f64 {
        if self.targets.is_empty() {
            return 1.0;
        }
        let healthy = self
            .targets
            .values()
            .filter(|s| s.status == HealthStatus::Healthy)
            .count();
        healthy as f64 / self.targets.len() as f64
    }

    /// Get failure rate for a target.
    pub fn failure_rate(&self, target_id: &str) -> Option<f64> {
        self.targets.get(target_id).map(|s| {
            if s.total_checks == 0 {
                0.0
            } else {
                s.total_failures as f64 / s.total_checks as f64
            }
        })
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new(HealthCheckConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_check(
        target: &str,
        status: HealthStatus,
        response_ms: u64,
        ts: u64,
    ) -> HealthCheckResult {
        HealthCheckResult {
            target_id: target.to_string(),
            status,
            response_ms,
            checked_at_ms: ts,
            message: None,
        }
    }

    #[test]
    fn test_register_and_check() {
        let mut hc = HealthChecker::new(HealthCheckConfig::default());
        hc.register_target("svc-a");
        assert_eq!(hc.status("svc-a"), Some(HealthStatus::Healthy));
        assert_eq!(hc.target_count(), 1);
    }

    #[test]
    fn test_unhealthy_after_threshold_failures() {
        let mut hc = HealthChecker::new(HealthCheckConfig {
            unhealthy_threshold: 3,
            ..Default::default()
        });
        hc.register_target("svc-a");

        hc.record_check(make_check("svc-a", HealthStatus::Unhealthy, 100, 1000));
        hc.record_check(make_check("svc-a", HealthStatus::Unhealthy, 100, 2000));
        assert_eq!(hc.status("svc-a"), Some(HealthStatus::Healthy)); // not yet

        hc.record_check(make_check("svc-a", HealthStatus::Unhealthy, 100, 3000));
        assert_eq!(hc.status("svc-a"), Some(HealthStatus::Unhealthy)); // now
    }

    #[test]
    fn test_recovery_after_threshold_successes() {
        let mut hc = HealthChecker::new(HealthCheckConfig {
            unhealthy_threshold: 2,
            healthy_threshold: 2,
            ..Default::default()
        });
        hc.register_target("svc-a");

        // Make unhealthy
        hc.record_check(make_check("svc-a", HealthStatus::Unhealthy, 100, 1000));
        hc.record_check(make_check("svc-a", HealthStatus::Unhealthy, 100, 2000));
        assert_eq!(hc.status("svc-a"), Some(HealthStatus::Unhealthy));

        // Recover
        hc.record_check(make_check("svc-a", HealthStatus::Healthy, 50, 3000));
        assert_eq!(hc.status("svc-a"), Some(HealthStatus::Unhealthy)); // not yet

        hc.record_check(make_check("svc-a", HealthStatus::Healthy, 50, 4000));
        assert_eq!(hc.status("svc-a"), Some(HealthStatus::Healthy)); // recovered
    }

    #[test]
    fn test_degraded_status() {
        let mut hc = HealthChecker::new(HealthCheckConfig::default());
        hc.register_target("svc-a");
        hc.record_check(make_check("svc-a", HealthStatus::Degraded, 200, 1000));
        assert_eq!(hc.status("svc-a"), Some(HealthStatus::Degraded));
    }

    #[test]
    fn test_aggregate_health_score() {
        let mut hc = HealthChecker::new(HealthCheckConfig {
            unhealthy_threshold: 1,
            ..Default::default()
        });
        hc.register_target("svc-a");
        hc.register_target("svc-b");
        hc.register_target("svc-c");
        hc.register_target("svc-d");

        hc.record_check(make_check("svc-a", HealthStatus::Unhealthy, 100, 1000));
        // 3 healthy, 1 unhealthy → 0.75
        assert!((hc.aggregate_health_score() - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_targets_due() {
        let mut hc = HealthChecker::new(HealthCheckConfig {
            interval_ms: 5000,
            ..Default::default()
        });
        hc.register_target("svc-a");
        hc.register_target("svc-b");

        hc.record_check(make_check("svc-a", HealthStatus::Healthy, 50, 1000));
        // At 5000: svc-a last check at 1000, interval=5000 → 5000-1000=4000 < 5000 → not due
        // svc-b last check at 0, interval=5000 → 5000-0=5000 >= 5000 → due
        let due = hc.targets_due(5000);
        assert_eq!(due.len(), 1);
        assert!(due.contains(&"svc-b".to_string()));
    }

    #[test]
    fn test_avg_response_time() {
        let mut hc = HealthChecker::new(HealthCheckConfig::default());
        hc.register_target("svc-a");
        hc.record_check(make_check("svc-a", HealthStatus::Healthy, 100, 1000));
        hc.record_check(make_check("svc-a", HealthStatus::Healthy, 200, 2000));
        // avg = (100 + 200) / 2 = 150
        assert!((hc.avg_response_ms("svc-a").unwrap() - 150.0).abs() < 1.0);
    }

    #[test]
    fn test_failure_rate() {
        let mut hc = HealthChecker::new(HealthCheckConfig::default());
        hc.register_target("svc-a");
        hc.record_check(make_check("svc-a", HealthStatus::Healthy, 50, 1000));
        hc.record_check(make_check("svc-a", HealthStatus::Healthy, 50, 2000));
        hc.record_check(make_check("svc-a", HealthStatus::Unhealthy, 50, 3000));
        hc.record_check(make_check("svc-a", HealthStatus::Timeout, 50, 4000));
        // 2 failures out of 4 → 0.5
        assert!((hc.failure_rate("svc-a").unwrap() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_unregister_target() {
        let mut hc = HealthChecker::new(HealthCheckConfig::default());
        hc.register_target("svc-a");
        assert!(hc.unregister_target("svc-a"));
        assert_eq!(hc.target_count(), 0);
        assert!(!hc.unregister_target("svc-a")); // already removed
    }

    #[test]
    fn test_healthy_and_unhealthy_targets() {
        let mut hc = HealthChecker::new(HealthCheckConfig {
            unhealthy_threshold: 1,
            ..Default::default()
        });
        hc.register_target("svc-a");
        hc.register_target("svc-b");
        hc.record_check(make_check("svc-b", HealthStatus::Unhealthy, 100, 1000));

        assert_eq!(hc.healthy_targets(), vec!["svc-a"]);
        assert_eq!(hc.unhealthy_targets(), vec!["svc-b"]);
    }
}
