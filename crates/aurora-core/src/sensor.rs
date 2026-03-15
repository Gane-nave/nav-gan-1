//! Sensor domain types — IMU, odometry, vehicle CAN signals.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::EntityId;

// ---------------------------------------------------------------------------
// IMU
// ---------------------------------------------------------------------------

/// Raw IMU sample (body frame).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImuSample {
    pub timestamp: DateTime<Utc>,
    /// Specific force (m/s²) in body X, Y, Z.
    pub accel_x: f64,
    pub accel_y: f64,
    pub accel_z: f64,
    /// Angular rate (rad/s) in body X, Y, Z.
    pub gyro_x: f64,
    pub gyro_y: f64,
    pub gyro_z: f64,
}

/// Magnetometer reading (µT, body frame).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MagnetometerSample {
    pub timestamp: DateTime<Utc>,
    pub mag_x: f64,
    pub mag_y: f64,
    pub mag_z: f64,
}

/// Barometric pressure sample.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BarometerSample {
    pub timestamp: DateTime<Utc>,
    /// Pressure in hPa (mbar).
    pub pressure_hpa: f64,
    /// Derived altitude in metres (if computed).
    pub altitude_m: Option<f64>,
    /// Temperature in °C (if available).
    pub temperature_c: Option<f64>,
}

// ---------------------------------------------------------------------------
// Odometry
// ---------------------------------------------------------------------------

/// Wheel odometry tick.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WheelOdometry {
    pub timestamp: DateTime<Utc>,
    /// Speed from wheel sensors (m/s).
    pub speed_mps: f64,
    /// Distance since last tick (m).
    pub delta_distance_m: f64,
    /// Wheel tick count (raw).
    pub tick_count: Option<u64>,
}

/// Camera-based visual odometry delta.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VisualOdometry {
    pub timestamp: DateTime<Utc>,
    pub delta_x_m: f64,
    pub delta_y_m: f64,
    pub delta_z_m: f64,
    pub delta_heading_rad: f64,
    pub confidence: f64,
    pub feature_count: u32,
}

// ---------------------------------------------------------------------------
// Vehicle CAN / OBD signals
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    pub timestamp: DateTime<Utc>,
    pub speed_kmh: Option<f64>,
    pub heading_deg: Option<f64>,
    pub steering_angle_deg: Option<f64>,
    pub brake_active: bool,
    pub abs_active: bool,
    pub esp_active: bool,
    pub wiper_active: bool,
    pub reverse_gear: bool,
    pub temperature_c: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleEvent {
    HardBrake,
    AbsActivation,
    EspActivation,
    SharpTurn,
    ReverseEngage,
    ReverseDisengage,
    WiperOn,
    WiperOff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleEventRecord {
    pub id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub event: VehicleEvent,
    pub details: Option<String>,
}

// ---------------------------------------------------------------------------
// Inertial navigation output (dead reckoning deltas)
// ---------------------------------------------------------------------------

/// Dead-reckoning delta produced by INS integration.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InertialDelta {
    pub timestamp: DateTime<Utc>,
    pub dt_s: f64,
    /// Position delta in local ENU (m).
    pub delta_east_m: f64,
    pub delta_north_m: f64,
    pub delta_up_m: f64,
    /// Heading delta (rad).
    pub delta_heading_rad: f64,
    /// Speed estimate (m/s).
    pub speed_mps: f64,
    /// Accumulated drift uncertainty (m).
    pub drift_uncertainty_m: f64,
}

// ---------------------------------------------------------------------------
// Sensor health
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorStatus {
    Active,
    Degraded,
    Failed,
    NotAvailable,
    Calibrating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorHealthReport {
    pub timestamp: DateTime<Utc>,
    pub imu_status: SensorStatus,
    pub magnetometer_status: SensorStatus,
    pub barometer_status: SensorStatus,
    pub wheel_odometry_status: SensorStatus,
    pub camera_status: SensorStatus,
    pub can_bus_status: SensorStatus,
}
