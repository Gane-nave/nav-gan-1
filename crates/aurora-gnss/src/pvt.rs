//! PVT (Position-Velocity-Time) solver.
//!
//! Computes least-squares PVT solutions per constellation and combined.

use aurora_core::gnss::{
    Constellation, PvtQuality, PvtSolution, SatelliteId, SatelliteMeasurement,
};
use aurora_core::types::EcefPosition;
use chrono::Utc;
use nalgebra::{DMatrix, DVector};
use thiserror::Error;
use tracing::debug;

/// Minimum satellites needed for a 3-D fix (position + clock).
const MIN_SATS_FOR_FIX: usize = 4;

/// Speed of light in m/s.
const C: f64 = 299_792_458.0;

#[derive(Debug, Error)]
pub enum PvtError {
    #[error("insufficient satellites: need {MIN_SATS_FOR_FIX}, have {0}")]
    InsufficientSatellites(usize),
    #[error("singular geometry matrix — cannot solve")]
    SingularGeometry,
    #[error("solution did not converge after {0} iterations")]
    DidNotConverge(u32),
    #[error("no satellite positions available")]
    NoSatellitePositions,
}

/// Least-squares PVT solver.
pub struct PvtSolver {
    max_iterations: u32,
    convergence_threshold_m: f64,
}

impl PvtSolver {
    pub fn new() -> Self {
        Self {
            max_iterations: 10,
            convergence_threshold_m: 1e-4,
        }
    }

    /// Solve PVT for a specific constellation.
    pub fn solve_single_constellation(
        &self,
        measurements: &[&SatelliteMeasurement],
        constellation: Constellation,
    ) -> Result<PvtSolution, PvtError> {
        let with_pos: Vec<&&SatelliteMeasurement> = measurements
            .iter()
            .filter(|m| m.satellite_position.is_some())
            .collect();

        if with_pos.len() < MIN_SATS_FOR_FIX {
            return Err(PvtError::InsufficientSatellites(with_pos.len()));
        }

        self.solve_iterative(&with_pos, &[constellation])
    }

    /// Solve combined PVT across all constellations with valid measurements.
    pub fn solve_combined(
        &self,
        measurements: &[&SatelliteMeasurement],
    ) -> Result<PvtSolution, PvtError> {
        let with_pos: Vec<&&SatelliteMeasurement> = measurements
            .iter()
            .filter(|m| m.satellite_position.is_some())
            .collect();

        if with_pos.len() < MIN_SATS_FOR_FIX {
            return Err(PvtError::InsufficientSatellites(with_pos.len()));
        }

        let constellations: Vec<Constellation> = with_pos
            .iter()
            .map(|m| m.satellite.constellation)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        self.solve_iterative(&with_pos, &constellations)
    }

    /// Iterative weighted least-squares PVT.
    fn solve_iterative(
        &self,
        measurements: &[&&SatelliteMeasurement],
        constellations: &[Constellation],
    ) -> Result<PvtSolution, PvtError> {
        let n = measurements.len();

        // State vector: [x, y, z, clock_bias]
        // Initial guess: centre of the Earth (will converge quickly).
        let mut state = DVector::from_element(4, 0.0);
        // Better initial guess: approximate from first satellite.
        if let Some(first) = measurements.first() {
            if let Some(pos) = &first.satellite_position {
                state[0] = pos.x_m * 0.98;
                state[1] = pos.y_m * 0.98;
                state[2] = pos.z_m * 0.98;
            }
        }

        let mut residuals;

        for iteration in 0..self.max_iterations {
            let mut h_rows = Vec::with_capacity(n);
            let mut dz = DVector::zeros(n);

            for (i, meas) in measurements.iter().enumerate() {
                let sat_pos = meas.satellite_position.as_ref().unwrap();
                let dx = sat_pos.x_m - state[0];
                let dy = sat_pos.y_m - state[1];
                let dz_val = sat_pos.z_m - state[2];
                let range = (dx * dx + dy * dy + dz_val * dz_val).sqrt();

                if range < 1.0 {
                    return Err(PvtError::SingularGeometry);
                }

                // Expected pseudorange.
                let expected = range + state[3];
                let observed = meas.pseudorange_m;

                dz[i] = observed - expected;

                // Geometry matrix row: [-dx/r, -dy/r, -dz/r, 1]
                h_rows.push(vec![-dx / range, -dy / range, -dz_val / range, 1.0]);
            }

            let h = DMatrix::from_fn(n, 4, |r, c| h_rows[r][c]);

            // Weighted least-squares: W = diag(CN0 / max_CN0)
            let max_cn0 = measurements
                .iter()
                .map(|m| m.cn0_dbhz)
                .fold(f64::NEG_INFINITY, f64::max);
            let w = DMatrix::from_diagonal(&DVector::from_fn(n, |i, _| {
                measurements[i].cn0_dbhz / max_cn0
            }));

            // Normal equation: (H^T W H)^-1 H^T W dz
            let ht = h.transpose();
            let htwh = &ht * &w * &h;

            let htwh_inv = match htwh.try_inverse() {
                Some(inv) => inv,
                None => return Err(PvtError::SingularGeometry),
            };

            let delta = &htwh_inv * &ht * &w * &dz;

            state += &delta;

            let correction_norm = delta.fixed_rows::<3>(0).norm();
            debug!(iteration, correction_m = correction_norm, "PVT iteration");

            if correction_norm < self.convergence_threshold_m {
                // Compute final residuals.
                residuals = Vec::with_capacity(n);
                for meas in measurements.iter() {
                    let sat_pos = meas.satellite_position.as_ref().unwrap();
                    let dx = sat_pos.x_m - state[0];
                    let dy = sat_pos.y_m - state[1];
                    let dz_val = sat_pos.z_m - state[2];
                    let range = (dx * dx + dy * dy + dz_val * dz_val).sqrt();
                    let expected = range + state[3];
                    residuals.push(meas.pseudorange_m - expected);
                }

                // Compute DOP from (H^T H)^-1
                let h_final = DMatrix::from_fn(n, 4, |r, c| h_rows[r][c]);
                let hth = h_final.transpose() * &h_final;
                let dop_matrix = hth.try_inverse().unwrap_or_else(|| DMatrix::identity(4, 4));

                let pdop = (dop_matrix[(0, 0)] + dop_matrix[(1, 1)] + dop_matrix[(2, 2)]).sqrt();
                let hdop = (dop_matrix[(0, 0)] + dop_matrix[(1, 1)]).sqrt();
                let vdop = dop_matrix[(2, 2)].sqrt();

                let sats_used: Vec<SatelliteId> =
                    measurements.iter().map(|m| m.satellite).collect();

                return Ok(PvtSolution {
                    timestamp: Utc::now(),
                    position: EcefPosition {
                        x_m: state[0],
                        y_m: state[1],
                        z_m: state[2],
                    },
                    velocity_ecef: [0.0, 0.0, 0.0], // Doppler-based velocity TBD
                    clock_bias_ns: state[3] / C * 1e9,
                    clock_drift_nps: 0.0,
                    satellites_used: sats_used,
                    constellations: constellations.to_vec(),
                    pdop,
                    hdop,
                    vdop,
                    residuals,
                    quality: PvtQuality::Autonomous,
                });
            }
        }

        Err(PvtError::DidNotConverge(self.max_iterations))
    }
}

impl Default for PvtSolver {
    fn default() -> Self {
        Self::new()
    }
}
