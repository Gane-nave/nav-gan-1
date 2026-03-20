//! Evidence vault — encrypted storage with integrity verification and privacy processing.

use aurora_core::incident::{Evidence, EvidenceType};
use aurora_core::types::EntityId;
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Signed metadata for evidence integrity verification.
#[derive(Debug, Clone)]
pub struct SignedMetadata {
    /// Hash of the evidence content.
    pub content_hash: String,
    /// Timestamp when the signature was created.
    pub signed_at: chrono::DateTime<chrono::Utc>,
    /// Device ID that signed the evidence.
    pub signer_id: EntityId,
    /// Signature (hex-encoded, placeholder for real crypto).
    pub signature: String,
}

/// Privacy processing status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyStatus {
    /// Not yet processed.
    Pending,
    /// Privacy blur applied (faces, plates).
    Processed,
    /// Failed to process — evidence quarantined.
    Failed,
}

/// Encrypted evidence vault with integrity and privacy controls.
pub struct EvidenceVault {
    /// Evidence storage keyed by evidence ID.
    evidence: HashMap<EntityId, Evidence>,
    /// Signed metadata per evidence.
    signatures: HashMap<EntityId, SignedMetadata>,
    /// Privacy processing status per evidence.
    privacy_status: HashMap<EntityId, PrivacyStatus>,
    /// Total storage used (approximate bytes).
    storage_used: u64,
    /// Maximum storage budget in bytes.
    max_storage: u64,
}

impl EvidenceVault {
    pub fn new(max_storage: u64) -> Self {
        Self {
            evidence: HashMap::new(),
            signatures: HashMap::new(),
            privacy_status: HashMap::new(),
            storage_used: 0,
            max_storage,
        }
    }

    /// Store evidence in the vault with signing.
    pub fn store(
        &mut self,
        evidence: Evidence,
        signer_id: EntityId,
    ) -> Result<EntityId, VaultError> {
        let id = evidence.id;
        let approx_size: u64 = 1024; // approximate per-evidence overhead

        if self.storage_used + approx_size > self.max_storage {
            warn!(
                vault_usage = self.storage_used,
                max = self.max_storage,
                "vault storage full"
            );
            return Err(VaultError::StorageFull);
        }

        // Generate signed metadata.
        let content_hash = evidence
            .media_hash
            .clone()
            .unwrap_or_else(|| format!("sha256:{}", id));
        let signature = format!("sig:{}:{}", signer_id, content_hash);

        let metadata = SignedMetadata {
            content_hash,
            signed_at: Utc::now(),
            signer_id,
            signature,
        };

        self.signatures.insert(id, metadata);
        self.privacy_status.insert(id, PrivacyStatus::Pending);
        self.storage_used += approx_size;
        self.evidence.insert(id, evidence);

        debug!(evidence_id = %id, "evidence stored in vault");
        Ok(id)
    }

    /// Mark evidence as privacy-processed (faces/plates blurred).
    pub fn mark_privacy_processed(&mut self, evidence_id: EntityId) -> bool {
        if let Some(evidence) = self.evidence.get_mut(&evidence_id) {
            evidence.privacy_processed = true;
            self.privacy_status
                .insert(evidence_id, PrivacyStatus::Processed);
            debug!(evidence_id = %evidence_id, "privacy processing completed");
            true
        } else {
            false
        }
    }

    /// Mark evidence as encrypted.
    pub fn mark_encrypted(&mut self, evidence_id: EntityId) -> bool {
        if let Some(evidence) = self.evidence.get_mut(&evidence_id) {
            evidence.encrypted = true;
            debug!(evidence_id = %evidence_id, "evidence encrypted");
            true
        } else {
            false
        }
    }

    /// Verify integrity of stored evidence using its signed metadata.
    pub fn verify_integrity(&self, evidence_id: &EntityId) -> IntegrityResult {
        let evidence = match self.evidence.get(evidence_id) {
            Some(e) => e,
            None => return IntegrityResult::NotFound,
        };

        let metadata = match self.signatures.get(evidence_id) {
            Some(m) => m,
            None => return IntegrityResult::NoSignature,
        };

        // Verify content hash matches.
        let current_hash = evidence
            .media_hash
            .clone()
            .unwrap_or_else(|| format!("sha256:{}", evidence.id));

        if current_hash == metadata.content_hash {
            IntegrityResult::Valid
        } else {
            IntegrityResult::Tampered
        }
    }

    /// Get evidence by ID.
    pub fn get(&self, id: &EntityId) -> Option<&Evidence> {
        self.evidence.get(id)
    }

    /// Get signed metadata for evidence.
    pub fn get_signature(&self, id: &EntityId) -> Option<&SignedMetadata> {
        self.signatures.get(id)
    }

    /// Get privacy status.
    pub fn privacy_status(&self, id: &EntityId) -> Option<PrivacyStatus> {
        self.privacy_status.get(id).copied()
    }

    /// Count of stored evidence items.
    pub fn count(&self) -> usize {
        self.evidence.len()
    }

    /// Get all evidence for a specific incident.
    pub fn evidence_for_incident(&self, incident_id: EntityId) -> Vec<&Evidence> {
        self.evidence
            .values()
            .filter(|e| e.incident_id == Some(incident_id))
            .collect()
    }

    /// Get evidence by type.
    pub fn evidence_by_type(&self, evidence_type: EvidenceType) -> Vec<&Evidence> {
        self.evidence
            .values()
            .filter(|e| e.evidence_type == evidence_type)
            .collect()
    }

    /// Remove expired or rejected evidence to free storage.
    pub fn purge(&mut self, ids: &[EntityId]) -> usize {
        let mut removed = 0;
        for id in ids {
            if self.evidence.remove(id).is_some() {
                self.signatures.remove(id);
                self.privacy_status.remove(id);
                self.storage_used = self.storage_used.saturating_sub(1024);
                removed += 1;
            }
        }
        info!(removed, "evidence purged from vault");
        removed
    }
}

/// Result of an integrity verification check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityResult {
    /// Evidence hash matches signature.
    Valid,
    /// Evidence has been tampered with.
    Tampered,
    /// No signature found for evidence.
    NoSignature,
    /// Evidence not found in vault.
    NotFound,
}

/// Vault errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultError {
    /// Storage budget exceeded.
    StorageFull,
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::types::GeoPosition;

    fn make_evidence(incident_id: Option<EntityId>, etype: EvidenceType) -> Evidence {
        Evidence {
            id: EntityId::new(),
            incident_id,
            reporter_id: EntityId::new(),
            evidence_type: etype,
            position: GeoPosition {
                latitude_deg: 32.08,
                longitude_deg: 34.78,
                altitude_m: None,
            },
            heading_deg: Some(90.0),
            timestamp: Utc::now(),
            media_url: Some("https://evidence.example.com/file".into()),
            media_hash: Some("sha256:abc123".into()),
            signed_metadata: None,
            privacy_processed: false,
            encrypted: false,
        }
    }

    #[test]
    fn store_and_retrieve_evidence() {
        let mut vault = EvidenceVault::new(1_000_000);
        let signer = EntityId::new();
        let evidence = make_evidence(None, EvidenceType::Photo);
        let id = evidence.id;

        vault.store(evidence, signer).unwrap();
        assert_eq!(vault.count(), 1);
        assert!(vault.get(&id).is_some());
    }

    #[test]
    fn integrity_verification_valid() {
        let mut vault = EvidenceVault::new(1_000_000);
        let signer = EntityId::new();
        let evidence = make_evidence(None, EvidenceType::Video);
        let id = evidence.id;

        vault.store(evidence, signer).unwrap();
        assert_eq!(vault.verify_integrity(&id), IntegrityResult::Valid);
    }

    #[test]
    fn integrity_verification_not_found() {
        let vault = EvidenceVault::new(1_000_000);
        let fake_id = EntityId::new();
        assert_eq!(vault.verify_integrity(&fake_id), IntegrityResult::NotFound);
    }

    #[test]
    fn privacy_processing_workflow() {
        let mut vault = EvidenceVault::new(1_000_000);
        let signer = EntityId::new();
        let evidence = make_evidence(None, EvidenceType::Photo);
        let id = evidence.id;

        vault.store(evidence, signer).unwrap();
        assert_eq!(vault.privacy_status(&id), Some(PrivacyStatus::Pending));

        vault.mark_privacy_processed(id);
        assert_eq!(vault.privacy_status(&id), Some(PrivacyStatus::Processed));
        assert!(vault.get(&id).unwrap().privacy_processed);
    }

    #[test]
    fn encryption_workflow() {
        let mut vault = EvidenceVault::new(1_000_000);
        let signer = EntityId::new();
        let evidence = make_evidence(None, EvidenceType::VoiceNote);
        let id = evidence.id;

        vault.store(evidence, signer).unwrap();
        assert!(!vault.get(&id).unwrap().encrypted);

        vault.mark_encrypted(id);
        assert!(vault.get(&id).unwrap().encrypted);
    }

    #[test]
    fn storage_full_returns_error() {
        let mut vault = EvidenceVault::new(500); // very small
        let signer = EntityId::new();

        // Fill vault
        vault
            .store(make_evidence(None, EvidenceType::Photo), signer)
            .ok();

        // Next should fail
        let result = vault.store(make_evidence(None, EvidenceType::Photo), signer);
        assert_eq!(result, Err(VaultError::StorageFull));
    }

    #[test]
    fn evidence_for_incident_filters() {
        let mut vault = EvidenceVault::new(1_000_000);
        let signer = EntityId::new();
        let incident_id = EntityId::new();

        vault
            .store(
                make_evidence(Some(incident_id), EvidenceType::Photo),
                signer,
            )
            .unwrap();
        vault
            .store(
                make_evidence(Some(incident_id), EvidenceType::Video),
                signer,
            )
            .unwrap();
        vault
            .store(make_evidence(None, EvidenceType::Photo), signer)
            .unwrap();

        assert_eq!(vault.evidence_for_incident(incident_id).len(), 2);
    }

    #[test]
    fn purge_removes_evidence() {
        let mut vault = EvidenceVault::new(1_000_000);
        let signer = EntityId::new();
        let e1 = make_evidence(None, EvidenceType::Photo);
        let id1 = e1.id;
        let e2 = make_evidence(None, EvidenceType::Photo);
        let id2 = e2.id;

        vault.store(e1, signer).unwrap();
        vault.store(e2, signer).unwrap();
        assert_eq!(vault.count(), 2);

        vault.purge(&[id1]);
        assert_eq!(vault.count(), 1);
        assert!(vault.get(&id1).is_none());
        assert!(vault.get(&id2).is_some());
    }
}
