//! Circuit breaker state machine — prevents cascading failures by tracking error rates.

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Normal operation — requests pass through.
    Closed,
    /// Failure threshold exceeded — requests are rejected.
    Open,
    /// Testing recovery — limited requests allowed.
    HalfOpen,
}

/// Configuration for a circuit breaker.
#[derive(Debug, Clone)]
pub struct BreakerConfig {
    /// Number of failures before opening the circuit.
    pub failure_threshold: u32,
    /// Number of successes in half-open state before closing.
    pub success_threshold: u32,
    /// Duration in milliseconds the circuit stays open before transitioning to half-open.
    pub open_duration_ms: u64,
    /// Maximum number of requests allowed in half-open state.
    pub half_open_max_requests: u32,
}

impl Default for BreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            open_duration_ms: 30_000,
            half_open_max_requests: 1,
        }
    }
}

/// A circuit breaker instance.
#[derive(Debug)]
pub struct CircuitBreaker {
    config: BreakerConfig,
    state: State,
    failure_count: u32,
    success_count: u32,
    half_open_requests: u32,
    last_failure_ms: u64,
    total_requests: u64,
    total_failures: u64,
    total_rejections: u64,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with the given configuration.
    pub fn new(config: BreakerConfig) -> Self {
        Self {
            config,
            state: State::Closed,
            failure_count: 0,
            success_count: 0,
            half_open_requests: 0,
            last_failure_ms: 0,
            total_requests: 0,
            total_failures: 0,
            total_rejections: 0,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(BreakerConfig::default())
    }

    /// Current state of the circuit breaker.
    pub fn state(&self) -> State {
        self.state
    }

    /// Check if a request is allowed at the given timestamp.
    /// Returns true if the request should proceed, false if rejected.
    pub fn allow_request(&mut self, now_ms: u64) -> bool {
        self.total_requests += 1;

        match self.state {
            State::Closed => true,
            State::Open => {
                if now_ms.saturating_sub(self.last_failure_ms) >= self.config.open_duration_ms {
                    self.state = State::HalfOpen;
                    self.half_open_requests = 0;
                    self.success_count = 0;
                    if self.half_open_requests < self.config.half_open_max_requests {
                        self.half_open_requests += 1;
                        true
                    } else {
                        self.total_rejections += 1;
                        false
                    }
                } else {
                    self.total_rejections += 1;
                    false
                }
            }
            State::HalfOpen => {
                if self.half_open_requests < self.config.half_open_max_requests {
                    self.half_open_requests += 1;
                    true
                } else {
                    self.total_rejections += 1;
                    false
                }
            }
        }
    }

    /// Record a successful request.
    pub fn record_success(&mut self) {
        match self.state {
            State::Closed => {
                self.failure_count = 0;
            }
            State::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = State::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            State::Open => {}
        }
    }

    /// Record a failed request.
    pub fn record_failure(&mut self, now_ms: u64) {
        self.total_failures += 1;
        self.last_failure_ms = now_ms;

        match self.state {
            State::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.config.failure_threshold {
                    self.state = State::Open;
                }
            }
            State::HalfOpen => {
                self.state = State::Open;
                self.failure_count = 0;
                self.success_count = 0;
            }
            State::Open => {}
        }
    }

    /// Manually reset the circuit breaker to closed state.
    pub fn reset(&mut self) {
        self.state = State::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.half_open_requests = 0;
    }

    /// Manually trip the circuit breaker to open state.
    pub fn trip(&mut self, now_ms: u64) {
        self.state = State::Open;
        self.last_failure_ms = now_ms;
    }

    /// Get statistics.
    pub fn stats(&self) -> BreakerStats {
        BreakerStats {
            state: self.state,
            failure_count: self.failure_count,
            total_requests: self.total_requests,
            total_failures: self.total_failures,
            total_rejections: self.total_rejections,
        }
    }
}

/// Circuit breaker statistics.
#[derive(Debug, Clone)]
pub struct BreakerStats {
    pub state: State,
    pub failure_count: u32,
    pub total_requests: u64,
    pub total_failures: u64,
    pub total_rejections: u64,
}

impl BreakerStats {
    /// Error rate as a fraction (0.0 to 1.0).
    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_failures as f64 / self.total_requests as f64
    }

    /// Rejection rate as a fraction.
    pub fn rejection_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_rejections as f64 / self.total_requests as f64
    }
}

/// Manages multiple named circuit breakers.
pub struct BreakerRegistry {
    breakers: std::collections::HashMap<String, CircuitBreaker>,
    default_config: BreakerConfig,
}

impl BreakerRegistry {
    /// Create a new registry with a default config for new breakers.
    pub fn new(default_config: BreakerConfig) -> Self {
        Self {
            breakers: std::collections::HashMap::new(),
            default_config,
        }
    }

    /// Get or create a circuit breaker by name.
    pub fn get_or_create(&mut self, name: &str) -> &mut CircuitBreaker {
        let config = self.default_config.clone();
        self.breakers
            .entry(name.to_string())
            .or_insert_with(|| CircuitBreaker::new(config))
    }

    /// Get a circuit breaker by name (read-only).
    pub fn get(&self, name: &str) -> Option<&CircuitBreaker> {
        self.breakers.get(name)
    }

    /// Remove a circuit breaker.
    pub fn remove(&mut self, name: &str) -> bool {
        self.breakers.remove(name).is_some()
    }

    /// List all breaker names.
    pub fn names(&self) -> Vec<&str> {
        self.breakers.keys().map(|s| s.as_str()).collect()
    }

    /// Reset all breakers.
    pub fn reset_all(&mut self) {
        for breaker in self.breakers.values_mut() {
            breaker.reset();
        }
    }

    /// Get all breakers in open state.
    pub fn open_breakers(&self) -> Vec<(&str, &CircuitBreaker)> {
        self.breakers
            .iter()
            .filter(|(_, b)| b.state() == State::Open)
            .map(|(n, b)| (n.as_str(), b))
            .collect()
    }

    /// Number of breakers.
    pub fn count(&self) -> usize {
        self.breakers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_closed() {
        let cb = CircuitBreaker::with_defaults();
        assert_eq!(cb.state(), State::Closed);
    }

    #[test]
    fn test_closed_allows_requests() {
        let mut cb = CircuitBreaker::with_defaults();
        assert!(cb.allow_request(1000));
        assert!(cb.allow_request(2000));
    }

    #[test]
    fn test_opens_after_threshold() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        });

        cb.record_failure(100);
        cb.record_failure(200);
        assert_eq!(cb.state(), State::Closed);

        cb.record_failure(300);
        assert_eq!(cb.state(), State::Open);
    }

    #[test]
    fn test_open_rejects_requests() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 1,
            open_duration_ms: 10_000,
            ..Default::default()
        });

        cb.record_failure(1000);
        assert_eq!(cb.state(), State::Open);
        assert!(!cb.allow_request(2000)); // still within open_duration
    }

    #[test]
    fn test_transitions_to_half_open() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 1,
            open_duration_ms: 1000,
            half_open_max_requests: 1,
            ..Default::default()
        });

        cb.record_failure(1000);
        assert_eq!(cb.state(), State::Open);

        // After open_duration, should transition to half-open
        assert!(cb.allow_request(2001));
        assert_eq!(cb.state(), State::HalfOpen);
    }

    #[test]
    fn test_half_open_success_closes() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 1,
            success_threshold: 2,
            open_duration_ms: 1000,
            half_open_max_requests: 5,
        });

        cb.record_failure(1000);
        assert!(cb.allow_request(2001)); // transitions to half-open
        cb.record_success();
        cb.record_success();
        assert_eq!(cb.state(), State::Closed);
    }

    #[test]
    fn test_half_open_failure_reopens() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 1,
            open_duration_ms: 1000,
            half_open_max_requests: 5,
            ..Default::default()
        });

        cb.record_failure(1000);
        assert!(cb.allow_request(2001)); // half-open
        cb.record_failure(2100);
        assert_eq!(cb.state(), State::Open);
    }

    #[test]
    fn test_manual_reset() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 1,
            ..Default::default()
        });

        cb.record_failure(1000);
        assert_eq!(cb.state(), State::Open);
        cb.reset();
        assert_eq!(cb.state(), State::Closed);
    }

    #[test]
    fn test_manual_trip() {
        let mut cb = CircuitBreaker::with_defaults();
        assert_eq!(cb.state(), State::Closed);
        cb.trip(5000);
        assert_eq!(cb.state(), State::Open);
    }

    #[test]
    fn test_stats() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 3,
            open_duration_ms: 10_000,
            ..Default::default()
        });

        cb.allow_request(100);
        cb.record_success();
        cb.allow_request(200);
        cb.record_failure(200);
        cb.allow_request(300);
        cb.record_failure(300);
        cb.allow_request(400);
        cb.record_failure(400); // opens

        // One more request should be rejected
        cb.allow_request(500);

        let stats = cb.stats();
        assert_eq!(stats.total_requests, 5);
        assert_eq!(stats.total_failures, 3);
        assert_eq!(stats.total_rejections, 1);
        assert!(stats.error_rate() > 0.5);
    }

    #[test]
    fn test_success_resets_failure_count() {
        let mut cb = CircuitBreaker::new(BreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        });

        cb.record_failure(100);
        cb.record_failure(200);
        cb.record_success(); // resets count
        cb.record_failure(300);
        cb.record_failure(400);
        assert_eq!(cb.state(), State::Closed); // still closed, only 2 consecutive failures
    }

    #[test]
    fn test_registry_get_or_create() {
        let mut registry = BreakerRegistry::new(BreakerConfig::default());
        {
            let cb = registry.get_or_create("service-a");
            assert_eq!(cb.state(), State::Closed);
        }
        assert_eq!(registry.count(), 1);
        registry.get_or_create("service-b");
        assert_eq!(registry.count(), 2);
    }

    #[test]
    fn test_registry_open_breakers() {
        let mut registry = BreakerRegistry::new(BreakerConfig {
            failure_threshold: 1,
            ..Default::default()
        });

        registry.get_or_create("healthy");
        registry.get_or_create("failing").record_failure(1000);

        let open = registry.open_breakers();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].0, "failing");
    }

    #[test]
    fn test_registry_reset_all() {
        let mut registry = BreakerRegistry::new(BreakerConfig {
            failure_threshold: 1,
            ..Default::default()
        });

        registry.get_or_create("a").record_failure(100);
        registry.get_or_create("b").record_failure(200);
        assert_eq!(registry.open_breakers().len(), 2);

        registry.reset_all();
        assert_eq!(registry.open_breakers().len(), 0);
    }
}
