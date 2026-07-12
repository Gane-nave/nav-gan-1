//! Service Level Objective (SLO) tracking.
//!
//! Tracks SLO compliance over rolling time windows, computing error budgets
//! and burn rates to proactively detect reliability issues.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum SloError {
    #[error("SLO not found: {0}")]
    NotFound(String),
    #[error("duplicate SLO: {0}")]
    Duplicate(String),
    #[error("invalid target: must be in (0, 1], got {0}")]
    InvalidTarget(f64),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// An SLO definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloDefinition {
    /// Unique name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Target (e.g., 0.999 for 99.9%).
    pub target: f64,
    /// Rolling window for evaluation.
    pub window: Duration,
    /// Labels for grouping.
    pub labels: HashMap<String, String>,
}

/// A single observation for SLO tracking.
#[derive(Debug, Clone)]
struct Observation {
    timestamp: DateTime<Utc>,
    good: bool,
}

/// Current SLO status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloStatus {
    /// SLO name.
    pub name: String,
    /// Current compliance ratio (0.0 to 1.0).
    pub current_ratio: f64,
    /// SLO target.
    pub target: f64,
    /// Whether currently meeting SLO.
    pub meeting_slo: bool,
    /// Remaining error budget ratio (1.0 = full budget remaining).
    pub error_budget_remaining: f64,
    /// Burn rate (error budget consumption rate; >1 means burning faster than budget).
    pub burn_rate: f64,
    /// Total observations in window.
    pub total_observations: usize,
    /// Good observations in window.
    pub good_observations: usize,
    /// Bad observations in window.
    pub bad_observations: usize,
}

// ---------------------------------------------------------------------------
// SLO tracker
// ---------------------------------------------------------------------------

/// Tracks SLO compliance over rolling windows.
pub struct SloTracker {
    definitions: HashMap<String, SloDefinition>,
    observations: HashMap<String, Vec<Observation>>,
}

impl SloTracker {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            observations: HashMap::new(),
        }
    }

    /// Register an SLO.
    pub fn register(&mut self, slo: SloDefinition) -> Result<(), SloError> {
        if slo.target <= 0.0 || slo.target > 1.0 {
            return Err(SloError::InvalidTarget(slo.target));
        }
        if self.definitions.contains_key(&slo.name) {
            return Err(SloError::Duplicate(slo.name));
        }
        self.observations.insert(slo.name.clone(), Vec::new());
        self.definitions.insert(slo.name.clone(), slo);
        Ok(())
    }

    /// Record an observation (good or bad) for an SLO.
    pub fn record(&mut self, name: &str, good: bool) -> Result<(), SloError> {
        if !self.definitions.contains_key(name) {
            return Err(SloError::NotFound(name.to_string()));
        }
        let obs = self.observations.entry(name.to_string()).or_default();
        obs.push(Observation {
            timestamp: Utc::now(),
            good,
        });
        Ok(())
    }

    /// Get the current status of an SLO.
    pub fn status(&self, name: &str) -> Result<SloStatus, SloError> {
        let def = self
            .definitions
            .get(name)
            .ok_or_else(|| SloError::NotFound(name.to_string()))?;

        let obs = self
            .observations
            .get(name)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let cutoff = Utc::now() - def.window;

        let in_window: Vec<&Observation> = obs.iter().filter(|o| o.timestamp >= cutoff).collect();
        let total = in_window.len();
        let good = in_window.iter().filter(|o| o.good).count();
        let bad = total - good;

        let current_ratio = if total == 0 {
            1.0
        } else {
            good as f64 / total as f64
        };

        let error_budget_total = 1.0 - def.target;
        let error_rate = if total == 0 {
            0.0
        } else {
            bad as f64 / total as f64
        };

        let error_budget_remaining = if error_budget_total > 0.0 {
            ((error_budget_total - error_rate) / error_budget_total).clamp(0.0, 1.0)
        } else if bad == 0 {
            1.0
        } else {
            0.0
        };

        let burn_rate = if error_budget_total > 0.0 {
            error_rate / error_budget_total
        } else if bad > 0 {
            f64::INFINITY
        } else {
            0.0
        };

        Ok(SloStatus {
            name: name.to_string(),
            current_ratio,
            target: def.target,
            meeting_slo: current_ratio >= def.target,
            error_budget_remaining,
            burn_rate,
            total_observations: total,
            good_observations: good,
            bad_observations: bad,
        })
    }

    /// Get status of all SLOs.
    pub fn all_statuses(&self) -> Vec<SloStatus> {
        self.definitions
            .keys()
            .filter_map(|name| self.status(name).ok())
            .collect()
    }

    /// Prune observations outside the window.
    pub fn prune(&mut self) {
        let now = Utc::now();
        for (name, obs) in &mut self.observations {
            if let Some(def) = self.definitions.get(name) {
                let cutoff = now - def.window;
                obs.retain(|o| o.timestamp >= cutoff);
            }
        }
    }

    /// Number of registered SLOs.
    pub fn slo_count(&self) -> usize {
        self.definitions.len()
    }

    /// Register default G.A.N.E NAV SLOs.
    pub fn register_defaults(&mut self) {
        let defaults = vec![
            SloDefinition {
                name: "api_availability".to_string(),
                description: "API availability (non-5xx responses)".to_string(),
                target: 0.999,
                window: Duration::days(30),
                labels: [("service".to_string(), "api".to_string())]
                    .into_iter()
                    .collect(),
            },
            SloDefinition {
                name: "api_latency_p99".to_string(),
                description: "P99 API latency under 500ms".to_string(),
                target: 0.99,
                window: Duration::days(30),
                labels: [("service".to_string(), "api".to_string())]
                    .into_iter()
                    .collect(),
            },
            SloDefinition {
                name: "position_accuracy".to_string(),
                description: "Position accuracy within 5m 95% of time".to_string(),
                target: 0.95,
                window: Duration::days(7),
                labels: [("service".to_string(), "navigation".to_string())]
                    .into_iter()
                    .collect(),
            },
            SloDefinition {
                name: "gnss_fix_availability".to_string(),
                description: "GNSS fix available 99.5% of time".to_string(),
                target: 0.995,
                window: Duration::days(7),
                labels: [("service".to_string(), "gnss".to_string())]
                    .into_iter()
                    .collect(),
            },
        ];

        for slo in defaults {
            let _ = self.register(slo);
        }
    }
}

impl Default for SloTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_slo(name: &str, target: f64) -> SloDefinition {
        SloDefinition {
            name: name.to_string(),
            description: format!("Test SLO: {name}"),
            target,
            window: Duration::hours(1),
            labels: HashMap::new(),
        }
    }

    #[test]
    fn register_and_record() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("avail", 0.99)).unwrap();
        tracker.record("avail", true).unwrap();
        tracker.record("avail", true).unwrap();
        tracker.record("avail", false).unwrap();

        let status = tracker.status("avail").unwrap();
        assert_eq!(status.total_observations, 3);
        assert_eq!(status.good_observations, 2);
        assert_eq!(status.bad_observations, 1);
    }

    #[test]
    fn meeting_slo_when_above_target() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("avail", 0.5)).unwrap();
        tracker.record("avail", true).unwrap();
        tracker.record("avail", true).unwrap();
        tracker.record("avail", false).unwrap();

        let status = tracker.status("avail").unwrap();
        assert!(status.meeting_slo); // 66% > 50%
    }

    #[test]
    fn not_meeting_slo_when_below_target() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("avail", 0.99)).unwrap();
        tracker.record("avail", true).unwrap();
        tracker.record("avail", false).unwrap();

        let status = tracker.status("avail").unwrap();
        assert!(!status.meeting_slo); // 50% < 99%
    }

    #[test]
    fn empty_slo_is_meeting() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("avail", 0.99)).unwrap();

        let status = tracker.status("avail").unwrap();
        assert!(status.meeting_slo); // No observations = 100%
        assert_eq!(status.current_ratio, 1.0);
    }

    #[test]
    fn duplicate_slo_rejected() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("avail", 0.99)).unwrap();
        assert!(matches!(
            tracker.register(test_slo("avail", 0.99)),
            Err(SloError::Duplicate(_))
        ));
    }

    #[test]
    fn invalid_target_rejected() {
        let mut tracker = SloTracker::new();
        assert!(matches!(
            tracker.register(test_slo("bad", 0.0)),
            Err(SloError::InvalidTarget(_))
        ));
        assert!(matches!(
            tracker.register(test_slo("bad", 1.5)),
            Err(SloError::InvalidTarget(_))
        ));
    }

    #[test]
    fn record_unknown_slo_fails() {
        let mut tracker = SloTracker::new();
        assert!(matches!(
            tracker.record("unknown", true),
            Err(SloError::NotFound(_))
        ));
    }

    #[test]
    fn burn_rate_calculation() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("avail", 0.99)).unwrap();
        // 10 good, 1 bad = 9.09% error rate vs 1% budget = ~9.09 burn rate
        for _ in 0..10 {
            tracker.record("avail", true).unwrap();
        }
        tracker.record("avail", false).unwrap();

        let status = tracker.status("avail").unwrap();
        assert!(status.burn_rate > 1.0); // Burning faster than budget
    }

    #[test]
    fn error_budget_remaining() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("avail", 0.99)).unwrap();
        // All good = full budget
        for _ in 0..100 {
            tracker.record("avail", true).unwrap();
        }

        let status = tracker.status("avail").unwrap();
        assert_eq!(status.error_budget_remaining, 1.0);
    }

    #[test]
    fn all_statuses() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("a", 0.99)).unwrap();
        tracker.register(test_slo("b", 0.95)).unwrap();
        assert_eq!(tracker.all_statuses().len(), 2);
    }

    #[test]
    fn register_defaults() {
        let mut tracker = SloTracker::new();
        tracker.register_defaults();
        assert_eq!(tracker.slo_count(), 4);
    }

    #[test]
    fn slo_target_boundary_1_0() {
        let mut tracker = SloTracker::new();
        tracker.register(test_slo("perfect", 1.0)).unwrap();
        tracker.record("perfect", true).unwrap();
        let status = tracker.status("perfect").unwrap();
        assert!(status.meeting_slo);
    }
}
