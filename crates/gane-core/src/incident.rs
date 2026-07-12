//! Incident and evidence domain types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EntityId, GeoPosition};

// ---------------------------------------------------------------------------
// Incident
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: EntityId,
    pub reporter_id: EntityId,
    pub position: GeoPosition,
    pub incident_type: IncidentType,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub description: Option<String>,
    pub evidence_ids: Vec<EntityId>,
    pub trust_score: f64,
    pub validation_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentType {
    Accident,
    RoadClosure,
    Construction,
    Hazard,
    Weather,
    Police,
    Congestion,
    BrokenTrafficSignal,
    Pothole,
    Flooding,
    Debris,
    StoppedVehicle,
    WrongWayDriver,
    AnimalOnRoad,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentStatus {
    Reported,
    Validated,
    Active,
    Resolving,
    Closed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentTimeline {
    pub incident_id: EntityId,
    pub entries: Vec<TimelineEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub timestamp: DateTime<Utc>,
    pub event: String,
    pub actor_id: Option<EntityId>,
    pub details: Option<String>,
}

// ---------------------------------------------------------------------------
// Evidence
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: EntityId,
    pub incident_id: Option<EntityId>,
    pub reporter_id: EntityId,
    pub evidence_type: EvidenceType,
    pub position: GeoPosition,
    pub heading_deg: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub media_url: Option<String>,
    pub media_hash: Option<String>,
    pub signed_metadata: Option<String>,
    pub privacy_processed: bool,
    pub encrypted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    Photo,
    Video,
    VoiceNote,
    SensorTriggered,
    PreRollBuffer,
    TextReport,
    AutoCapture,
}
