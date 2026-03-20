//! Rollout strategies — percentage-based, user-segment, gradual rollout for feature flags.

use std::collections::HashSet;

/// Rollout strategy type.
#[derive(Debug, Clone, PartialEq)]
pub enum RolloutStrategy {
    /// All users get the feature.
    AllUsers,
    /// No users get the feature.
    NoUsers,
    /// Percentage-based rollout (0–100).
    Percentage(f64),
    /// Specific user IDs.
    UserList(HashSet<String>),
    /// Gradual ramp — starts at `start_pct`, increases by `step_pct` every `interval_ms`.
    GradualRamp {
        start_pct: f64,
        step_pct: f64,
        interval_ms: u64,
        max_pct: f64,
    },
}

/// A rollout rule for a feature flag.
#[derive(Debug, Clone)]
pub struct RolloutRule {
    /// Rule identifier.
    pub id: String,
    /// Strategy to apply.
    pub strategy: RolloutStrategy,
    /// Priority (higher = evaluated first).
    pub priority: u32,
    /// Whether the rule is active.
    pub active: bool,
    /// Creation time.
    pub created_at_ms: u64,
}

impl RolloutRule {
    /// Create a new rollout rule.
    pub fn new(id: &str, strategy: RolloutStrategy, now_ms: u64) -> Self {
        Self {
            id: id.to_string(),
            strategy,
            priority: 0,
            active: true,
            created_at_ms: now_ms,
        }
    }

    /// Set priority.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Evaluate whether a user should see the feature.
    pub fn evaluate(&self, user_id: &str, now_ms: u64) -> bool {
        if !self.active {
            return false;
        }
        match &self.strategy {
            RolloutStrategy::AllUsers => true,
            RolloutStrategy::NoUsers => false,
            RolloutStrategy::Percentage(pct) => {
                let hash = simple_hash(user_id);
                let bucket = (hash % 10000) as f64 / 100.0;
                bucket < *pct
            }
            RolloutStrategy::UserList(users) => users.contains(user_id),
            RolloutStrategy::GradualRamp {
                start_pct,
                step_pct,
                interval_ms,
                max_pct,
            } => {
                let elapsed = now_ms.saturating_sub(self.created_at_ms);
                let steps = if *interval_ms > 0 {
                    elapsed / interval_ms
                } else {
                    0
                };
                let current_pct = (start_pct + step_pct * steps as f64).min(*max_pct);
                let hash = simple_hash(user_id);
                let bucket = (hash % 10000) as f64 / 100.0;
                bucket < current_pct
            }
        }
    }
}

/// Simple deterministic hash for user bucketing.
fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u64);
    }
    hash
}

/// Rollout manager — manages rollout rules for features.
pub struct RolloutManager {
    rules: Vec<RolloutRule>,
}

impl RolloutManager {
    /// Create a new rollout manager.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rollout rule.
    pub fn add_rule(&mut self, rule: RolloutRule) {
        self.rules.push(rule);
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Evaluate whether a user should see the feature.
    /// Returns the first matching active rule's result.
    pub fn evaluate(&self, user_id: &str, now_ms: u64) -> bool {
        for rule in &self.rules {
            if rule.active {
                return rule.evaluate(user_id, now_ms);
            }
        }
        false
    }

    /// Get total rule count.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Get active rule count.
    pub fn active_rule_count(&self) -> usize {
        self.rules.iter().filter(|r| r.active).count()
    }

    /// Deactivate a rule by ID.
    pub fn deactivate(&mut self, id: &str) -> bool {
        for rule in &mut self.rules {
            if rule.id == id {
                rule.active = false;
                return true;
            }
        }
        false
    }

    /// Remove a rule by ID.
    pub fn remove_rule(&mut self, id: &str) -> bool {
        let len_before = self.rules.len();
        self.rules.retain(|r| r.id != id);
        self.rules.len() < len_before
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
    fn test_all_users_strategy() {
        let rule = RolloutRule::new("r1", RolloutStrategy::AllUsers, 0);
        assert!(rule.evaluate("user1", 0));
        assert!(rule.evaluate("user2", 0));
    }

    #[test]
    fn test_no_users_strategy() {
        let rule = RolloutRule::new("r1", RolloutStrategy::NoUsers, 0);
        assert!(!rule.evaluate("user1", 0));
    }

    #[test]
    fn test_percentage_strategy() {
        let rule = RolloutRule::new("r1", RolloutStrategy::Percentage(50.0), 0);

        let mut true_count = 0;
        for i in 0..1000 {
            if rule.evaluate(&format!("user_{i}"), 0) {
                true_count += 1;
            }
        }
        // Should be roughly 50% (within tolerance)
        assert!(true_count > 350, "Expected ~500, got {true_count}");
        assert!(true_count < 650, "Expected ~500, got {true_count}");
    }

    #[test]
    fn test_percentage_deterministic() {
        let rule = RolloutRule::new("r1", RolloutStrategy::Percentage(50.0), 0);

        let result1 = rule.evaluate("user_42", 0);
        let result2 = rule.evaluate("user_42", 1000);
        assert_eq!(result1, result2, "Same user should get same result");
    }

    #[test]
    fn test_user_list_strategy() {
        let users: HashSet<String> = vec!["alice".to_string(), "bob".to_string()]
            .into_iter()
            .collect();
        let rule = RolloutRule::new("r1", RolloutStrategy::UserList(users), 0);

        assert!(rule.evaluate("alice", 0));
        assert!(rule.evaluate("bob", 0));
        assert!(!rule.evaluate("charlie", 0));
    }

    #[test]
    fn test_gradual_ramp() {
        let rule = RolloutRule::new(
            "r1",
            RolloutStrategy::GradualRamp {
                start_pct: 0.0,
                step_pct: 10.0,
                interval_ms: 1000,
                max_pct: 100.0,
            },
            0,
        );

        // At time 0, 0% should see it
        let mut count_0 = 0;
        for i in 0..1000 {
            if rule.evaluate(&format!("u{i}"), 0) {
                count_0 += 1;
            }
        }

        // At time 5000, 50% should see it
        let mut count_5 = 0;
        for i in 0..1000 {
            if rule.evaluate(&format!("u{i}"), 5000) {
                count_5 += 1;
            }
        }

        assert!(count_5 > count_0, "More users at 50% than 0%");
    }

    #[test]
    fn test_inactive_rule() {
        let mut rule = RolloutRule::new("r1", RolloutStrategy::AllUsers, 0);
        rule.active = false;
        assert!(!rule.evaluate("user1", 0));
    }

    #[test]
    fn test_rollout_manager_priority() {
        let mut mgr = RolloutManager::new();
        mgr.add_rule(RolloutRule::new("low", RolloutStrategy::NoUsers, 0).with_priority(1));
        mgr.add_rule(RolloutRule::new("high", RolloutStrategy::AllUsers, 0).with_priority(10));

        assert!(mgr.evaluate("user1", 0));
    }

    #[test]
    fn test_rollout_manager_deactivate() {
        let mut mgr = RolloutManager::new();
        mgr.add_rule(RolloutRule::new("r1", RolloutStrategy::AllUsers, 0).with_priority(10));
        mgr.add_rule(RolloutRule::new("r2", RolloutStrategy::NoUsers, 0).with_priority(1));

        assert!(mgr.evaluate("u", 0));
        mgr.deactivate("r1");
        assert!(!mgr.evaluate("u", 0));
    }

    #[test]
    fn test_rollout_manager_remove() {
        let mut mgr = RolloutManager::new();
        mgr.add_rule(RolloutRule::new("r1", RolloutStrategy::AllUsers, 0));
        assert_eq!(mgr.rule_count(), 1);
        assert!(mgr.remove_rule("r1"));
        assert_eq!(mgr.rule_count(), 0);
    }
}
