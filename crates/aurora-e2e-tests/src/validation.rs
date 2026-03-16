//! Validation framework — system-level invariant checking and regression guards.

use std::collections::HashMap;

/// Severity level for validation findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational note.
    Info,
    /// Minor issue that doesn't block.
    Warning,
    /// Significant issue.
    Error,
    /// System-breaking issue.
    Critical,
}

/// A single validation finding.
#[derive(Debug, Clone)]
pub struct Finding {
    /// Unique rule ID.
    pub rule_id: String,
    /// Severity level.
    pub severity: Severity,
    /// Human-readable message.
    pub message: String,
    /// Component where the finding was detected.
    pub component: String,
}

/// Validation rule definition.
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Unique rule ID.
    pub id: String,
    /// Rule description.
    pub description: String,
    /// Category tag.
    pub category: String,
    /// Whether this rule is enabled.
    pub enabled: bool,
}

/// System validator — runs invariant checks across the system.
pub struct SystemValidator {
    rules: Vec<ValidationRule>,
    findings: Vec<Finding>,
    rule_results: HashMap<String, bool>,
}

impl SystemValidator {
    /// Create a new validator.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            findings: Vec::new(),
            rule_results: HashMap::new(),
        }
    }

    /// Register a validation rule.
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.push(rule);
    }

    /// Get rule count.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Get enabled rule count.
    pub fn enabled_rule_count(&self) -> usize {
        self.rules.iter().filter(|r| r.enabled).count()
    }

    /// Record a rule result (pass/fail).
    pub fn record_result(&mut self, rule_id: &str, passed: bool) {
        self.rule_results.insert(rule_id.to_string(), passed);
    }

    /// Record a finding.
    pub fn add_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }

    /// Get all findings.
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    /// Get findings by severity.
    pub fn findings_by_severity(&self, severity: Severity) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .collect()
    }

    /// Get findings for a specific component.
    pub fn findings_for_component(&self, component: &str) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|f| f.component == component)
            .collect()
    }

    /// Check if all rules passed.
    pub fn all_passed(&self) -> bool {
        let enabled_ids: Vec<&str> = self
            .rules
            .iter()
            .filter(|r| r.enabled)
            .map(|r| r.id.as_str())
            .collect();
        enabled_ids
            .iter()
            .all(|id| self.rule_results.get(*id).copied().unwrap_or(false))
    }

    /// Get pass rate as percentage.
    pub fn pass_rate(&self) -> f64 {
        if self.rule_results.is_empty() {
            return 0.0;
        }
        let passed = self.rule_results.values().filter(|v| **v).count();
        (passed as f64 / self.rule_results.len() as f64) * 100.0
    }

    /// Get the highest severity finding.
    pub fn max_severity(&self) -> Option<Severity> {
        self.findings.iter().map(|f| f.severity).max()
    }

    /// Check if there are any critical findings.
    pub fn has_critical(&self) -> bool {
        self.findings
            .iter()
            .any(|f| f.severity == Severity::Critical)
    }

    /// Get rules by category.
    pub fn rules_by_category(&self, category: &str) -> Vec<&ValidationRule> {
        self.rules
            .iter()
            .filter(|r| r.category == category)
            .collect()
    }

    /// Generate a summary report.
    pub fn summary(&self) -> String {
        let total = self.rule_results.len();
        let passed = self.rule_results.values().filter(|v| **v).count();
        let critical = self.findings_by_severity(Severity::Critical).len();
        let errors = self.findings_by_severity(Severity::Error).len();
        format!(
            "Rules: {passed}/{total} passed, Findings: {} (critical: {critical}, errors: {errors})",
            self.findings.len()
        )
    }
}

impl Default for SystemValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_count_rules() {
        let mut v = SystemValidator::new();
        v.add_rule(ValidationRule {
            id: "R001".to_string(),
            description: "GNSS accuracy".to_string(),
            category: "positioning".to_string(),
            enabled: true,
        });
        v.add_rule(ValidationRule {
            id: "R002".to_string(),
            description: "Fusion latency".to_string(),
            category: "performance".to_string(),
            enabled: false,
        });
        assert_eq!(v.rule_count(), 2);
        assert_eq!(v.enabled_rule_count(), 1);
    }

    #[test]
    fn test_all_passed() {
        let mut v = SystemValidator::new();
        v.add_rule(ValidationRule {
            id: "R001".to_string(),
            description: "test".to_string(),
            category: "test".to_string(),
            enabled: true,
        });
        v.record_result("R001", true);
        assert!(v.all_passed());

        v.record_result("R001", false);
        assert!(!v.all_passed());
    }

    #[test]
    fn test_disabled_rules_ignored() {
        let mut v = SystemValidator::new();
        v.add_rule(ValidationRule {
            id: "R001".to_string(),
            description: "test".to_string(),
            category: "test".to_string(),
            enabled: false,
        });
        // Disabled rule not checked → all_passed is vacuously true
        // (no enabled rules require results)
        assert!(v.all_passed());
    }

    #[test]
    fn test_findings_by_severity() {
        let mut v = SystemValidator::new();
        v.add_finding(Finding {
            rule_id: "R001".to_string(),
            severity: Severity::Warning,
            message: "minor issue".to_string(),
            component: "gnss".to_string(),
        });
        v.add_finding(Finding {
            rule_id: "R002".to_string(),
            severity: Severity::Critical,
            message: "critical issue".to_string(),
            component: "fusion".to_string(),
        });
        assert_eq!(v.findings_by_severity(Severity::Warning).len(), 1);
        assert_eq!(v.findings_by_severity(Severity::Critical).len(), 1);
        assert_eq!(v.findings_by_severity(Severity::Info).len(), 0);
    }

    #[test]
    fn test_findings_for_component() {
        let mut v = SystemValidator::new();
        v.add_finding(Finding {
            rule_id: "R001".to_string(),
            severity: Severity::Error,
            message: "issue1".to_string(),
            component: "gnss".to_string(),
        });
        v.add_finding(Finding {
            rule_id: "R002".to_string(),
            severity: Severity::Warning,
            message: "issue2".to_string(),
            component: "gnss".to_string(),
        });
        v.add_finding(Finding {
            rule_id: "R003".to_string(),
            severity: Severity::Info,
            message: "issue3".to_string(),
            component: "routing".to_string(),
        });
        assert_eq!(v.findings_for_component("gnss").len(), 2);
        assert_eq!(v.findings_for_component("routing").len(), 1);
    }

    #[test]
    fn test_max_severity() {
        let mut v = SystemValidator::new();
        assert!(v.max_severity().is_none());
        v.add_finding(Finding {
            rule_id: "R1".to_string(),
            severity: Severity::Warning,
            message: "w".to_string(),
            component: "c".to_string(),
        });
        assert_eq!(v.max_severity(), Some(Severity::Warning));
        v.add_finding(Finding {
            rule_id: "R2".to_string(),
            severity: Severity::Error,
            message: "e".to_string(),
            component: "c".to_string(),
        });
        assert_eq!(v.max_severity(), Some(Severity::Error));
    }

    #[test]
    fn test_has_critical() {
        let mut v = SystemValidator::new();
        assert!(!v.has_critical());
        v.add_finding(Finding {
            rule_id: "R1".to_string(),
            severity: Severity::Critical,
            message: "bad".to_string(),
            component: "c".to_string(),
        });
        assert!(v.has_critical());
    }

    #[test]
    fn test_pass_rate() {
        let mut v = SystemValidator::new();
        assert_eq!(v.pass_rate(), 0.0);
        v.record_result("R1", true);
        v.record_result("R2", true);
        v.record_result("R3", false);
        let rate = v.pass_rate();
        assert!((rate - 66.666).abs() < 0.01);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
        assert!(Severity::Error < Severity::Critical);
    }

    #[test]
    fn test_rules_by_category() {
        let mut v = SystemValidator::new();
        v.add_rule(ValidationRule {
            id: "R1".to_string(),
            description: "d".to_string(),
            category: "perf".to_string(),
            enabled: true,
        });
        v.add_rule(ValidationRule {
            id: "R2".to_string(),
            description: "d".to_string(),
            category: "safety".to_string(),
            enabled: true,
        });
        assert_eq!(v.rules_by_category("perf").len(), 1);
        assert_eq!(v.rules_by_category("safety").len(), 1);
        assert_eq!(v.rules_by_category("unknown").len(), 0);
    }

    #[test]
    fn test_summary_format() {
        let mut v = SystemValidator::new();
        v.record_result("R1", true);
        v.add_finding(Finding {
            rule_id: "R1".to_string(),
            severity: Severity::Warning,
            message: "w".to_string(),
            component: "c".to_string(),
        });
        let s = v.summary();
        assert!(s.contains("1/1 passed"));
        assert!(s.contains("Findings: 1"));
    }
}
