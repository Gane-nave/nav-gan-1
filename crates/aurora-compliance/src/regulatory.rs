//! Regulatory compliance — jurisdiction-specific rules, certifications, and compliance checks.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Compliance status of a regulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    PartiallyCompliant,
    NotApplicable,
    UnderReview,
}

/// A regulatory requirement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regulation {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub jurisdiction: String,
    pub description: String,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub mandatory: bool,
}

/// A compliance check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub regulation_id: Uuid,
    pub status: ComplianceStatus,
    pub checked_at: DateTime<Utc>,
    pub checked_by: String,
    pub findings: Vec<String>,
    pub remediation: Option<String>,
    pub next_review: Option<DateTime<Utc>>,
}

/// A compliance certification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certification {
    pub id: Uuid,
    pub name: String,
    pub issuer: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub scope: String,
}

impl Certification {
    /// Check if the certification is currently valid.
    pub fn is_valid(&self, now: DateTime<Utc>) -> bool {
        now >= self.issued_at && now <= self.expires_at
    }
}

/// Regulatory compliance manager — tracks regulations, checks, and certifications.
pub struct RegulatoryManager {
    regulations: RwLock<Vec<Regulation>>,
    checks: RwLock<Vec<ComplianceCheck>>,
    certifications: RwLock<Vec<Certification>>,
}

impl RegulatoryManager {
    /// Create a new manager.
    pub fn new() -> Self {
        Self {
            regulations: RwLock::new(Vec::new()),
            checks: RwLock::new(Vec::new()),
            certifications: RwLock::new(Vec::new()),
        }
    }

    /// Register a regulation.
    pub fn add_regulation(&self, regulation: Regulation) {
        self.regulations.write().push(regulation);
    }

    /// Record a compliance check.
    pub fn record_check(&self, check: ComplianceCheck) {
        self.checks.write().push(check);
    }

    /// Add a certification.
    pub fn add_certification(&self, cert: Certification) {
        self.certifications.write().push(cert);
    }

    /// Get the latest check for a regulation.
    pub fn latest_check(&self, regulation_id: Uuid) -> Option<ComplianceCheck> {
        self.checks
            .read()
            .iter()
            .filter(|c| c.regulation_id == regulation_id)
            .max_by_key(|c| c.checked_at)
            .cloned()
    }

    /// Get overall compliance status for a jurisdiction.
    pub fn jurisdiction_status(&self, jurisdiction: &str) -> Vec<(Regulation, ComplianceStatus)> {
        let regulations = self.regulations.read();
        let checks = self.checks.read();

        regulations
            .iter()
            .filter(|r| r.jurisdiction == jurisdiction)
            .map(|r| {
                let status = checks
                    .iter()
                    .filter(|c| c.regulation_id == r.id)
                    .max_by_key(|c| c.checked_at)
                    .map(|c| c.status)
                    .unwrap_or(ComplianceStatus::UnderReview);
                (r.clone(), status)
            })
            .collect()
    }

    /// Get regulations due for review (last check is older than `days` ago or never checked).
    pub fn due_for_review(&self, days: i64, now: DateTime<Utc>) -> Vec<Regulation> {
        let regulations = self.regulations.read();
        let checks = self.checks.read();
        let threshold = now - chrono::Duration::days(days);

        regulations
            .iter()
            .filter(|r| {
                let latest = checks
                    .iter()
                    .filter(|c| c.regulation_id == r.id)
                    .max_by_key(|c| c.checked_at);
                match latest {
                    None => true, // never checked
                    Some(c) => c.checked_at < threshold,
                }
            })
            .cloned()
            .collect()
    }

    /// Get expiring certifications within N days.
    pub fn expiring_certifications(&self, days: i64, now: DateTime<Utc>) -> Vec<Certification> {
        let horizon = now + chrono::Duration::days(days);
        self.certifications
            .read()
            .iter()
            .filter(|c| c.is_valid(now) && c.expires_at <= horizon)
            .cloned()
            .collect()
    }

    /// Get all valid certifications.
    pub fn valid_certifications(&self, now: DateTime<Utc>) -> Vec<Certification> {
        self.certifications
            .read()
            .iter()
            .filter(|c| c.is_valid(now))
            .cloned()
            .collect()
    }

    /// Count non-compliant regulations.
    pub fn non_compliant_count(&self) -> usize {
        let regulations = self.regulations.read();
        let checks = self.checks.read();
        regulations
            .iter()
            .filter(|r| {
                checks
                    .iter()
                    .filter(|c| c.regulation_id == r.id)
                    .max_by_key(|c| c.checked_at)
                    .is_some_and(|c| c.status == ComplianceStatus::NonCompliant)
            })
            .count()
    }

    /// Get all regulations.
    pub fn regulations(&self) -> Vec<Regulation> {
        self.regulations.read().clone()
    }

    /// Get all jurisdictions.
    pub fn jurisdictions(&self) -> Vec<String> {
        let regs = self.regulations.read();
        let mut jurisdictions: Vec<String> = regs.iter().map(|r| r.jurisdiction.clone()).collect();
        jurisdictions.sort();
        jurisdictions.dedup();
        jurisdictions
    }
}

impl Default for RegulatoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_regulation(code: &str, jurisdiction: &str) -> Regulation {
        Regulation {
            id: Uuid::new_v4(),
            code: code.to_string(),
            name: format!("Regulation {code}"),
            jurisdiction: jurisdiction.to_string(),
            description: "Test regulation".to_string(),
            effective_date: Utc::now() - chrono::Duration::days(365),
            expiry_date: None,
            mandatory: true,
        }
    }

    fn make_check(regulation_id: Uuid, status: ComplianceStatus) -> ComplianceCheck {
        ComplianceCheck {
            regulation_id,
            status,
            checked_at: Utc::now(),
            checked_by: "auditor".to_string(),
            findings: Vec::new(),
            remediation: None,
            next_review: None,
        }
    }

    fn make_cert(name: &str, days_until_expiry: i64) -> Certification {
        Certification {
            id: Uuid::new_v4(),
            name: name.to_string(),
            issuer: "CertBody".to_string(),
            issued_at: Utc::now() - chrono::Duration::days(180),
            expires_at: Utc::now() + chrono::Duration::days(days_until_expiry),
            scope: "Navigation".to_string(),
        }
    }

    #[test]
    fn test_add_and_check_regulation() {
        let mgr = RegulatoryManager::new();
        let reg = make_regulation("GDPR-001", "EU");
        let rid = reg.id;
        mgr.add_regulation(reg);
        mgr.record_check(make_check(rid, ComplianceStatus::Compliant));
        let latest = mgr.latest_check(rid).unwrap();
        assert_eq!(latest.status, ComplianceStatus::Compliant);
    }

    #[test]
    fn test_jurisdiction_status() {
        let mgr = RegulatoryManager::new();
        let r1 = make_regulation("EU-001", "EU");
        let r2 = make_regulation("EU-002", "EU");
        let r1id = r1.id;
        mgr.add_regulation(r1);
        mgr.add_regulation(r2);
        mgr.record_check(make_check(r1id, ComplianceStatus::Compliant));
        // r2 has no check → UnderReview
        let status = mgr.jurisdiction_status("EU");
        assert_eq!(status.len(), 2);
    }

    #[test]
    fn test_due_for_review() {
        let mgr = RegulatoryManager::new();
        let r1 = make_regulation("R1", "US");
        let r2 = make_regulation("R2", "US");
        let r1id = r1.id;
        mgr.add_regulation(r1);
        mgr.add_regulation(r2);
        mgr.record_check(make_check(r1id, ComplianceStatus::Compliant));
        // r1 just checked, r2 never checked → r2 is due
        let due = mgr.due_for_review(30, Utc::now());
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].code, "R2");
    }

    #[test]
    fn test_certification_validity() {
        let cert = make_cert("ISO 27001", 30);
        assert!(cert.is_valid(Utc::now()));
        let expired_cert = Certification {
            expires_at: Utc::now() - chrono::Duration::days(1),
            ..make_cert("expired", 0)
        };
        assert!(!expired_cert.is_valid(Utc::now()));
    }

    #[test]
    fn test_expiring_certifications() {
        let mgr = RegulatoryManager::new();
        mgr.add_certification(make_cert("Soon", 15));
        mgr.add_certification(make_cert("Later", 180));
        let expiring = mgr.expiring_certifications(30, Utc::now());
        assert_eq!(expiring.len(), 1);
        assert_eq!(expiring[0].name, "Soon");
    }

    #[test]
    fn test_non_compliant_count() {
        let mgr = RegulatoryManager::new();
        let r1 = make_regulation("R1", "EU");
        let r2 = make_regulation("R2", "EU");
        let r1id = r1.id;
        let r2id = r2.id;
        mgr.add_regulation(r1);
        mgr.add_regulation(r2);
        mgr.record_check(make_check(r1id, ComplianceStatus::Compliant));
        mgr.record_check(make_check(r2id, ComplianceStatus::NonCompliant));
        assert_eq!(mgr.non_compliant_count(), 1);
    }

    #[test]
    fn test_jurisdictions() {
        let mgr = RegulatoryManager::new();
        mgr.add_regulation(make_regulation("R1", "EU"));
        mgr.add_regulation(make_regulation("R2", "US"));
        mgr.add_regulation(make_regulation("R3", "EU"));
        let js = mgr.jurisdictions();
        assert_eq!(js, vec!["EU", "US"]);
    }

    #[test]
    fn test_valid_certifications() {
        let mgr = RegulatoryManager::new();
        mgr.add_certification(make_cert("Valid", 90));
        let expired = Certification {
            expires_at: Utc::now() - chrono::Duration::days(1),
            ..make_cert("Expired", 0)
        };
        mgr.add_certification(expired);
        let valid = mgr.valid_certifications(Utc::now());
        assert_eq!(valid.len(), 1);
        assert_eq!(valid[0].name, "Valid");
    }

    #[test]
    fn test_no_checks() {
        let mgr = RegulatoryManager::new();
        assert!(mgr.latest_check(Uuid::new_v4()).is_none());
        assert_eq!(mgr.non_compliant_count(), 0);
    }

    #[test]
    fn test_findings_and_remediation() {
        let mgr = RegulatoryManager::new();
        let reg = make_regulation("SEC-001", "US");
        let rid = reg.id;
        mgr.add_regulation(reg);
        let mut check = make_check(rid, ComplianceStatus::PartiallyCompliant);
        check.findings = vec![
            "Missing encryption".to_string(),
            "Weak passwords".to_string(),
        ];
        check.remediation = Some("Upgrade to AES-256".to_string());
        mgr.record_check(check);
        let latest = mgr.latest_check(rid).unwrap();
        assert_eq!(latest.findings.len(), 2);
        assert!(latest.remediation.is_some());
    }
}
