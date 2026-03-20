//! Dead reckoning engine — integrates IMU and odometry for position propagation.

use aurora_core::sensor::InertialDelta;
use aurora_core::types::GeoPosition;
use chrono::{DateTime, Utc};
use tracing::debug;

use crate::imu::CorrectedImu;
use crate::odometry::OdometryOutput;

/// Dead reckoning engine that propagates position using inertial + odometry.
pub struct DeadReckoningEngine {
    /// Current estimated position (lat/lon/alt).
    position: Option<GeoPosition>,
    /// Current heading in radians from north.
    heading_rad: f64,
    /// Current speed in m/s.
    speed_mps: f64,
    /// Accumulated drift uncertainty (m).
    drift_uncertainty_m: f64,
    /// Drift growth rate (m/s) — depends on sensor quality.
    drift_rate_mps: f64,
    /// Last update timestamp.
    last_update: Option<DateTime<Utc>>,
    /// Total dead reckoning time without GNSS update.
    dr_duration_s: f64,
}

impl DeadReckoningEngine {
    pub fn new() -> Self {
        Self {
            position: None,
            heading_rad: 0.0,
            speed_mps: 0.0,
            drift_uncertainty_m: 0.0,
            drift_rate_mps: 0.5, // Consumer IMU drift ~0.5 m/s
            last_update: None,
            dr_duration_s: 0.0,
        }
    }

    /// Set the initial position and heading from GNSS.
    pub fn initialize(
        &mut self,
        position: GeoPosition,
        heading_deg: f64,
        timestamp: DateTime<Utc>,
    ) {
        self.position = Some(position);
        self.heading_rad = heading_deg.to_radians();
        self.drift_uncertainty_m = 0.0;
        self.last_update = Some(timestamp);
        self.dr_duration_s = 0.0;
        debug!("dead reckoning initialized from GNSS");
    }

    /// Reset drift uncertainty when GNSS fix is restored.
    pub fn gnss_update(
        &mut self,
        position: GeoPosition,
        heading_deg: f64,
        timestamp: DateTime<Utc>,
    ) {
        self.position = Some(position);
        self.heading_rad = heading_deg.to_radians();
        self.drift_uncertainty_m = 0.0;
        self.last_update = Some(timestamp);
        self.dr_duration_s = 0.0;
    }

    /// Propagate position using corrected IMU data.
    pub fn propagate_imu(&mut self, imu: &CorrectedImu) -> Option<InertialDelta> {
        let last_ts = self.last_update?;
        let dt = (imu.timestamp - last_ts).num_milliseconds() as f64 / 1000.0;
        if dt <= 0.0 || dt > 1.0 {
            return None; // Skip unreasonable dt
        }

        // Update heading from gyro Z (yaw rate).
        let delta_heading = imu.angular_rate.z * dt;
        self.heading_rad += delta_heading;
        // Normalize heading to [0, 2π).
        self.heading_rad = self.heading_rad.rem_euclid(std::f64::consts::TAU);

        // Integrate acceleration for velocity update.
        // Simple trapezoidal integration in local frame.
        let accel_forward = imu.specific_force.x * self.heading_rad.cos()
            + imu.specific_force.y * self.heading_rad.sin();
        let accel_lateral = -imu.specific_force.x * self.heading_rad.sin()
            + imu.specific_force.y * self.heading_rad.cos();

        // Position deltas in ENU (East, North, Up).
        let delta_east =
            self.speed_mps * self.heading_rad.sin() * dt + 0.5 * accel_lateral * dt * dt;
        let delta_north =
            self.speed_mps * self.heading_rad.cos() * dt + 0.5 * accel_forward * dt * dt;
        let delta_up = imu.specific_force.z * dt * dt * 0.5;

        // Update speed.
        let speed_delta = (imu.specific_force.x.powi(2) + imu.specific_force.y.powi(2)).sqrt() * dt;
        self.speed_mps = (self.speed_mps + speed_delta).max(0.0);

        // Update position.
        if let Some(ref mut pos) = self.position {
            // Approximate lat/lon update from ENU deltas.
            let meters_per_deg_lat = 111_320.0;
            let meters_per_deg_lon = 111_320.0 * pos.latitude_deg.to_radians().cos();

            if meters_per_deg_lon > 0.1 {
                pos.latitude_deg += delta_north / meters_per_deg_lat;
                pos.longitude_deg += delta_east / meters_per_deg_lon;
            }
            if let Some(ref mut alt) = pos.altitude_m {
                *alt += delta_up;
            }
        }

        // Grow drift uncertainty.
        self.drift_uncertainty_m += self.drift_rate_mps * dt;
        self.dr_duration_s += dt;
        self.last_update = Some(imu.timestamp);

        Some(InertialDelta {
            timestamp: imu.timestamp,
            dt_s: dt,
            delta_east_m: delta_east,
            delta_north_m: delta_north,
            delta_up_m: delta_up,
            delta_heading_rad: delta_heading,
            speed_mps: self.speed_mps,
            drift_uncertainty_m: self.drift_uncertainty_m,
        })
    }

    /// Update speed from odometry (more accurate than IMU integration).
    pub fn update_from_odometry(&mut self, odo: &OdometryOutput) {
        self.speed_mps = odo.speed_mps;
    }

    pub fn current_position(&self) -> Option<&GeoPosition> {
        self.position.as_ref()
    }

    pub fn heading_deg(&self) -> f64 {
        self.heading_rad.to_degrees()
    }

    pub fn drift_uncertainty_m(&self) -> f64 {
        self.drift_uncertainty_m
    }

    pub fn dr_duration_s(&self) -> f64 {
        self.dr_duration_s
    }

    /// Set drift rate based on sensor quality.
    pub fn set_drift_rate(&mut self, rate_mps: f64) {
        self.drift_rate_mps = rate_mps;
    }
}

impl Default for DeadReckoningEngine {
    fn default() -> Self {
        Self::new()
    }
}
