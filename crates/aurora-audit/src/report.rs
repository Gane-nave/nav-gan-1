//! Compliance reporting — generate compliance reports, summaries, and dashboards.

use std::collections::HashMap;

/// Report type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportType {
    /// Full compliance report.
    Full,
    /// Executive summary.
    Summary,
    /// Incident report.
    Incident,
    /// Periodic review.
    PeriodicReview,
}

/// Report status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportStatus {
    /// Report is being generated.
    Generating,
    /// Report is ready.
    Ready,
    /// Report generation failed.
    Failed,
}

/// A finding in a compliance report.
#[derive(Debug, Clone)]
pub struct Finding {
    /// Finding identifier.
    pub id: String,
    /// Severity (1=low, 5=critical).
    pub severity: u8,
    /// Title.
    pub title: String,
    /// Description.
    pub description: String,
    /// Recommendation.
    pub recommendation: String,
    /// Whether the finding has been resolved.
    pub resolved: bool,
}

impl Finding {
    /// Create a new finding.
    pub fn new(id: &str, severity: u8, title: &str) -> Self {
        Self {
            id: id.to_string(),
            severity: severity.min(5),
            title: title.to_string(),
            description: String::new(),
            recommendation: String::new(),
            resolved: false,
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set recommendation.
    pub fn with_recommendation(mut self, rec: &str) -> Self {
        self.recommendation = rec.to_string();
        self
    }

    /// Mark as resolved.
    pub fn resolve(&mut self) {
        self.resolved = true;
    }

    /// Whether this is a critical finding.
    pub fn is_critical(&self) -> bool {
        self.severity >= 4
    }
}

/// A compliance report.
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    /// Report identifier.
    pub id: String,
    /// Report title.
    pub title: String,
    /// Report type.
    pub report_type: ReportType,
    /// Report status.
    pub status: ReportStatus,
    /// Generated timestamp.
    pub generated_at_ms: u64,
    /// Reporting period start.
    pub period_start_ms: u64,
    /// Reporting period end.
    pub period_end_ms: u64,
    /// Overall compliance score (0.0–1.0).
    pub compliance_score: f64,
    /// Findings.
    pub findings: Vec<Finding>,
    /// Summary metrics.
    pub metrics: HashMap<String, f64>,
}

impl ComplianceReport {
    /// Create a new compliance report.
    pub fn new(
        id: &str,
        title: &str,
        report_type: ReportType,
        period_start_ms: u64,
        period_end_ms: u64,
        now_ms: u64,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            report_type,
            status: ReportStatus::Generating,
            generated_at_ms: now_ms,
            period_start_ms,
            period_end_ms,
            compliance_score: 0.0,
            findings: Vec::new(),
            metrics: HashMap::new(),
        }
    }

    /// Add a finding.
    pub fn add_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }

    /// Set compliance score.
    pub fn set_score(&mut self, score: f64) {
        self.compliance_score = score.clamp(0.0, 1.0);
    }

    /// Set a metric.
    pub fn set_metric(&mut self, key: &str, value: f64) {
        self.metrics.insert(key.to_string(), value);
    }

    /// Mark report as ready.
    pub fn finalize(&mut self) {
        self.status = ReportStatus::Ready;
    }

    /// Mark report as failed.
    pub fn fail(&mut self) {
        self.status = ReportStatus::Failed;
    }

    /// Get total finding count.
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Get critical finding count.
    pub fn critical_finding_count(&self) -> usize {
        self.findings.iter().filter(|f| f.is_critical()).count()
    }

    /// Get unresolved finding count.
    pub fn unresolved_finding_count(&self) -> usize {
        self.findings.iter().filter(|f| !f.resolved).count()
    }

    /// Get resolved finding count.
    pub fn resolved_finding_count(&self) -> usize {
        self.findings.iter().filter(|f| f.resolved).count()
    }

    /// Get reporting period duration in days.
    pub fn period_days(&self) -> f64 {
        (self.period_end_ms - self.period_start_ms) as f64 / 86_400_000.0
    }
}

/// Report generator — creates and manages compliance reports.
pub struct ReportGenerator {
    reports: Vec<ComplianceReport>,
}

impl ReportGenerator {
    /// Create a new report generator.
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
        }
    }

    /// Generate a new report.
    pub fn generate(
        &mut self,
        id: &str,
        title: &str,
        report_type: ReportType,
        period_start_ms: u64,
        period_end_ms: u64,
        now_ms: u64,
    ) -> &mut ComplianceReport {
        let report = ComplianceReport::new(
            id,
            title,
            report_type,
            period_start_ms,
            period_end_ms,
            now_ms,
        );
        self.reports.push(report);
        self.reports.last_mut().unwrap()
    }

    /// Get a report by ID.
    pub fn get_report(&self, id: &str) -> Option<&ComplianceReport> {
        self.reports.iter().find(|r| r.id == id)
    }

    /// Get all reports.
    pub fn reports(&self) -> &[ComplianceReport] {
        &self.reports
    }

    /// Total report count.
    pub fn count(&self) -> usize {
        self.reports.len()
    }

    /// Get reports by type.
    pub fn reports_by_type(&self, report_type: ReportType) -> Vec<&ComplianceReport> {
        self.reports
            .iter()
            .filter(|r| r.report_type == report_type)
            .collect()
    }

    /// Get ready reports.
    pub fn ready_reports(&self) -> Vec<&ComplianceReport> {
        self.reports
            .iter()
            .filter(|r| r.status == ReportStatus::Ready)
            .collect()
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_lifecycle() {
        let mut report = ComplianceReport::new(
            "rpt-1",
            "Q1 2026 Compliance",
            ReportType::Full,
            0,
            7_776_000_000, // 90 days
            8_000_000_000,
        );
        assert_eq!(report.status, ReportStatus::Generating);

        report.set_score(0.95);
        report.add_finding(Finding::new("f1", 2, "Minor Issue"));
        report.finalize();

        assert_eq!(report.status, ReportStatus::Ready);
        assert!((report.compliance_score - 0.95).abs() < f64::EPSILON);
        assert_eq!(report.finding_count(), 1);
    }

    #[test]
    fn test_finding_severity() {
        let f1 = Finding::new("f1", 2, "Low");
        let f2 = Finding::new("f2", 5, "Critical");

        assert!(!f1.is_critical());
        assert!(f2.is_critical());
    }

    #[test]
    fn test_finding_resolve() {
        let mut finding = Finding::new("f1", 3, "Medium Issue");
        assert!(!finding.resolved);
        finding.resolve();
        assert!(finding.resolved);
    }

    #[test]
    fn test_report_finding_counts() {
        let mut report = ComplianceReport::new("r", "Test", ReportType::Summary, 0, 1000, 2000);

        let mut f1 = Finding::new("f1", 5, "Critical");
        f1.resolve();
        report.add_finding(f1);
        report.add_finding(Finding::new("f2", 2, "Minor"));
        report.add_finding(Finding::new("f3", 4, "Major"));

        assert_eq!(report.finding_count(), 3);
        assert_eq!(report.critical_finding_count(), 2); // severity >= 4
        assert_eq!(report.resolved_finding_count(), 1);
        assert_eq!(report.unresolved_finding_count(), 2);
    }

    #[test]
    fn test_report_metrics() {
        let mut report = ComplianceReport::new("r", "Test", ReportType::Summary, 0, 1000, 2000);
        report.set_metric("total_events", 10000.0);
        report.set_metric("avg_response_ms", 45.5);

        assert_eq!(report.metrics.get("total_events"), Some(&10000.0));
        assert_eq!(report.metrics.get("avg_response_ms"), Some(&45.5));
    }

    #[test]
    fn test_report_period_days() {
        let report = ComplianceReport::new(
            "r",
            "Test",
            ReportType::PeriodicReview,
            0,
            86_400_000 * 30,
            0,
        );
        assert!((report.period_days() - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_report_score_clamping() {
        let mut report = ComplianceReport::new("r", "Test", ReportType::Summary, 0, 1000, 0);
        report.set_score(1.5);
        assert!((report.compliance_score - 1.0).abs() < f64::EPSILON);

        report.set_score(-0.5);
        assert!((report.compliance_score - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_report_generator() {
        let mut gen = ReportGenerator::new();
        let report = gen.generate("r1", "Q1 Report", ReportType::Full, 0, 1000, 2000);
        report.set_score(0.9);
        report.finalize();

        assert_eq!(gen.count(), 1);
        assert_eq!(gen.ready_reports().len(), 1);
        assert_eq!(gen.reports_by_type(ReportType::Full).len(), 1);
    }

    #[test]
    fn test_report_generator_multiple() {
        let mut gen = ReportGenerator::new();
        gen.generate("r1", "Report 1", ReportType::Full, 0, 1000, 2000)
            .finalize();
        gen.generate("r2", "Report 2", ReportType::Summary, 0, 1000, 3000);

        assert_eq!(gen.count(), 2);
        assert_eq!(gen.ready_reports().len(), 1);
        assert!(gen.get_report("r1").is_some());
    }

    #[test]
    fn test_finding_builder() {
        let finding = Finding::new("f1", 3, "Issue")
            .with_description("Something is wrong")
            .with_recommendation("Fix it");

        assert_eq!(finding.description, "Something is wrong");
        assert_eq!(finding.recommendation, "Fix it");
    }

    #[test]
    fn test_report_failed() {
        let mut report = ComplianceReport::new("r", "Test", ReportType::Incident, 0, 1000, 0);
        report.fail();
        assert_eq!(report.status, ReportStatus::Failed);
    }
}
