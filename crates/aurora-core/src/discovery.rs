//! Road and lane discovery domain types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EntityId, GeoPosition};

// ---------------------------------------------------------------------------
// Road discovery
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadDiscoveryCandidate {
    pub id: EntityId,
    pub trace_ids: Vec<EntityId>,
    pub geometry: Vec<GeoPosition>,
    pub confidence: f64,
    pub discovery_type: DiscoveryType,
    pub status: DiscoveryStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryType {
    NewRoad,
    GeometryChange,
    OneWayChange,
    SpeedLimitChange,
    RoadClosure,
    Construction,
    LaneCountChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryStatus {
    Detected,
    Clustering,
    Validated,
    Merged,
    Rejected,
}

// ---------------------------------------------------------------------------
// Lane discovery
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneDiscoveryCandidate {
    pub id: EntityId,
    pub segment_id: EntityId,
    pub inferred_lane_count: u8,
    pub inferred_one_way: Option<bool>,
    pub confidence: f64,
    pub trace_count: u32,
    pub status: DiscoveryStatus,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Topology revision
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyRevision {
    pub id: EntityId,
    pub region: String,
    pub version: u64,
    pub changes: Vec<TopologyChange>,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyChange {
    pub change_type: TopologyChangeType,
    pub entity_type: String,
    pub entity_id: EntityId,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyChangeType {
    Added,
    Modified,
    Removed,
}

// ---------------------------------------------------------------------------
// Map merge artifact
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMergeArtifact {
    pub id: EntityId,
    pub discovery_ids: Vec<EntityId>,
    pub revision_id: EntityId,
    pub merge_strategy: MergeStrategy,
    pub trust_weighted: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeStrategy {
    Automatic,
    ManualReview,
    TrustWeighted,
    EvidenceBased,
}
