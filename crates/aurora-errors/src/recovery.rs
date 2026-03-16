//! Recovery strategies — fallback handlers, graceful degradation, and recovery orchestration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Recovery action to take when an error occurs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryAction {
    /// Retry the operation.
    Retry,
    /// Use a cached/fallback value.
    Fallback,
    /// Skip the operation and continue.
    Skip,
    /// Escalate to a higher-level handler.
    Escalate,
    /// Shut down the affected subsystem gracefully.
    GracefulShutdown,
    /// Switch to a degraded mode of operation.
    Degrade,
    /// No action — log and continue.
    LogOnly,
}

/// Recovery policy — maps error conditions to recovery actions.
#[derive(Debug, Clone)]
pub struct RecoveryPolicy {
    name: String,
    rules: Vec<RecoveryRule>,
    default_action: RecoveryAction,
}

/// A single recovery rule.
#[derive(Debug, Clone)]
pub struct RecoveryRule {
    pub category: super::classification::ErrorCategory,
    pub min_severity: super::classification::Severity,
    pub action: RecoveryAction,
    pub description: String,
}

impl RecoveryPolicy {
    /// Create a new recovery policy with a default action.
    pub fn new(name: &str, default_action: RecoveryAction) -> Self {
        Self {
            name: name.to_string(),
            rules: Vec::new(),
            default_action,
        }
    }

    /// Add a recovery rule.
    pub fn add_rule(&mut self, rule: RecoveryRule) {
        self.rules.push(rule);
    }

    /// Determine the recovery action for an error.
    pub fn action_for(
        &self,
        category: super::classification::ErrorCategory,
        severity: super::classification::Severity,
    ) -> &RecoveryAction {
        // Find the first matching rule (most specific first)
        for rule in &self.rules {
            if rule.category == category && severity >= rule.min_severity {
                return &rule.action;
            }
        }
        &self.default_action
    }

    /// Get the policy name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all rules.
    pub fn rules(&self) -> &[RecoveryRule] {
        &self.rules
    }
}

/// Degradation level — how much functionality is reduced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DegradationLevel {
    /// Full functionality.
    Normal,
    /// Minor features disabled.
    Minor,
    /// Significant features disabled, core still works.
    Moderate,
    /// Only essential functionality available.
    Severe,
    /// Minimal operation — emergency mode.
    Emergency,
}

/// Degradation manager — tracks and manages system degradation state.
pub struct DegradationManager {
    level: parking_lot::RwLock<DegradationLevel>,
    disabled_features: parking_lot::RwLock<Vec<String>>,
    feature_requirements: HashMap<String, DegradationLevel>,
}

impl DegradationManager {
    /// Create a new degradation manager.
    pub fn new() -> Self {
        Self {
            level: parking_lot::RwLock::new(DegradationLevel::Normal),
            disabled_features: parking_lot::RwLock::new(Vec::new()),
            feature_requirements: HashMap::new(),
        }
    }

    /// Register a feature with its minimum required degradation level.
    pub fn register_feature(&mut self, name: &str, min_level: DegradationLevel) {
        self.feature_requirements
            .insert(name.to_string(), min_level);
    }

    /// Set the current degradation level.
    pub fn set_level(&self, level: DegradationLevel) {
        *self.level.write() = level;

        // Update disabled features based on new level
        let mut disabled = self.disabled_features.write();
        disabled.clear();
        for (feature, min_level) in &self.feature_requirements {
            if level > *min_level {
                disabled.push(feature.clone());
            }
        }
    }

    /// Get the current degradation level.
    pub fn level(&self) -> DegradationLevel {
        *self.level.read()
    }

    /// Check if a specific feature is available at the current degradation level.
    pub fn is_feature_available(&self, feature: &str) -> bool {
        if let Some(min_level) = self.feature_requirements.get(feature) {
            *self.level.read() <= *min_level
        } else {
            // Unknown features are assumed always available
            true
        }
    }

    /// Get list of currently disabled features.
    pub fn disabled_features(&self) -> Vec<String> {
        self.disabled_features.read().clone()
    }

    /// Get a summary of the degradation state.
    pub fn summary(&self) -> DegradationSummary {
        DegradationSummary {
            level: self.level(),
            disabled_features: self.disabled_features(),
            total_features: self.feature_requirements.len(),
            available_features: self.feature_requirements.len()
                - self.disabled_features.read().len(),
        }
    }
}

impl Default for DegradationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of degradation state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationSummary {
    pub level: DegradationLevel,
    pub disabled_features: Vec<String>,
    pub total_features: usize,
    pub available_features: usize,
}

/// Health check result for a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub subsystem: String,
    pub healthy: bool,
    pub message: String,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// Health monitor — tracks subsystem health and triggers recovery.
pub struct HealthMonitor {
    checks: parking_lot::RwLock<HashMap<String, HealthCheckResult>>,
}

impl HealthMonitor {
    /// Create a new health monitor.
    pub fn new() -> Self {
        Self {
            checks: parking_lot::RwLock::new(HashMap::new()),
        }
    }

    /// Record a health check result.
    pub fn record_check(&self, result: HealthCheckResult) {
        self.checks.write().insert(result.subsystem.clone(), result);
    }

    /// Get the health status of a subsystem.
    pub fn status(&self, subsystem: &str) -> Option<HealthCheckResult> {
        self.checks.read().get(subsystem).cloned()
    }

    /// Check if all subsystems are healthy.
    pub fn all_healthy(&self) -> bool {
        self.checks.read().values().all(|r| r.healthy)
    }

    /// Get unhealthy subsystems.
    pub fn unhealthy(&self) -> Vec<HealthCheckResult> {
        self.checks
            .read()
            .values()
            .filter(|r| !r.healthy)
            .cloned()
            .collect()
    }

    /// Total number of monitored subsystems.
    pub fn subsystem_count(&self) -> usize {
        self.checks.read().len()
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classification::{ErrorCategory, Severity};

    #[test]
    fn test_recovery_policy_default_action() {
        let policy = RecoveryPolicy::new("test", RecoveryAction::LogOnly);
        let action = policy.action_for(ErrorCategory::Network, Severity::Warning);
        assert_eq!(*action, RecoveryAction::LogOnly);
    }

    #[test]
    fn test_recovery_policy_matching_rule() {
        let mut policy = RecoveryPolicy::new("nav", RecoveryAction::LogOnly);
        policy.add_rule(RecoveryRule {
            category: ErrorCategory::Network,
            min_severity: Severity::Warning,
            action: RecoveryAction::Retry,
            description: "Retry network errors".to_string(),
        });
        policy.add_rule(RecoveryRule {
            category: ErrorCategory::Hardware,
            min_severity: Severity::Critical,
            action: RecoveryAction::GracefulShutdown,
            description: "Shutdown on critical hardware failure".to_string(),
        });

        assert_eq!(
            *policy.action_for(ErrorCategory::Network, Severity::Error),
            RecoveryAction::Retry
        );
        assert_eq!(
            *policy.action_for(ErrorCategory::Hardware, Severity::Critical),
            RecoveryAction::GracefulShutdown
        );
        // Below min severity — falls through to default
        assert_eq!(
            *policy.action_for(ErrorCategory::Hardware, Severity::Warning),
            RecoveryAction::LogOnly
        );
    }

    #[test]
    fn test_degradation_levels_ordering() {
        assert!(DegradationLevel::Normal < DegradationLevel::Minor);
        assert!(DegradationLevel::Minor < DegradationLevel::Moderate);
        assert!(DegradationLevel::Moderate < DegradationLevel::Severe);
        assert!(DegradationLevel::Severe < DegradationLevel::Emergency);
    }

    #[test]
    fn test_degradation_manager_feature_availability() {
        let mut mgr = DegradationManager::new();
        mgr.register_feature("3d_view", DegradationLevel::Minor);
        mgr.register_feature("voice_nav", DegradationLevel::Moderate);
        mgr.register_feature("basic_routing", DegradationLevel::Emergency);

        // Normal — all available
        assert!(mgr.is_feature_available("3d_view"));
        assert!(mgr.is_feature_available("voice_nav"));
        assert!(mgr.is_feature_available("basic_routing"));

        // Moderate — 3d_view disabled
        mgr.set_level(DegradationLevel::Moderate);
        assert!(!mgr.is_feature_available("3d_view"));
        assert!(mgr.is_feature_available("voice_nav"));
        assert!(mgr.is_feature_available("basic_routing"));

        // Severe — 3d_view and voice_nav disabled
        mgr.set_level(DegradationLevel::Severe);
        assert!(!mgr.is_feature_available("3d_view"));
        assert!(!mgr.is_feature_available("voice_nav"));
        assert!(mgr.is_feature_available("basic_routing"));
    }

    #[test]
    fn test_degradation_manager_summary() {
        let mut mgr = DegradationManager::new();
        mgr.register_feature("a", DegradationLevel::Minor);
        mgr.register_feature("b", DegradationLevel::Severe);

        mgr.set_level(DegradationLevel::Moderate);
        let summary = mgr.summary();
        assert_eq!(summary.level, DegradationLevel::Moderate);
        assert_eq!(summary.total_features, 2);
        assert_eq!(summary.disabled_features.len(), 1);
        assert_eq!(summary.available_features, 1);
    }

    #[test]
    fn test_health_monitor() {
        let monitor = HealthMonitor::new();
        monitor.record_check(HealthCheckResult {
            subsystem: "gnss".to_string(),
            healthy: true,
            message: "OK".to_string(),
            last_check: chrono::Utc::now(),
        });
        monitor.record_check(HealthCheckResult {
            subsystem: "imu".to_string(),
            healthy: false,
            message: "No data".to_string(),
            last_check: chrono::Utc::now(),
        });

        assert!(!monitor.all_healthy());
        assert_eq!(monitor.unhealthy().len(), 1);
        assert_eq!(monitor.unhealthy()[0].subsystem, "imu");
        assert_eq!(monitor.subsystem_count(), 2);
    }

    #[test]
    fn test_health_monitor_all_healthy() {
        let monitor = HealthMonitor::new();
        monitor.record_check(HealthCheckResult {
            subsystem: "a".to_string(),
            healthy: true,
            message: "OK".to_string(),
            last_check: chrono::Utc::now(),
        });
        assert!(monitor.all_healthy());
    }

    #[test]
    fn test_unknown_feature_always_available() {
        let mgr = DegradationManager::new();
        mgr.set_level(DegradationLevel::Emergency);
        // Unknown features should be available
        assert!(mgr.is_feature_available("unknown_feature"));
    }
}
