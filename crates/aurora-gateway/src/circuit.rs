//! Circuit breaking — protects backends from cascading failures by
//! tracking error rates and opening the circuit when thresholds are exceeded.

use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Circuit state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Normal operation — requests pass through.
    Closed,
    /// Failures exceeded threshold — requests are rejected.
    Open,
    /// Testing — a limited number of requests pass through to check recovery.
    HalfOpen,
}

/// Circuit breaker configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitConfig {
    /// Number of failures before opening the circuit.
    pub failure_threshold: u32,
    /// Seconds to wait before transitioning from Open to HalfOpen.
    pub recovery_timeout_s: u64,
    /// Number of successful requests in HalfOpen to close the circuit.
    pub success_threshold: u32,
    /// Time window in seconds for counting failures.
    pub window_s: u64,
}

impl Default for CircuitConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout_s: 30,
            success_threshold: 3,
            window_s: 60,
        }
    }
}

/// Internal state for a circuit.
struct CircuitInner {
    state: CircuitState,
    failures: Vec<DateTime<Utc>>,
    half_open_successes: u32,
    half_open_allowed: u32,
    last_state_change: DateTime<Utc>,
    total_rejected: u64,
}

/// Result of attempting to pass through a circuit breaker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitResult {
    /// Request is allowed through.
    Allowed,
    /// Request is rejected because the circuit is open.
    Rejected,
}

/// Circuit breaker that tracks failures per service.
pub struct CircuitBreaker {
    config: CircuitConfig,
    circuits: Mutex<HashMap<String, CircuitInner>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    pub fn new(config: CircuitConfig) -> Self {
        Self {
            config,
            circuits: Mutex::new(HashMap::new()),
        }
    }

    /// Check if a request to the given service is allowed.
    pub fn check(&self, service: &str) -> CircuitResult {
        self.check_at(service, Utc::now())
    }

    /// Check with explicit timestamp (for testing).
    pub fn check_at(&self, service: &str, now: DateTime<Utc>) -> CircuitResult {
        let mut circuits = self.circuits.lock();
        let circuit = circuits
            .entry(service.to_string())
            .or_insert_with(|| CircuitInner {
                state: CircuitState::Closed,
                failures: Vec::new(),
                half_open_successes: 0,
                half_open_allowed: 0,
                last_state_change: now,
                total_rejected: 0,
            });

        match circuit.state {
            CircuitState::Closed => CircuitResult::Allowed,
            CircuitState::Open => {
                // Check if recovery timeout has elapsed
                let elapsed = (now - circuit.last_state_change).num_seconds();
                if elapsed >= self.config.recovery_timeout_s as i64 {
                    circuit.state = CircuitState::HalfOpen;
                    circuit.half_open_successes = 0;
                    circuit.half_open_allowed = self.config.success_threshold;
                    circuit.last_state_change = now;
                    CircuitResult::Allowed
                } else {
                    circuit.total_rejected += 1;
                    CircuitResult::Rejected
                }
            }
            CircuitState::HalfOpen => {
                // Limit probe traffic in half-open state
                if circuit.half_open_allowed > 0 {
                    circuit.half_open_allowed -= 1;
                    CircuitResult::Allowed
                } else {
                    circuit.total_rejected += 1;
                    CircuitResult::Rejected
                }
            }
        }
    }

    /// Record a successful request.
    pub fn record_success(&self, service: &str) {
        self.record_success_at(service, Utc::now());
    }

    /// Record success with explicit timestamp.
    pub fn record_success_at(&self, service: &str, now: DateTime<Utc>) {
        let mut circuits = self.circuits.lock();
        if let Some(circuit) = circuits.get_mut(service) {
            match circuit.state {
                CircuitState::HalfOpen => {
                    circuit.half_open_successes += 1;
                    if circuit.half_open_successes >= self.config.success_threshold {
                        circuit.state = CircuitState::Closed;
                        circuit.failures.clear();
                        circuit.last_state_change = now;
                    }
                }
                CircuitState::Closed => {
                    // Success in closed state — prune old failures
                    let window_start = now - chrono::Duration::seconds(self.config.window_s as i64);
                    circuit.failures.retain(|t| *t >= window_start);
                }
                CircuitState::Open => {}
            }
        }
    }

    /// Record a failed request.
    pub fn record_failure(&self, service: &str) {
        self.record_failure_at(service, Utc::now());
    }

    /// Record failure with explicit timestamp.
    pub fn record_failure_at(&self, service: &str, now: DateTime<Utc>) {
        let mut circuits = self.circuits.lock();
        let circuit = circuits
            .entry(service.to_string())
            .or_insert_with(|| CircuitInner {
                state: CircuitState::Closed,
                failures: Vec::new(),
                half_open_successes: 0,
                half_open_allowed: 0,
                last_state_change: now,
                total_rejected: 0,
            });

        match circuit.state {
            CircuitState::Closed => {
                // Prune old failures outside the window
                let window_start = now - chrono::Duration::seconds(self.config.window_s as i64);
                circuit.failures.retain(|t| *t >= window_start);
                circuit.failures.push(now);

                if circuit.failures.len() as u32 >= self.config.failure_threshold {
                    circuit.state = CircuitState::Open;
                    circuit.last_state_change = now;
                }
            }
            CircuitState::HalfOpen => {
                // Any failure in half-open state reopens the circuit
                circuit.state = CircuitState::Open;
                circuit.last_state_change = now;
                circuit.half_open_successes = 0;
            }
            CircuitState::Open => {}
        }
    }

    /// Get the current state of a circuit.
    pub fn state(&self, service: &str) -> CircuitState {
        self.circuits
            .lock()
            .get(service)
            .map_or(CircuitState::Closed, |c| c.state)
    }

    /// Get the total number of rejected requests for a service.
    pub fn total_rejected(&self, service: &str) -> u64 {
        self.circuits
            .lock()
            .get(service)
            .map_or(0, |c| c.total_rejected)
    }

    /// Reset a circuit to closed state.
    pub fn reset(&self, service: &str) {
        self.circuits.lock().remove(service);
    }

    /// Number of tracked circuits.
    pub fn circuit_count(&self) -> usize {
        self.circuits.lock().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_closed_allows_requests() {
        let cb = CircuitBreaker::new(CircuitConfig::default());
        assert_eq!(cb.check("svc"), CircuitResult::Allowed);
        assert_eq!(cb.state("svc"), CircuitState::Closed);
    }

    #[test]
    fn test_opens_after_threshold() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 3,
            ..Default::default()
        });

        let now = Utc::now();
        cb.check_at("svc", now);
        cb.record_failure_at("svc", now);
        cb.record_failure_at("svc", now);
        assert_eq!(cb.state("svc"), CircuitState::Closed); // 2 < 3

        cb.record_failure_at("svc", now);
        assert_eq!(cb.state("svc"), CircuitState::Open); // 3 >= 3
    }

    #[test]
    fn test_open_rejects_requests() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 1,
            recovery_timeout_s: 30,
            ..Default::default()
        });

        let now = Utc::now();
        cb.check_at("svc", now);
        cb.record_failure_at("svc", now);
        assert_eq!(cb.check_at("svc", now), CircuitResult::Rejected);
        assert!(cb.total_rejected("svc") > 0);
    }

    #[test]
    fn test_half_open_after_timeout() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 1,
            recovery_timeout_s: 10,
            ..Default::default()
        });

        let now = Utc::now();
        cb.check_at("svc", now);
        cb.record_failure_at("svc", now);
        assert_eq!(cb.state("svc"), CircuitState::Open);

        // Wait for recovery timeout
        let later = now + Duration::seconds(11);
        assert_eq!(cb.check_at("svc", later), CircuitResult::Allowed);
        assert_eq!(cb.state("svc"), CircuitState::HalfOpen);
    }

    #[test]
    fn test_half_open_to_closed() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 1,
            recovery_timeout_s: 1,
            success_threshold: 2,
            ..Default::default()
        });

        let now = Utc::now();
        cb.check_at("svc", now);
        cb.record_failure_at("svc", now);

        let later = now + Duration::seconds(2);
        cb.check_at("svc", later); // transitions to HalfOpen
        assert_eq!(cb.state("svc"), CircuitState::HalfOpen);

        cb.record_success_at("svc", later);
        assert_eq!(cb.state("svc"), CircuitState::HalfOpen); // 1 < 2

        cb.record_success_at("svc", later);
        assert_eq!(cb.state("svc"), CircuitState::Closed); // 2 >= 2
    }

    #[test]
    fn test_half_open_failure_reopens() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 1,
            recovery_timeout_s: 1,
            success_threshold: 3,
            ..Default::default()
        });

        let now = Utc::now();
        cb.check_at("svc", now);
        cb.record_failure_at("svc", now);

        let later = now + Duration::seconds(2);
        cb.check_at("svc", later); // HalfOpen
        assert_eq!(cb.state("svc"), CircuitState::HalfOpen);

        cb.record_failure_at("svc", later); // fails in HalfOpen → Open
        assert_eq!(cb.state("svc"), CircuitState::Open);
    }

    #[test]
    fn test_failures_outside_window_dont_count() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 3,
            window_s: 10,
            ..Default::default()
        });

        let now = Utc::now();
        // 2 failures at time 0
        cb.check_at("svc", now);
        cb.record_failure_at("svc", now);
        cb.record_failure_at("svc", now);

        // 1 failure at time 15 (first 2 should be pruned)
        let later = now + Duration::seconds(15);
        cb.record_failure_at("svc", later);
        assert_eq!(cb.state("svc"), CircuitState::Closed); // only 1 in window
    }

    #[test]
    fn test_reset() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 1,
            ..Default::default()
        });
        let now = Utc::now();
        cb.check_at("svc", now);
        cb.record_failure_at("svc", now);
        assert_eq!(cb.state("svc"), CircuitState::Open);

        cb.reset("svc");
        assert_eq!(cb.state("svc"), CircuitState::Closed);
    }

    #[test]
    fn test_per_service_isolation() {
        let cb = CircuitBreaker::new(CircuitConfig {
            failure_threshold: 1,
            ..Default::default()
        });
        let now = Utc::now();
        cb.check_at("svc1", now);
        cb.record_failure_at("svc1", now);
        assert_eq!(cb.state("svc1"), CircuitState::Open);
        assert_eq!(cb.state("svc2"), CircuitState::Closed); // isolated
    }

    #[test]
    fn test_circuit_count() {
        let cb = CircuitBreaker::new(CircuitConfig::default());
        cb.check("a");
        cb.record_failure("b");
        assert_eq!(cb.circuit_count(), 2);
    }

    #[test]
    fn test_serialization() {
        let config = CircuitConfig {
            failure_threshold: 10,
            recovery_timeout_s: 60,
            success_threshold: 5,
            window_s: 120,
        };
        let json = serde_json::to_string(&config).unwrap();
        let de: CircuitConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(de.failure_threshold, 10);
        assert_eq!(de.recovery_timeout_s, 60);
    }
}
