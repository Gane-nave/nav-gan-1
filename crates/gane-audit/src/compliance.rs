//! Compliance management — define and check compliance rules, track violations.

use std::collections::HashMap;

/// Compliance rule status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceStatus {
    /// Rule is being met.
    Compliant,
    /// Rule has minor violations.
    Warning,
    /// Rule is violated.
    NonCompliant,
    /// Rule has not been evaluated.
    Unknown,
}

/// A compliance rule.
#[derive(Debug, Clone)]
pub struct ComplianceRule {
    /// Rule identifier.
    pub id: String,
    /// Rule name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Regulation/standard reference (e.g., "GDPR Art. 17").
    pub regulation: String,
    /// Current status.
    pub status: ComplianceStatus,
    /// Last checked timestamp.
    pub last_checked_ms: Option<u64>,
    /// Violation count.
    violations: u64,
    /// Check count.
    checks: u64,
}

impl ComplianceRule {
    /// Create a new compliance rule.
    pub fn new(id: &str, name: &str, regulation: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            regulation: regulation.to_string(),
            status: ComplianceStatus::Unknown,
            last_checked_ms: None,
            violations: 0,
            checks: 0,
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Record a compliance check result.
    pub fn record_check(&mut self, status: ComplianceStatus, now_ms: u64) {
        self.status = status;
        self.last_checked_ms = Some(now_ms);
        self.checks += 1;
        if status == ComplianceStatus::NonCompliant {
            self.violations += 1;
        }
    }

    /// Get violation count.
    pub fn violation_count(&self) -> u64 {
        self.violations
    }

    /// Get check count.
    pub fn check_count(&self) -> u64 {
        self.checks
    }

    /// Get compliance rate (0.0–1.0).
    pub fn compliance_rate(&self) -> f64 {
        if self.checks == 0 {
            return 1.0;
        }
        (self.checks - self.violations) as f64 / self.checks as f64
    }

    /// Whether the rule is currently compliant.
    pub fn is_compliant(&self) -> bool {
        self.status == ComplianceStatus::Compliant
    }
}

/// Compliance checker — manages rules and tracks overall compliance.
pub struct ComplianceChecker {
    rules: HashMap<String, ComplianceRule>,
}

impl ComplianceChecker {
    /// Create a new compliance checker.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Add a compliance rule.
    pub fn add_rule(&mut self, rule: ComplianceRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    /// Get a rule by ID.
    pub fn get_rule(&self, id: &str) -> Option<&ComplianceRule> {
        self.rules.get(id)
    }

    /// Get a mutable rule by ID.
    pub fn get_rule_mut(&mut self, id: &str) -> Option<&mut ComplianceRule> {
        self.rules.get_mut(id)
    }

    /// Record a check result for a rule.
    pub fn record_check(&mut self, rule_id: &str, status: ComplianceStatus, now_ms: u64) -> bool {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.record_check(status, now_ms);
            true
        } else {
            false
        }
    }

    /// Get all compliant rules.
    pub fn compliant_rules(&self) -> Vec<&ComplianceRule> {
        self.rules.values().filter(|r| r.is_compliant()).collect()
    }

    /// Get all non-compliant rules.
    pub fn non_compliant_rules(&self) -> Vec<&ComplianceRule> {
        self.rules
            .values()
            .filter(|r| r.status == ComplianceStatus::NonCompliant)
            .collect()
    }

    /// Get overall compliance score (0.0–1.0).
    pub fn overall_score(&self) -> f64 {
        if self.rules.is_empty() {
            return 1.0;
        }
        let compliant = self.rules.values().filter(|r| r.is_compliant()).count();
        compliant as f64 / self.rules.len() as f64
    }

    /// Total rule count.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Get rules by regulation.
    pub fn rules_by_regulation(&self, regulation: &str) -> Vec<&ComplianceRule> {
        self.rules
            .values()
            .filter(|r| r.regulation == regulation)
            .collect()
    }

    /// Get total violations across all rules.
    pub fn total_violations(&self) -> u64 {
        self.rules.values().map(|r| r.violation_count()).sum()
    }

    /// Remove a rule.
    pub fn remove_rule(&mut self, id: &str) -> Option<ComplianceRule> {
        self.rules.remove(id)
    }
}

impl Default for ComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_lifecycle() {
        let mut rule = ComplianceRule::new("gdpr-1", "Data Deletion", "GDPR Art. 17");
        assert_eq!(rule.status, ComplianceStatus::Unknown);

        rule.record_check(ComplianceStatus::Compliant, 1000);
        assert!(rule.is_compliant());
        assert_eq!(rule.check_count(), 1);
        assert_eq!(rule.violation_count(), 0);
    }

    #[test]
    fn test_rule_violation() {
        let mut rule = ComplianceRule::new("r1", "Test", "REG");
        rule.record_check(ComplianceStatus::Compliant, 1000);
        rule.record_check(ComplianceStatus::NonCompliant, 2000);
        rule.record_check(ComplianceStatus::Compliant, 3000);

        assert_eq!(rule.violation_count(), 1);
        assert_eq!(rule.check_count(), 3);
        assert!((rule.compliance_rate() - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_checker_overall_score() {
        let mut checker = ComplianceChecker::new();
        let mut r1 = ComplianceRule::new("r1", "A", "REG");
        r1.record_check(ComplianceStatus::Compliant, 0);
        let mut r2 = ComplianceRule::new("r2", "B", "REG");
        r2.record_check(ComplianceStatus::NonCompliant, 0);
        let mut r3 = ComplianceRule::new("r3", "C", "REG");
        r3.record_check(ComplianceStatus::Compliant, 0);

        checker.add_rule(r1);
        checker.add_rule(r2);
        checker.add_rule(r3);

        assert!((checker.overall_score() - 2.0 / 3.0).abs() < 0.01);
        assert_eq!(checker.compliant_rules().len(), 2);
        assert_eq!(checker.non_compliant_rules().len(), 1);
    }

    #[test]
    fn test_checker_by_regulation() {
        let mut checker = ComplianceChecker::new();
        checker.add_rule(ComplianceRule::new("r1", "A", "GDPR"));
        checker.add_rule(ComplianceRule::new("r2", "B", "GDPR"));
        checker.add_rule(ComplianceRule::new("r3", "C", "SOC2"));

        assert_eq!(checker.rules_by_regulation("GDPR").len(), 2);
        assert_eq!(checker.rules_by_regulation("SOC2").len(), 1);
    }

    #[test]
    fn test_checker_total_violations() {
        let mut checker = ComplianceChecker::new();
        let mut r1 = ComplianceRule::new("r1", "A", "REG");
        r1.record_check(ComplianceStatus::NonCompliant, 0);
        r1.record_check(ComplianceStatus::NonCompliant, 1);
        let mut r2 = ComplianceRule::new("r2", "B", "REG");
        r2.record_check(ComplianceStatus::NonCompliant, 0);

        checker.add_rule(r1);
        checker.add_rule(r2);

        assert_eq!(checker.total_violations(), 3);
    }

    #[test]
    fn test_checker_record_check() {
        let mut checker = ComplianceChecker::new();
        checker.add_rule(ComplianceRule::new("r1", "A", "REG"));

        assert!(checker.record_check("r1", ComplianceStatus::Compliant, 1000));
        assert!(!checker.record_check("nonexistent", ComplianceStatus::Compliant, 1000));
    }

    #[test]
    fn test_checker_remove_rule() {
        let mut checker = ComplianceChecker::new();
        checker.add_rule(ComplianceRule::new("r1", "A", "REG"));
        assert_eq!(checker.rule_count(), 1);
        checker.remove_rule("r1");
        assert_eq!(checker.rule_count(), 0);
    }

    #[test]
    fn test_rule_with_description() {
        let rule = ComplianceRule::new("r1", "Data Deletion", "GDPR Art. 17")
            .with_description("Users must be able to request deletion of their data");
        assert_eq!(
            rule.description,
            "Users must be able to request deletion of their data"
        );
    }
}
