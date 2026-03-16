//! Self-healing engine — automatically detects and repairs common system issues.

use std::collections::HashMap;
use std::time::Instant;

/// Repair action type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairAction {
    /// Restart the subsystem.
    Restart,
    /// Clear and rebuild caches.
    ClearCache,
    /// Re-establish connections.
    Reconnect,
    /// Fall back to backup data source.
    Fallback,
    /// Reduce resource usage.
    ReduceLoad,
    /// Reset to known good state.
    Reset,
    /// No action needed.
    None,
}

/// Repair result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairResult {
    /// Repair succeeded.
    Success,
    /// Repair partially fixed the issue.
    Partial,
    /// Repair failed.
    Failed,
    /// Repair was not attempted.
    Skipped,
}

/// A repair rule — maps a condition to a repair action.
#[derive(Debug, Clone)]
pub struct RepairRule {
    /// Rule name.
    pub name: String,
    /// Target subsystem.
    pub subsystem: String,
    /// Condition description.
    pub condition: String,
    /// Action to take.
    pub action: RepairAction,
    /// Maximum attempts before giving up.
    pub max_attempts: u32,
    /// Cooldown between attempts (seconds).
    pub cooldown_secs: u64,
}

/// Record of a repair attempt.
#[derive(Debug, Clone)]
pub struct RepairRecord {
    /// Rule that triggered the repair.
    pub rule_name: String,
    /// Subsystem.
    pub subsystem: String,
    /// Action taken.
    pub action: RepairAction,
    /// Result.
    pub result: RepairResult,
    /// Timestamp.
    pub attempted_at: Instant,
    /// Attempt number.
    pub attempt: u32,
}

/// Self-healing engine — manages repair rules and executes auto-recovery.
pub struct SelfHealEngine {
    rules: Vec<RepairRule>,
    history: Vec<RepairRecord>,
    attempt_counts: HashMap<String, u32>,
    last_attempt: HashMap<String, Instant>,
    max_history: usize,
}

impl SelfHealEngine {
    /// Create a new self-healing engine.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            history: Vec::new(),
            attempt_counts: HashMap::new(),
            last_attempt: HashMap::new(),
            max_history: 10000,
        }
    }

    /// Add a repair rule.
    pub fn add_rule(&mut self, rule: RepairRule) {
        self.rules.push(rule);
    }

    /// Get rule count.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Check if a repair can be attempted for a rule (respects cooldown and max attempts).
    pub fn can_attempt(&self, rule_name: &str) -> bool {
        let rule = match self.rules.iter().find(|r| r.name == rule_name) {
            Some(r) => r,
            None => return false,
        };

        let attempts = self.attempt_counts.get(rule_name).copied().unwrap_or(0);
        if attempts >= rule.max_attempts {
            return false;
        }

        if let Some(last) = self.last_attempt.get(rule_name) {
            if last.elapsed().as_secs() < rule.cooldown_secs {
                return false;
            }
        }

        true
    }

    /// Execute a repair action for a given rule name.
    pub fn attempt_repair(
        &mut self,
        rule_name: &str,
        result: RepairResult,
    ) -> Option<RepairRecord> {
        if !self.can_attempt(rule_name) {
            return None;
        }

        let rule = self.rules.iter().find(|r| r.name == rule_name)?.clone();
        let attempt = self.attempt_counts.get(rule_name).copied().unwrap_or(0) + 1;

        let record = RepairRecord {
            rule_name: rule_name.to_string(),
            subsystem: rule.subsystem.clone(),
            action: rule.action,
            result,
            attempted_at: Instant::now(),
            attempt,
        };

        self.attempt_counts.insert(rule_name.to_string(), attempt);
        self.last_attempt
            .insert(rule_name.to_string(), Instant::now());

        self.history.push(record.clone());
        while self.history.len() > self.max_history {
            self.history.remove(0);
        }

        Some(record)
    }

    /// Reset attempt counter for a rule (e.g., after manual fix).
    pub fn reset_attempts(&mut self, rule_name: &str) {
        self.attempt_counts.remove(rule_name);
        self.last_attempt.remove(rule_name);
    }

    /// Get repair history.
    pub fn history(&self) -> &[RepairRecord] {
        &self.history
    }

    /// Get history count.
    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    /// Get success rate across all repairs.
    pub fn success_rate(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        let successes = self
            .history
            .iter()
            .filter(|r| r.result == RepairResult::Success)
            .count();
        successes as f64 / self.history.len() as f64
    }

    /// Get repairs for a specific subsystem.
    pub fn repairs_for(&self, subsystem: &str) -> Vec<&RepairRecord> {
        self.history
            .iter()
            .filter(|r| r.subsystem == subsystem)
            .collect()
    }

    /// Get the recommended action for a subsystem issue.
    pub fn recommend_action(&self, subsystem: &str) -> RepairAction {
        for rule in &self.rules {
            if rule.subsystem == subsystem && self.can_attempt(&rule.name) {
                return rule.action;
            }
        }
        RepairAction::None
    }
}

impl Default for SelfHealEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gnss_restart_rule() -> RepairRule {
        RepairRule {
            name: "gnss_restart".to_string(),
            subsystem: "gnss".to_string(),
            condition: "GNSS signal lost".to_string(),
            action: RepairAction::Restart,
            max_attempts: 3,
            cooldown_secs: 0,
        }
    }

    fn cache_clear_rule() -> RepairRule {
        RepairRule {
            name: "map_cache_clear".to_string(),
            subsystem: "map".to_string(),
            condition: "Map cache corrupted".to_string(),
            action: RepairAction::ClearCache,
            max_attempts: 2,
            cooldown_secs: 0,
        }
    }

    #[test]
    fn test_add_rule() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());
        assert_eq!(engine.rule_count(), 1);
    }

    #[test]
    fn test_attempt_repair_success() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());
        let record = engine.attempt_repair("gnss_restart", RepairResult::Success);
        assert!(record.is_some());
        let record = record.unwrap();
        assert_eq!(record.action, RepairAction::Restart);
        assert_eq!(record.result, RepairResult::Success);
        assert_eq!(record.attempt, 1);
    }

    #[test]
    fn test_max_attempts_enforced() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule()); // max_attempts=3

        // 3 attempts succeed
        assert!(engine
            .attempt_repair("gnss_restart", RepairResult::Failed)
            .is_some());
        assert!(engine
            .attempt_repair("gnss_restart", RepairResult::Failed)
            .is_some());
        assert!(engine
            .attempt_repair("gnss_restart", RepairResult::Failed)
            .is_some());
        // 4th attempt blocked
        assert!(engine
            .attempt_repair("gnss_restart", RepairResult::Failed)
            .is_none());
    }

    #[test]
    fn test_reset_attempts() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());

        for _ in 0..3 {
            engine.attempt_repair("gnss_restart", RepairResult::Failed);
        }
        assert!(!engine.can_attempt("gnss_restart"));

        engine.reset_attempts("gnss_restart");
        assert!(engine.can_attempt("gnss_restart"));
    }

    #[test]
    fn test_nonexistent_rule_returns_none() {
        let mut engine = SelfHealEngine::new();
        assert!(engine
            .attempt_repair("nonexistent", RepairResult::Success)
            .is_none());
    }

    #[test]
    fn test_history() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());
        engine.add_rule(cache_clear_rule());

        engine.attempt_repair("gnss_restart", RepairResult::Success);
        engine.attempt_repair("map_cache_clear", RepairResult::Failed);

        assert_eq!(engine.history_count(), 2);
    }

    #[test]
    fn test_success_rate() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());
        engine.add_rule(cache_clear_rule());

        engine.attempt_repair("gnss_restart", RepairResult::Success);
        engine.attempt_repair("map_cache_clear", RepairResult::Failed);

        assert!((engine.success_rate() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_success_rate_empty() {
        let engine = SelfHealEngine::new();
        assert_eq!(engine.success_rate(), 0.0);
    }

    #[test]
    fn test_repairs_for_subsystem() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());
        engine.add_rule(cache_clear_rule());

        engine.attempt_repair("gnss_restart", RepairResult::Success);
        engine.attempt_repair("map_cache_clear", RepairResult::Failed);

        let gnss_repairs = engine.repairs_for("gnss");
        assert_eq!(gnss_repairs.len(), 1);
        assert_eq!(gnss_repairs[0].subsystem, "gnss");
    }

    #[test]
    fn test_recommend_action() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());

        assert_eq!(engine.recommend_action("gnss"), RepairAction::Restart);
        assert_eq!(engine.recommend_action("unknown"), RepairAction::None);
    }

    #[test]
    fn test_recommend_action_exhausted() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());

        for _ in 0..3 {
            engine.attempt_repair("gnss_restart", RepairResult::Failed);
        }
        // All attempts exhausted — no action recommended
        assert_eq!(engine.recommend_action("gnss"), RepairAction::None);
    }

    #[test]
    fn test_attempt_increments_counter() {
        let mut engine = SelfHealEngine::new();
        engine.add_rule(gnss_restart_rule());

        let r1 = engine
            .attempt_repair("gnss_restart", RepairResult::Failed)
            .unwrap();
        assert_eq!(r1.attempt, 1);
        let r2 = engine
            .attempt_repair("gnss_restart", RepairResult::Success)
            .unwrap();
        assert_eq!(r2.attempt, 2);
    }
}
