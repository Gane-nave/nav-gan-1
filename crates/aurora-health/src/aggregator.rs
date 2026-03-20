//! Aggregator — combines health checks from multiple components into system-wide status.

use std::collections::HashMap;

use crate::checker::{HealthCheckResult, HealthHistory, HealthStatus};

/// Dependency criticality level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Criticality {
    /// Required — system cannot operate without this component.
    Required,
    /// Important — system is degraded without this component.
    Important,
    /// Optional — system can operate fully without this component.
    Optional,
}

/// A registered dependency with its criticality.
#[derive(Debug)]
struct Dependency {
    name: String,
    criticality: Criticality,
}

/// Aggregated system health report.
#[derive(Debug, Clone)]
pub struct SystemHealthReport {
    /// Overall system status.
    pub status: HealthStatus,
    /// Number of healthy components.
    pub healthy_count: usize,
    /// Number of degraded components.
    pub degraded_count: usize,
    /// Number of unhealthy components.
    pub unhealthy_count: usize,
    /// Number of unknown components.
    pub unknown_count: usize,
    /// Total components.
    pub total_count: usize,
    /// Per-component status.
    pub components: Vec<(String, HealthStatus)>,
    /// System uptime ratio (0.0 – 1.0).
    pub overall_availability: f64,
}

impl SystemHealthReport {
    /// Whether the system is ready to serve traffic.
    pub fn is_ready(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy | HealthStatus::Degraded)
    }

    /// Whether the system is alive (not fully unhealthy).
    pub fn is_alive(&self) -> bool {
        self.status != HealthStatus::Unhealthy
    }
}

/// Aggregates health checks from multiple components.
pub struct HealthAggregator {
    dependencies: Vec<Dependency>,
    histories: HashMap<String, HealthHistory>,
    max_history: usize,
}

impl HealthAggregator {
    /// Create a new health aggregator.
    pub fn new(max_history: usize) -> Self {
        Self {
            dependencies: Vec::new(),
            histories: HashMap::new(),
            max_history,
        }
    }

    /// Register a dependency.
    pub fn register(&mut self, name: &str, criticality: Criticality) {
        if self.histories.contains_key(name) {
            return; // already registered
        }
        self.dependencies.push(Dependency {
            name: name.to_string(),
            criticality,
        });
        self.histories
            .insert(name.to_string(), HealthHistory::new(name, self.max_history));
    }

    /// Record a health check result for a component.
    pub fn record(&mut self, result: HealthCheckResult) {
        let name = result.component.clone();
        if let Some(history) = self.histories.get_mut(&name) {
            history.record(result);
        }
    }

    /// Get status for a single component.
    pub fn component_status(&self, name: &str) -> HealthStatus {
        self.histories
            .get(name)
            .map(|h| h.current_status())
            .unwrap_or(HealthStatus::Unknown)
    }

    /// Get availability for a single component.
    pub fn component_availability(&self, name: &str) -> f64 {
        self.histories
            .get(name)
            .map(|h| h.availability())
            .unwrap_or(0.0)
    }

    /// Generate a system health report.
    pub fn report(&self) -> SystemHealthReport {
        let mut healthy = 0usize;
        let mut degraded = 0usize;
        let mut unhealthy = 0usize;
        let mut unknown = 0usize;
        let mut components = Vec::new();
        let mut total_avail = 0.0f64;

        let mut any_required_unhealthy = false;
        let mut any_important_unhealthy = false;

        for dep in &self.dependencies {
            let status = self.component_status(&dep.name);
            match status {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Degraded => degraded += 1,
                HealthStatus::Unhealthy => {
                    unhealthy += 1;
                    if dep.criticality == Criticality::Required {
                        any_required_unhealthy = true;
                    }
                    if dep.criticality == Criticality::Important {
                        any_important_unhealthy = true;
                    }
                }
                HealthStatus::Unknown => unknown += 1,
            }
            components.push((dep.name.clone(), status));
            total_avail += self.component_availability(&dep.name);
        }

        let total = self.dependencies.len();
        let overall_availability = if total > 0 {
            total_avail / total as f64
        } else {
            1.0
        };

        let status = if any_required_unhealthy {
            HealthStatus::Unhealthy
        } else if any_important_unhealthy || degraded > 0 {
            HealthStatus::Degraded
        } else if unknown > 0 && healthy == 0 {
            HealthStatus::Unknown
        } else {
            HealthStatus::Healthy
        };

        SystemHealthReport {
            status,
            healthy_count: healthy,
            degraded_count: degraded,
            unhealthy_count: unhealthy,
            unknown_count: unknown,
            total_count: total,
            components,
            overall_availability,
        }
    }

    /// Number of registered dependencies.
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::HealthCheckResult;

    fn setup_aggregator() -> HealthAggregator {
        let mut agg = HealthAggregator::new(10);
        agg.register("gnss", Criticality::Required);
        agg.register("imu", Criticality::Required);
        agg.register("map_db", Criticality::Important);
        agg.register("traffic", Criticality::Optional);
        agg
    }

    #[test]
    fn test_all_healthy() {
        let mut agg = setup_aggregator();
        agg.record(HealthCheckResult::healthy("gnss", 1000));
        agg.record(HealthCheckResult::healthy("imu", 1000));
        agg.record(HealthCheckResult::healthy("map_db", 1000));
        agg.record(HealthCheckResult::healthy("traffic", 1000));

        let report = agg.report();
        assert_eq!(report.status, HealthStatus::Healthy);
        assert_eq!(report.healthy_count, 4);
        assert!(report.is_ready());
        assert!(report.is_alive());
    }

    #[test]
    fn test_required_unhealthy() {
        let mut agg = setup_aggregator();
        agg.record(HealthCheckResult::unhealthy("gnss", "no signal", 1000));
        agg.record(HealthCheckResult::healthy("imu", 1000));
        agg.record(HealthCheckResult::healthy("map_db", 1000));
        agg.record(HealthCheckResult::healthy("traffic", 1000));

        let report = agg.report();
        assert_eq!(report.status, HealthStatus::Unhealthy);
        assert!(!report.is_ready());
    }

    #[test]
    fn test_important_unhealthy() {
        let mut agg = setup_aggregator();
        agg.record(HealthCheckResult::healthy("gnss", 1000));
        agg.record(HealthCheckResult::healthy("imu", 1000));
        agg.record(HealthCheckResult::unhealthy("map_db", "corrupt", 1000));
        agg.record(HealthCheckResult::healthy("traffic", 1000));

        let report = agg.report();
        assert_eq!(report.status, HealthStatus::Degraded);
        assert!(report.is_ready());
    }

    #[test]
    fn test_optional_unhealthy() {
        let mut agg = setup_aggregator();
        agg.record(HealthCheckResult::healthy("gnss", 1000));
        agg.record(HealthCheckResult::healthy("imu", 1000));
        agg.record(HealthCheckResult::healthy("map_db", 1000));
        agg.record(HealthCheckResult::unhealthy("traffic", "offline", 1000));

        let report = agg.report();
        // Optional unhealthy doesn't affect status unless there are other degraded
        assert_eq!(report.unhealthy_count, 1);
    }

    #[test]
    fn test_no_checks_unknown() {
        let agg = setup_aggregator();
        let report = agg.report();
        assert_eq!(report.status, HealthStatus::Unknown);
        assert_eq!(report.unknown_count, 4);
    }

    #[test]
    fn test_duplicate_register() {
        let mut agg = HealthAggregator::new(10);
        agg.register("gnss", Criticality::Required);
        agg.register("gnss", Criticality::Optional); // should be ignored
        assert_eq!(agg.dependency_count(), 1);
    }

    #[test]
    fn test_component_availability() {
        let mut agg = HealthAggregator::new(10);
        agg.register("gnss", Criticality::Required);
        agg.record(HealthCheckResult::healthy("gnss", 1000));
        agg.record(HealthCheckResult::healthy("gnss", 2000));
        agg.record(HealthCheckResult::unhealthy("gnss", "err", 3000));
        agg.record(HealthCheckResult::healthy("gnss", 4000));

        assert!((agg.component_availability("gnss") - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_overall_availability() {
        let mut agg = HealthAggregator::new(10);
        agg.register("a", Criticality::Required);
        agg.register("b", Criticality::Required);

        agg.record(HealthCheckResult::healthy("a", 1000));
        agg.record(HealthCheckResult::healthy("b", 1000));
        agg.record(HealthCheckResult::unhealthy("b", "err", 2000));

        let report = agg.report();
        // a: 1.0, b: 0.5 => average 0.75
        assert!((report.overall_availability - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_report_components_list() {
        let mut agg = setup_aggregator();
        agg.record(HealthCheckResult::healthy("gnss", 1000));
        agg.record(HealthCheckResult::degraded("imu", "noisy", 1000));
        agg.record(HealthCheckResult::unhealthy("map_db", "err", 1000));

        let report = agg.report();
        assert_eq!(report.total_count, 4);
        assert_eq!(report.components.len(), 4);
    }

    #[test]
    fn test_empty_aggregator() {
        let agg = HealthAggregator::new(10);
        let report = agg.report();
        assert_eq!(report.status, HealthStatus::Healthy);
        assert_eq!(report.total_count, 0);
        assert!((report.overall_availability - 1.0).abs() < f64::EPSILON);
    }
}
