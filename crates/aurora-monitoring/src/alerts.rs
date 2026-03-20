//! Alerting rules engine.
//!
//! Defines configurable alert rules that monitor metric values and
//! trigger notifications when thresholds are breached.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum AlertError {
    #[error("alert rule not found: {0}")]
    RuleNotFound(String),
    #[error("duplicate rule name: {0}")]
    DuplicateRule(String),
    #[error("invalid threshold: {0}")]
    InvalidThreshold(String),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Severity levels for alerts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Critical,
    Emergency,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Critical => "critical",
            Severity::Emergency => "emergency",
        }
    }
}

/// Comparison operator for threshold checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

impl Operator {
    pub fn evaluate(&self, value: f64, threshold: f64) -> bool {
        match self {
            Operator::GreaterThan => value > threshold,
            Operator::GreaterThanOrEqual => value >= threshold,
            Operator::LessThan => value < threshold,
            Operator::LessThanOrEqual => value <= threshold,
            Operator::Equal => (value - threshold).abs() < f64::EPSILON,
            Operator::NotEqual => (value - threshold).abs() >= f64::EPSILON,
        }
    }
}

/// An alert rule definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Unique name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Metric name to monitor.
    pub metric_name: String,
    /// Comparison operator.
    pub operator: Operator,
    /// Threshold value.
    pub threshold: f64,
    /// Severity when triggered.
    pub severity: Severity,
    /// How long the condition must persist before firing.
    pub for_duration: Duration,
    /// Whether the rule is enabled.
    pub enabled: bool,
    /// Labels for grouping/routing.
    pub labels: HashMap<String, String>,
}

/// State of an alert rule evaluation.
#[derive(Debug, Clone)]
pub enum AlertState {
    /// Condition not met.
    Ok,
    /// Condition met but within `for_duration`.
    Pending { since: DateTime<Utc> },
    /// Condition met for longer than `for_duration`.
    Firing {
        since: DateTime<Utc>,
        last_value: f64,
    },
}

/// A fired alert instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiredAlert {
    pub rule_name: String,
    pub severity: Severity,
    pub metric_value: f64,
    pub threshold: f64,
    pub fired_at: DateTime<Utc>,
    pub description: String,
    pub labels: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Alert engine
// ---------------------------------------------------------------------------

/// Manages alert rules and evaluates them against metric values.
pub struct AlertEngine {
    rules: HashMap<String, AlertRule>,
    states: HashMap<String, AlertState>,
    fired: Vec<FiredAlert>,
}

impl AlertEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            states: HashMap::new(),
            fired: Vec::new(),
        }
    }

    /// Register an alert rule.
    pub fn add_rule(&mut self, rule: AlertRule) -> Result<(), AlertError> {
        if self.rules.contains_key(&rule.name) {
            return Err(AlertError::DuplicateRule(rule.name));
        }
        self.states.insert(rule.name.clone(), AlertState::Ok);
        self.rules.insert(rule.name.clone(), rule);
        Ok(())
    }

    /// Remove an alert rule.
    pub fn remove_rule(&mut self, name: &str) -> Result<(), AlertError> {
        if self.rules.remove(name).is_none() {
            return Err(AlertError::RuleNotFound(name.to_string()));
        }
        self.states.remove(name);
        Ok(())
    }

    /// Evaluate a metric value against all matching rules.
    pub fn evaluate(&mut self, metric_name: &str, value: f64) -> Vec<FiredAlert> {
        let now = Utc::now();
        let mut newly_fired = Vec::new();

        let matching_rules: Vec<AlertRule> = self
            .rules
            .values()
            .filter(|r| r.enabled && r.metric_name == metric_name)
            .cloned()
            .collect();

        for rule in &matching_rules {
            let condition_met = rule.operator.evaluate(value, rule.threshold);
            let state = self
                .states
                .entry(rule.name.clone())
                .or_insert(AlertState::Ok);

            *state = if condition_met {
                match state {
                    AlertState::Ok => {
                        if rule.for_duration.is_zero() {
                            // Zero for_duration: fire immediately
                            let alert = FiredAlert {
                                rule_name: rule.name.clone(),
                                severity: rule.severity,
                                metric_value: value,
                                threshold: rule.threshold,
                                fired_at: now,
                                description: rule.description.clone(),
                                labels: rule.labels.clone(),
                            };
                            newly_fired.push(alert.clone());
                            self.fired.push(alert);
                            AlertState::Firing {
                                since: now,
                                last_value: value,
                            }
                        } else {
                            AlertState::Pending { since: now }
                        }
                    }
                    AlertState::Pending { since } => {
                        if now - *since >= rule.for_duration {
                            let alert = FiredAlert {
                                rule_name: rule.name.clone(),
                                severity: rule.severity,
                                metric_value: value,
                                threshold: rule.threshold,
                                fired_at: now,
                                description: rule.description.clone(),
                                labels: rule.labels.clone(),
                            };
                            newly_fired.push(alert.clone());
                            self.fired.push(alert);
                            AlertState::Firing {
                                since: *since,
                                last_value: value,
                            }
                        } else {
                            AlertState::Pending { since: *since }
                        }
                    }
                    AlertState::Firing { since, .. } => AlertState::Firing {
                        since: *since,
                        last_value: value,
                    },
                }
            } else {
                AlertState::Ok
            };
        }

        newly_fired
    }

    /// Get all currently firing alerts.
    pub fn firing_alerts(&self) -> Vec<&str> {
        self.states
            .iter()
            .filter_map(|(name, state)| {
                if matches!(state, AlertState::Firing { .. }) {
                    Some(name.as_str())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get the history of all fired alerts.
    pub fn alert_history(&self) -> &[FiredAlert] {
        &self.fired
    }

    /// Number of registered rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Register default AURORA NAV alert rules.
    pub fn register_defaults(&mut self) {
        let defaults = vec![
            AlertRule {
                name: "high_error_rate".to_string(),
                description: "Error rate exceeds 5%".to_string(),
                metric_name: "aurora_http_error_rate".to_string(),
                operator: Operator::GreaterThan,
                threshold: 0.05,
                severity: Severity::Critical,
                for_duration: Duration::minutes(5),
                enabled: true,
                labels: [("team".to_string(), "platform".to_string())]
                    .into_iter()
                    .collect(),
            },
            AlertRule {
                name: "high_latency_p99".to_string(),
                description: "P99 latency exceeds 500ms".to_string(),
                metric_name: "aurora_http_latency_p99".to_string(),
                operator: Operator::GreaterThan,
                threshold: 0.5,
                severity: Severity::Warning,
                for_duration: Duration::minutes(10),
                enabled: true,
                labels: [("team".to_string(), "platform".to_string())]
                    .into_iter()
                    .collect(),
            },
            AlertRule {
                name: "gnss_satellite_low".to_string(),
                description: "GNSS tracked satellites below minimum".to_string(),
                metric_name: "aurora_gnss_tracked_satellites".to_string(),
                operator: Operator::LessThan,
                threshold: 4.0,
                severity: Severity::Warning,
                for_duration: Duration::minutes(2),
                enabled: true,
                labels: [("team".to_string(), "navigation".to_string())]
                    .into_iter()
                    .collect(),
            },
            AlertRule {
                name: "integrity_degraded".to_string(),
                description: "Navigation integrity degraded".to_string(),
                metric_name: "aurora_integrity_score".to_string(),
                operator: Operator::LessThan,
                threshold: 0.8,
                severity: Severity::Critical,
                for_duration: Duration::minutes(1),
                enabled: true,
                labels: [("team".to_string(), "navigation".to_string())]
                    .into_iter()
                    .collect(),
            },
        ];

        for rule in defaults {
            let _ = self.add_rule(rule);
        }
    }
}

impl Default for AlertEngine {
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

    fn test_rule(name: &str, metric: &str, op: Operator, threshold: f64) -> AlertRule {
        AlertRule {
            name: name.to_string(),
            description: format!("Test rule: {name}"),
            metric_name: metric.to_string(),
            operator: op,
            threshold,
            severity: Severity::Warning,
            for_duration: Duration::zero(),
            enabled: true,
            labels: HashMap::new(),
        }
    }

    #[test]
    fn add_and_evaluate_rule() {
        let mut engine = AlertEngine::new();
        engine
            .add_rule(test_rule(
                "high_cpu",
                "cpu_usage",
                Operator::GreaterThan,
                0.9,
            ))
            .unwrap();

        // Below threshold — no alert
        let fired = engine.evaluate("cpu_usage", 0.5);
        assert!(fired.is_empty());

        // Above threshold — fires immediately (for_duration=0)
        let fired = engine.evaluate("cpu_usage", 0.95);
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].rule_name, "high_cpu");
    }

    #[test]
    fn duplicate_rule_rejected() {
        let mut engine = AlertEngine::new();
        engine
            .add_rule(test_rule("r1", "m", Operator::GreaterThan, 1.0))
            .unwrap();
        assert!(matches!(
            engine.add_rule(test_rule("r1", "m", Operator::GreaterThan, 1.0)),
            Err(AlertError::DuplicateRule(_))
        ));
    }

    #[test]
    fn remove_rule() {
        let mut engine = AlertEngine::new();
        engine
            .add_rule(test_rule("r1", "m", Operator::GreaterThan, 1.0))
            .unwrap();
        assert!(engine.remove_rule("r1").is_ok());
        assert!(matches!(
            engine.remove_rule("r1"),
            Err(AlertError::RuleNotFound(_))
        ));
    }

    #[test]
    fn operator_evaluations() {
        assert!(Operator::GreaterThan.evaluate(2.0, 1.0));
        assert!(!Operator::GreaterThan.evaluate(1.0, 2.0));
        assert!(Operator::LessThan.evaluate(1.0, 2.0));
        assert!(Operator::GreaterThanOrEqual.evaluate(1.0, 1.0));
        assert!(Operator::LessThanOrEqual.evaluate(1.0, 1.0));
        assert!(Operator::Equal.evaluate(1.0, 1.0));
        assert!(Operator::NotEqual.evaluate(1.0, 2.0));
    }

    #[test]
    fn condition_clears_when_metric_recovers() {
        let mut engine = AlertEngine::new();
        engine
            .add_rule(test_rule("high", "cpu", Operator::GreaterThan, 0.9))
            .unwrap();

        engine.evaluate("cpu", 0.95); // fires
        assert_eq!(engine.firing_alerts().len(), 1);

        engine.evaluate("cpu", 0.5); // recovers
        assert_eq!(engine.firing_alerts().len(), 0);
    }

    #[test]
    fn disabled_rule_not_evaluated() {
        let mut engine = AlertEngine::new();
        let mut rule = test_rule("r1", "m", Operator::GreaterThan, 1.0);
        rule.enabled = false;
        engine.add_rule(rule).unwrap();

        let fired = engine.evaluate("m", 999.0);
        assert!(fired.is_empty());
    }

    #[test]
    fn alert_history_preserved() {
        let mut engine = AlertEngine::new();
        engine
            .add_rule(test_rule("r1", "m", Operator::GreaterThan, 1.0))
            .unwrap();
        engine.evaluate("m", 2.0);
        engine.evaluate("m", 0.5); // recover
        engine.evaluate("m", 3.0); // fire again
                                   // First fire goes Pending→Firing, second goes Ok→Pending
                                   // With for_duration=0: first fires immediately, then recovers, then pending again
        assert!(!engine.alert_history().is_empty());
    }

    #[test]
    fn register_defaults() {
        let mut engine = AlertEngine::new();
        engine.register_defaults();
        assert_eq!(engine.rule_count(), 4);
    }

    #[test]
    fn severity_as_str() {
        assert_eq!(Severity::Info.as_str(), "info");
        assert_eq!(Severity::Warning.as_str(), "warning");
        assert_eq!(Severity::Critical.as_str(), "critical");
        assert_eq!(Severity::Emergency.as_str(), "emergency");
    }

    #[test]
    fn unrelated_metric_does_not_trigger() {
        let mut engine = AlertEngine::new();
        engine
            .add_rule(test_rule("r1", "cpu", Operator::GreaterThan, 0.9))
            .unwrap();
        let fired = engine.evaluate("memory", 999.0);
        assert!(fired.is_empty());
    }
}
