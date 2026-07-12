//! Quota policy — combines limits and trackers for enforcement.

use std::collections::HashMap;

/// Result of a quota enforcement check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotaDecision {
    /// Request is allowed.
    Allow,
    /// Request is denied (hard limit exceeded).
    Deny { reason: String },
    /// Request is allowed but a warning is issued.
    AllowWithWarning { reason: String },
}

impl QuotaDecision {
    /// Whether the decision allows the request.
    pub fn is_allowed(&self) -> bool {
        !matches!(self, QuotaDecision::Deny { .. })
    }

    /// Whether a warning was issued.
    pub fn has_warning(&self) -> bool {
        matches!(self, QuotaDecision::AllowWithWarning { .. })
    }
}

/// A quota rule: name, hard limit, soft limit.
#[derive(Debug, Clone)]
pub struct QuotaRule {
    /// Rule name.
    pub name: String,
    /// Hard limit (deny above this).
    pub hard_limit: u64,
    /// Soft limit (warn above this).
    pub soft_limit: u64,
    /// Whether the rule is active.
    pub active: bool,
}

impl QuotaRule {
    /// Create a new quota rule.
    pub fn new(name: &str, hard_limit: u64) -> Self {
        Self {
            name: name.to_string(),
            hard_limit,
            soft_limit: (hard_limit as f64 * 0.8) as u64,
            active: true,
        }
    }

    /// Set custom soft limit.
    pub fn with_soft_limit(mut self, soft_limit: u64) -> Self {
        self.soft_limit = soft_limit;
        self
    }

    /// Evaluate the rule against a current value.
    pub fn evaluate(&self, current: u64) -> QuotaDecision {
        if !self.active {
            return QuotaDecision::Allow;
        }
        if current > self.hard_limit {
            QuotaDecision::Deny {
                reason: format!(
                    "{}: usage {} exceeds hard limit {}",
                    self.name, current, self.hard_limit
                ),
            }
        } else if current > self.soft_limit {
            QuotaDecision::AllowWithWarning {
                reason: format!(
                    "{}: usage {} exceeds soft limit {}",
                    self.name, current, self.soft_limit
                ),
            }
        } else {
            QuotaDecision::Allow
        }
    }
}

/// A quota policy that manages multiple rules.
pub struct QuotaPolicy {
    rules: HashMap<String, QuotaRule>,
    enforcement_count: u64,
    deny_count: u64,
    warn_count: u64,
}

impl QuotaPolicy {
    /// Create a new empty policy.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            enforcement_count: 0,
            deny_count: 0,
            warn_count: 0,
        }
    }

    /// Add a rule to the policy.
    pub fn add_rule(&mut self, rule: QuotaRule) {
        self.rules.insert(rule.name.clone(), rule);
    }

    /// Remove a rule by name.
    pub fn remove_rule(&mut self, name: &str) -> bool {
        self.rules.remove(name).is_some()
    }

    /// Get a rule by name.
    pub fn get_rule(&self, name: &str) -> Option<&QuotaRule> {
        self.rules.get(name)
    }

    /// Enforce a single rule by name against a current value.
    pub fn enforce(&mut self, rule_name: &str, current: u64) -> QuotaDecision {
        self.enforcement_count += 1;
        let decision = match self.rules.get(rule_name) {
            Some(rule) => rule.evaluate(current),
            None => QuotaDecision::Allow, // no rule = allow
        };

        match &decision {
            QuotaDecision::Deny { .. } => self.deny_count += 1,
            QuotaDecision::AllowWithWarning { .. } => self.warn_count += 1,
            QuotaDecision::Allow => {}
        }

        decision
    }

    /// Enforce all rules against a map of current values.
    pub fn enforce_all(&mut self, values: &HashMap<String, u64>) -> Vec<QuotaDecision> {
        let mut decisions = Vec::new();
        let rule_names: Vec<String> = self.rules.keys().cloned().collect();
        for name in rule_names {
            let current = values.get(&name).copied().unwrap_or(0);
            let decision = self.enforce(&name, current);
            decisions.push(decision);
        }
        decisions
    }

    /// Number of rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Total enforcement checks performed.
    pub fn enforcement_count(&self) -> u64 {
        self.enforcement_count
    }

    /// Total denials.
    pub fn deny_count(&self) -> u64 {
        self.deny_count
    }

    /// Total warnings.
    pub fn warn_count(&self) -> u64 {
        self.warn_count
    }

    /// All rule names.
    pub fn rule_names(&self) -> Vec<&str> {
        self.rules.keys().map(|k| k.as_str()).collect()
    }
}

impl Default for QuotaPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_allow() {
        let rule = QuotaRule::new("mem", 1000);
        assert_eq!(rule.evaluate(500), QuotaDecision::Allow);
    }

    #[test]
    fn test_rule_warn() {
        let rule = QuotaRule::new("mem", 1000);
        let decision = rule.evaluate(850);
        assert!(decision.is_allowed());
        assert!(decision.has_warning());
    }

    #[test]
    fn test_rule_deny() {
        let rule = QuotaRule::new("mem", 1000);
        let decision = rule.evaluate(1100);
        assert!(!decision.is_allowed());
    }

    #[test]
    fn test_rule_inactive() {
        let mut rule = QuotaRule::new("mem", 1000);
        rule.active = false;
        assert_eq!(rule.evaluate(9999), QuotaDecision::Allow);
    }

    #[test]
    fn test_rule_custom_soft_limit() {
        let rule = QuotaRule::new("mem", 1000).with_soft_limit(900);
        assert_eq!(rule.evaluate(850), QuotaDecision::Allow);
        assert!(rule.evaluate(950).has_warning());
    }

    #[test]
    fn test_policy_add_enforce() {
        let mut policy = QuotaPolicy::new();
        policy.add_rule(QuotaRule::new("cpu", 100));
        policy.add_rule(QuotaRule::new("mem", 1000));

        assert_eq!(policy.rule_count(), 2);

        let decision = policy.enforce("cpu", 50);
        assert_eq!(decision, QuotaDecision::Allow);
        assert_eq!(policy.enforcement_count(), 1);
    }

    #[test]
    fn test_policy_deny_counting() {
        let mut policy = QuotaPolicy::new();
        policy.add_rule(QuotaRule::new("cpu", 100));

        policy.enforce("cpu", 50); // allow
        policy.enforce("cpu", 150); // deny
        policy.enforce("cpu", 90); // warn

        assert_eq!(policy.deny_count(), 1);
        assert_eq!(policy.warn_count(), 1);
    }

    #[test]
    fn test_policy_enforce_missing_rule() {
        let mut policy = QuotaPolicy::new();
        let decision = policy.enforce("nonexistent", 9999);
        assert_eq!(decision, QuotaDecision::Allow);
    }

    #[test]
    fn test_policy_remove_rule() {
        let mut policy = QuotaPolicy::new();
        policy.add_rule(QuotaRule::new("cpu", 100));
        assert!(policy.remove_rule("cpu"));
        assert!(!policy.remove_rule("cpu")); // already removed
        assert_eq!(policy.rule_count(), 0);
    }

    #[test]
    fn test_policy_enforce_all() {
        let mut policy = QuotaPolicy::new();
        policy.add_rule(QuotaRule::new("cpu", 100));
        policy.add_rule(QuotaRule::new("mem", 1000));

        let mut values = HashMap::new();
        values.insert("cpu".to_string(), 50);
        values.insert("mem".to_string(), 1500);

        let decisions = policy.enforce_all(&values);
        assert_eq!(decisions.len(), 2);

        let denials: Vec<_> = decisions.iter().filter(|d| !d.is_allowed()).collect();
        assert_eq!(denials.len(), 1); // mem exceeded
    }

    #[test]
    fn test_policy_default() {
        let policy = QuotaPolicy::default();
        assert_eq!(policy.rule_count(), 0);
    }
}
