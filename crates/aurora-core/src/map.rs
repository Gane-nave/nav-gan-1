//! Map domain types — tiles, road graph, lane graph, indoor, parking.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EntityId, GeoPosition};

// ---------------------------------------------------------------------------
// Map tiles
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapTile {
    pub id: EntityId,
    pub zoom_level: u8,
    pub tile_x: u32,
    pub tile_y: u32,
    pub version: u64,
    pub data_hash: String,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Road graph
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadGraph {
    pub id: EntityId,
    pub region: String,
    pub nodes: Vec<RoadNode>,
    pub segments: Vec<RoadSegment>,
    pub version: u64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadNode {
    pub id: EntityId,
    pub position: GeoPosition,
    pub node_type: RoadNodeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoadNodeType {
    Intersection,
    DeadEnd,
    Merge,
    Split,
    Roundabout,
    TrafficSignal,
    StopSign,
    YieldSign,
    PedestrianCrossing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadSegment {
    pub id: EntityId,
    pub from_node: EntityId,
    pub to_node: EntityId,
    pub geometry: Vec<GeoPosition>,
    pub road_class: RoadClass,
    pub one_way: bool,
    pub speed_limit_kmh: Option<f64>,
    pub lane_count: Option<u8>,
    pub surface_type: SurfaceType,
    pub bridge: bool,
    pub tunnel: bool,
    pub toll: bool,
    pub weight_limit_kg: Option<f64>,
    pub height_limit_m: Option<f64>,
    pub hazmat_restricted: bool,
    pub length_m: f64,
    pub travel_time_s: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoadClass {
    Motorway,
    Trunk,
    Primary,
    Secondary,
    Tertiary,
    Residential,
    Service,
    Unclassified,
    Pedestrian,
    Cycleway,
    Track,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceType {
    Asphalt,
    Concrete,
    Gravel,
    Dirt,
    Cobblestone,
    Paved,
    Unpaved,
    Unknown,
}

// ---------------------------------------------------------------------------
// Lane graph
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneGraph {
    pub id: EntityId,
    pub segment_id: EntityId,
    pub lanes: Vec<Lane>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lane {
    pub id: EntityId,
    pub index: u8,
    pub lane_type: LaneType,
    pub width_m: Option<f64>,
    pub geometry: Vec<GeoPosition>,
    pub allowed_turns: Vec<TurnDirection>,
    pub speed_limit_kmh: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneType {
    Driving,
    Bus,
    Bicycle,
    Parking,
    Shoulder,
    Turning,
    Merging,
    Exit,
    Hov,
    Reversible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnDirection {
    Straight,
    Left,
    Right,
    UTurn,
    SlightLeft,
    SlightRight,
    SharpLeft,
    SharpRight,
    MergeLeft,
    MergeRight,
}

// ---------------------------------------------------------------------------
// Turn restrictions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnRestriction {
    pub id: EntityId,
    pub from_segment: EntityId,
    pub via_node: EntityId,
    pub to_segment: EntityId,
    pub restriction_type: RestrictionType,
    pub time_condition: Option<String>,
    pub vehicle_condition: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestrictionType {
    NoLeftTurn,
    NoRightTurn,
    NoUTurn,
    NoStraightOn,
    OnlyLeftTurn,
    OnlyRightTurn,
    OnlyStraightOn,
}

// ---------------------------------------------------------------------------
// Indoor & parking graphs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndoorGraph {
    pub id: EntityId,
    pub building_id: EntityId,
    pub floor: i8,
    pub nodes: Vec<IndoorNode>,
    pub edges: Vec<IndoorEdge>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndoorNode {
    pub id: EntityId,
    pub position: GeoPosition,
    pub floor: i8,
    pub node_type: IndoorNodeType,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndoorNodeType {
    Corridor,
    Room,
    Entrance,
    Exit,
    Elevator,
    Stairs,
    Escalator,
    Gate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndoorEdge {
    pub from_node: EntityId,
    pub to_node: EntityId,
    pub distance_m: f64,
    pub accessible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingGraph {
    pub id: EntityId,
    pub facility_id: EntityId,
    pub levels: Vec<ParkingLevel>,
    pub entries: Vec<ParkingEntry>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingLevel {
    pub level: i8,
    pub total_spots: u32,
    pub available_spots: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingEntry {
    pub id: EntityId,
    pub position: GeoPosition,
    pub clearance_m: Option<f64>,
}

// ---------------------------------------------------------------------------
// Elevation model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevationSample {
    pub position: GeoPosition,
    pub elevation_m: f64,
    pub source: ElevationSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElevationSource {
    Srtm,
    Lidar,
    Survey,
    Interpolated,
}
