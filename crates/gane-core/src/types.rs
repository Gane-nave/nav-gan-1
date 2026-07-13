//! Core primitive types and common value objects used across G.A.N.E NAV.

use chrono::{DateTime, Utc};
use nalgebra::Matrix3;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Geographic primitives
// ---------------------------------------------------------------------------

/// WGS-84 geodetic position with optional altitude and uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GeoPosition {
    /// Latitude in degrees (WGS-84).
    pub latitude_deg: f64,
    /// Longitude in degrees (WGS-84).
    pub longitude_deg: f64,
    /// Altitude above WGS-84 ellipsoid in metres.
    pub altitude_m: Option<f64>,
}

/// Earth-Centred Earth-Fixed (ECEF) Cartesian position in metres.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EcefPosition {
    pub x_m: f64,
    pub y_m: f64,
    pub z_m: f64,
}

/// Velocity in a local East-North-Up frame (m/s).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EnuVelocity {
    pub east_mps: f64,
    pub north_mps: f64,
    pub up_mps: f64,
}

/// 3-D uncertainty ellipse (semi-axes in metres) with orientation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UncertaintyEllipse {
    pub semi_major_m: f64,
    pub semi_minor_m: f64,
    pub semi_vertical_m: f64,
    /// Orientation of the semi-major axis clockwise from north (degrees).
    pub orientation_deg: f64,
}

/// Full 3×3 covariance matrix for position (ENU frame, m²).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CovarianceMatrix {
    /// Row-major 3×3 entries.
    pub entries: [f64; 9],
}

impl CovarianceMatrix {
    pub fn from_nalgebra(m: &Matrix3<f64>) -> Self {
        let mut entries = [0.0; 9];
        for (i, val) in m.iter().enumerate() {
            entries[i] = *val;
        }
        Self { entries }
    }

    pub fn to_nalgebra(&self) -> Matrix3<f64> {
        Matrix3::from_row_slice(&self.entries)
    }

    pub fn identity_scaled(scale: f64) -> Self {
        Self::from_nalgebra(&(Matrix3::identity() * scale))
    }
}

/// Heading / bearing.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Heading {
    /// True heading in degrees [0, 360).
    pub true_heading_deg: f64,
    /// Magnetic heading in degrees (if available).
    pub magnetic_heading_deg: Option<f64>,
    /// 1-σ uncertainty in degrees.
    pub uncertainty_deg: f64,
}

// ---------------------------------------------------------------------------
// Timestamped wrapper
// ---------------------------------------------------------------------------

/// Generic timestamped value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timestamped<T> {
    pub timestamp: DateTime<Utc>,
    pub value: T,
}

impl<T> Timestamped<T> {
    pub fn now(value: T) -> Self {
        Self {
            timestamp: Utc::now(),
            value,
        }
    }

    pub fn at(timestamp: DateTime<Utc>, value: T) -> Self {
        Self { timestamp, value }
    }
}

// ---------------------------------------------------------------------------
// Entity identifiers
// ---------------------------------------------------------------------------

/// Strongly-typed entity identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// User & Device
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: EntityId,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub mobility_identity: Option<MobilityIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobilityIdentity {
    pub id: EntityId,
    pub user_id: EntityId,
    pub vehicle_profiles: Vec<EntityId>,
    pub preferred_mode: TransportMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: EntityId,
    pub user_id: EntityId,
    pub device_type: DeviceType,
    pub gnss_capabilities: GnssCapabilities,
    pub sensor_capabilities: SensorCapabilities,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Smartphone,
    Tablet,
    VehicleHeadUnit,
    OBDDongle,
    DedicatedNavigator,
    EdgeGateway,
    FleetTracker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnssCapabilities {
    pub gps: bool,
    pub galileo: bool,
    pub glonass: bool,
    pub beidou: bool,
    pub dual_band: bool,
    pub raw_measurements: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorCapabilities {
    pub imu: bool,
    pub gyroscope: bool,
    pub accelerometer: bool,
    pub magnetometer: bool,
    pub barometer: bool,
    pub camera: bool,
    pub lidar: bool,
    pub radar: bool,
    pub wheel_odometry: bool,
    pub can_bus: bool,
}

// ---------------------------------------------------------------------------
// Vehicle & Driver
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleProfile {
    pub id: EntityId,
    pub owner_id: EntityId,
    pub vehicle_type: VehicleType,
    pub dimensions: VehicleDimensions,
    pub energy_type: EnergyType,
    pub weight_kg: Option<f64>,
    pub max_speed_kmh: Option<f64>,
    pub hazmat_class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleType {
    PrivateCar,
    CommercialVehicle,
    Truck,
    EmergencyVehicle,
    Bus,
    Motorcycle,
    ElectricVehicle,
    MicroMobility,
    Bicycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VehicleDimensions {
    pub length_m: f64,
    pub width_m: f64,
    pub height_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyType {
    Gasoline,
    Diesel,
    Electric,
    Hybrid,
    Hydrogen,
    CNG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportMode {
    PrivateCar,
    CommercialVehicle,
    Truck,
    EmergencyVehicle,
    Pedestrian,
    Bicycle,
    MicroMobility,
    ElectricVehicle,
    PublicTransit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverState {
    pub id: EntityId,
    pub user_id: EntityId,
    pub fatigue_level: f64,
    pub cognitive_load: f64,
    pub driving_style: DrivingStyle,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrivingStyle {
    Calm,
    Normal,
    Aggressive,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverBehaviorProfile {
    pub id: EntityId,
    pub user_id: EntityId,
    pub avg_reaction_time_s: f64,
    pub hard_brake_frequency: f64,
    pub sharp_turn_frequency: f64,
    pub preferred_speed_factor: f64,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// UI / Mode State
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsageMode {
    Explore,
    Search,
    RoutePlanning,
    RouteComparison,
    ActiveNavigation,
    DriveSafe,
    Professional,
    Offline,
    SatelliteMinimal,
    FleetOperations,
    EmergencyResponse,
    EV,
    Truck,
    Pedestrian,
    Bicycle,
    MicroMobility,
    GnssFullPrecision,
    GnssDegraded,
    InsContinuity,
    EmergencyBoundedLocalization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIState {
    pub id: EntityId,
    pub user_id: EntityId,
    pub active_mode: UsageMode,
    pub active_layers: Vec<String>,
    pub day_night: DayNightMode,
    pub minimal_mode: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayNightMode {
    Day,
    Night,
    Auto,
}

// ---------------------------------------------------------------------------
// Navigation position output
// ---------------------------------------------------------------------------

/// The fused navigation solution — the primary output of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusedPosition {
    pub timestamp: DateTime<Utc>,
    pub position: GeoPosition,
    pub ecef: Option<EcefPosition>,
    pub velocity: EnuVelocity,
    pub heading: Heading,
    pub uncertainty: UncertaintyEllipse,
    pub covariance: CovarianceMatrix,
    pub confidence: f64,
    pub integrity_state: IntegrityLevel,
    pub continuity_mode: ContinuityMode,
    pub source_contributions: Vec<SourceContribution>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrityLevel {
    /// All checks pass; full trust.
    Nominal,
    /// Minor anomaly detected; solution still usable.
    Caution,
    /// Significant integrity concern; solution may be unreliable.
    Warning,
    /// Integrity compromised; fallback active.
    Alert,
    /// No valid solution available.
    NoSolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinuityMode {
    /// Full GNSS + Dual Frequency + Corrections.
    ModeA,
    /// Multi-GNSS without corrections.
    ModeB,
    /// GNSS degraded + INS fused.
    ModeC,
    /// INS + Odometry + Map Matching (dead reckoning).
    ModeD,
    /// Emergency bounded localization.
    ModeE,
}

impl std::fmt::Display for ContinuityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModeA => write!(f, "A: Full GNSS+Corrections"),
            Self::ModeB => write!(f, "B: Multi-GNSS"),
            Self::ModeC => write!(f, "C: GNSS Degraded+INS"),
            Self::ModeD => write!(f, "D: Dead Reckoning"),
            Self::ModeE => write!(f, "E: Emergency Bounded"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceContribution {
    pub source: NavigationSource,
    pub weight: f64,
    pub trust_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavigationSource {
    GpsL1,
    GpsL5,
    GalileoE1,
    GalileoE5a,
    GlonassL1,
    GlonassL2,
    BeidouB1,
    BeidouB2a,
    Sbas,
    Ppp,
    Rtk,
    Nrtk,
    Imu,
    WheelOdometry,
    CameraOdometry,
    Barometer,
    MapMatching,
    VisualSlam,
    Lidar,
    Radar,
}

// ---------------------------------------------------------------------------
// Audit
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub actor_id: EntityId,
    pub action: String,
    pub entity_type: String,
    pub entity_id: EntityId,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyLog {
    pub id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub algorithm: String,
    pub version: String,
    pub input_hash: String,
    pub output_hash: String,
    pub parameters: serde_json::Value,
}
