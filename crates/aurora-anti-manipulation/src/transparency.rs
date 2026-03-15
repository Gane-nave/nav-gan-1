//! Transparency logger — immutable algorithm audit trails for accountability.

use aurora_core::types::{EntityId, TransparencyLog};
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info};

/// An appeal against an algorithmic decision.
#[derive(Debug, Clone)]
pub struct Appeal {
    pub id: EntityId,
    pub log_id: EntityId,
    pub appellant_id: EntityId,
    pub reason: String,
    pub status: AppealStatus,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub resolution: Option<String>,
}

/// Status of an appeal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppealStatus {
    Submitted,
    UnderReview,
    Upheld,
    Overturned,
    Dismissed,
}

/// Immutable transparency logger for algorithm decisions.
///
/// Records every significant algorithmic decision with its inputs, outputs,
/// parameters, and version, enabling full accountability and auditability.
pub struct TransparencyLogger {
    /// Immutable log entries (append-only).
    logs: Vec<TransparencyLog>,
    /// Index by algorithm name for efficient lookup.
    by_algorithm: HashMap<String, Vec<usize>>,
    /// Appeals against algorithmic decisions.
    appeals: HashMap<EntityId, Appeal>,
}

impl TransparencyLogger {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            by_algorithm: HashMap::new(),
            appeals: HashMap::new(),
        }
    }

    /// Record an algorithmic decision.
    pub fn record(
        &mut self,
        algorithm: &str,
        version: &str,
        input_hash: &str,
        output_hash: &str,
        parameters: Value,
    ) -> EntityId {
        let id = EntityId::new();
        let log = TransparencyLog {
            id,
            timestamp: Utc::now(),
            algorithm: algorithm.into(),
            version: version.into(),
            input_hash: input_hash.into(),
            output_hash: output_hash.into(),
            parameters,
        };

        let index = self.logs.len();
        self.logs.push(log);

        self.by_algorithm
            .entry(algorithm.into())
            .or_default()
            .push(index);

        debug!(algorithm, version, log_id = %id, "transparency log recorded");
        id
    }

    /// Get a log entry by ID.
    pub fn get(&self, id: &EntityId) -> Option<&TransparencyLog> {
        self.logs.iter().find(|l| l.id == *id)
    }

    /// Get all log entries for a specific algorithm.
    pub fn logs_for_algorithm(&self, algorithm: &str) -> Vec<&TransparencyLog> {
        self.by_algorithm
            .get(algorithm)
            .map(|indices| indices.iter().map(|&i| &self.logs[i]).collect())
            .unwrap_or_default()
    }

    /// Total number of log entries.
    pub fn count(&self) -> usize {
        self.logs.len()
    }

    /// Get all unique algorithms that have been logged.
    pub fn algorithms(&self) -> Vec<&str> {
        self.by_algorithm.keys().map(|s| s.as_str()).collect()
    }

    /// Submit an appeal against an algorithmic decision.
    pub fn submit_appeal(
        &mut self,
        log_id: EntityId,
        appellant_id: EntityId,
        reason: &str,
    ) -> Option<EntityId> {
        // Verify the log entry exists.
        self.get(&log_id)?;

        let appeal_id = EntityId::new();
        let appeal = Appeal {
            id: appeal_id,
            log_id,
            appellant_id,
            reason: reason.into(),
            status: AppealStatus::Submitted,
            submitted_at: Utc::now(),
            resolved_at: None,
            resolution: None,
        };

        info!(appeal_id = %appeal_id, log_id = %log_id, "appeal submitted");
        self.appeals.insert(appeal_id, appeal);
        Some(appeal_id)
    }

    /// Resolve an appeal.
    pub fn resolve_appeal(
        &mut self,
        appeal_id: EntityId,
        status: AppealStatus,
        resolution: &str,
    ) -> bool {
        if let Some(appeal) = self.appeals.get_mut(&appeal_id) {
            appeal.status = status;
            appeal.resolution = Some(resolution.into());
            appeal.resolved_at = Some(Utc::now());
            info!(appeal_id = %appeal_id, status = ?status, "appeal resolved");
            true
        } else {
            false
        }
    }

    /// Get an appeal by ID.
    pub fn get_appeal(&self, id: &EntityId) -> Option<&Appeal> {
        self.appeals.get(id)
    }

    /// Get all pending appeals.
    pub fn pending_appeals(&self) -> Vec<&Appeal> {
        self.appeals
            .values()
            .filter(|a| {
                a.status == AppealStatus::Submitted || a.status == AppealStatus::UnderReview
            })
            .collect()
    }

    /// Verify the integrity of the log (no gaps, monotonic timestamps).
    pub fn verify_integrity(&self) -> bool {
        if self.logs.len() < 2 {
            return true;
        }

        for i in 1..self.logs.len() {
            if self.logs[i].timestamp < self.logs[i - 1].timestamp {
                return false; // non-monotonic timestamp
            }
        }
        true
    }

    /// Export all logs as JSON for external audit.
    pub fn export_json(&self) -> Value {
        serde_json::to_value(&self.logs).unwrap_or(Value::Array(Vec::new()))
    }
}

impl Default for TransparencyLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_retrieve_log() {
        let mut logger = TransparencyLogger::new();
        let id = logger.record(
            "route_ranking",
            "1.0.0",
            "sha256:input_abc",
            "sha256:output_xyz",
            serde_json::json!({"weight_time": 0.35, "weight_risk": 0.20}),
        );

        let log = logger.get(&id).unwrap();
        assert_eq!(log.algorithm, "route_ranking");
        assert_eq!(log.version, "1.0.0");
    }

    #[test]
    fn logs_indexed_by_algorithm() {
        let mut logger = TransparencyLogger::new();

        logger.record("route_ranking", "1.0.0", "h1", "h2", Value::Null);
        logger.record("route_ranking", "1.0.0", "h3", "h4", Value::Null);
        logger.record("risk_scoring", "2.0.0", "h5", "h6", Value::Null);

        assert_eq!(logger.logs_for_algorithm("route_ranking").len(), 2);
        assert_eq!(logger.logs_for_algorithm("risk_scoring").len(), 1);
        assert_eq!(logger.logs_for_algorithm("unknown").len(), 0);
    }

    #[test]
    fn appeal_workflow() {
        let mut logger = TransparencyLogger::new();
        let log_id = logger.record("trust_scoring", "1.0.0", "h1", "h2", Value::Null);

        let user = EntityId::new();
        let appeal_id = logger
            .submit_appeal(log_id, user, "My trust score was unfairly reduced")
            .unwrap();

        let appeal = logger.get_appeal(&appeal_id).unwrap();
        assert_eq!(appeal.status, AppealStatus::Submitted);

        logger.resolve_appeal(appeal_id, AppealStatus::Upheld, "Score adjustment applied");
        let resolved = logger.get_appeal(&appeal_id).unwrap();
        assert_eq!(resolved.status, AppealStatus::Upheld);
        assert!(resolved.resolution.is_some());
    }

    #[test]
    fn appeal_requires_valid_log() {
        let mut logger = TransparencyLogger::new();
        let fake_id = EntityId::new();
        let user = EntityId::new();

        let result = logger.submit_appeal(fake_id, user, "Invalid log");
        assert!(result.is_none());
    }

    #[test]
    fn integrity_verification() {
        let mut logger = TransparencyLogger::new();
        logger.record("algo1", "1.0", "h1", "h2", Value::Null);
        logger.record("algo2", "1.0", "h3", "h4", Value::Null);
        logger.record("algo3", "1.0", "h5", "h6", Value::Null);

        assert!(logger.verify_integrity());
    }

    #[test]
    fn pending_appeals_filtered() {
        let mut logger = TransparencyLogger::new();
        let l1 = logger.record("algo1", "1.0", "h1", "h2", Value::Null);
        let l2 = logger.record("algo2", "1.0", "h3", "h4", Value::Null);

        let user = EntityId::new();
        let a1 = logger.submit_appeal(l1, user, "Reason 1").unwrap();
        logger.submit_appeal(l2, user, "Reason 2").unwrap();

        assert_eq!(logger.pending_appeals().len(), 2);

        logger.resolve_appeal(a1, AppealStatus::Dismissed, "No merit");
        assert_eq!(logger.pending_appeals().len(), 1);
    }

    #[test]
    fn export_json_valid() {
        let mut logger = TransparencyLogger::new();
        logger.record(
            "algo1",
            "1.0",
            "h1",
            "h2",
            serde_json::json!({"key": "val"}),
        );

        let exported = logger.export_json();
        assert!(exported.is_array());
        assert_eq!(exported.as_array().unwrap().len(), 1);
    }
}
