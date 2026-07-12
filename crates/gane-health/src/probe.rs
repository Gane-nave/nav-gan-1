//! Probes — readiness and liveness probe definitions.

use crate::checker::{HealthCheckResult, HealthStatus};

/// Type of probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeType {
    /// Readiness: Is the component ready to accept traffic?
    Readiness,
    /// Liveness: Is the component alive and should not be restarted?
    Liveness,
    /// Startup: Has the component finished starting up?
    Startup,
}

impl ProbeType {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            ProbeType::Readiness => "readiness",
            ProbeType::Liveness => "liveness",
            ProbeType::Startup => "startup",
        }
    }
}

/// Configuration for a probe.
#[derive(Debug, Clone)]
pub struct ProbeConfig {
    /// Type of probe.
    pub probe_type: ProbeType,
    /// Interval between checks in milliseconds.
    pub interval_ms: u64,
    /// Timeout for each check in milliseconds.
    pub timeout_ms: u64,
    /// Number of consecutive successes before marking healthy.
    pub success_threshold: u32,
    /// Number of consecutive failures before marking unhealthy.
    pub failure_threshold: u32,
    /// Initial delay before first check in milliseconds.
    pub initial_delay_ms: u64,
}

impl ProbeConfig {
    /// Create a readiness probe config with defaults.
    pub fn readiness() -> Self {
        Self {
            probe_type: ProbeType::Readiness,
            interval_ms: 5000,
            timeout_ms: 3000,
            success_threshold: 1,
            failure_threshold: 3,
            initial_delay_ms: 0,
        }
    }

    /// Create a liveness probe config with defaults.
    pub fn liveness() -> Self {
        Self {
            probe_type: ProbeType::Liveness,
            interval_ms: 10000,
            timeout_ms: 5000,
            success_threshold: 1,
            failure_threshold: 5,
            initial_delay_ms: 15000,
        }
    }

    /// Create a startup probe config with defaults.
    pub fn startup() -> Self {
        Self {
            probe_type: ProbeType::Startup,
            interval_ms: 2000,
            timeout_ms: 10000,
            success_threshold: 1,
            failure_threshold: 30,
            initial_delay_ms: 0,
        }
    }

    /// Set check interval.
    pub fn with_interval(mut self, interval_ms: u64) -> Self {
        self.interval_ms = interval_ms;
        self
    }

    /// Set timeout.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Set failure threshold.
    pub fn with_failure_threshold(mut self, threshold: u32) -> Self {
        self.failure_threshold = threshold;
        self
    }
}

/// Probe state machine — tracks whether a probe considers the component healthy.
pub struct ProbeState {
    config: ProbeConfig,
    status: HealthStatus,
    consecutive_successes: u32,
    consecutive_failures: u32,
    last_check_ms: u64,
    total_checks: u64,
    started: bool,
}

impl ProbeState {
    /// Create a new probe state.
    pub fn new(config: ProbeConfig) -> Self {
        Self {
            config,
            status: HealthStatus::Unknown,
            consecutive_successes: 0,
            consecutive_failures: 0,
            last_check_ms: 0,
            total_checks: 0,
            started: false,
        }
    }

    /// Record a health check result and update state.
    pub fn record(&mut self, result: &HealthCheckResult) {
        self.total_checks += 1;
        self.last_check_ms = result.checked_at_ms;
        self.started = true;

        if result.status.is_operational() {
            self.consecutive_successes += 1;
            self.consecutive_failures = 0;

            if self.consecutive_successes >= self.config.success_threshold {
                self.status = HealthStatus::Healthy;
            }
        } else {
            self.consecutive_failures += 1;
            self.consecutive_successes = 0;

            if self.consecutive_failures >= self.config.failure_threshold {
                self.status = HealthStatus::Unhealthy;
            } else if self.status == HealthStatus::Healthy {
                self.status = HealthStatus::Degraded;
            }
        }
    }

    /// Whether a check is due based on interval.
    pub fn is_check_due(&self, now_ms: u64) -> bool {
        if !self.started {
            return now_ms >= self.config.initial_delay_ms;
        }
        now_ms.saturating_sub(self.last_check_ms) >= self.config.interval_ms
    }

    /// Current status.
    pub fn status(&self) -> HealthStatus {
        self.status
    }

    /// Probe type.
    pub fn probe_type(&self) -> ProbeType {
        self.config.probe_type
    }

    /// Total number of checks performed.
    pub fn total_checks(&self) -> u64 {
        self.total_checks
    }

    /// Whether the probe has started.
    pub fn has_started(&self) -> bool {
        self.started
    }

    /// The probe configuration.
    pub fn config(&self) -> &ProbeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::HealthCheckResult;

    #[test]
    fn test_probe_config_defaults() {
        let r = ProbeConfig::readiness();
        assert_eq!(r.probe_type, ProbeType::Readiness);
        assert_eq!(r.interval_ms, 5000);
        assert_eq!(r.failure_threshold, 3);

        let l = ProbeConfig::liveness();
        assert_eq!(l.probe_type, ProbeType::Liveness);
        assert_eq!(l.initial_delay_ms, 15000);
    }

    #[test]
    fn test_probe_config_builder() {
        let c = ProbeConfig::readiness()
            .with_interval(1000)
            .with_timeout(500)
            .with_failure_threshold(5);
        assert_eq!(c.interval_ms, 1000);
        assert_eq!(c.timeout_ms, 500);
        assert_eq!(c.failure_threshold, 5);
    }

    #[test]
    fn test_probe_initial_unknown() {
        let state = ProbeState::new(ProbeConfig::readiness());
        assert_eq!(state.status(), HealthStatus::Unknown);
        assert!(!state.has_started());
    }

    #[test]
    fn test_probe_becomes_healthy() {
        let mut state = ProbeState::new(ProbeConfig::readiness());
        state.record(&HealthCheckResult::healthy("gnss", 1000));
        assert_eq!(state.status(), HealthStatus::Healthy);
        assert!(state.has_started());
    }

    #[test]
    fn test_probe_failure_threshold() {
        let config = ProbeConfig::readiness().with_failure_threshold(3);
        let mut state = ProbeState::new(config);

        state.record(&HealthCheckResult::healthy("gnss", 1000));
        assert_eq!(state.status(), HealthStatus::Healthy);

        state.record(&HealthCheckResult::unhealthy("gnss", "err", 2000));
        assert_eq!(state.status(), HealthStatus::Degraded);

        state.record(&HealthCheckResult::unhealthy("gnss", "err", 3000));
        assert_eq!(state.status(), HealthStatus::Degraded);

        state.record(&HealthCheckResult::unhealthy("gnss", "err", 4000));
        assert_eq!(state.status(), HealthStatus::Unhealthy);
    }

    #[test]
    fn test_probe_recovery() {
        let config = ProbeConfig::readiness().with_failure_threshold(2);
        let mut state = ProbeState::new(config);

        state.record(&HealthCheckResult::unhealthy("gnss", "err", 1000));
        state.record(&HealthCheckResult::unhealthy("gnss", "err", 2000));
        assert_eq!(state.status(), HealthStatus::Unhealthy);

        state.record(&HealthCheckResult::healthy("gnss", 3000));
        assert_eq!(state.status(), HealthStatus::Healthy);
    }

    #[test]
    fn test_check_due_initial_delay() {
        let config = ProbeConfig::liveness(); // initial_delay_ms = 15000
        let state = ProbeState::new(config);

        assert!(!state.is_check_due(10000));
        assert!(state.is_check_due(15000));
    }

    #[test]
    fn test_check_due_interval() {
        let config = ProbeConfig::readiness().with_interval(5000);
        let mut state = ProbeState::new(config);
        state.record(&HealthCheckResult::healthy("gnss", 1000));

        assert!(!state.is_check_due(3000));
        assert!(state.is_check_due(6000));
    }

    #[test]
    fn test_total_checks() {
        let mut state = ProbeState::new(ProbeConfig::readiness());
        state.record(&HealthCheckResult::healthy("gnss", 1000));
        state.record(&HealthCheckResult::healthy("gnss", 2000));
        state.record(&HealthCheckResult::unhealthy("gnss", "x", 3000));
        assert_eq!(state.total_checks(), 3);
    }

    #[test]
    fn test_probe_type_display() {
        assert_eq!(ProbeType::Readiness.as_str(), "readiness");
        assert_eq!(ProbeType::Liveness.as_str(), "liveness");
        assert_eq!(ProbeType::Startup.as_str(), "startup");
    }
}
