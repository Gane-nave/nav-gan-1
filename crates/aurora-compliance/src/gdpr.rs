//! GDPR compliance — data subject rights, consent management, and data processing records.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Legal basis for data processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalBasis {
    Consent,
    Contract,
    LegalObligation,
    VitalInterest,
    PublicTask,
    LegitimateInterest,
}

/// Status of a consent record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentStatus {
    Granted,
    Denied,
    Withdrawn,
    Expired,
}

/// A consent record for a specific data processing purpose.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub purpose: String,
    pub legal_basis: LegalBasis,
    pub status: ConsentStatus,
    pub granted_at: Option<DateTime<Utc>>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub version: u32,
}

/// Type of data subject request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectRequestType {
    Access,
    Rectification,
    Erasure,
    Portability,
    Restriction,
    Objection,
}

/// Status of a data subject request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,
    InProgress,
    Completed,
    Denied,
    Expired,
}

/// A data subject request (DSR).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubjectRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub request_type: SubjectRequestType,
    pub status: RequestStatus,
    pub submitted_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deadline: DateTime<Utc>,
    pub notes: String,
}

/// GDPR compliance manager — manages consent, data subject requests, and processing records.
pub struct GdprManager {
    consents: RwLock<Vec<ConsentRecord>>,
    requests: RwLock<Vec<DataSubjectRequest>>,
    response_deadline_days: i64,
}

impl GdprManager {
    /// Create a new GDPR manager. `response_deadline_days` is the maximum days to respond to DSRs.
    pub fn new(response_deadline_days: i64) -> Self {
        Self {
            consents: RwLock::new(Vec::new()),
            requests: RwLock::new(Vec::new()),
            response_deadline_days,
        }
    }

    /// Record consent for a purpose.
    pub fn grant_consent(
        &self,
        user_id: Uuid,
        purpose: &str,
        legal_basis: LegalBasis,
        expires_at: Option<DateTime<Utc>>,
    ) -> Uuid {
        let mut consents = self.consents.write();
        let version = consents
            .iter()
            .filter(|c| c.user_id == user_id && c.purpose == purpose)
            .count() as u32
            + 1;

        let id = Uuid::new_v4();
        consents.push(ConsentRecord {
            id,
            user_id,
            purpose: purpose.to_string(),
            legal_basis,
            status: ConsentStatus::Granted,
            granted_at: Some(Utc::now()),
            withdrawn_at: None,
            expires_at,
            version,
        });
        id
    }

    /// Withdraw consent for a purpose.
    pub fn withdraw_consent(&self, user_id: Uuid, purpose: &str) -> bool {
        let mut consents = self.consents.write();
        let mut found = false;
        for c in consents.iter_mut() {
            if c.user_id == user_id && c.purpose == purpose && c.status == ConsentStatus::Granted {
                c.status = ConsentStatus::Withdrawn;
                c.withdrawn_at = Some(Utc::now());
                found = true;
            }
        }
        found
    }

    /// Check if consent is currently active for a purpose.
    pub fn has_consent(&self, user_id: Uuid, purpose: &str) -> bool {
        let consents = self.consents.read();
        let now = Utc::now();
        consents.iter().any(|c| {
            c.user_id == user_id
                && c.purpose == purpose
                && c.status == ConsentStatus::Granted
                && c.expires_at.map_or(true, |exp| now <= exp)
        })
    }

    /// Get all consents for a user.
    pub fn user_consents(&self, user_id: Uuid) -> Vec<ConsentRecord> {
        self.consents
            .read()
            .iter()
            .filter(|c| c.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Submit a data subject request.
    pub fn submit_request(
        &self,
        user_id: Uuid,
        request_type: SubjectRequestType,
        notes: &str,
    ) -> Uuid {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let deadline = now + chrono::Duration::days(self.response_deadline_days);

        self.requests.write().push(DataSubjectRequest {
            id,
            user_id,
            request_type,
            status: RequestStatus::Pending,
            submitted_at: now,
            completed_at: None,
            deadline,
            notes: notes.to_string(),
        });
        id
    }

    /// Complete a data subject request.
    pub fn complete_request(&self, request_id: Uuid) -> bool {
        let mut requests = self.requests.write();
        if let Some(r) = requests.iter_mut().find(|r| r.id == request_id) {
            r.status = RequestStatus::Completed;
            r.completed_at = Some(Utc::now());
            return true;
        }
        false
    }

    /// Deny a data subject request.
    pub fn deny_request(&self, request_id: Uuid, reason: &str) -> bool {
        let mut requests = self.requests.write();
        if let Some(r) = requests.iter_mut().find(|r| r.id == request_id) {
            r.status = RequestStatus::Denied;
            r.notes = format!("{} | Denied: {}", r.notes, reason);
            return true;
        }
        false
    }

    /// Get pending requests that are approaching their deadline.
    pub fn overdue_requests(&self, now: DateTime<Utc>) -> Vec<DataSubjectRequest> {
        self.requests
            .read()
            .iter()
            .filter(|r| r.status == RequestStatus::Pending && now > r.deadline)
            .cloned()
            .collect()
    }

    /// Get all requests for a user.
    pub fn user_requests(&self, user_id: Uuid) -> Vec<DataSubjectRequest> {
        self.requests
            .read()
            .iter()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Generate a data export for a user (for portability/access requests).
    pub fn export_user_data(&self, user_id: Uuid) -> HashMap<String, serde_json::Value> {
        let mut data = HashMap::new();
        let consents = self.user_consents(user_id);
        data.insert(
            "consents".to_string(),
            serde_json::to_value(&consents).unwrap_or_default(),
        );
        let requests = self.user_requests(user_id);
        data.insert(
            "requests".to_string(),
            serde_json::to_value(&requests).unwrap_or_default(),
        );
        data
    }

    /// Expire stale consents. Returns count expired.
    pub fn expire_stale_consents(&self, now: DateTime<Utc>) -> usize {
        let mut consents = self.consents.write();
        let mut expired = 0;
        for c in consents.iter_mut() {
            if c.status == ConsentStatus::Granted {
                if let Some(exp) = c.expires_at {
                    if now > exp {
                        c.status = ConsentStatus::Expired;
                        expired += 1;
                    }
                }
            }
        }
        expired
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_and_check_consent() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        mgr.grant_consent(uid, "navigation_tracking", LegalBasis::Consent, None);
        assert!(mgr.has_consent(uid, "navigation_tracking"));
        assert!(!mgr.has_consent(uid, "marketing"));
    }

    #[test]
    fn test_withdraw_consent() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        mgr.grant_consent(uid, "analytics", LegalBasis::Consent, None);
        assert!(mgr.withdraw_consent(uid, "analytics"));
        assert!(!mgr.has_consent(uid, "analytics"));
    }

    #[test]
    fn test_consent_expiration() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        let expired = Utc::now() - chrono::Duration::hours(1);
        mgr.grant_consent(uid, "temp", LegalBasis::Consent, Some(expired));
        assert!(!mgr.has_consent(uid, "temp"));
    }

    #[test]
    fn test_consent_versioning() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        mgr.grant_consent(uid, "tracking", LegalBasis::Consent, None);
        mgr.withdraw_consent(uid, "tracking");
        mgr.grant_consent(uid, "tracking", LegalBasis::Consent, None);
        let consents = mgr.user_consents(uid);
        assert_eq!(consents.len(), 2);
        assert_eq!(consents[1].version, 2);
    }

    #[test]
    fn test_submit_and_complete_request() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        let rid = mgr.submit_request(uid, SubjectRequestType::Access, "Please provide my data");
        let requests = mgr.user_requests(uid);
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].status, RequestStatus::Pending);
        assert!(mgr.complete_request(rid));
        let reqs = mgr.user_requests(uid);
        assert_eq!(reqs[0].status, RequestStatus::Completed);
        assert!(reqs[0].completed_at.is_some());
    }

    #[test]
    fn test_deny_request() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        let rid = mgr.submit_request(uid, SubjectRequestType::Erasure, "Delete me");
        assert!(mgr.deny_request(rid, "Legal hold"));
        let reqs = mgr.user_requests(uid);
        assert_eq!(reqs[0].status, RequestStatus::Denied);
        assert!(reqs[0].notes.contains("Legal hold"));
    }

    #[test]
    fn test_overdue_requests() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        mgr.submit_request(uid, SubjectRequestType::Access, "Need data");
        let future = Utc::now() + chrono::Duration::days(31);
        let overdue = mgr.overdue_requests(future);
        assert_eq!(overdue.len(), 1);
    }

    #[test]
    fn test_no_overdue_when_within_deadline() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        mgr.submit_request(uid, SubjectRequestType::Access, "Need data");
        let overdue = mgr.overdue_requests(Utc::now());
        assert!(overdue.is_empty());
    }

    #[test]
    fn test_export_user_data() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        mgr.grant_consent(uid, "tracking", LegalBasis::Consent, None);
        mgr.submit_request(uid, SubjectRequestType::Portability, "Export my data");
        let export = mgr.export_user_data(uid);
        assert!(export.contains_key("consents"));
        assert!(export.contains_key("requests"));
    }

    #[test]
    fn test_expire_stale_consents() {
        let mgr = GdprManager::new(30);
        let uid = Uuid::new_v4();
        let past = Utc::now() - chrono::Duration::hours(1);
        mgr.grant_consent(uid, "temp", LegalBasis::Consent, Some(past));
        mgr.grant_consent(uid, "permanent", LegalBasis::Consent, None);
        let expired = mgr.expire_stale_consents(Utc::now());
        assert_eq!(expired, 1);
    }

    #[test]
    fn test_different_users_isolated() {
        let mgr = GdprManager::new(30);
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        mgr.grant_consent(u1, "tracking", LegalBasis::Consent, None);
        assert!(mgr.has_consent(u1, "tracking"));
        assert!(!mgr.has_consent(u2, "tracking"));
    }
}
