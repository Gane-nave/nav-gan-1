//! Circuit breaker — prevents cascading failures by temporarily disabling failing operations.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Normal operation — requests pass through.
    Closed,
    /// Failures exceeded threshold — requests are rejected.
    Open,
    /// Testing recovery — limited requests pass through.
    HalfOpen,
}

/// Circuit breaker configuration.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit.
    pub failure_threshold: u32,
    /// Duration to keep the circuit open before testing recovery.
    pub recovery_timeout: Duration,
    /// Number of successes in half-open state to close the circuit.
    pub success_threshold: u32,
    /// Maximum number of test requests in half-open state.
    pub half_open_max_requests: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
            success_threshold: 3,
            half_open_max_requests: 3,
        }
    }
}

/// All mutable circuit breaker state, protected by a single lock to prevent TOCTOU races.
struct CircuitBreakerInner {
    state: CircuitState,
    failure_count: u32,
    success_count: u32,
    half_open_requests: u32,
    last_failure_time: Option<Instant>,
    total_requests: u64,
    total_failures: u64,
    total_rejections: u64,
}

/// Circuit breaker — tracks failures and manages state transitions.
pub struct CircuitBreaker {
    name: String,
    config: CircuitBreakerConfig,
    inner: RwLock<CircuitBreakerInner>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with the given name and config.
    pub fn new(name: &str, config: CircuitBreakerConfig) -> Self {
        Self {
            name: name.to_string(),
            config,
            inner: RwLock::new(CircuitBreakerInner {
                state: CircuitState::Closed,
                failure_count: 0,
                success_count: 0,
                half_open_requests: 0,
                last_failure_time: None,
                total_requests: 0,
                total_failures: 0,
                total_rejections: 0,
            }),
        }
    }

    /// Create with default config.
    pub fn with_defaults(name: &str) -> Self {
        Self::new(name, CircuitBreakerConfig::default())
    }

    /// Get the circuit breaker name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the current state.
    pub fn state(&self) -> CircuitState {
        self.inner.read().state
    }

    /// Check if a request is allowed. Returns true if the request can proceed.
    pub fn allow_request(&self) -> bool {
        let mut inner = self.inner.write();
        inner.total_requests += 1;

        match inner.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if recovery timeout has elapsed
                if let Some(last_failure) = inner.last_failure_time {
                    if last_failure.elapsed() >= self.config.recovery_timeout {
                        // Transition to half-open
                        inner.state = CircuitState::HalfOpen;
                        inner.half_open_requests = 1; // This request counts
                        inner.success_count = 0;
                        true
                    } else {
                        inner.total_rejections += 1;
                        false
                    }
                } else {
                    inner.total_rejections += 1;
                    false
                }
            }
            CircuitState::HalfOpen => {
                if inner.half_open_requests < self.config.half_open_max_requests {
                    inner.half_open_requests += 1;
                    true
                } else {
                    inner.total_rejections += 1;
                    false
                }
            }
        }
    }

    /// Record a successful operation.
    pub fn record_success(&self) {
        let mut inner = self.inner.write();
        match inner.state {
            CircuitState::Closed => {
                inner.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                inner.success_count += 1;
                if inner.success_count >= self.config.success_threshold {
                    // Recovery successful — close the circuit
                    inner.state = CircuitState::Closed;
                    inner.failure_count = 0;
                }
            }
            CircuitState::Open => {}
        }
    }

    /// Record a failed operation.
    pub fn record_failure(&self) {
        let mut inner = self.inner.write();
        inner.total_failures += 1;
        inner.last_failure_time = Some(Instant::now());

        match inner.state {
            CircuitState::Closed => {
                inner.failure_count += 1;
                if inner.failure_count >= self.config.failure_threshold {
                    inner.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                // Recovery failed — re-open the circuit
                inner.state = CircuitState::Open;
                inner.success_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    /// Force the circuit to a specific state (for testing/admin).
    pub fn force_state(&self, state: CircuitState) {
        let mut inner = self.inner.write();
        inner.state = state;
        if state == CircuitState::Closed {
            inner.failure_count = 0;
            inner.success_count = 0;
        }
    }

    /// Get circuit breaker statistics.
    pub fn stats(&self) -> CircuitBreakerStats {
        let inner = self.inner.read();
        CircuitBreakerStats {
            name: self.name.clone(),
            state: inner.state,
            failure_count: inner.failure_count,
            total_requests: inner.total_requests,
            total_failures: inner.total_failures,
            total_rejections: inner.total_rejections,
        }
    }
}

/// Circuit breaker statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerStats {
    pub name: String,
    pub state: CircuitState,
    pub failure_count: u32,
    pub total_requests: u64,
    pub total_failures: u64,
    pub total_rejections: u64,
}

impl CircuitBreakerStats {
    /// Failure rate (0.0 to 1.0).
    pub fn failure_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.total_failures as f64 / self.total_requests as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_starts_closed() {
        let cb = CircuitBreaker::with_defaults("test");
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.allow_request());
    }

    #[test]
    fn test_circuit_opens_after_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };
        let cb = CircuitBreaker::new("test", config);

        assert_eq!(cb.state(), CircuitState::Closed);
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Closed);
        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Open);
    }

    #[test]
    fn test_circuit_rejects_when_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout: Duration::from_secs(3600), // Long timeout
            ..Default::default()
        };
        let cb = CircuitBreaker::new("test", config);

        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Open);
        assert!(!cb.allow_request());
    }

    #[test]
    fn test_circuit_half_open_after_timeout() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout: Duration::from_millis(0), // Immediate recovery
            ..Default::default()
        };
        let cb = CircuitBreaker::new("test", config);

        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Open);

        // Allow request should transition to HalfOpen
        assert!(cb.allow_request());
        assert_eq!(cb.state(), CircuitState::HalfOpen);
    }

    #[test]
    fn test_circuit_closes_after_success_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout: Duration::from_millis(0),
            success_threshold: 2,
            half_open_max_requests: 5,
        };
        let cb = CircuitBreaker::new("test", config);

        cb.record_failure();
        cb.allow_request(); // Transitions to HalfOpen

        cb.record_success();
        assert_eq!(cb.state(), CircuitState::HalfOpen);
        cb.record_success();
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_reopens_on_half_open_failure() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout: Duration::from_millis(0),
            success_threshold: 3,
            half_open_max_requests: 5,
        };
        let cb = CircuitBreaker::new("test", config);

        cb.record_failure(); // Open
        cb.allow_request(); // HalfOpen
        cb.record_failure(); // Back to Open

        assert_eq!(cb.state(), CircuitState::Open);
    }

    #[test]
    fn test_success_resets_failure_count() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };
        let cb = CircuitBreaker::new("test", config);

        cb.record_failure();
        cb.record_failure();
        cb.record_success(); // Resets failure count
        cb.record_failure();
        cb.record_failure();
        // Should still be closed (only 2 consecutive failures)
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_force_state() {
        let cb = CircuitBreaker::with_defaults("test");
        cb.force_state(CircuitState::Open);
        assert_eq!(cb.state(), CircuitState::Open);
        cb.force_state(CircuitState::Closed);
        assert_eq!(cb.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_stats() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            recovery_timeout: Duration::from_secs(3600),
            ..Default::default()
        };
        let cb = CircuitBreaker::new("gnss_service", config);

        cb.allow_request();
        cb.record_success();
        cb.allow_request();
        cb.record_failure();
        cb.allow_request();
        cb.record_failure();
        // Now open
        cb.allow_request(); // Rejected

        let stats = cb.stats();
        assert_eq!(stats.name, "gnss_service");
        assert_eq!(stats.state, CircuitState::Open);
        assert_eq!(stats.total_requests, 4);
        assert_eq!(stats.total_failures, 2);
        assert_eq!(stats.total_rejections, 1);
        assert!((stats.failure_rate() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_half_open_max_requests() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout: Duration::from_millis(0),
            success_threshold: 5,
            half_open_max_requests: 2,
        };
        let cb = CircuitBreaker::new("test", config);

        cb.record_failure(); // Open
        assert!(cb.allow_request()); // HalfOpen, request 1
        assert!(cb.allow_request()); // HalfOpen, request 2
        assert!(!cb.allow_request()); // Rejected — max half-open requests
    }
}
