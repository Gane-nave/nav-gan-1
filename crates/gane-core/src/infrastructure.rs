//! Infrastructure and smart-city domain types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EntityId, GeoPosition};

// ---------------------------------------------------------------------------
// City state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityState {
    pub id: EntityId,
    pub city_name: String,
    pub region: String,
    pub population: Option<u64>,
    pub timezone: String,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Infrastructure state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureState {
    pub id: EntityId,
    pub segment_id: EntityId,
    pub condition: InfrastructureCondition,
    pub last_inspection: Option<DateTime<Utc>>,
    pub issues: Vec<InfrastructureIssue>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfrastructureCondition {
    Good,
    Fair,
    Poor,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureIssue {
    pub issue_type: InfrastructureIssueType,
    pub position: GeoPosition,
    pub severity: f64,
    pub reported_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfrastructureIssueType {
    Pothole,
    LaneMarkingDegradation,
    SignalMalfunction,
    SurfaceCracking,
    Flooding,
    IcePatch,
    Debris,
    GuardrailDamage,
}

// ---------------------------------------------------------------------------
// Traffic signal state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSignalState {
    pub id: EntityId,
    pub position: GeoPosition,
    pub current_phase: TrafficPhase,
    pub time_to_change_s: Option<f64>,
    pub cycle_duration_s: f64,
    pub spat_available: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrafficPhase {
    Green,
    Yellow,
    Red,
    FlashingYellow,
    FlashingRed,
    Off,
    Unknown,
}

// ---------------------------------------------------------------------------
// Traffic flow model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficFlowModel {
    pub id: EntityId,
    pub segment_id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub speed_kmh: f64,
    pub free_flow_speed_kmh: f64,
    pub density_vehicles_per_km: f64,
    pub flow_vehicles_per_hour: f64,
    pub level_of_service: LevelOfService,
    pub occupancy: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LevelOfService {
    A,
    B,
    C,
    D,
    E,
    F,
}

// ---------------------------------------------------------------------------
// Infrastructure health record
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureHealthRecord {
    pub id: EntityId,
    pub region: String,
    pub overall_score: f64,
    pub road_condition_score: f64,
    pub signal_health_score: f64,
    pub marking_score: f64,
    pub computed_at: DateTime<Utc>,
}
