//! Energy and charging domain types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EntityId, GeoPosition};

// ---------------------------------------------------------------------------
// Energy profile
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProfile {
    pub id: EntityId,
    pub vehicle_id: EntityId,
    pub battery_capacity_kwh: Option<f64>,
    pub current_soc_pct: f64,
    pub avg_consumption_kwh_per_100km: f64,
    pub regen_efficiency: f64,
    pub climate_load_kw: f64,
    pub estimated_range_km: f64,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Charging station
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingStation {
    pub id: EntityId,
    pub position: GeoPosition,
    pub name: String,
    pub operator: Option<String>,
    pub connectors: Vec<Connector>,
    pub available_count: u32,
    pub total_count: u32,
    pub pricing: Option<String>,
    pub open_24h: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub connector_type: ConnectorType,
    pub max_power_kw: f64,
    pub available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorType {
    Type1,
    Type2,
    CCS1,
    CCS2,
    CHAdeMO,
    Tesla,
    GBT,
}

// ---------------------------------------------------------------------------
// Parking spot
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingSpot {
    pub id: EntityId,
    pub position: GeoPosition,
    pub spot_type: ParkingSpotType,
    pub available: bool,
    pub legal: bool,
    pub max_duration_min: Option<u32>,
    pub price_per_hour: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParkingSpotType {
    OnStreet,
    OffStreet,
    Garage,
    EvCharging,
    Handicapped,
    Loading,
    Residential,
}
