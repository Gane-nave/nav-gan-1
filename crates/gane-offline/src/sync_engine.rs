//! Sync engine — deterministic offline-to-online synchronization.
//!
//! Implements the Offline-First Architecture principle with conflict resolution.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use std::collections::VecDeque;
use tracing::{debug, info, warn};

/// A queued operation waiting to be synced.
#[derive(Debug, Clone)]
pub struct SyncOperation {
    pub id: EntityId,
    pub operation_type: SyncOperationType,
    pub entity_type: String,
    pub entity_id: EntityId,
    pub payload: String,
    pub created_at: DateTime<Utc>,
    pub retry_count: u32,
    pub status: SyncStatus,
}

/// Type of sync operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncOperationType {
    Create,
    Update,
    Delete,
}

/// Status of a sync operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Conflict,
}

/// Conflict resolution strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictStrategy {
    /// Local changes win.
    LocalWins,
    /// Server changes win.
    ServerWins,
    /// Most recent change wins.
    LastWriteWins,
    /// Queue for manual resolution.
    Manual,
}

/// Conflict record for audit.
#[derive(Debug, Clone)]
pub struct ConflictRecord {
    pub id: EntityId,
    pub operation_id: EntityId,
    pub entity_type: String,
    pub entity_id: EntityId,
    pub local_version: String,
    pub server_version: String,
    pub resolution: ConflictStrategy,
    pub resolved_at: DateTime<Utc>,
}

/// Deterministic synchronization engine.
pub struct SyncEngine {
    /// Queue of pending operations (FIFO for determinism).
    queue: VecDeque<SyncOperation>,
    /// Conflict resolution strategy.
    conflict_strategy: ConflictStrategy,
    /// Maximum retry count before marking as failed.
    _max_retries: u32,
    /// Conflict history.
    conflicts: Vec<ConflictRecord>,
    /// Whether the device is currently online.
    is_online: bool,
    /// Total operations synced.
    total_synced: u64,
    /// Total conflicts encountered.
    total_conflicts: u64,
}

impl SyncEngine {
    pub fn new(conflict_strategy: ConflictStrategy) -> Self {
        Self {
            queue: VecDeque::new(),
            conflict_strategy,
            _max_retries: 3,
            conflicts: Vec::new(),
            is_online: false,
            total_synced: 0,
            total_conflicts: 0,
        }
    }

    /// Enqueue an operation for sync.
    pub fn enqueue(&mut self, op: SyncOperation) {
        debug!(
            entity = %op.entity_id,
            op_type = ?op.operation_type,
            "enqueuing sync operation"
        );
        self.queue.push_back(op);
    }

    /// Create and enqueue a new operation.
    pub fn queue_operation(
        &mut self,
        op_type: SyncOperationType,
        entity_type: String,
        entity_id: EntityId,
        payload: String,
    ) {
        let op = SyncOperation {
            id: EntityId::new(),
            operation_type: op_type,
            entity_type,
            entity_id,
            payload,
            created_at: Utc::now(),
            retry_count: 0,
            status: SyncStatus::Pending,
        };
        self.enqueue(op);
    }

    /// Process the next pending operation.
    /// Returns the operation if one was processed, None if queue is empty.
    pub fn process_next(&mut self) -> Option<SyncOperation> {
        if !self.is_online {
            debug!("offline — skipping sync");
            return None;
        }

        let mut op = self.queue.pop_front()?;
        op.status = SyncStatus::InProgress;

        // Simulate sync attempt.
        // In production, this would make an HTTP call to the server.
        op.status = SyncStatus::Completed;
        self.total_synced += 1;

        info!(
            entity = %op.entity_id,
            op_type = ?op.operation_type,
            "sync operation completed"
        );

        Some(op)
    }

    /// Process all pending operations.
    pub fn flush(&mut self) -> Vec<SyncOperation> {
        let mut results = Vec::new();
        while let Some(op) = self.process_next() {
            results.push(op);
        }
        results
    }

    /// Record a conflict.
    pub fn record_conflict(
        &mut self,
        operation_id: EntityId,
        entity_type: String,
        entity_id: EntityId,
        local_version: String,
        server_version: String,
    ) -> ConflictRecord {
        self.total_conflicts += 1;

        let record = ConflictRecord {
            id: EntityId::new(),
            operation_id,
            entity_type,
            entity_id,
            local_version,
            server_version,
            resolution: self.conflict_strategy,
            resolved_at: Utc::now(),
        };

        warn!(
            entity = %record.entity_id,
            strategy = ?self.conflict_strategy,
            "sync conflict resolved"
        );

        self.conflicts.push(record.clone());
        record
    }

    /// Set online status.
    pub fn set_online(&mut self, online: bool) {
        if online && !self.is_online {
            info!(
                pending = self.queue.len(),
                "device came online — ready to sync"
            );
        }
        self.is_online = online;
    }

    /// Whether the device is online.
    pub fn is_online(&self) -> bool {
        self.is_online
    }

    /// Number of pending operations.
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }

    /// Total operations synced.
    pub fn total_synced(&self) -> u64 {
        self.total_synced
    }

    /// Total conflicts encountered.
    pub fn total_conflicts(&self) -> u64 {
        self.total_conflicts
    }

    /// Get conflict history.
    pub fn conflicts(&self) -> &[ConflictRecord] {
        &self.conflicts
    }

    /// Get the conflict resolution strategy.
    pub fn conflict_strategy(&self) -> ConflictStrategy {
        self.conflict_strategy
    }

    /// Set the conflict resolution strategy.
    pub fn set_conflict_strategy(&mut self, strategy: ConflictStrategy) {
        self.conflict_strategy = strategy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_and_count() {
        let mut engine = SyncEngine::new(ConflictStrategy::LastWriteWins);
        assert_eq!(engine.pending_count(), 0);

        engine.queue_operation(
            SyncOperationType::Create,
            "MapTile".into(),
            EntityId::new(),
            "{}".into(),
        );
        assert_eq!(engine.pending_count(), 1);
    }

    #[test]
    fn offline_does_not_process() {
        let mut engine = SyncEngine::new(ConflictStrategy::LastWriteWins);
        engine.queue_operation(
            SyncOperationType::Create,
            "MapTile".into(),
            EntityId::new(),
            "{}".into(),
        );

        // Offline — should not process.
        let result = engine.process_next();
        assert!(result.is_none());
        assert_eq!(engine.pending_count(), 1);
    }

    #[test]
    fn online_processes_operations() {
        let mut engine = SyncEngine::new(ConflictStrategy::LastWriteWins);
        engine.set_online(true);

        engine.queue_operation(
            SyncOperationType::Create,
            "MapTile".into(),
            EntityId::new(),
            "{}".into(),
        );
        engine.queue_operation(
            SyncOperationType::Update,
            "MapTile".into(),
            EntityId::new(),
            "{}".into(),
        );

        let results = engine.flush();
        assert_eq!(results.len(), 2);
        assert_eq!(engine.pending_count(), 0);
        assert_eq!(engine.total_synced(), 2);
    }

    #[test]
    fn conflict_recording() {
        let mut engine = SyncEngine::new(ConflictStrategy::LocalWins);
        let record = engine.record_conflict(
            EntityId::new(),
            "RoadSegment".into(),
            EntityId::new(),
            "v1".into(),
            "v2".into(),
        );
        assert_eq!(record.resolution, ConflictStrategy::LocalWins);
        assert_eq!(engine.total_conflicts(), 1);
        assert_eq!(engine.conflicts().len(), 1);
    }

    #[test]
    fn fifo_ordering() {
        let mut engine = SyncEngine::new(ConflictStrategy::LastWriteWins);
        engine.set_online(true);

        let id1 = EntityId::new();
        let id2 = EntityId::new();
        engine.queue_operation(SyncOperationType::Create, "A".into(), id1, "{}".into());
        engine.queue_operation(SyncOperationType::Create, "B".into(), id2, "{}".into());

        let first = engine.process_next().unwrap();
        assert_eq!(first.entity_id, id1);
        let second = engine.process_next().unwrap();
        assert_eq!(second.entity_id, id2);
    }
}
