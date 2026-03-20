//! WCAG compliance checking — automated accessibility auditing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WCAG success criterion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    pub id: String,
    pub name: String,
    pub level: ConformanceLevel,
    pub principle: WcagPrinciple,
    pub description: String,
}

/// WCAG conformance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConformanceLevel {
    A,
    Aa,
    Aaa,
}

/// WCAG principle (POUR).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WcagPrinciple {
    Perceivable,
    Operable,
    Understandable,
    Robust,
}

/// Audit result for a single criterion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    pub criterion: SuccessCriterion,
    pub status: AuditStatus,
    pub findings: Vec<Finding>,
}

/// Audit status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditStatus {
    Pass,
    Fail,
    Warning,
    NotApplicable,
    NotTested,
}

/// An individual finding within an audit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub element_id: String,
    pub message: String,
    pub severity: FindingSeverity,
    pub suggestion: Option<String>,
}

/// Finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingSeverity {
    Critical,
    Major,
    Minor,
    Info,
}

/// WCAG audit report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub results: Vec<AuditResult>,
    pub timestamp: String,
    pub summary: AuditSummary,
}

/// Summary of audit results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    pub total_criteria: usize,
    pub passed: usize,
    pub failed: usize,
    pub warnings: usize,
    pub not_applicable: usize,
    pub not_tested: usize,
    pub conformance_level: Option<ConformanceLevel>,
}

impl AuditReport {
    /// Build a report from results.
    pub fn from_results(results: Vec<AuditResult>) -> Self {
        let summary = Self::compute_summary(&results);
        Self {
            results,
            timestamp: "now".to_string(),
            summary,
        }
    }

    fn compute_summary(results: &[AuditResult]) -> AuditSummary {
        let mut passed = 0;
        let mut failed = 0;
        let mut warnings = 0;
        let mut not_applicable = 0;
        let mut not_tested = 0;

        let mut level_a_pass = true;
        let mut level_aa_pass = true;

        for r in results {
            match r.status {
                AuditStatus::Pass => passed += 1,
                AuditStatus::Fail => {
                    failed += 1;
                    match r.criterion.level {
                        ConformanceLevel::A => level_a_pass = false,
                        ConformanceLevel::Aa => level_aa_pass = false,
                        ConformanceLevel::Aaa => {}
                    }
                }
                AuditStatus::Warning => warnings += 1,
                AuditStatus::NotApplicable => not_applicable += 1,
                AuditStatus::NotTested => not_tested += 1,
            }
        }

        let conformance_level = if level_a_pass && level_aa_pass {
            Some(ConformanceLevel::Aa)
        } else if level_a_pass {
            Some(ConformanceLevel::A)
        } else {
            None
        };

        AuditSummary {
            total_criteria: results.len(),
            passed,
            failed,
            warnings,
            not_applicable,
            not_tested,
            conformance_level,
        }
    }

    /// Get results by principle.
    pub fn by_principle(&self, principle: WcagPrinciple) -> Vec<&AuditResult> {
        self.results
            .iter()
            .filter(|r| r.criterion.principle == principle)
            .collect()
    }

    /// Get failing results.
    pub fn failures(&self) -> Vec<&AuditResult> {
        self.results
            .iter()
            .filter(|r| r.status == AuditStatus::Fail)
            .collect()
    }
}

/// Navigation-specific accessibility checks.
pub struct NavAccessibilityChecker {
    rules: Vec<Box<dyn AccessibilityRule>>,
}

/// A single accessibility rule.
pub trait AccessibilityRule {
    /// Rule identifier.
    fn id(&self) -> &str;
    /// Human-readable name.
    fn name(&self) -> &str;
    /// Check the rule against a set of elements.
    fn check(&self, elements: &[ElementInfo]) -> AuditResult;
}

/// Simplified element info for checking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInfo {
    pub id: String,
    pub element_type: String,
    pub label: Option<String>,
    pub role: Option<String>,
    pub contrast_ratio: Option<f64>,
    pub font_size: Option<f64>,
    pub has_focus_indicator: bool,
    pub tab_index: Option<i32>,
    pub properties: HashMap<String, String>,
}

/// Rule: All interactive elements must have labels.
pub struct LabelRule;

impl AccessibilityRule for LabelRule {
    fn id(&self) -> &str {
        "label-required"
    }

    fn name(&self) -> &str {
        "Interactive elements must have labels"
    }

    fn check(&self, elements: &[ElementInfo]) -> AuditResult {
        let interactive = ["button", "link", "input", "select", "slider"];
        let mut findings = Vec::new();

        for el in elements {
            if interactive.contains(&el.element_type.as_str()) && el.label.is_none() {
                findings.push(Finding {
                    element_id: el.id.clone(),
                    message: format!(
                        "{} '{}' is missing an accessible label",
                        el.element_type, el.id
                    ),
                    severity: FindingSeverity::Critical,
                    suggestion: Some("Add aria-label or aria-labelledby".to_string()),
                });
            }
        }

        let status = if findings.is_empty() {
            AuditStatus::Pass
        } else {
            AuditStatus::Fail
        };

        AuditResult {
            criterion: SuccessCriterion {
                id: "1.3.1".to_string(),
                name: "Info and Relationships".to_string(),
                level: ConformanceLevel::A,
                principle: WcagPrinciple::Perceivable,
                description:
                    "Information conveyed through presentation can be programmatically determined"
                        .to_string(),
            },
            status,
            findings,
        }
    }
}

/// Rule: Focus indicators must be visible.
pub struct FocusIndicatorRule;

impl AccessibilityRule for FocusIndicatorRule {
    fn id(&self) -> &str {
        "focus-indicator"
    }

    fn name(&self) -> &str {
        "Interactive elements must have visible focus indicators"
    }

    fn check(&self, elements: &[ElementInfo]) -> AuditResult {
        let interactive = ["button", "link", "input", "select"];
        let mut findings = Vec::new();

        for el in elements {
            if interactive.contains(&el.element_type.as_str()) && !el.has_focus_indicator {
                findings.push(Finding {
                    element_id: el.id.clone(),
                    message: format!("{} '{}' lacks a focus indicator", el.element_type, el.id),
                    severity: FindingSeverity::Major,
                    suggestion: Some("Add visible :focus styles".to_string()),
                });
            }
        }

        let status = if findings.is_empty() {
            AuditStatus::Pass
        } else {
            AuditStatus::Fail
        };

        AuditResult {
            criterion: SuccessCriterion {
                id: "2.4.7".to_string(),
                name: "Focus Visible".to_string(),
                level: ConformanceLevel::Aa,
                principle: WcagPrinciple::Operable,
                description: "Any keyboard operable UI has a visible focus indicator".to_string(),
            },
            status,
            findings,
        }
    }
}

/// Rule: Minimum contrast ratio.
pub struct ContrastRule {
    pub min_ratio: f64,
}

impl AccessibilityRule for ContrastRule {
    fn id(&self) -> &str {
        "contrast-minimum"
    }

    fn name(&self) -> &str {
        "Text must meet minimum contrast ratio"
    }

    fn check(&self, elements: &[ElementInfo]) -> AuditResult {
        let mut findings = Vec::new();

        for el in elements {
            if let Some(ratio) = el.contrast_ratio {
                let min = if el.font_size.unwrap_or(16.0) >= 24.0 {
                    3.0 // Large text
                } else {
                    self.min_ratio
                };

                if ratio < min {
                    findings.push(Finding {
                        element_id: el.id.clone(),
                        message: format!(
                            "Contrast ratio {:.1}:1 is below minimum {:.1}:1",
                            ratio, min
                        ),
                        severity: FindingSeverity::Critical,
                        suggestion: Some("Increase color contrast".to_string()),
                    });
                }
            }
        }

        let status = if findings.is_empty() {
            AuditStatus::Pass
        } else {
            AuditStatus::Fail
        };

        AuditResult {
            criterion: SuccessCriterion {
                id: "1.4.3".to_string(),
                name: "Contrast (Minimum)".to_string(),
                level: ConformanceLevel::Aa,
                principle: WcagPrinciple::Perceivable,
                description: "Text has a contrast ratio of at least 4.5:1".to_string(),
            },
            status,
            findings,
        }
    }
}

impl NavAccessibilityChecker {
    /// Create a new checker with default navigation rules.
    pub fn new() -> Self {
        let rules: Vec<Box<dyn AccessibilityRule>> = vec![
            Box::new(LabelRule),
            Box::new(FocusIndicatorRule),
            Box::new(ContrastRule { min_ratio: 4.5 }),
        ];
        Self { rules }
    }

    /// Run all rules against a set of elements.
    pub fn audit(&self, elements: &[ElementInfo]) -> AuditReport {
        let results: Vec<AuditResult> = self.rules.iter().map(|r| r.check(elements)).collect();
        AuditReport::from_results(results)
    }

    /// Number of rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for NavAccessibilityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_element(id: &str, typ: &str) -> ElementInfo {
        ElementInfo {
            id: id.to_string(),
            element_type: typ.to_string(),
            label: Some("Test".to_string()),
            role: None,
            contrast_ratio: Some(10.0),
            font_size: Some(16.0),
            has_focus_indicator: true,
            tab_index: Some(0),
            properties: HashMap::new(),
        }
    }

    #[test]
    fn test_label_rule_pass() {
        let rule = LabelRule;
        let elements = vec![make_element("btn1", "button")];
        let result = rule.check(&elements);
        assert_eq!(result.status, AuditStatus::Pass);
    }

    #[test]
    fn test_label_rule_fail() {
        let rule = LabelRule;
        let mut el = make_element("btn1", "button");
        el.label = None;
        let result = rule.check(&[el]);
        assert_eq!(result.status, AuditStatus::Fail);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.findings[0].severity, FindingSeverity::Critical);
    }

    #[test]
    fn test_focus_indicator_rule_pass() {
        let rule = FocusIndicatorRule;
        let elements = vec![make_element("btn1", "button")];
        let result = rule.check(&elements);
        assert_eq!(result.status, AuditStatus::Pass);
    }

    #[test]
    fn test_focus_indicator_rule_fail() {
        let rule = FocusIndicatorRule;
        let mut el = make_element("link1", "link");
        el.has_focus_indicator = false;
        let result = rule.check(&[el]);
        assert_eq!(result.status, AuditStatus::Fail);
    }

    #[test]
    fn test_contrast_rule_pass() {
        let rule = ContrastRule { min_ratio: 4.5 };
        let elements = vec![make_element("text1", "text")];
        let result = rule.check(&elements);
        assert_eq!(result.status, AuditStatus::Pass);
    }

    #[test]
    fn test_contrast_rule_fail() {
        let rule = ContrastRule { min_ratio: 4.5 };
        let mut el = make_element("text1", "text");
        el.contrast_ratio = Some(2.0);
        let result = rule.check(&[el]);
        assert_eq!(result.status, AuditStatus::Fail);
    }

    #[test]
    fn test_contrast_large_text_relaxed() {
        let rule = ContrastRule { min_ratio: 4.5 };
        let mut el = make_element("heading", "text");
        el.contrast_ratio = Some(3.5);
        el.font_size = Some(24.0);
        let result = rule.check(&[el]);
        assert_eq!(result.status, AuditStatus::Pass);
    }

    #[test]
    fn test_audit_report_summary() {
        let results = vec![
            AuditResult {
                criterion: SuccessCriterion {
                    id: "1.1".to_string(),
                    name: "Test A".to_string(),
                    level: ConformanceLevel::A,
                    principle: WcagPrinciple::Perceivable,
                    description: "Test".to_string(),
                },
                status: AuditStatus::Pass,
                findings: vec![],
            },
            AuditResult {
                criterion: SuccessCriterion {
                    id: "1.2".to_string(),
                    name: "Test AA".to_string(),
                    level: ConformanceLevel::Aa,
                    principle: WcagPrinciple::Operable,
                    description: "Test".to_string(),
                },
                status: AuditStatus::Fail,
                findings: vec![],
            },
        ];

        let report = AuditReport::from_results(results);
        assert_eq!(report.summary.passed, 1);
        assert_eq!(report.summary.failed, 1);
        assert_eq!(report.summary.conformance_level, Some(ConformanceLevel::A));
    }

    #[test]
    fn test_audit_report_full_compliance() {
        let results = vec![
            AuditResult {
                criterion: SuccessCriterion {
                    id: "1.1".to_string(),
                    name: "A criterion".to_string(),
                    level: ConformanceLevel::A,
                    principle: WcagPrinciple::Perceivable,
                    description: "Test".to_string(),
                },
                status: AuditStatus::Pass,
                findings: vec![],
            },
            AuditResult {
                criterion: SuccessCriterion {
                    id: "1.2".to_string(),
                    name: "AA criterion".to_string(),
                    level: ConformanceLevel::Aa,
                    principle: WcagPrinciple::Operable,
                    description: "Test".to_string(),
                },
                status: AuditStatus::Pass,
                findings: vec![],
            },
        ];

        let report = AuditReport::from_results(results);
        assert_eq!(report.summary.conformance_level, Some(ConformanceLevel::Aa));
    }

    #[test]
    fn test_nav_checker() {
        let checker = NavAccessibilityChecker::new();
        assert_eq!(checker.rule_count(), 3);

        let elements = vec![make_element("btn", "button")];
        let report = checker.audit(&elements);
        assert_eq!(report.summary.total_criteria, 3);
        assert_eq!(report.summary.passed, 3);
    }

    #[test]
    fn test_nav_checker_mixed_results() {
        let checker = NavAccessibilityChecker::new();
        let mut bad_btn = make_element("bad", "button");
        bad_btn.label = None;
        bad_btn.has_focus_indicator = false;

        let report = checker.audit(&[bad_btn]);
        assert!(report.summary.failed > 0);
    }

    #[test]
    fn test_report_by_principle() {
        let checker = NavAccessibilityChecker::new();
        let elements = vec![make_element("btn", "button")];
        let report = checker.audit(&elements);

        let perceivable = report.by_principle(WcagPrinciple::Perceivable);
        assert!(!perceivable.is_empty());
    }

    #[test]
    fn test_non_interactive_elements_skip_label_check() {
        let rule = LabelRule;
        let mut el = make_element("div1", "div");
        el.label = None;
        let result = rule.check(&[el]);
        assert_eq!(result.status, AuditStatus::Pass);
    }
}
