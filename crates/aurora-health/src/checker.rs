//! Health checker — tracks the health status of individual components.

/// Health status of a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Component is healthy and fully operational.
    Healthy,
    /// Component is degraded but still functional.
    Degraded,
    /// Component is unhealthy and not operational.
    Unhealthy,
    /// Component status is unknown (not yet checked).
    Unknown,
}

impl HealthStatus {
    /// Severity level (lower = better).
    pub fn severity(&self) -> u8 {
        match self {
            HealthStatus::Healthy => 0,
            HealthStatus::Unknown => 1,
            HealthStatus::Degraded => 2,
            HealthStatus::Unhealthy => 3,
        }
    }

    /// Whether the component is considered operational.
    pub fn is_operational(&self) -> bool {
        matches!(self, HealthStatus::Healthy | HealthStatus::Degraded)
    }

    /// Short display string.
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unhealthy => "unhealthy",
            HealthStatus::Unknown => "unknown",
        }
    }
}

/// Details about a component's health check.
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Component name.
    pub component: String,
    /// Current status.
    pub status: HealthStatus,
    /// Human-readable message.
    pub message: String,
    /// Timestamp of the check (epoch ms).
    pub checked_at_ms: u64,
    /// Duration of the check in microseconds.
    pub duration_us: u64,
    /// Optional metadata key-value pairs.
    pub metadata: Vec<(String, String)>,
}

impl HealthCheckResult {
    /// Create a healthy result.
    pub fn healthy(component: &str, checked_at_ms: u64) -> Self {
        Self {
            component: component.to_string(),
            status: HealthStatus::Healthy,
            message: "OK".to_string(),
            checked_at_ms,
            duration_us: 0,
            metadata: Vec::new(),
        }
    }

    /// Create a degraded result.
    pub fn degraded(component: &str, message: &str, checked_at_ms: u64) -> Self {
        Self {
            component: component.to_string(),
            status: HealthStatus::Degraded,
            message: message.to_string(),
            checked_at_ms,
            duration_us: 0,
            metadata: Vec::new(),
        }
    }

    /// Create an unhealthy result.
    pub fn unhealthy(component: &str, message: &str, checked_at_ms: u64) -> Self {
        Self {
            component: component.to_string(),
            status: HealthStatus::Unhealthy,
            message: message.to_string(),
            checked_at_ms,
            duration_us: 0,
            metadata: Vec::new(),
        }
    }

    /// Set check duration.
    pub fn with_duration(mut self, duration_us: u64) -> Self {
        self.duration_us = duration_us;
        self
    }

    /// Add a metadata entry.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.push((key.to_string(), value.to_string()));
        self
    }

    /// Whether the check is stale (older than max_age_ms).
    pub fn is_stale(&self, now_ms: u64, max_age_ms: u64) -> bool {
        now_ms.saturating_sub(self.checked_at_ms) > max_age_ms
    }
}

/// Tracks health check history for a component.
pub struct HealthHistory {
    component: String,
    results: Vec<HealthCheckResult>,
    max_history: usize,
    consecutive_failures: u32,
    consecutive_successes: u32,
}

impl HealthHistory {
    /// Create a new health history tracker.
    pub fn new(component: &str, max_history: usize) -> Self {
        Self {
            component: component.to_string(),
            results: Vec::new(),
            max_history,
            consecutive_failures: 0,
            consecutive_successes: 0,
        }
    }

    /// Record a health check result.
    pub fn record(&mut self, result: HealthCheckResult) {
        if result.status.is_operational() {
            self.consecutive_successes += 1;
            self.consecutive_failures = 0;
        } else {
            self.consecutive_failures += 1;
            self.consecutive_successes = 0;
        }

        self.results.push(result);
        if self.results.len() > self.max_history {
            self.results.remove(0);
        }
    }

    /// Latest health check result.
    pub fn latest(&self) -> Option<&HealthCheckResult> {
        self.results.last()
    }

    /// Current status (from latest check).
    pub fn current_status(&self) -> HealthStatus {
        self.latest()
            .map(|r| r.status)
            .unwrap_or(HealthStatus::Unknown)
    }

    /// Component name.
    pub fn component(&self) -> &str {
        &self.component
    }

    /// Number of consecutive failures.
    pub fn consecutive_failures(&self) -> u32 {
        self.consecutive_failures
    }

    /// Number of consecutive successes.
    pub fn consecutive_successes(&self) -> u32 {
        self.consecutive_successes
    }

    /// Availability rate over the history window.
    pub fn availability(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        let healthy = self
            .results
            .iter()
            .filter(|r| r.status.is_operational())
            .count();
        healthy as f64 / self.results.len() as f64
    }

    /// Average check duration in microseconds.
    pub fn avg_duration_us(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.results.iter().map(|r| r.duration_us).sum();
        sum as f64 / self.results.len() as f64
    }

    /// History length.
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Whether history is empty.
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_severity() {
        assert!(HealthStatus::Healthy.severity() < HealthStatus::Degraded.severity());
        assert!(HealthStatus::Degraded.severity() < HealthStatus::Unhealthy.severity());
    }

    #[test]
    fn test_status_operational() {
        assert!(HealthStatus::Healthy.is_operational());
        assert!(HealthStatus::Degraded.is_operational());
        assert!(!HealthStatus::Unhealthy.is_operational());
        assert!(!HealthStatus::Unknown.is_operational());
    }

    #[test]
    fn test_healthy_result() {
        let r = HealthCheckResult::healthy("gnss", 1000)
            .with_duration(500)
            .with_metadata("satellites", "12");
        assert_eq!(r.status, HealthStatus::Healthy);
        assert_eq!(r.duration_us, 500);
        assert_eq!(r.metadata.len(), 1);
    }

    #[test]
    fn test_stale_check() {
        let r = HealthCheckResult::healthy("gnss", 1000);
        assert!(!r.is_stale(2000, 5000));
        assert!(r.is_stale(7000, 5000));
    }

    #[test]
    fn test_history_records() {
        let mut h = HealthHistory::new("gnss", 5);
        h.record(HealthCheckResult::healthy("gnss", 1000));
        h.record(HealthCheckResult::healthy("gnss", 2000));
        assert_eq!(h.len(), 2);
        assert_eq!(h.current_status(), HealthStatus::Healthy);
    }

    #[test]
    fn test_history_max_capacity() {
        let mut h = HealthHistory::new("gnss", 3);
        h.record(HealthCheckResult::healthy("gnss", 1000));
        h.record(HealthCheckResult::healthy("gnss", 2000));
        h.record(HealthCheckResult::healthy("gnss", 3000));
        h.record(HealthCheckResult::healthy("gnss", 4000));
        assert_eq!(h.len(), 3);
        assert_eq!(h.latest().unwrap().checked_at_ms, 4000);
    }

    #[test]
    fn test_consecutive_failures() {
        let mut h = HealthHistory::new("db", 10);
        h.record(HealthCheckResult::healthy("db", 1000));
        h.record(HealthCheckResult::unhealthy("db", "timeout", 2000));
        h.record(HealthCheckResult::unhealthy("db", "timeout", 3000));
        assert_eq!(h.consecutive_failures(), 2);
        assert_eq!(h.consecutive_successes(), 0);
    }

    #[test]
    fn test_availability() {
        let mut h = HealthHistory::new("api", 10);
        h.record(HealthCheckResult::healthy("api", 1000));
        h.record(HealthCheckResult::healthy("api", 2000));
        h.record(HealthCheckResult::unhealthy("api", "err", 3000));
        h.record(HealthCheckResult::healthy("api", 4000));
        assert!((h.availability() - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_avg_duration() {
        let mut h = HealthHistory::new("api", 10);
        h.record(HealthCheckResult::healthy("api", 1000).with_duration(100));
        h.record(HealthCheckResult::healthy("api", 2000).with_duration(200));
        h.record(HealthCheckResult::healthy("api", 3000).with_duration(300));
        assert!((h.avg_duration_us() - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_success_resets_failures() {
        let mut h = HealthHistory::new("db", 10);
        h.record(HealthCheckResult::unhealthy("db", "err", 1000));
        h.record(HealthCheckResult::unhealthy("db", "err", 2000));
        h.record(HealthCheckResult::healthy("db", 3000));
        assert_eq!(h.consecutive_failures(), 0);
        assert_eq!(h.consecutive_successes(), 1);
    }
}
