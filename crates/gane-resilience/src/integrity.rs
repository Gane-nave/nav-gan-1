//! Data integrity — verification of offline data consistency.
//!
//! Checksums, version vectors, and tamper detection for data stored
//! on the device while offline.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, warn};

/// A checksum algorithm used for data integrity verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChecksumAlgorithm {
    Crc32,
    Sha256,
    Blake3,
}

/// Result of an integrity check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrityResult {
    /// Data is intact and matches checksum.
    Valid,
    /// Checksum mismatch — data may be corrupted.
    Corrupted,
    /// No checksum available for verification.
    Unknown,
    /// Data has been tampered with (signature mismatch).
    Tampered,
}

/// A versioned data record with checksum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    pub entity_id: EntityId,
    pub entity_type: String,
    pub version: u64,
    pub checksum: String,
    pub algorithm: ChecksumAlgorithm,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub last_verified: DateTime<Utc>,
    pub result: IntegrityResult,
}

/// Version vector for conflict-free replicated data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionVector {
    pub node_id: String,
    pub counters: HashMap<String, u64>,
}

impl VersionVector {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            counters: HashMap::new(),
        }
    }

    /// Increment the local counter.
    pub fn increment(&mut self) {
        let counter = self.counters.entry(self.node_id.clone()).or_insert(0);
        *counter += 1;
    }

    /// Merge with another version vector (take max of each counter).
    pub fn merge(&mut self, other: &VersionVector) {
        for (node, &count) in &other.counters {
            let local = self.counters.entry(node.clone()).or_insert(0);
            if count > *local {
                *local = count;
            }
        }
    }

    /// Check if this vector dominates (is causally after) another.
    pub fn dominates(&self, other: &VersionVector) -> bool {
        // self dominates other if all counters in other are <= self,
        // and at least one is strictly less.
        let mut dominated = false;
        for (node, &count) in &other.counters {
            let self_count = self.counters.get(node).copied().unwrap_or(0);
            if self_count < count {
                return false;
            }
            if self_count > count {
                dominated = true;
            }
        }
        // Also check if self has nodes not in other.
        for (node, &count) in &self.counters {
            if count > 0 && !other.counters.contains_key(node) {
                dominated = true;
            }
        }
        dominated
    }

    /// Check if two vectors are concurrent (neither dominates).
    pub fn is_concurrent(&self, other: &VersionVector) -> bool {
        !self.dominates(other) && !other.dominates(self) && self.counters != other.counters
    }

    /// Get the local counter value.
    pub fn local_counter(&self) -> u64 {
        self.counters.get(&self.node_id).copied().unwrap_or(0)
    }
}

/// Data integrity verifier — tracks checksums and detects corruption.
pub struct IntegrityVerifier {
    records: HashMap<EntityId, DataRecord>,
    default_algorithm: ChecksumAlgorithm,
    total_checks: u64,
    total_corrupted: u64,
    total_tampered: u64,
}

impl IntegrityVerifier {
    pub fn new(algorithm: ChecksumAlgorithm) -> Self {
        Self {
            records: HashMap::new(),
            default_algorithm: algorithm,
            total_checks: 0,
            total_corrupted: 0,
            total_tampered: 0,
        }
    }

    /// Register a data record with its checksum.
    pub fn register(
        &mut self,
        entity_id: EntityId,
        entity_type: &str,
        checksum: &str,
        size_bytes: u64,
    ) {
        let now = Utc::now();
        let record = DataRecord {
            entity_id,
            entity_type: entity_type.to_string(),
            version: 1,
            checksum: checksum.to_string(),
            algorithm: self.default_algorithm,
            size_bytes,
            created_at: now,
            last_verified: now,
            result: IntegrityResult::Valid,
        };
        debug!(entity = %entity_id, checksum = checksum, "registered data record");
        self.records.insert(entity_id, record);
    }

    /// Verify a data record against an expected checksum.
    /// Returns the integrity result.
    pub fn verify(&mut self, entity_id: &EntityId, actual_checksum: &str) -> IntegrityResult {
        self.total_checks += 1;

        let Some(record) = self.records.get_mut(entity_id) else {
            return IntegrityResult::Unknown;
        };

        record.last_verified = Utc::now();

        if record.checksum == actual_checksum {
            record.result = IntegrityResult::Valid;
            debug!(entity = %entity_id, "integrity check passed");
            IntegrityResult::Valid
        } else {
            record.result = IntegrityResult::Corrupted;
            self.total_corrupted += 1;
            warn!(
                entity = %entity_id,
                expected = %record.checksum,
                actual = actual_checksum,
                "integrity check FAILED — data corrupted"
            );
            IntegrityResult::Corrupted
        }
    }

    /// Mark a record as tampered (external detection).
    pub fn mark_tampered(&mut self, entity_id: &EntityId) -> bool {
        if let Some(record) = self.records.get_mut(entity_id) {
            record.result = IntegrityResult::Tampered;
            self.total_tampered += 1;
            warn!(entity = %entity_id, "data marked as TAMPERED");
            true
        } else {
            false
        }
    }

    /// Update the checksum for a record (after legitimate modification).
    pub fn update_checksum(
        &mut self,
        entity_id: &EntityId,
        new_checksum: &str,
        new_size: u64,
    ) -> bool {
        if let Some(record) = self.records.get_mut(entity_id) {
            record.checksum = new_checksum.to_string();
            record.size_bytes = new_size;
            record.version += 1;
            record.result = IntegrityResult::Valid;
            debug!(
                entity = %entity_id,
                version = record.version,
                "checksum updated"
            );
            true
        } else {
            false
        }
    }

    /// Get all corrupted records.
    pub fn corrupted_records(&self) -> Vec<&DataRecord> {
        self.records
            .values()
            .filter(|r| r.result == IntegrityResult::Corrupted)
            .collect()
    }

    /// Get all tampered records.
    pub fn tampered_records(&self) -> Vec<&DataRecord> {
        self.records
            .values()
            .filter(|r| r.result == IntegrityResult::Tampered)
            .collect()
    }

    /// Get a record by entity id.
    pub fn record(&self, entity_id: &EntityId) -> Option<&DataRecord> {
        self.records.get(entity_id)
    }

    /// Total integrity checks performed.
    pub fn total_checks(&self) -> u64 {
        self.total_checks
    }

    /// Total corrupted detections.
    pub fn total_corrupted(&self) -> u64 {
        self.total_corrupted
    }

    /// Total tampered detections.
    pub fn total_tampered(&self) -> u64 {
        self.total_tampered
    }

    /// Number of tracked records.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}

impl Default for IntegrityVerifier {
    fn default() -> Self {
        Self::new(ChecksumAlgorithm::Crc32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_verify_valid() {
        let mut v = IntegrityVerifier::new(ChecksumAlgorithm::Sha256);
        let id = EntityId::new();
        v.register(id, "MapTile", "abc123", 1024);

        let result = v.verify(&id, "abc123");
        assert_eq!(result, IntegrityResult::Valid);
        assert_eq!(v.total_checks(), 1);
        assert_eq!(v.total_corrupted(), 0);
    }

    #[test]
    fn verify_detects_corruption() {
        let mut v = IntegrityVerifier::new(ChecksumAlgorithm::Crc32);
        let id = EntityId::new();
        v.register(id, "Route", "expected_hash", 512);

        let result = v.verify(&id, "different_hash");
        assert_eq!(result, IntegrityResult::Corrupted);
        assert_eq!(v.total_corrupted(), 1);
        assert_eq!(v.corrupted_records().len(), 1);
    }

    #[test]
    fn verify_unknown_entity() {
        let mut v = IntegrityVerifier::default();
        let result = v.verify(&EntityId::new(), "hash");
        assert_eq!(result, IntegrityResult::Unknown);
    }

    #[test]
    fn mark_tampered() {
        let mut v = IntegrityVerifier::default();
        let id = EntityId::new();
        v.register(id, "Evidence", "hash", 256);

        assert!(v.mark_tampered(&id));
        assert_eq!(v.tampered_records().len(), 1);
        assert_eq!(v.total_tampered(), 1);
    }

    #[test]
    fn update_checksum_increments_version() {
        let mut v = IntegrityVerifier::default();
        let id = EntityId::new();
        v.register(id, "Tile", "v1hash", 100);

        assert!(v.update_checksum(&id, "v2hash", 200));
        let record = v.record(&id).unwrap();
        assert_eq!(record.version, 2);
        assert_eq!(record.checksum, "v2hash");
        assert_eq!(record.size_bytes, 200);
    }

    #[test]
    fn update_checksum_nonexistent_returns_false() {
        let mut v = IntegrityVerifier::default();
        assert!(!v.update_checksum(&EntityId::new(), "hash", 100));
    }

    #[test]
    fn version_vector_increment_and_merge() {
        let mut v1 = VersionVector::new("node-a");
        v1.increment();
        v1.increment();
        assert_eq!(v1.local_counter(), 2);

        let mut v2 = VersionVector::new("node-b");
        v2.increment();

        v1.merge(&v2);
        assert_eq!(v1.counters.get("node-a"), Some(&2));
        assert_eq!(v1.counters.get("node-b"), Some(&1));
    }

    #[test]
    fn version_vector_dominates() {
        let mut v1 = VersionVector::new("a");
        v1.increment();
        v1.increment();

        let mut v2 = VersionVector::new("a");
        v2.increment();

        assert!(v1.dominates(&v2));
        assert!(!v2.dominates(&v1));
    }

    #[test]
    fn version_vector_concurrent() {
        let mut v1 = VersionVector::new("a");
        v1.increment();

        let mut v2 = VersionVector::new("b");
        v2.increment();

        assert!(v1.is_concurrent(&v2));
        assert!(v2.is_concurrent(&v1));
    }

    #[test]
    fn version_vector_equal_not_concurrent() {
        let v1 = VersionVector::new("a");
        let v2 = VersionVector::new("b");
        // Both empty — equal, not concurrent.
        assert!(!v1.is_concurrent(&v2));
    }

    #[test]
    fn mark_tampered_nonexistent_returns_false() {
        let mut v = IntegrityVerifier::default();
        assert!(!v.mark_tampered(&EntityId::new()));
    }
}
