//! Communication domain types — channels, messages, voice, presence.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::EntityId;

// ---------------------------------------------------------------------------
// Groups & Channels
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: EntityId,
    pub name: String,
    pub group_type: GroupType,
    pub member_ids: Vec<EntityId>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupType {
    TripChannel,
    IncidentChannel,
    FleetChannel,
    ConvoyChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripChannel {
    pub id: EntityId,
    pub group_id: EntityId,
    pub route_id: EntityId,
    pub shared_eta: bool,
    pub location_sharing: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentChannel {
    pub id: EntityId,
    pub group_id: EntityId,
    pub incident_id: EntityId,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetChannel {
    pub id: EntityId,
    pub group_id: EntityId,
    pub fleet_id: EntityId,
    pub dispatch_enabled: bool,
    pub active: bool,
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: EntityId,
    pub channel_id: EntityId,
    pub sender_id: EntityId,
    pub message_type: MessageType,
    pub content: String,
    pub sent_at: DateTime<Utc>,
    pub read: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Location,
    Alert,
    Dispatch,
    Evidence,
    System,
}

// ---------------------------------------------------------------------------
// Voice
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSession {
    pub id: EntityId,
    pub channel_id: EntityId,
    pub participants: Vec<EntityId>,
    pub status: VoiceSessionStatus,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceSessionStatus {
    Active,
    OnHold,
    Ended,
}

// ---------------------------------------------------------------------------
// Presence
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceState {
    pub user_id: EntityId,
    pub online: bool,
    pub driving: bool,
    pub last_seen: DateTime<Utc>,
    pub current_channel: Option<EntityId>,
}
