//! Priority queue — offline operation queue with urgency-based ordering.
//!
//! Operations are queued while offline and drained in priority order
//! when connectivity is restored. Supports TTL expiration and deduplication.

use aurora_core::types::EntityId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, warn};

/// Priority level for queued operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum QueuePriority {
    /// Background — sync when convenient.
    Low,
    /// Normal user-initiated operations.
    Normal,
    /// Time-sensitive — sync as soon as possible.
    High,
    /// Safety-critical — must sync immediately when online.
    Critical,
}

/// A queued offline operation with priority and TTL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueEntry {
    pub id: EntityId,
    pub priority: QueuePriority,
    pub operation_type: String,
    pub entity_type: String,
    pub entity_id: EntityId,
    pub payload: String,
    pub created_at: DateTime<Utc>,
    /// Time-to-live — entry expires after this duration.
    pub ttl_seconds: Option<u64>,
    /// Number of sync attempts.
    pub attempts: u32,
    /// Maximum sync attempts before dropping.
    pub max_attempts: u32,
    pub status: EntryStatus,
}

/// Status of a queue entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryStatus {
    Pending,
    Processing,
    Completed,
    Expired,
    Failed,
}

/// Priority queue for offline operations.
pub struct PriorityQueue {
    entries: Vec<QueueEntry>,
    /// Deduplication map: entity_id → most recent entry id.
    dedup_map: HashMap<EntityId, EntityId>,
    /// Maximum queue size.
    max_size: usize,
    /// Total entries processed.
    total_processed: u64,
    /// Total entries expired.
    total_expired: u64,
    /// Total entries failed.
    total_failed: u64,
}

impl PriorityQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            dedup_map: HashMap::new(),
            max_size,
            total_processed: 0,
            total_expired: 0,
            total_failed: 0,
        }
    }

    /// Enqueue an operation. If the queue is full, drops the lowest-priority entry.
    /// Returns the entry id, or None if the entry was rejected (lower priority than all).
    pub fn enqueue(&mut self, entry: QueueEntry) -> Option<EntityId> {
        // Deduplicate: if we already have an entry for this entity, replace it
        // only if the new entry has equal or higher priority.
        if let Some(existing_id) = self.dedup_map.get(&entry.entity_id).copied() {
            if let Some(existing) = self.entries.iter().find(|e| e.id == existing_id) {
                if entry.priority < existing.priority {
                    debug!(
                        entity = %entry.entity_id,
                        "dedup: new entry has lower priority, keeping existing"
                    );
                    return None;
                }
            }
            // Remove old entry for this entity.
            self.entries.retain(|e| e.id != existing_id);
        }

        let entry_id = entry.id;
        let entity_id = entry.entity_id;

        if self.entries.len() >= self.max_size {
            // Find lowest priority entry and drop it.
            if let Some(min_idx) = self
                .entries
                .iter()
                .enumerate()
                .min_by_key(|(_, e)| e.priority)
                .map(|(i, _)| i)
            {
                if self.entries[min_idx].priority >= entry.priority {
                    debug!("queue full, new entry has lowest priority — rejected");
                    return None;
                }
                let dropped = self.entries.remove(min_idx);
                self.dedup_map.remove(&dropped.entity_id);
                warn!(
                    dropped = %dropped.id,
                    priority = ?dropped.priority,
                    "queue full — dropped lowest priority entry"
                );
            }
        }

        self.dedup_map.insert(entity_id, entry_id);
        self.entries.push(entry);
        Some(entry_id)
    }

    /// Dequeue the highest-priority, oldest entry. Skips expired entries.
    pub fn dequeue(&mut self) -> Option<QueueEntry> {
        self.expire_stale();

        // Sort by priority (descending), then by creation time (ascending = oldest first).
        self.entries.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then(a.created_at.cmp(&b.created_at))
        });

        let idx = self
            .entries
            .iter()
            .position(|e| e.status == EntryStatus::Pending)?;

        let mut entry = self.entries.remove(idx);
        entry.status = EntryStatus::Processing;
        entry.attempts += 1;
        self.dedup_map.remove(&entry.entity_id);
        self.total_processed += 1;
        Some(entry)
    }

    /// Mark an entry as failed and re-enqueue if retries remain.
    pub fn mark_failed(&mut self, mut entry: QueueEntry) {
        if entry.attempts >= entry.max_attempts {
            entry.status = EntryStatus::Failed;
            self.total_failed += 1;
            warn!(
                id = %entry.id,
                attempts = entry.attempts,
                "entry exhausted retries — marking failed"
            );
        } else {
            entry.status = EntryStatus::Pending;
            self.dedup_map.insert(entry.entity_id, entry.id);
            self.entries.push(entry);
        }
    }

    /// Expire entries that have exceeded their TTL.
    fn expire_stale(&mut self) {
        let now = Utc::now();
        let mut expired_count = 0u64;

        self.entries.retain(|e| {
            if let Some(ttl) = e.ttl_seconds {
                let expiry = e.created_at + Duration::seconds(ttl as i64);
                if now > expiry && e.status == EntryStatus::Pending {
                    expired_count += 1;
                    return false;
                }
            }
            true
        });

        self.total_expired += expired_count;
        if expired_count > 0 {
            debug!(count = expired_count, "expired stale queue entries");
            // Clean up dedup map.
            let entry_ids: std::collections::HashSet<EntityId> =
                self.entries.iter().map(|e| e.entity_id).collect();
            self.dedup_map.retain(|k, _| entry_ids.contains(k));
        }
    }

    /// Number of pending entries.
    pub fn pending_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.status == EntryStatus::Pending)
            .count()
    }

    /// Total queue size (all statuses).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Total entries processed.
    pub fn total_processed(&self) -> u64 {
        self.total_processed
    }

    /// Total entries expired.
    pub fn total_expired(&self) -> u64 {
        self.total_expired
    }

    /// Total entries failed.
    pub fn total_failed(&self) -> u64 {
        self.total_failed
    }

    /// Maximum queue size.
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Create a new queue entry helper.
    pub fn new_entry(
        priority: QueuePriority,
        operation_type: &str,
        entity_type: &str,
        entity_id: EntityId,
        payload: &str,
        ttl_seconds: Option<u64>,
    ) -> QueueEntry {
        QueueEntry {
            id: EntityId::new(),
            priority,
            operation_type: operation_type.to_string(),
            entity_type: entity_type.to_string(),
            entity_id,
            payload: payload.to_string(),
            created_at: Utc::now(),
            ttl_seconds,
            attempts: 0,
            max_attempts: 3,
            status: EntryStatus::Pending,
        }
    }
}

impl Default for PriorityQueue {
    fn default() -> Self {
        Self::new(10_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(priority: QueuePriority, entity_id: EntityId) -> QueueEntry {
        PriorityQueue::new_entry(priority, "update", "Position", entity_id, "{}", None)
    }

    #[test]
    fn enqueue_and_dequeue_by_priority() {
        let mut q = PriorityQueue::new(100);
        let low_id = EntityId::new();
        let high_id = EntityId::new();

        q.enqueue(make_entry(QueuePriority::Low, low_id));
        q.enqueue(make_entry(QueuePriority::Critical, high_id));

        let first = q.dequeue().unwrap();
        assert_eq!(first.entity_id, high_id);
        assert_eq!(first.priority, QueuePriority::Critical);

        let second = q.dequeue().unwrap();
        assert_eq!(second.entity_id, low_id);
    }

    #[test]
    fn dedup_replaces_lower_priority() {
        let mut q = PriorityQueue::new(100);
        let entity = EntityId::new();

        q.enqueue(make_entry(QueuePriority::Low, entity));
        assert_eq!(q.len(), 1);

        // Higher priority for same entity replaces.
        q.enqueue(make_entry(QueuePriority::Critical, entity));
        assert_eq!(q.len(), 1);

        let entry = q.dequeue().unwrap();
        assert_eq!(entry.priority, QueuePriority::Critical);
    }

    #[test]
    fn dedup_rejects_lower_priority() {
        let mut q = PriorityQueue::new(100);
        let entity = EntityId::new();

        q.enqueue(make_entry(QueuePriority::Critical, entity));
        let result = q.enqueue(make_entry(QueuePriority::Low, entity));
        assert!(result.is_none());
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn full_queue_drops_lowest() {
        let mut q = PriorityQueue::new(2);
        let e1 = EntityId::new();
        let e2 = EntityId::new();
        let e3 = EntityId::new();

        q.enqueue(make_entry(QueuePriority::Low, e1));
        q.enqueue(make_entry(QueuePriority::Normal, e2));
        assert_eq!(q.len(), 2);

        // High priority should evict Low.
        q.enqueue(make_entry(QueuePriority::High, e3));
        assert_eq!(q.len(), 2);

        let first = q.dequeue().unwrap();
        assert_eq!(first.priority, QueuePriority::High);
        let second = q.dequeue().unwrap();
        assert_eq!(second.priority, QueuePriority::Normal);
    }

    #[test]
    fn full_queue_rejects_lowest_priority_entry() {
        let mut q = PriorityQueue::new(2);
        let e1 = EntityId::new();
        let e2 = EntityId::new();
        let e3 = EntityId::new();

        q.enqueue(make_entry(QueuePriority::Normal, e1));
        q.enqueue(make_entry(QueuePriority::High, e2));

        // Low priority can't evict anything.
        let result = q.enqueue(make_entry(QueuePriority::Low, e3));
        assert!(result.is_none());
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn retry_on_failure() {
        let mut q = PriorityQueue::new(100);
        let entity = EntityId::new();
        q.enqueue(make_entry(QueuePriority::Normal, entity));

        let entry = q.dequeue().unwrap();
        assert_eq!(entry.attempts, 1);

        // Re-enqueue on failure.
        q.mark_failed(entry);
        assert_eq!(q.pending_count(), 1);

        let entry2 = q.dequeue().unwrap();
        assert_eq!(entry2.attempts, 2);
    }

    #[test]
    fn max_retries_exhausted() {
        let mut q = PriorityQueue::new(100);
        let entity = EntityId::new();
        let mut entry = make_entry(QueuePriority::Normal, entity);
        entry.max_attempts = 2;
        q.enqueue(entry);

        let e1 = q.dequeue().unwrap();
        q.mark_failed(e1);
        let e2 = q.dequeue().unwrap();
        assert_eq!(e2.attempts, 2);
        q.mark_failed(e2);

        // Should be gone — exhausted retries.
        assert_eq!(q.pending_count(), 0);
        assert_eq!(q.total_failed(), 1);
    }

    #[test]
    fn ttl_expiration() {
        let mut q = PriorityQueue::new(100);
        let entity = EntityId::new();
        let mut entry = make_entry(QueuePriority::Normal, entity);
        // Set TTL to 0 so it expires immediately.
        entry.ttl_seconds = Some(0);
        entry.created_at = Utc::now() - Duration::seconds(1);
        q.enqueue(entry);

        assert_eq!(q.len(), 1);
        let result = q.dequeue();
        assert!(result.is_none());
        assert_eq!(q.total_expired(), 1);
    }

    #[test]
    fn empty_queue_returns_none() {
        let mut q = PriorityQueue::new(100);
        assert!(q.dequeue().is_none());
        assert!(q.is_empty());
    }

    #[test]
    fn default_queue_has_10k_capacity() {
        let q = PriorityQueue::default();
        assert_eq!(q.max_size(), 10_000);
    }

    #[test]
    fn processed_counter_increments() {
        let mut q = PriorityQueue::new(100);
        q.enqueue(make_entry(QueuePriority::Normal, EntityId::new()));
        q.enqueue(make_entry(QueuePriority::Normal, EntityId::new()));

        q.dequeue();
        q.dequeue();
        assert_eq!(q.total_processed(), 2);
    }
}
