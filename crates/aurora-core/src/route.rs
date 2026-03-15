//! Route planning domain types — plans, stops, segments, corridors.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EntityId, GeoPosition, TransportMode};

// ---------------------------------------------------------------------------
// Route plan
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePlan {
    pub id: EntityId,
    pub user_id: EntityId,
    pub transport_mode: TransportMode,
    pub stops: Vec<Stop>,
    pub segments: Vec<RouteSegment>,
    pub total_distance_m: f64,
    pub total_duration_s: f64,
    pub eta: DateTime<Utc>,
    pub eta_confidence: f64,
    pub alternatives: Vec<EntityId>,
    pub optimization: OptimizationObjective,
    pub status: RouteStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stop {
    pub id: EntityId,
    pub position: GeoPosition,
    pub label: Option<String>,
    pub stop_type: StopType,
    pub time_window: Option<TimeWindow>,
    pub duration_s: Option<f64>,
    pub precedence: Option<u32>,
    pub arrived: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopType {
    Origin,
    Destination,
    Waypoint,
    ChargingStop,
    RestStop,
    PickupPoint,
    DropoffPoint,
    ParkingFacility,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeWindow {
    pub earliest: DateTime<Utc>,
    pub latest: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSegment {
    pub id: EntityId,
    pub from_stop: EntityId,
    pub to_stop: EntityId,
    pub geometry: Vec<GeoPosition>,
    pub distance_m: f64,
    pub duration_s: f64,
    pub road_class: Option<String>,
    pub speed_limit_kmh: Option<f64>,
    pub maneuvers: Vec<Maneuver>,
    pub risk_score: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maneuver {
    pub position: GeoPosition,
    pub maneuver_type: ManeuverType,
    pub instruction: String,
    pub distance_to_m: f64,
    pub street_name: Option<String>,
    pub lane_guidance: Option<LaneGuidance>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManeuverType {
    Depart,
    Arrive,
    TurnLeft,
    TurnRight,
    TurnSlightLeft,
    TurnSlightRight,
    TurnSharpLeft,
    TurnSharpRight,
    UTurn,
    Continue,
    MergeLeft,
    MergeRight,
    KeepLeft,
    KeepRight,
    Roundabout,
    ExitRoundabout,
    RampOn,
    RampOff,
    Fork,
    EnterTunnel,
    ExitTunnel,
    EnterParkingFacility,
    ExitParkingFacility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneGuidance {
    pub total_lanes: u8,
    pub recommended_lanes: Vec<u8>,
    pub arrows: Vec<String>,
}

// ---------------------------------------------------------------------------
// Corridor
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Corridor {
    pub id: EntityId,
    pub route_id: EntityId,
    pub width_m: f64,
    pub center_line: Vec<GeoPosition>,
    pub segments: Vec<EntityId>,
}

// ---------------------------------------------------------------------------
// Optimization
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationObjective {
    Fastest,
    Shortest,
    Safest,
    MostStable,
    LeastCognitiveLoad,
    MostEcoFriendly,
    LowestEmissions,
    LowestEnergy,
    Balanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteStatus {
    Planned,
    Active,
    Rerouting,
    Completed,
    Cancelled,
}

// ---------------------------------------------------------------------------
// ETA probability
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtaDistribution {
    pub route_id: EntityId,
    /// Expected (mean) ETA in seconds.
    pub expected_s: f64,
    /// Standard deviation.
    pub std_dev_s: f64,
    /// 5th percentile (optimistic).
    pub p5_s: f64,
    /// 50th percentile (median).
    pub p50_s: f64,
    /// 95th percentile (pessimistic).
    pub p95_s: f64,
    /// Probability of arriving within time window.
    pub on_time_probability: f64,
    /// Probability of delay > 5 min.
    pub delay_probability: f64,
    /// Probability of blockage on route.
    pub blockage_probability: f64,
    /// Confidence index [0, 1].
    pub confidence: f64,
    /// Volatility index [0, 1].
    pub volatility: f64,
}
