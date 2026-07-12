//! Load balancing — distributes requests across backend instances
//! using round-robin, weighted, or least-connections strategies.

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Load balancing strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceStrategy {
    /// Round-robin: each backend gets requests in sequence.
    RoundRobin,
    /// Weighted: backends get traffic proportional to their weight.
    Weighted,
    /// Least connections: prefer the backend with fewest active connections.
    LeastConnections,
}

/// Health status of a backend instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// A backend instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backend {
    pub id: Uuid,
    pub address: String,
    pub weight: u32,
    pub health: HealthStatus,
}

/// Internal tracking state for a backend.
struct BackendState {
    backend: Backend,
    active_connections: u64,
    total_requests: u64,
}

/// Load balancer that distributes requests across backends.
pub struct LoadBalancer {
    strategy: BalanceStrategy,
    backends: Mutex<Vec<BackendState>>,
    round_robin_idx: Mutex<usize>,
}

impl LoadBalancer {
    /// Create a new load balancer.
    pub fn new(strategy: BalanceStrategy) -> Self {
        Self {
            strategy,
            backends: Mutex::new(Vec::new()),
            round_robin_idx: Mutex::new(0),
        }
    }

    /// Add a backend.
    pub fn add_backend(&self, backend: Backend) {
        self.backends.lock().push(BackendState {
            backend,
            active_connections: 0,
            total_requests: 0,
        });
    }

    /// Remove a backend by ID.
    pub fn remove_backend(&self, id: Uuid) -> bool {
        let mut backends = self.backends.lock();
        if let Some(idx) = backends.iter().position(|b| b.backend.id == id) {
            backends.remove(idx);
            true
        } else {
            false
        }
    }

    /// Update the health status of a backend.
    pub fn set_health(&self, id: Uuid, health: HealthStatus) {
        let mut backends = self.backends.lock();
        if let Some(b) = backends.iter_mut().find(|b| b.backend.id == id) {
            b.backend.health = health;
        }
    }

    /// Select the next backend for a request. Returns None if no healthy backends.
    pub fn select(&self) -> Option<Backend> {
        let mut backends = self.backends.lock();
        let healthy: Vec<usize> = backends
            .iter()
            .enumerate()
            .filter(|(_, b)| b.backend.health != HealthStatus::Unhealthy)
            .map(|(i, _)| i)
            .collect();

        if healthy.is_empty() {
            return None;
        }

        let idx = match self.strategy {
            BalanceStrategy::RoundRobin => {
                let mut rr = self.round_robin_idx.lock();
                let pos = *rr % healthy.len();
                *rr = rr.wrapping_add(1);
                healthy[pos]
            }
            BalanceStrategy::Weighted => {
                // Select based on weight proportion
                let total_weight: u32 = healthy.iter().map(|&i| backends[i].backend.weight).sum();
                if total_weight == 0 {
                    healthy[0]
                } else {
                    // Pick the backend with highest weight-to-requests ratio
                    *healthy
                        .iter()
                        .min_by(|&&a, &&b| {
                            let ratio_a = backends[a].total_requests as f64
                                / backends[a].backend.weight.max(1) as f64;
                            let ratio_b = backends[b].total_requests as f64
                                / backends[b].backend.weight.max(1) as f64;
                            ratio_a
                                .partial_cmp(&ratio_b)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .unwrap()
                }
            }
            BalanceStrategy::LeastConnections => *healthy
                .iter()
                .min_by_key(|&&i| backends[i].active_connections)
                .unwrap(),
        };

        backends[idx].active_connections += 1;
        backends[idx].total_requests += 1;
        Some(backends[idx].backend.clone())
    }

    /// Release a connection from a backend (call when request completes).
    pub fn release(&self, id: Uuid) {
        let mut backends = self.backends.lock();
        if let Some(b) = backends.iter_mut().find(|b| b.backend.id == id) {
            b.active_connections = b.active_connections.saturating_sub(1);
        }
    }

    /// Number of registered backends.
    pub fn backend_count(&self) -> usize {
        self.backends.lock().len()
    }

    /// Number of healthy backends.
    pub fn healthy_count(&self) -> usize {
        self.backends
            .lock()
            .iter()
            .filter(|b| b.backend.health != HealthStatus::Unhealthy)
            .count()
    }

    /// Get stats for all backends.
    pub fn stats(&self) -> Vec<BackendStats> {
        self.backends
            .lock()
            .iter()
            .map(|b| BackendStats {
                id: b.backend.id,
                address: b.backend.address.clone(),
                health: b.backend.health,
                active_connections: b.active_connections,
                total_requests: b.total_requests,
            })
            .collect()
    }
}

/// Stats snapshot for a backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendStats {
    pub id: Uuid,
    pub address: String,
    pub health: HealthStatus,
    pub active_connections: u64,
    pub total_requests: u64,
}

/// Health check configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub interval_s: u64,
    pub timeout_ms: u64,
    pub unhealthy_threshold: u32,
    pub healthy_threshold: u32,
    pub path: String,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_s: 10,
            timeout_ms: 3000,
            unhealthy_threshold: 3,
            healthy_threshold: 2,
            path: "/health".to_string(),
        }
    }
}

/// Simulated health checker that tracks consecutive failures.
pub struct HealthChecker {
    config: HealthCheckConfig,
    failure_counts: HashMap<Uuid, u32>,
    success_counts: HashMap<Uuid, u32>,
}

impl HealthChecker {
    /// Create a new health checker.
    pub fn new(config: HealthCheckConfig) -> Self {
        Self {
            config,
            failure_counts: HashMap::new(),
            success_counts: HashMap::new(),
        }
    }

    /// Record a successful health check.
    pub fn record_success(&mut self, id: Uuid) -> HealthStatus {
        self.failure_counts.insert(id, 0);
        let count = self.success_counts.entry(id).or_insert(0);
        *count += 1;
        if *count >= self.config.healthy_threshold {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        }
    }

    /// Record a failed health check.
    pub fn record_failure(&mut self, id: Uuid) -> HealthStatus {
        self.success_counts.insert(id, 0);
        let count = self.failure_counts.entry(id).or_insert(0);
        *count += 1;
        if *count >= self.config.unhealthy_threshold {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        }
    }

    /// Get the config.
    pub fn config(&self) -> &HealthCheckConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_backend(addr: &str, weight: u32) -> Backend {
        Backend {
            id: Uuid::new_v4(),
            address: addr.to_string(),
            weight,
            health: HealthStatus::Healthy,
        }
    }

    #[test]
    fn test_round_robin() {
        let lb = LoadBalancer::new(BalanceStrategy::RoundRobin);
        let b1 = make_backend("svc1:8080", 1);
        let b2 = make_backend("svc2:8080", 1);
        lb.add_backend(b1);
        lb.add_backend(b2);

        let s1 = lb.select().unwrap().address.clone();
        let s2 = lb.select().unwrap().address.clone();
        let s3 = lb.select().unwrap().address.clone();
        // Should alternate
        assert_eq!(s1, s3); // wraps around
        assert_ne!(s1, s2);
    }

    #[test]
    fn test_least_connections() {
        let lb = LoadBalancer::new(BalanceStrategy::LeastConnections);
        let b1 = make_backend("svc1:8080", 1);
        let b1_id = b1.id;
        let b2 = make_backend("svc2:8080", 1);
        lb.add_backend(b1);
        lb.add_backend(b2);

        // First select → both at 0 connections, picks first
        let sel1 = lb.select().unwrap();
        // Now svc1 has 1 connection, svc2 has 0
        let sel2 = lb.select().unwrap();
        assert_ne!(sel1.address, sel2.address);

        // Release svc1's connection
        lb.release(b1_id);
        // Now svc1 has 0, svc2 has 1 → should pick svc1
        let sel3 = lb.select().unwrap();
        assert_eq!(sel3.id, b1_id);
    }

    #[test]
    fn test_weighted_distribution() {
        let lb = LoadBalancer::new(BalanceStrategy::Weighted);
        let b1 = make_backend("heavy:8080", 3);
        let b2 = make_backend("light:8080", 1);
        let b1_id = b1.id;
        lb.add_backend(b1);
        lb.add_backend(b2);

        // First selection should prefer the heavier backend
        let sel = lb.select().unwrap();
        // Both have 0 requests, but b1 has higher weight → lower ratio
        assert_eq!(sel.id, b1_id);
    }

    #[test]
    fn test_unhealthy_backend_excluded() {
        let lb = LoadBalancer::new(BalanceStrategy::RoundRobin);
        let b1 = make_backend("svc1:8080", 1);
        let b1_id = b1.id;
        let b2 = make_backend("svc2:8080", 1);
        lb.add_backend(b1);
        lb.add_backend(b2);

        lb.set_health(b1_id, HealthStatus::Unhealthy);

        // Should only route to svc2
        for _ in 0..5 {
            let sel = lb.select().unwrap();
            assert_eq!(sel.address, "svc2:8080");
        }
    }

    #[test]
    fn test_no_healthy_backends() {
        let lb = LoadBalancer::new(BalanceStrategy::RoundRobin);
        let b = make_backend("svc1:8080", 1);
        let bid = b.id;
        lb.add_backend(b);
        lb.set_health(bid, HealthStatus::Unhealthy);
        assert!(lb.select().is_none());
    }

    #[test]
    fn test_stats() {
        let lb = LoadBalancer::new(BalanceStrategy::RoundRobin);
        lb.add_backend(make_backend("svc1:8080", 1));
        lb.select();
        let stats = lb.stats();
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].total_requests, 1);
        assert_eq!(stats[0].active_connections, 1);
    }

    #[test]
    fn test_health_checker_success() {
        let mut hc = HealthChecker::new(HealthCheckConfig {
            healthy_threshold: 2,
            unhealthy_threshold: 3,
            ..Default::default()
        });
        let id = Uuid::new_v4();
        assert_eq!(hc.record_success(id), HealthStatus::Degraded);
        assert_eq!(hc.record_success(id), HealthStatus::Healthy);
    }

    #[test]
    fn test_health_checker_failure() {
        let mut hc = HealthChecker::new(HealthCheckConfig {
            healthy_threshold: 2,
            unhealthy_threshold: 3,
            ..Default::default()
        });
        let id = Uuid::new_v4();
        assert_eq!(hc.record_failure(id), HealthStatus::Degraded);
        assert_eq!(hc.record_failure(id), HealthStatus::Degraded);
        assert_eq!(hc.record_failure(id), HealthStatus::Unhealthy);
    }

    #[test]
    fn test_health_checker_recovery() {
        let mut hc = HealthChecker::new(HealthCheckConfig::default());
        let id = Uuid::new_v4();
        hc.record_failure(id);
        hc.record_failure(id);
        // Success resets failure count
        hc.record_success(id);
        assert_eq!(hc.record_failure(id), HealthStatus::Degraded); // count restarts at 1
    }

    #[test]
    fn test_remove_backend() {
        let lb = LoadBalancer::new(BalanceStrategy::RoundRobin);
        let b = make_backend("svc1:8080", 1);
        let bid = b.id;
        lb.add_backend(b);
        assert_eq!(lb.backend_count(), 1);
        assert!(lb.remove_backend(bid));
        assert_eq!(lb.backend_count(), 0);
    }
}
