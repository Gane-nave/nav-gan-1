//! Synchronization, offline, and satellite communication domain types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::EntityId;

// ---------------------------------------------------------------------------
// Offline queue
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineQueue {
    pub id: EntityId,
    pub device_id: EntityId,
    pub entries: Vec<OfflineQueueEntry>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineQueueEntry {
    pub id: EntityId,
    pub event_data: serde_json::Value,
    pub queued_at: DateTime<Utc>,
    pub priority: u32,
    pub retry_count: u32,
    pub flushed: bool,
}

// ---------------------------------------------------------------------------
// Synchronization session
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationSession {
    pub id: EntityId,
    pub device_id: EntityId,
    pub status: SyncStatus,
    pub items_total: u64,
    pub items_synced: u64,
    pub conflicts: Vec<EntityId>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    InProgress,
    Completed,
    Failed,
    Cancelled,
    ConflictsDetected,
}

// ---------------------------------------------------------------------------
// Conflict resolution
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionRecord {
    pub id: EntityId,
    pub sync_session_id: EntityId,
    pub entity_type: String,
    pub entity_id: EntityId,
    pub local_version: serde_json::Value,
    pub remote_version: serde_json::Value,
    pub resolution: ConflictResolution,
    pub resolved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    AcceptLocal,
    AcceptRemote,
    Merged,
    Manual,
}

// ---------------------------------------------------------------------------
// Satellite packets
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatellitePacket {
    pub id: EntityId,
    pub packet_type: SatellitePacketType,
    pub payload: Vec<u8>,
    pub payload_size_bytes: u32,
    pub timestamp: DateTime<Utc>,
    pub sent: bool,
    pub acknowledged: bool,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SatellitePacketType {
    Sos,
    CheckIn,
    MinimalIncident,
    MinimalDispatch,
    ProofOfLocation,
    StoreAndForward,
    TimestampIntegrity,
}

// ---------------------------------------------------------------------------
// Mesh packets
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshPacket {
    pub id: EntityId,
    pub mesh_type: MeshTransport,
    pub payload: Vec<u8>,
    pub hop_count: u8,
    pub max_hops: u8,
    pub origin_device: EntityId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeshTransport {
    Bluetooth,
    WifiDirect,
}

// ---------------------------------------------------------------------------
// Replay session
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaySession {
    pub id: EntityId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub device_id: EntityId,
    pub description: Option<String>,
    pub status: ReplayStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayStatus {
    Recording,
    Completed,
    Playing,
    Archived,
}
