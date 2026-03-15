//! Fleet operations domain types — tasks, assignments, dispatch, SLA.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EntityId, GeoPosition};

// ---------------------------------------------------------------------------
// Task
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: EntityId,
    pub title: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub location: Option<GeoPosition>,
    pub time_window: Option<TaskTimeWindow>,
    pub skills_required: Vec<String>,
    pub capacity_units: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    Delivery,
    Pickup,
    Service,
    Inspection,
    Emergency,
    Patrol,
    Transfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Urgent,
    Emergency,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TaskTimeWindow {
    pub earliest: DateTime<Utc>,
    pub latest: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Assignment
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: EntityId,
    pub task_id: EntityId,
    pub driver_id: EntityId,
    pub vehicle_id: EntityId,
    pub route_id: Option<EntityId>,
    pub status: AssignmentStatus,
    pub assigned_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub proof: Option<ProofRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignmentStatus {
    Assigned,
    Accepted,
    EnRoute,
    OnSite,
    Completed,
    Failed,
    Reassigned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRecord {
    pub proof_type: ProofType,
    pub timestamp: DateTime<Utc>,
    pub position: GeoPosition,
    pub evidence_id: Option<EntityId>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofType {
    ProofOfVisit,
    ProofOfDelivery,
    ProofOfPickup,
    ProofOfService,
    Signature,
    Photo,
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispatch {
    pub id: EntityId,
    pub assignments: Vec<EntityId>,
    pub dispatcher_id: Option<EntityId>,
    pub dispatch_type: DispatchType,
    pub status: DispatchStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchType {
    Automatic,
    Manual,
    Emergency,
    Satellite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchStatus {
    Created,
    Dispatched,
    InProgress,
    Completed,
    Cancelled,
}

// ---------------------------------------------------------------------------
// Service Level Agreement
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLevelAgreement {
    pub id: EntityId,
    pub name: String,
    pub max_response_time_s: f64,
    pub max_completion_time_s: f64,
    pub availability_target: f64,
    pub penalties: Vec<SlaPenalty>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaPenalty {
    pub condition: String,
    pub penalty_type: String,
    pub value: f64,
}
