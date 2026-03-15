//! High-level fusion engine orchestrating the EKF with measurement processing.

use aurora_core::types::{
    ContinuityMode, CovarianceMatrix, EnuVelocity, FusedPosition, GeoPosition, Heading,
    IntegrityLevel,
};
use chrono::{DateTime, Utc};
use tracing::info;

use crate::ekf::NavigationEkf;
use crate::measurement::{FusionMeasurement, MeasurementType};
use crate::state::FusionState;

/// Origin point for the ENU frame.
#[derive(Debug, Clone, Copy)]
struct EnuOrigin {
    lat_rad: f64,
    lon_rad: f64,
    alt_m: f64,
}

/// High-level fusion engine that wraps the EKF and manages
/// coordinate transforms, measurement routing, and state export.
pub struct FusionEngine {
    ekf: NavigationEkf,
    state: FusionState,
    origin: Option<EnuOrigin>,
    last_gnss_time: Option<DateTime<Utc>>,
    measurement_count: u64,
}

impl FusionEngine {
    pub fn new() -> Self {
        Self {
            ekf: NavigationEkf::new(),
            state: FusionState::new(),
            origin: None,
            last_gnss_time: None,
            measurement_count: 0,
        }
    }

    /// Process a measurement through the fusion filter.
    pub fn process_measurement(&mut self, meas: &FusionMeasurement) {
        // Predict to measurement time.
        if let Some(_last_ts) = self
            .state
            .timestamp
            .checked_add_signed(chrono::Duration::zero())
        {
            let dt = (meas.timestamp - self.state.timestamp).num_milliseconds() as f64 / 1000.0;
            if dt > 0.0 && dt < 10.0 {
                self.ekf.predict(dt);
            }
        }

        // Route measurement to the appropriate EKF update.
        match &meas.measurement_type {
            MeasurementType::GnssPosition {
                position,
                accuracy_m,
                vertical_accuracy_m: _,
            } => {
                self.process_gnss_position(
                    position,
                    *accuracy_m,
                    meas.trust_weight,
                    meas.timestamp,
                );
            }
            MeasurementType::GnssVelocity {
                east_mps,
                north_mps,
                up_mps,
                accuracy_mps,
            } => {
                let sigma = accuracy_mps / meas.trust_weight.max(0.01);
                self.ekf
                    .update_velocity(*east_mps, *north_mps, *up_mps, sigma);
            }
            MeasurementType::GnssHeading {
                heading_deg,
                accuracy_deg,
            } => {
                let sigma_rad = accuracy_deg.to_radians() / meas.trust_weight.max(0.01);
                self.ekf.update_heading(heading_deg.to_radians(), sigma_rad);
            }
            MeasurementType::InertialDelta {
                delta_east_m,
                delta_north_m,
                delta_up_m,
                delta_heading_rad,
                dt_s: _,
                uncertainty_m,
            } => {
                // For INS, we apply the delta as a relative position update.
                let current_pos = self.ekf.position_enu();
                let new_e = current_pos.x + delta_east_m;
                let new_n = current_pos.y + delta_north_m;
                let new_u = current_pos.z + delta_up_m;
                self.ekf
                    .update_position(new_e, new_n, new_u, *uncertainty_m);

                if delta_heading_rad.abs() > 1e-6 {
                    let new_heading = self.ekf.heading_rad() + delta_heading_rad;
                    self.ekf.update_heading(new_heading, 0.1); // ~6° uncertainty
                }
            }
            MeasurementType::OdometrySpeed {
                speed_mps,
                accuracy_mps,
            } => {
                self.ekf.update_speed(*speed_mps, *accuracy_mps);
            }
            MeasurementType::BarometricAltitude {
                altitude_m,
                accuracy_m,
            } => {
                // Update only the vertical component.
                let pos = self.ekf.position_enu();
                self.ekf
                    .update_position(pos.x, pos.y, *altitude_m, *accuracy_m);
            }
            MeasurementType::MapMatchPosition {
                position,
                accuracy_m,
                road_heading_deg,
            } => {
                self.process_gnss_position(
                    position,
                    *accuracy_m,
                    meas.trust_weight,
                    meas.timestamp,
                );
                self.ekf.update_heading(road_heading_deg.to_radians(), 0.17); // ~10°
            }
            MeasurementType::MagneticHeading {
                heading_deg,
                accuracy_deg,
            } => {
                self.ekf
                    .update_heading(heading_deg.to_radians(), accuracy_deg.to_radians());
            }
        }

        self.state.timestamp = meas.timestamp;
        self.measurement_count += 1;
        self.update_state_from_ekf();
    }

    /// Get the current fused navigation solution.
    pub fn current_solution(&self) -> FusedPosition {
        self.state.to_fused_position()
    }

    /// Get the internal fusion state.
    pub fn state(&self) -> &FusionState {
        &self.state
    }

    /// Set the continuity mode (called by the continuity manager).
    pub fn set_continuity_mode(&mut self, mode: ContinuityMode) {
        self.state.continuity_mode = mode;
    }

    /// Set the integrity level (called by the integrity engine).
    pub fn set_integrity(&mut self, level: IntegrityLevel) {
        self.state.integrity = level;
    }

    /// Total measurements processed.
    pub fn measurement_count(&self) -> u64 {
        self.measurement_count
    }

    /// Reset the filter.
    pub fn reset(&mut self) {
        self.ekf.reset();
        self.state = FusionState::new();
        self.origin = None;
        self.last_gnss_time = None;
        self.measurement_count = 0;
    }

    // -- internal --------------------------------------------------------

    fn process_gnss_position(
        &mut self,
        position: &GeoPosition,
        accuracy_m: f64,
        trust_weight: f64,
        timestamp: DateTime<Utc>,
    ) {
        // Establish ENU origin on first fix.
        if self.origin.is_none() {
            self.origin = Some(EnuOrigin {
                lat_rad: position.latitude_deg.to_radians(),
                lon_rad: position.longitude_deg.to_radians(),
                alt_m: position.altitude_m.unwrap_or(0.0),
            });
            info!(
                "ENU origin established at ({}, {})",
                position.latitude_deg, position.longitude_deg
            );
        }

        let origin = self.origin.unwrap();
        let (e, n, u) = geodetic_to_enu(position, &origin);

        // Scale measurement noise by inverse trust weight.
        let sigma = accuracy_m / trust_weight.max(0.01);
        self.ekf.update_position(e, n, u, sigma);
        self.last_gnss_time = Some(timestamp);

        if !self.state.initialised {
            self.state.initialise_from_gnss(
                *position,
                EnuVelocity {
                    east_mps: 0.0,
                    north_mps: 0.0,
                    up_mps: 0.0,
                },
                0.0,
                timestamp,
                accuracy_m,
            );
        }
    }

    fn update_state_from_ekf(&mut self) {
        if let Some(origin) = self.origin {
            let pos_enu = self.ekf.position_enu();
            let (lat, lon, alt) = enu_to_geodetic(pos_enu.x, pos_enu.y, pos_enu.z, &origin);

            self.state.position = GeoPosition {
                latitude_deg: lat,
                longitude_deg: lon,
                altitude_m: Some(alt),
            };
        }

        let vel_enu = self.ekf.velocity_enu();
        self.state.velocity = EnuVelocity {
            east_mps: vel_enu.x,
            north_mps: vel_enu.y,
            up_mps: vel_enu.z,
        };

        let heading_deg = self.ekf.heading_rad().to_degrees().rem_euclid(360.0);
        self.state.heading = Heading {
            true_heading_deg: heading_deg,
            magnetic_heading_deg: None,
            uncertainty_deg: self.ekf.p[(6, 6)].sqrt().to_degrees(),
        };

        self.state.confidence = compute_confidence(
            self.ekf.position_uncertainty_m(),
            self.state.continuity_mode,
        );

        // Update position covariance (extract 3×3 from full state).
        let mut entries = [0.0; 9];
        for i in 0..3 {
            for j in 0..3 {
                entries[i * 3 + j] = self.ekf.p[(i, j)];
            }
        }
        self.state.position_covariance = CovarianceMatrix { entries };
    }
}

impl Default for FusionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert geodetic (lat/lon/alt) to local ENU relative to an origin.
fn geodetic_to_enu(pos: &GeoPosition, origin: &EnuOrigin) -> (f64, f64, f64) {
    let lat = pos.latitude_deg.to_radians();
    let lon = pos.longitude_deg.to_radians();
    let alt = pos.altitude_m.unwrap_or(0.0);

    let dlat = lat - origin.lat_rad;
    let dlon = lon - origin.lon_rad;
    let dalt = alt - origin.alt_m;

    let r_earth = 6_371_000.0;
    let north = dlat * r_earth;
    let east = dlon * r_earth * origin.lat_rad.cos();
    let up = dalt;

    (east, north, up)
}

/// Convert local ENU back to geodetic.
fn enu_to_geodetic(east: f64, north: f64, up: f64, origin: &EnuOrigin) -> (f64, f64, f64) {
    let r_earth = 6_371_000.0;
    let lat = origin.lat_rad + north / r_earth;
    let lon = origin.lon_rad + east / (r_earth * origin.lat_rad.cos());
    let alt = origin.alt_m + up;

    (lat.to_degrees(), lon.to_degrees(), alt)
}

/// Compute overall confidence from uncertainty and mode.
fn compute_confidence(uncertainty_m: f64, mode: ContinuityMode) -> f64 {
    let mode_factor = match mode {
        ContinuityMode::ModeA => 1.0,
        ContinuityMode::ModeB => 0.9,
        ContinuityMode::ModeC => 0.7,
        ContinuityMode::ModeD => 0.4,
        ContinuityMode::ModeE => 0.2,
    };

    let accuracy_factor = if uncertainty_m < 2.0 {
        1.0
    } else if uncertainty_m < 10.0 {
        1.0 - (uncertainty_m - 2.0) / 16.0
    } else if uncertainty_m < 50.0 {
        0.5 - (uncertainty_m - 10.0) / 80.0
    } else {
        0.1
    };

    (mode_factor * accuracy_factor).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measurement::FusionMeasurement;
    use aurora_core::types::NavigationSource;

    #[test]
    fn engine_initialises_from_first_gnss_fix() {
        let mut engine = FusionEngine::new();
        assert!(!engine.state().initialised);

        let meas = FusionMeasurement::gnss_position(
            GeoPosition {
                latitude_deg: 32.0,
                longitude_deg: 34.8,
                altitude_m: Some(50.0),
            },
            5.0,
            10.0,
            NavigationSource::GpsL1,
            1.0,
            Utc::now(),
        );

        engine.process_measurement(&meas);
        assert!(engine.state().initialised);
        assert!(engine.state().confidence > 0.0);
    }

    #[test]
    fn multiple_gnss_fixes_improve_accuracy() {
        let mut engine = FusionEngine::new();
        let pos = GeoPosition {
            latitude_deg: 32.0,
            longitude_deg: 34.8,
            altitude_m: Some(50.0),
        };

        let mut ts = Utc::now();
        for i in 0..10 {
            ts = ts + chrono::Duration::seconds(1);
            let meas =
                FusionMeasurement::gnss_position(pos, 3.0, 6.0, NavigationSource::GpsL1, 1.0, ts);
            engine.process_measurement(&meas);
        }

        assert!(engine.state().horizontal_accuracy_m() < 100.0);
    }
}
