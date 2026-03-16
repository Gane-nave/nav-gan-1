//! Circuit breaker — prevents cascading failures by cutting off unhealthy services.

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation — requests flow through.
    Closed,
    /// Tripped — all requests are rejected immediately.
    Open,
    /// Testing — allows a limited number of requests through to check recovery.
    HalfOpen,
}

/// Circuit breaker configuration.
#[derive(Debug, Clone)]
pub struct CircuitConfig {
    /// Number of failures before tripping open.
    pub failure_threshold: u32,
    /// Duration in millis to stay open before transitioning to half-open.
    pub open_duration_ms: u64,
    /// Number of test requests allowed in half-open state.
    pub half_open_max_requests: u32,
    /// Success threshold in half-open to close the circuit.
    pub recovery_threshold: u32,
}

impl Default for CircuitConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            open_duration_ms: 30_000,
            half_open_max_requests: 3,
            recovery_threshold: 2,
        }
    }
}

/// Circuit breaker instance.
pub struct CircuitBreaker {
    /// Service/endpoint name.
    name: String,
    /// Current state.
    state: CircuitState,
    /// Configuration.
    config: CircuitConfig,
    /// Consecutive failures in Closed state.
    failure_count: u32,
    /// Successes in HalfOpen state.
    half_open_successes: u32,
    /// Requests allowed through in HalfOpen state.
    half_open_requests: u32,
    /// Timestamp when circuit was opened (epoch millis).
    opened_at_ms: Option<u64>,
    /// Total times circuit has tripped.
    trip_count: u64,
    /// Total requests blocked by open circuit.
    blocked_count: u64,
    /// Total successful requests.
    total_successes: u64,
    /// Total failed requests.
    total_failures: u64,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    pub fn new(name: &str, config: CircuitConfig) -> Self {
        Self {
            name: name.to_string(),
            state: CircuitState::Closed,
            config,
            failure_count: 0,
            half_open_successes: 0,
            half_open_requests: 0,
            opened_at_ms: None,
            trip_count: 0,
            blocked_count: 0,
            total_successes: 0,
            total_failures: 0,
        }
    }

    /// Check if a request should be allowed through.
    pub fn allow_request(&mut self, now_ms: u64) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if open duration has elapsed
                if let Some(opened_at) = self.opened_at_ms {
                    if now_ms.saturating_sub(opened_at) >= self.config.open_duration_ms {
                        self.state = CircuitState::HalfOpen;
                        self.half_open_successes = 0;
                        self.half_open_requests = 0;
                        return self.allow_half_open();
                    }
                }
                self.blocked_count += 1;
                false
            }
            CircuitState::HalfOpen => self.allow_half_open(),
        }
    }

    fn allow_half_open(&mut self) -> bool {
        if self.half_open_requests < self.config.half_open_max_requests {
            self.half_open_requests += 1;
            true
        } else {
            self.blocked_count += 1;
            false
        }
    }

    /// Record a successful request.
    pub fn record_success(&mut self) {
        self.total_successes += 1;
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0; // reset consecutive failures
            }
            CircuitState::HalfOpen => {
                self.half_open_successes += 1;
                if self.half_open_successes >= self.config.recovery_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                }
            }
            CircuitState::Open => {} // shouldn't happen
        }
    }

    /// Record a failed request.
    pub fn record_failure(&mut self, now_ms: u64) {
        self.total_failures += 1;
        match self.state {
            CircuitState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.config.failure_threshold {
                    self.trip(now_ms);
                }
            }
            CircuitState::HalfOpen => {
                // Any failure in half-open trips back to open
                self.trip(now_ms);
            }
            CircuitState::Open => {} // shouldn't happen
        }
    }

    /// Trip the circuit breaker open.
    fn trip(&mut self, now_ms: u64) {
        self.state = CircuitState::Open;
        self.opened_at_ms = Some(now_ms);
        self.trip_count += 1;
    }

    /// Manually reset the circuit breaker to closed.
    pub fn reset(&mut self) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.half_open_successes = 0;
        self.half_open_requests = 0;
        self.opened_at_ms = None;
    }

    /// Get current state.
    pub fn state(&self) -> CircuitState {
        self.state
    }

    /// Get name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get trip count.
    pub fn trip_count(&self) -> u64 {
        self.trip_count
    }

    /// Get blocked count.
    pub fn blocked_count(&self) -> u64 {
        self.blocked_count
    }

    /// Get failure count (consecutive, in Closed state).
    pub fn failure_count(&self) -> u32 {
        self.failure_count
    }

    /// Get total successes.
    pub fn total_successes(&self) -> u64 {
        self.total_successes
    }

    /// Get total failures.
    pub fn total_failures(&self) -> u64 {
        self.total_failures
    }

    /// Get error rate.
    pub fn error_rate(&self) -> f64 {
        let total = self.total_successes + self.total_failures;
        if total == 0 {
            return 0.0;
        }
        self.total_failures as f64 / total as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_breaker() -> CircuitBreaker {
        CircuitBreaker::new(
            "test",
            CircuitConfig {
                failure_threshold: 3,
                open_duration_ms: 5000,
                half_open_max_requests: 2,
                recovery_threshold: 2,
            },
        )
    }

    #[test]
    fn test_closed_allows_requests() {
        let mut cb = default_breaker();
        assert!(cb.allow_request(1000));
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_trips_after_threshold_failures() {
        let mut cb = default_breaker();
        cb.allow_request(1000);
        cb.record_failure(1000);
        cb.allow_request(1000);
        cb.record_failure(1000);
        assert_eq!(cb.state(), CircuitState::Closed);
        cb.allow_request(1000);
        cb.record_failure(1000); // 3rd failure → trips
        assert_eq!(cb.state(), CircuitState::Open);
        assert_eq!(cb.trip_count(), 1);
    }

    #[test]
    fn test_open_blocks_requests() {
        let mut cb = default_breaker();
        // Trip it
        for _ in 0..3 {
            cb.allow_request(1000);
            cb.record_failure(1000);
        }
        assert!(!cb.allow_request(2000)); // blocked
        assert_eq!(cb.blocked_count(), 1);
    }

    #[test]
    fn test_open_transitions_to_half_open() {
        let mut cb = default_breaker();
        for _ in 0..3 {
            cb.allow_request(1000);
            cb.record_failure(1000);
        }
        // After open_duration_ms (5000), should transition to HalfOpen
        assert!(cb.allow_request(6000));
        assert_eq!(cb.state(), CircuitState::HalfOpen);
    }

    #[test]
    fn test_half_open_recovers_on_successes() {
        let mut cb = default_breaker();
        for _ in 0..3 {
            cb.allow_request(1000);
            cb.record_failure(1000);
        }
        // Transition to HalfOpen
        assert!(cb.allow_request(6000));
        cb.record_success();
        assert!(cb.allow_request(6000));
        cb.record_success(); // 2nd success → recovery threshold met
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_half_open_trips_on_failure() {
        let mut cb = default_breaker();
        for _ in 0..3 {
            cb.allow_request(1000);
            cb.record_failure(1000);
        }
        // Transition to HalfOpen
        cb.allow_request(6000);
        cb.record_failure(6000); // failure in half-open → trips again
        assert_eq!(cb.state(), CircuitState::Open);
        assert_eq!(cb.trip_count(), 2);
    }

    #[test]
    fn test_half_open_limits_requests() {
        let mut cb = default_breaker();
        for _ in 0..3 {
            cb.allow_request(1000);
            cb.record_failure(1000);
        }
        // Transition to HalfOpen
        assert!(cb.allow_request(6000)); // 1st test request
        assert!(cb.allow_request(6000)); // 2nd test request
        assert!(!cb.allow_request(6000)); // blocked: max_requests exceeded
    }

    #[test]
    fn test_success_resets_failure_count() {
        let mut cb = default_breaker();
        cb.allow_request(1000);
        cb.record_failure(1000);
        cb.allow_request(1000);
        cb.record_failure(1000);
        assert_eq!(cb.failure_count(), 2);
        cb.allow_request(1000);
        cb.record_success(); // resets
        assert_eq!(cb.failure_count(), 0);
    }

    #[test]
    fn test_manual_reset() {
        let mut cb = default_breaker();
        for _ in 0..3 {
            cb.allow_request(1000);
            cb.record_failure(1000);
        }
        assert_eq!(cb.state(), CircuitState::Open);
        cb.reset();
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.allow_request(1000));
    }

    #[test]
    fn test_error_rate() {
        let mut cb = default_breaker();
        cb.allow_request(1000);
        cb.record_success();
        cb.allow_request(1000);
        cb.record_success();
        cb.allow_request(1000);
        cb.record_failure(1000);
        // 1 failure, 2 successes → 1/3 ≈ 0.333
        assert!((cb.error_rate() - 1.0 / 3.0).abs() < 0.01);
    }
}
