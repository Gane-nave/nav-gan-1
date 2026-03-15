//! Measurement models for the fusion filter.

use aurora_core::types::{GeoPosition, NavigationSource};
use chrono::{DateTime, Utc};

/// A measurement that can be fused into the navigation filter.
#[derive(Debug, Clone)]
pub struct FusionMeasurement {
    pub timestamp: DateTime<Utc>,
    pub source: NavigationSource,
    pub measurement_type: MeasurementType,
    pub trust_weight: f64,
}

/// Discriminated measurement types.
#[derive(Debug, Clone)]
pub enum MeasurementType {
    /// Absolute position from GNSS (lat, lon, alt with uncertainty).
    GnssPosition {
        position: GeoPosition,
        accuracy_m: f64,
        vertical_accuracy_m: f64,
    },
    /// Velocity from GNSS Doppler.
    GnssVelocity {
        east_mps: f64,
        north_mps: f64,
        up_mps: f64,
        accuracy_mps: f64,
    },
    /// Heading from GNSS course-over-ground.
    GnssHeading {
        heading_deg: f64,
        accuracy_deg: f64,
    },
    /// Inertial delta from IMU dead reckoning.
    InertialDelta {
        delta_east_m: f64,
        delta_north_m: f64,
        delta_up_m: f64,
        delta_heading_rad: f64,
        dt_s: f64,
        uncertainty_m: f64,
    },
    /// Speed from wheel odometry.
    OdometrySpeed {
        speed_mps: f64,
        accuracy_mps: f64,
    },
    /// Barometric altitude.
    BarometricAltitude {
        altitude_m: f64,
        accuracy_m: f64,
    },
    /// Map-matched position constraint.
    MapMatchPosition {
        position: GeoPosition,
        accuracy_m: f64,
        road_heading_deg: f64,
    },
    /// Heading from magnetometer.
    MagneticHeading {
        heading_deg: f64,
        accuracy_deg: f64,
    },
}

impl FusionMeasurement {
    pub fn gnss_position(
        position: GeoPosition,
        accuracy_m: f64,
        vertical_accuracy_m: f64,
        source: NavigationSource,
        trust: f64,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            timestamp,
            source,
            measurement_type: MeasurementType::GnssPosition {
                position,
                accuracy_m,
                vertical_accuracy_m,
            },
            trust_weight: trust,
        }
    }

    pub fn inertial_delta(
        delta_east_m: f64,
        delta_north_m: f64,
        delta_up_m: f64,
        delta_heading_rad: f64,
        dt_s: f64,
        uncertainty_m: f64,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            timestamp,
            source: NavigationSource::Imu,
            measurement_type: MeasurementType::InertialDelta {
                delta_east_m,
                delta_north_m,
                delta_up_m,
                delta_heading_rad,
                dt_s,
                uncertainty_m,
            },
            trust_weight: 1.0,
        }
    }

    pub fn odometry_speed(speed_mps: f64, accuracy_mps: f64, timestamp: DateTime<Utc>) -> Self {
        Self {
            timestamp,
            source: NavigationSource::WheelOdometry,
            measurement_type: MeasurementType::OdometrySpeed {
                speed_mps,
                accuracy_mps,
            },
            trust_weight: 1.0,
        }
    }
}
