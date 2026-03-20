//! Extended Kalman Filter implementation.
//!
//! State vector (9-D):
//!   [east, north, up, ve, vn, vu, heading, accel_bias_x, accel_bias_y]
//!
//! This simplified EKF operates in a local ENU frame anchored at the
//! first GNSS fix. A production system would use ECEF + quaternion attitude.

use nalgebra::{DMatrix, DVector, Vector3};
use tracing::{debug, warn};

/// 9-dimensional state for the navigation EKF.
pub const STATE_DIM: usize = 9;

/// Extended Kalman Filter for navigation fusion.
pub struct NavigationEkf {
    /// State estimate [e, n, u, ve, vn, vu, heading, abx, aby].
    pub x: DVector<f64>,
    /// State covariance (9×9).
    pub p: DMatrix<f64>,
    /// Process noise spectral density.
    pub q_position: f64,
    pub q_velocity: f64,
    pub q_heading: f64,
    pub q_bias: f64,
}

impl NavigationEkf {
    pub fn new() -> Self {
        let x = DVector::zeros(STATE_DIM);
        let mut p = DMatrix::identity(STATE_DIM, STATE_DIM);
        // Large initial uncertainty.
        p[(0, 0)] = 1e6; // east
        p[(1, 1)] = 1e6; // north
        p[(2, 2)] = 1e6; // up
        p[(3, 3)] = 100.0; // ve
        p[(4, 4)] = 100.0; // vn
        p[(5, 5)] = 100.0; // vu
        p[(6, 6)] = (180.0_f64).to_radians().powi(2); // heading
        p[(7, 7)] = 1.0; // accel bias x
        p[(8, 8)] = 1.0; // accel bias y

        Self {
            x,
            p,
            q_position: 0.01,
            q_velocity: 0.1,
            q_heading: 0.001,
            q_bias: 1e-5,
        }
    }

    /// Time update (prediction step).
    ///
    /// Propagates the state forward by dt seconds using a constant-velocity model
    /// with heading-coupled dynamics.
    pub fn predict(&mut self, dt: f64) {
        if dt <= 0.0 || dt > 10.0 {
            warn!(dt, "unreasonable dt — skipping prediction");
            return;
        }

        let _heading = self.x[6];
        let _speed = (self.x[3].powi(2) + self.x[4].powi(2)).sqrt();

        // State transition: position += velocity * dt
        self.x[0] += self.x[3] * dt; // east
        self.x[1] += self.x[4] * dt; // north
        self.x[2] += self.x[5] * dt; // up

        // Build the state transition Jacobian F.
        let mut f = DMatrix::identity(STATE_DIM, STATE_DIM);
        f[(0, 3)] = dt; // de/dve
        f[(1, 4)] = dt; // dn/dvn
        f[(2, 5)] = dt; // du/dvu

        // Process noise Q.
        let mut q = DMatrix::zeros(STATE_DIM, STATE_DIM);
        let dt2 = dt * dt;
        let dt3 = dt2 * dt;
        // Position process noise (integrated velocity noise).
        q[(0, 0)] = self.q_position * dt3 / 3.0;
        q[(1, 1)] = self.q_position * dt3 / 3.0;
        q[(2, 2)] = self.q_position * dt3 / 3.0;
        // Velocity process noise.
        q[(3, 3)] = self.q_velocity * dt;
        q[(4, 4)] = self.q_velocity * dt;
        q[(5, 5)] = self.q_velocity * dt;
        // Heading process noise.
        q[(6, 6)] = self.q_heading * dt;
        // Bias process noise (slowly varying).
        q[(7, 7)] = self.q_bias * dt;
        q[(8, 8)] = self.q_bias * dt;

        // Covariance propagation: P = F * P * F^T + Q
        self.p = &f * &self.p * f.transpose() + q;

        debug!(dt, "EKF prediction step");
    }

    /// Measurement update for a 3-D position observation (ENU).
    ///
    /// z = [east, north, up], R = diag(σ²)
    pub fn update_position(&mut self, z_east: f64, z_north: f64, z_up: f64, sigma_m: f64) {
        let meas_dim = 3;
        let mut h = DMatrix::zeros(meas_dim, STATE_DIM);
        h[(0, 0)] = 1.0; // east
        h[(1, 1)] = 1.0; // north
        h[(2, 2)] = 1.0; // up

        let r_val = sigma_m * sigma_m;
        let mut r = DMatrix::zeros(meas_dim, meas_dim);
        r[(0, 0)] = r_val;
        r[(1, 1)] = r_val;
        r[(2, 2)] = r_val * 4.0; // Vertical typically less accurate

        let z = DVector::from_vec(vec![z_east, z_north, z_up]);
        let z_pred = &h * &self.x;
        let innovation = z - z_pred;

        self.apply_kalman_update(&h, &r, &innovation);
    }

    /// Measurement update for velocity (ENU).
    pub fn update_velocity(&mut self, ve: f64, vn: f64, vu: f64, sigma_mps: f64) {
        let meas_dim = 3;
        let mut h = DMatrix::zeros(meas_dim, STATE_DIM);
        h[(0, 3)] = 1.0;
        h[(1, 4)] = 1.0;
        h[(2, 5)] = 1.0;

        let r_val = sigma_mps * sigma_mps;
        let mut r = DMatrix::zeros(meas_dim, meas_dim);
        r[(0, 0)] = r_val;
        r[(1, 1)] = r_val;
        r[(2, 2)] = r_val;

        let z = DVector::from_vec(vec![ve, vn, vu]);
        let z_pred = &h * &self.x;
        let innovation = z - z_pred;

        self.apply_kalman_update(&h, &r, &innovation);
    }

    /// Measurement update for heading.
    pub fn update_heading(&mut self, heading_rad: f64, sigma_rad: f64) {
        let meas_dim = 1;
        let mut h = DMatrix::zeros(meas_dim, STATE_DIM);
        h[(0, 6)] = 1.0;

        let mut r = DMatrix::zeros(meas_dim, meas_dim);
        r[(0, 0)] = sigma_rad * sigma_rad;

        // Handle angle wrapping.
        let mut innovation = DVector::zeros(meas_dim);
        let mut diff = heading_rad - self.x[6];
        while diff > std::f64::consts::PI {
            diff -= std::f64::consts::TAU;
        }
        while diff < -std::f64::consts::PI {
            diff += std::f64::consts::TAU;
        }
        innovation[0] = diff;

        self.apply_kalman_update(&h, &r, &innovation);
    }

    /// Measurement update for speed-only (scalar).
    pub fn update_speed(&mut self, speed_mps: f64, sigma_mps: f64) {
        // Speed constrains the magnitude of horizontal velocity.
        let predicted_speed = (self.x[3].powi(2) + self.x[4].powi(2)).sqrt().max(1e-6);

        let meas_dim = 1;
        let mut h = DMatrix::zeros(meas_dim, STATE_DIM);
        h[(0, 3)] = self.x[3] / predicted_speed;
        h[(0, 4)] = self.x[4] / predicted_speed;

        let mut r = DMatrix::zeros(meas_dim, meas_dim);
        r[(0, 0)] = sigma_mps * sigma_mps;

        let mut innovation = DVector::zeros(meas_dim);
        innovation[0] = speed_mps - predicted_speed;

        self.apply_kalman_update(&h, &r, &innovation);
    }

    /// Apply the standard Kalman update equations.
    fn apply_kalman_update(
        &mut self,
        h: &DMatrix<f64>,
        r: &DMatrix<f64>,
        innovation: &DVector<f64>,
    ) {
        // S = H P H^T + R
        let s = h * &self.p * h.transpose() + r;

        // K = P H^T S^-1
        let s_inv = match s.clone().try_inverse() {
            Some(inv) => inv,
            None => {
                warn!("singular innovation covariance — skipping update");
                return;
            }
        };
        let k = &self.p * h.transpose() * s_inv;

        // State update: x = x + K * innovation
        self.x += &k * innovation;

        // Covariance update: P = (I - K H) P
        let i = DMatrix::identity(STATE_DIM, STATE_DIM);
        self.p = (&i - &k * h) * &self.p;

        // Enforce symmetry.
        self.p = (&self.p + self.p.transpose()) * 0.5;

        debug!(
            innovation_norm = innovation.norm(),
            "EKF measurement update"
        );
    }

    /// Get current position estimate in ENU.
    pub fn position_enu(&self) -> Vector3<f64> {
        Vector3::new(self.x[0], self.x[1], self.x[2])
    }

    /// Get current velocity estimate in ENU.
    pub fn velocity_enu(&self) -> Vector3<f64> {
        Vector3::new(self.x[3], self.x[4], self.x[5])
    }

    /// Get current heading estimate in radians.
    pub fn heading_rad(&self) -> f64 {
        self.x[6]
    }

    /// Get position uncertainty (horizontal, 1-σ) in metres.
    pub fn position_uncertainty_m(&self) -> f64 {
        (self.p[(0, 0)] + self.p[(1, 1)]).sqrt()
    }

    /// Get vertical uncertainty (1-σ) in metres.
    pub fn vertical_uncertainty_m(&self) -> f64 {
        self.p[(2, 2)].sqrt()
    }

    /// Reset filter to initial state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for NavigationEkf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prediction_grows_uncertainty() {
        let mut ekf = NavigationEkf::new();
        let p_before = ekf.position_uncertainty_m();
        ekf.predict(1.0);
        let p_after = ekf.position_uncertainty_m();
        assert!(p_after >= p_before);
    }

    #[test]
    fn position_update_reduces_uncertainty() {
        let mut ekf = NavigationEkf::new();
        ekf.predict(1.0);
        let p_before = ekf.position_uncertainty_m();
        ekf.update_position(100.0, 200.0, 50.0, 5.0);
        let p_after = ekf.position_uncertainty_m();
        assert!(p_after < p_before);
    }

    #[test]
    fn position_converges_to_measurement() {
        let mut ekf = NavigationEkf::new();
        for _ in 0..20 {
            ekf.predict(1.0);
            ekf.update_position(100.0, 200.0, 50.0, 2.0);
        }
        let pos = ekf.position_enu();
        assert!((pos.x - 100.0).abs() < 5.0);
        assert!((pos.y - 200.0).abs() < 5.0);
    }

    #[test]
    fn speed_update_constrains_velocity() {
        let mut ekf = NavigationEkf::new();
        ekf.x[3] = 10.0;
        ekf.x[4] = 0.0;
        ekf.update_speed(15.0, 0.5);
        let speed = (ekf.x[3].powi(2) + ekf.x[4].powi(2)).sqrt();
        assert!((speed - 15.0).abs() < 5.0);
    }
}
