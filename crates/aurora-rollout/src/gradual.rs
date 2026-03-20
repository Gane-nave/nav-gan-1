//! Gradual rollout — percentage-based rollouts with cohort tracking.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rollout stage — tracks the current rollout percentage and cohort.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutStage {
    pub name: String,
    pub percentage: f64,
    pub description: String,
}

/// Rollout plan — a sequence of stages for gradual feature release.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutPlan {
    pub feature_name: String,
    pub stages: Vec<RolloutStage>,
    pub current_stage_index: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl RolloutPlan {
    /// Create a new rollout plan.
    pub fn new(feature_name: &str, stages: Vec<RolloutStage>) -> Self {
        Self {
            feature_name: feature_name.to_string(),
            stages,
            current_stage_index: 0,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create a standard canary rollout plan (1% → 10% → 50% → 100%).
    pub fn canary(feature_name: &str) -> Self {
        Self::new(
            feature_name,
            vec![
                RolloutStage {
                    name: "canary".to_string(),
                    percentage: 1.0,
                    description: "Initial canary release".to_string(),
                },
                RolloutStage {
                    name: "early_adopters".to_string(),
                    percentage: 10.0,
                    description: "Early adopter rollout".to_string(),
                },
                RolloutStage {
                    name: "half".to_string(),
                    percentage: 50.0,
                    description: "50% rollout".to_string(),
                },
                RolloutStage {
                    name: "general".to_string(),
                    percentage: 100.0,
                    description: "General availability".to_string(),
                },
            ],
        )
    }

    /// Get the current stage.
    pub fn current_stage(&self) -> Option<&RolloutStage> {
        self.stages.get(self.current_stage_index)
    }

    /// Get the current rollout percentage.
    pub fn current_percentage(&self) -> f64 {
        self.current_stage().map(|s| s.percentage).unwrap_or(0.0)
    }

    /// Advance to the next stage. Returns the new stage or None if already at the last stage.
    pub fn advance(&mut self) -> Option<&RolloutStage> {
        if self.current_stage_index + 1 < self.stages.len() {
            self.current_stage_index += 1;
            self.current_stage()
        } else {
            None
        }
    }

    /// Roll back to the previous stage. Returns the new stage or None if already at the first stage.
    pub fn rollback(&mut self) -> Option<&RolloutStage> {
        if self.current_stage_index > 0 {
            self.current_stage_index -= 1;
            self.current_stage()
        } else {
            None
        }
    }

    /// Check if the rollout is complete (at 100%).
    pub fn is_complete(&self) -> bool {
        self.current_percentage() >= 100.0
    }

    /// Total number of stages.
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}

/// Gradual rollout manager — manages multiple concurrent rollouts.
pub struct RolloutManager {
    plans: RwLock<HashMap<String, RolloutPlan>>,
}

impl RolloutManager {
    /// Create a new rollout manager.
    pub fn new() -> Self {
        Self {
            plans: RwLock::new(HashMap::new()),
        }
    }

    /// Register a rollout plan.
    pub fn register_plan(&self, plan: RolloutPlan) {
        self.plans.write().insert(plan.feature_name.clone(), plan);
    }

    /// Check if a user should see a feature based on the rollout percentage.
    /// Uses a deterministic hash of (feature_name, user_id) to ensure consistent assignment.
    pub fn is_user_included(&self, feature_name: &str, user_id: &str) -> bool {
        let plans = self.plans.read();
        let plan = match plans.get(feature_name) {
            Some(p) => p,
            None => return false,
        };

        let percentage = plan.current_percentage();
        if percentage >= 100.0 {
            return true;
        }
        if percentage <= 0.0 {
            return false;
        }

        // Deterministic hash for consistent user assignment
        let hash = Self::hash_user(feature_name, user_id);
        let bucket = (hash % 10000) as f64 / 100.0; // 0.00 to 99.99
        bucket < percentage
    }

    /// Deterministic hash for user bucketing.
    fn hash_user(feature: &str, user_id: &str) -> u64 {
        let mut hash: u64 = 5381;
        for byte in feature
            .bytes()
            .chain(b"\0".iter().copied())
            .chain(user_id.bytes())
        {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash
    }

    /// Advance a rollout to the next stage.
    pub fn advance(&self, feature_name: &str) -> Option<f64> {
        let mut plans = self.plans.write();
        let plan = plans.get_mut(feature_name)?;
        plan.advance();
        Some(plan.current_percentage())
    }

    /// Roll back a rollout to the previous stage.
    pub fn rollback(&self, feature_name: &str) -> Option<f64> {
        let mut plans = self.plans.write();
        let plan = plans.get_mut(feature_name)?;
        plan.rollback();
        Some(plan.current_percentage())
    }

    /// Get current rollout info for a feature.
    pub fn get_plan(&self, feature_name: &str) -> Option<RolloutPlan> {
        self.plans.read().get(feature_name).cloned()
    }

    /// List all active rollouts.
    pub fn active_rollouts(&self) -> Vec<RolloutPlan> {
        self.plans
            .read()
            .values()
            .filter(|p| !p.is_complete())
            .cloned()
            .collect()
    }

    /// Number of registered rollouts.
    pub fn plan_count(&self) -> usize {
        self.plans.read().len()
    }
}

impl Default for RolloutManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rollout_plan_canary() {
        let plan = RolloutPlan::canary("new_routing");
        assert_eq!(plan.stage_count(), 4);
        assert_eq!(plan.current_percentage(), 1.0);
        assert!(!plan.is_complete());
    }

    #[test]
    fn test_rollout_advance() {
        let mut plan = RolloutPlan::canary("feature_x");
        assert_eq!(plan.current_percentage(), 1.0);

        plan.advance();
        assert_eq!(plan.current_percentage(), 10.0);

        plan.advance();
        assert_eq!(plan.current_percentage(), 50.0);

        plan.advance();
        assert_eq!(plan.current_percentage(), 100.0);
        assert!(plan.is_complete());

        // Can't advance past last stage
        assert!(plan.advance().is_none());
        assert_eq!(plan.current_percentage(), 100.0);
    }

    #[test]
    fn test_rollout_rollback() {
        let mut plan = RolloutPlan::canary("feature_y");
        plan.advance(); // 10%
        plan.advance(); // 50%

        plan.rollback();
        assert_eq!(plan.current_percentage(), 10.0);

        plan.rollback();
        assert_eq!(plan.current_percentage(), 1.0);

        // Can't rollback past first stage
        assert!(plan.rollback().is_none());
        assert_eq!(plan.current_percentage(), 1.0);
    }

    #[test]
    fn test_rollout_manager_user_inclusion() {
        let mgr = RolloutManager::new();
        let mut plan = RolloutPlan::canary("test_feature");
        // Advance to 100% so all users are included
        plan.advance();
        plan.advance();
        plan.advance();
        mgr.register_plan(plan);

        assert!(mgr.is_user_included("test_feature", "any_user"));
    }

    #[test]
    fn test_rollout_manager_zero_percent() {
        let mgr = RolloutManager::new();
        mgr.register_plan(RolloutPlan::new(
            "disabled_feature",
            vec![RolloutStage {
                name: "off".to_string(),
                percentage: 0.0,
                description: "Disabled".to_string(),
            }],
        ));

        assert!(!mgr.is_user_included("disabled_feature", "any_user"));
    }

    #[test]
    fn test_rollout_manager_nonexistent_feature() {
        let mgr = RolloutManager::new();
        assert!(!mgr.is_user_included("nonexistent", "user_1"));
    }

    #[test]
    fn test_rollout_manager_deterministic_assignment() {
        let mgr = RolloutManager::new();
        mgr.register_plan(RolloutPlan::canary("test_consistency"));

        // Same user always gets the same result
        let r1 = mgr.is_user_included("test_consistency", "user_42");
        let r2 = mgr.is_user_included("test_consistency", "user_42");
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_rollout_manager_advance_and_rollback() {
        let mgr = RolloutManager::new();
        mgr.register_plan(RolloutPlan::canary("feature_z"));

        let pct = mgr.advance("feature_z").unwrap();
        assert_eq!(pct, 10.0);

        let pct = mgr.rollback("feature_z").unwrap();
        assert_eq!(pct, 1.0);
    }

    #[test]
    fn test_active_rollouts() {
        let mgr = RolloutManager::new();
        mgr.register_plan(RolloutPlan::canary("active_feature"));
        let mut complete = RolloutPlan::canary("done_feature");
        complete.advance();
        complete.advance();
        complete.advance(); // 100%
        mgr.register_plan(complete);

        let active = mgr.active_rollouts();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].feature_name, "active_feature");
    }

    #[test]
    fn test_plan_count() {
        let mgr = RolloutManager::new();
        mgr.register_plan(RolloutPlan::canary("a"));
        mgr.register_plan(RolloutPlan::canary("b"));
        assert_eq!(mgr.plan_count(), 2);
    }
}
