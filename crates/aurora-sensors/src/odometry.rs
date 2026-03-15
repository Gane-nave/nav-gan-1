//! Odometry processing — wheel and visual odometry.

use aurora_core::sensor::{VisualOdometry, WheelOdometry};
use chrono::{DateTime, Utc};
use tracing::debug;

/// Processes wheel odometry ticks into continuous distance/speed estimates.
pub struct OdometryProcessor {
    /// Cumulative distance since start (m).
    total_distance_m: f64,
    /// Last speed reading (m/s).
    last_speed_mps: f64,
    /// Last update time.
    last_update: Option<DateTime<Utc>>,
    /// Wheel scale factor (for calibration).
    wheel_scale_factor: f64,
}

impl OdometryProcessor {
    pub fn new() -> Self {
        Self {
            total_distance_m: 0.0,
            last_speed_mps: 0.0,
            last_update: None,
            wheel_scale_factor: 1.0,
        }
    }

    /// Process a wheel odometry tick.
    pub fn process_wheel(&mut self, sample: &WheelOdometry) -> OdometryOutput {
        let delta = sample.delta_distance_m * self.wheel_scale_factor;
        self.total_distance_m += delta;
        self.last_speed_mps = sample.speed_mps * self.wheel_scale_factor;
        self.last_update = Some(sample.timestamp);

        debug!(
            delta_m = delta,
            speed_mps = self.last_speed_mps,
            total_m = self.total_distance_m,
            "wheel odometry processed"
        );

        OdometryOutput {
            timestamp: sample.timestamp,
            delta_distance_m: delta,
            speed_mps: self.last_speed_mps,
            total_distance_m: self.total_distance_m,
            source: OdometrySource::Wheel,
        }
    }

    /// Process a visual odometry delta.
    pub fn process_visual(&self, sample: &VisualOdometry) -> OdometryOutput {
        let delta = (sample.delta_x_m.powi(2)
            + sample.delta_y_m.powi(2)
            + sample.delta_z_m.powi(2))
        .sqrt();

        OdometryOutput {
            timestamp: sample.timestamp,
            delta_distance_m: delta,
            speed_mps: 0.0, // Would need dt for speed
            total_distance_m: self.total_distance_m + delta,
            source: OdometrySource::Visual,
        }
    }

    /// Set wheel scale factor for calibration.
    pub fn set_wheel_scale_factor(&mut self, factor: f64) {
        self.wheel_scale_factor = factor;
    }

    pub fn total_distance_m(&self) -> f64 {
        self.total_distance_m
    }

    pub fn last_speed_mps(&self) -> f64 {
        self.last_speed_mps
    }
}

impl Default for OdometryProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Processed odometry output.
#[derive(Debug, Clone)]
pub struct OdometryOutput {
    pub timestamp: DateTime<Utc>,
    pub delta_distance_m: f64,
    pub speed_mps: f64,
    pub total_distance_m: f64,
    pub source: OdometrySource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OdometrySource {
    Wheel,
    Visual,
}
