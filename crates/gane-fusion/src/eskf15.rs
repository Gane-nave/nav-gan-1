//! Canonical 15-state error-state Kalman filter (ESKF).
//!
//! Nominal state: position p (ENU, m), velocity v (ENU, m/s), attitude q
//! (unit quaternion body→ENU), accelerometer bias b_a (m/s²), gyroscope
//! bias b_g (rad/s). Error state (15):
//!
//! ```text
//! δx = [ δp(3) | δv(3) | δθ(3) | δb_a(3) | δb_g(3) ]
//! ```
//!
//! Propagation follows the strapdown/ESKF formulation of Solà,
//! "Quaternion kinematics for the error-state Kalman filter" (2017) —
//! the same reference the product's TypeScript prototype cites; this Rust
//! implementation supersedes both that prototype and the 9-state
//! `NavigationEkf` as the single canonical filter (Blueprint TD-3).

use nalgebra::{Matrix3, SMatrix, SVector, UnitQuaternion, Vector3};

/// Error-state dimension.
pub const ERR_DIM: usize = 15;

/// Gravity in the ENU frame (z up).
const GRAVITY_ENU: Vector3<f64> = Vector3::new(0.0, 0.0, -9.81);

type Mat15 = SMatrix<f64, ERR_DIM, ERR_DIM>;

/// Skew-symmetric matrix of `v` ([v]×).
fn skew(v: &Vector3<f64>) -> Matrix3<f64> {
    Matrix3::new(0.0, -v.z, v.y, v.z, 0.0, -v.x, -v.y, v.x, 0.0)
}

/// Tunable continuous-time noise densities.
#[derive(Debug, Clone)]
pub struct Eskf15Config {
    /// Accelerometer white noise (m/s²/√Hz).
    pub sigma_accel: f64,
    /// Gyroscope white noise (rad/s/√Hz).
    pub sigma_gyro: f64,
    /// Accelerometer bias random walk.
    pub sigma_accel_bias: f64,
    /// Gyroscope bias random walk.
    pub sigma_gyro_bias: f64,
}

impl Default for Eskf15Config {
    fn default() -> Self {
        Self {
            sigma_accel: 0.1,
            sigma_gyro: 0.01,
            sigma_accel_bias: 1e-4,
            sigma_gyro_bias: 1e-5,
        }
    }
}

/// 15-state error-state Kalman filter.
pub struct Eskf15 {
    // Nominal state.
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub attitude: UnitQuaternion<f64>,
    pub accel_bias: Vector3<f64>,
    pub gyro_bias: Vector3<f64>,
    /// Error-state covariance (15×15).
    pub p: Mat15,
    pub config: Eskf15Config,
}

impl Default for Eskf15 {
    fn default() -> Self {
        Self::new()
    }
}

impl Eskf15 {
    pub fn new() -> Self {
        let mut p = Mat15::zeros();
        for i in 0..3 {
            p[(i, i)] = 1e4; // position
            p[(3 + i, 3 + i)] = 100.0; // velocity
            p[(6 + i, 6 + i)] = 1.0; // attitude (rad²)
            p[(9 + i, 9 + i)] = 0.1; // accel bias
            p[(12 + i, 12 + i)] = 0.01; // gyro bias
        }
        Self {
            position: Vector3::zeros(),
            velocity: Vector3::zeros(),
            attitude: UnitQuaternion::identity(),
            accel_bias: Vector3::zeros(),
            gyro_bias: Vector3::zeros(),
            p,
            config: Eskf15Config::default(),
        }
    }

    /// Strapdown propagation with an IMU sample (body frame), Solà eqs. 260–262.
    pub fn predict(&mut self, accel_body: Vector3<f64>, gyro_body: Vector3<f64>, dt: f64) {
        if !(0.0..=1.0).contains(&dt) || dt == 0.0 {
            return; // reject non-positive or absurd sensor intervals
        }

        let a_corr = accel_body - self.accel_bias;
        let w_corr = gyro_body - self.gyro_bias;
        let r = self.attitude.to_rotation_matrix();

        // Nominal-state integration.
        let a_enu = r * a_corr + GRAVITY_ENU;
        self.position += self.velocity * dt + 0.5 * a_enu * dt * dt;
        self.velocity += a_enu * dt;
        let dq = UnitQuaternion::from_scaled_axis(w_corr * dt);
        self.attitude *= dq;
        self.attitude.renormalize();

        // Error-state transition F (discrete, first order).
        let mut f = Mat15::identity();
        let i3 = Matrix3::identity() * dt;
        f.fixed_view_mut::<3, 3>(0, 3).copy_from(&i3); // δp ← δv
        let r_m = r.matrix();
        f.fixed_view_mut::<3, 3>(3, 6)
            .copy_from(&(-r_m * skew(&a_corr) * dt)); // δv ← δθ
        f.fixed_view_mut::<3, 3>(3, 9).copy_from(&(-r_m * dt)); // δv ← δb_a
        f.fixed_view_mut::<3, 3>(6, 6)
            .copy_from(&(Matrix3::identity() - skew(&(w_corr * dt)))); // δθ ← δθ
        f.fixed_view_mut::<3, 3>(6, 12)
            .copy_from(&(-Matrix3::identity() * dt)); // δθ ← δb_g

        // Process noise (discrete approximation).
        let c = &self.config;
        let mut q = Mat15::zeros();
        for i in 0..3 {
            q[(3 + i, 3 + i)] = (c.sigma_accel * dt).powi(2);
            q[(6 + i, 6 + i)] = (c.sigma_gyro * dt).powi(2);
            q[(9 + i, 9 + i)] = (c.sigma_accel_bias * dt).powi(2) * dt;
            q[(12 + i, 12 + i)] = (c.sigma_gyro_bias * dt).powi(2) * dt;
        }

        self.p = f * self.p * f.transpose() + q;
        self.symmetrize();
    }

    /// Generic 3-D linear update with measurement Jacobian block at `offset`.
    ///
    /// Returns the normalized innovation squared (chi²-style gate value) so
    /// integrity layers can reject outliers ("no silent failure").
    fn update3(&mut self, offset: usize, innovation: Vector3<f64>, sigma: f64) -> f64 {
        // S = H P Hᵀ + R  (H selects a 3-block ⇒ S = P_block + R).
        let p_block: Matrix3<f64> = self.p.fixed_view::<3, 3>(offset, offset).into();
        let s = p_block + Matrix3::identity() * sigma * sigma;
        let Some(s_inv) = s.try_inverse() else {
            return f64::INFINITY;
        };
        let nis = (innovation.transpose() * s_inv * innovation)[(0, 0)];

        // K = P Hᵀ S⁻¹  (15×3), with H selecting the 3-block at `offset`.
        let ph_t = self.p.fixed_columns::<3>(offset).into_owned();
        let k = ph_t * s_inv;
        let dx: SVector<f64, ERR_DIM> = k * innovation;

        // (I − K H) P  ⇒  P −= K · (H P), where H P = the 3 rows at `offset`.
        let hp = self.p.fixed_rows::<3>(offset).into_owned();
        self.p -= k * hp;

        self.inject(&dx);
        self.symmetrize();
        nis
    }

    /// Inject the error estimate into the nominal state and reset it.
    fn inject(&mut self, dx: &SVector<f64, ERR_DIM>) {
        self.position += Vector3::new(dx[0], dx[1], dx[2]);
        self.velocity += Vector3::new(dx[3], dx[4], dx[5]);
        let dtheta = Vector3::new(dx[6], dx[7], dx[8]);
        self.attitude *= UnitQuaternion::from_scaled_axis(dtheta);
        self.attitude.renormalize();
        self.accel_bias += Vector3::new(dx[9], dx[10], dx[11]);
        self.gyro_bias += Vector3::new(dx[12], dx[13], dx[14]);
    }

    fn symmetrize(&mut self) {
        self.p = (self.p + self.p.transpose()) * 0.5;
    }

    /// GNSS position update (ENU metres, 1-σ). Returns the NIS gate value.
    pub fn update_position(&mut self, measured: Vector3<f64>, sigma_m: f64) -> f64 {
        let innovation = measured - self.position;
        self.update3(0, innovation, sigma_m)
    }

    /// GNSS velocity update (ENU m/s). Returns the NIS gate value.
    pub fn update_velocity(&mut self, measured: Vector3<f64>, sigma_mps: f64) -> f64 {
        let innovation = measured - self.velocity;
        self.update3(3, innovation, sigma_mps)
    }

    /// Zero-velocity update (ZUPT) — apply when the stationarity detector fires.
    pub fn update_zupt(&mut self, sigma_mps: f64) -> f64 {
        let innovation = -self.velocity;
        self.update3(3, innovation, sigma_mps)
    }

    /// Generic scalar update with measurement Jacobian `h` (15-vec).
    /// Returns the NIS gate value.
    fn update_scalar(&mut self, h: SVector<f64, ERR_DIM>, innovation: f64, sigma: f64) -> f64 {
        let ph = self.p * h;
        let s = (h.transpose() * ph)[(0, 0)] + sigma * sigma;
        if s <= 0.0 {
            return f64::INFINITY;
        }
        let nis = innovation * innovation / s;
        let k = ph / s;
        let dx: SVector<f64, ERR_DIM> = k * innovation;
        self.p -= k * (h.transpose() * self.p);
        self.inject(&dx);
        self.symmetrize();
        nis
    }

    /// Heading (yaw) update in radians, small roll/pitch approximation
    /// (δyaw ≈ δθ_z). Innovation is wrapped to (−π, π]. Returns NIS.
    pub fn update_heading(&mut self, yaw_meas_rad: f64, sigma_rad: f64) -> f64 {
        let mut innovation = yaw_meas_rad - self.heading_rad();
        while innovation > std::f64::consts::PI {
            innovation -= 2.0 * std::f64::consts::PI;
        }
        while innovation <= -std::f64::consts::PI {
            innovation += 2.0 * std::f64::consts::PI;
        }
        let mut h = SVector::<f64, ERR_DIM>::zeros();
        h[8] = 1.0; // δθ_z
        self.update_scalar(h, innovation, sigma_rad)
    }

    /// Scalar ground-speed update (‖v‖). Near standstill the direction is
    /// unobservable: a near-zero measured speed becomes a ZUPT, otherwise
    /// the sample is skipped. Returns NIS.
    pub fn update_speed(&mut self, speed_mps: f64, sigma_mps: f64) -> f64 {
        let v_norm = self.velocity.norm();
        if v_norm < 0.1 {
            return if speed_mps.abs() < 0.5 {
                self.update_zupt(sigma_mps.max(0.05))
            } else {
                f64::INFINITY // direction unknown — cannot apply
            };
        }
        let dir = self.velocity / v_norm;
        let mut h = SVector::<f64, ERR_DIM>::zeros();
        h[3] = dir.x;
        h[4] = dir.y;
        h[5] = dir.z;
        self.update_scalar(h, speed_mps - v_norm, sigma_mps)
    }

    /// Coast propagation when no IMU sample is available: assumes a
    /// non-accelerating, non-rotating vehicle (constant velocity) while the
    /// covariance still grows. Long gaps are integrated in ≤1 s chunks.
    pub fn predict_coast(&mut self, dt: f64) {
        if dt <= 0.0 {
            return;
        }
        // Synthetic specific force that exactly cancels gravity in ENU.
        let mut remaining = dt.min(10.0);
        while remaining > 0.0 {
            let step = remaining.min(1.0);
            let a_body = self.attitude.inverse() * Vector3::new(0.0, 0.0, 9.81) + self.accel_bias;
            let w_body = self.gyro_bias;
            self.predict(a_body, w_body, step);
            remaining -= step;
        }
    }

    /// Re-initialize state and covariance, keeping the noise configuration.
    pub fn reset(&mut self) {
        let config = self.config.clone();
        *self = Self {
            config,
            ..Self::new()
        };
    }

    /// Yaw (heading) in radians, ENU convention.
    pub fn heading_rad(&self) -> f64 {
        let (_, _, yaw) = self.attitude.euler_angles();
        yaw
    }

    /// Position in the local ENU frame (m).
    pub fn position_enu(&self) -> Vector3<f64> {
        self.position
    }

    /// Velocity in the local ENU frame (m/s).
    pub fn velocity_enu(&self) -> Vector3<f64> {
        self.velocity
    }

    /// 1-σ horizontal position uncertainty (m).
    pub fn horizontal_uncertainty_m(&self) -> f64 {
        (self.p[(0, 0)] + self.p[(1, 1)]).sqrt()
    }

    /// Alias kept for `FusionEngine` compatibility.
    pub fn position_uncertainty_m(&self) -> f64 {
        self.horizontal_uncertainty_m()
    }

    /// 1-σ vertical position uncertainty (m).
    pub fn vertical_uncertainty_m(&self) -> f64 {
        self.p[(2, 2)].max(0.0).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    /// A stationary IMU measures exactly the gravity reaction upward.
    fn stationary_imu() -> (Vector3<f64>, Vector3<f64>) {
        (Vector3::new(0.0, 0.0, 9.81), Vector3::zeros())
    }

    #[test]
    fn uncertainty_grows_without_updates() {
        let mut f = Eskf15::new();
        let before = f.horizontal_uncertainty_m();
        let (a, w) = stationary_imu();
        for _ in 0..100 {
            f.predict(a, w, 0.01);
        }
        assert!(f.horizontal_uncertainty_m() > before * 0.99);
        assert!(f.p[(3, 3)] > 100.0, "velocity variance must grow");
    }

    #[test]
    fn converges_to_position_fixes() {
        let mut f = Eskf15::new();
        let truth = Vector3::new(120.0, -40.0, 8.0);
        let (a, w) = stationary_imu();
        for _ in 0..50 {
            f.predict(a, w, 0.1);
            f.update_position(truth, 2.0);
            f.update_zupt(0.05);
        }
        assert_relative_eq!(f.position.x, truth.x, epsilon = 1.0);
        assert_relative_eq!(f.position.y, truth.y, epsilon = 1.0);
        assert!(
            f.horizontal_uncertainty_m() < 3.0,
            "must be confident after 50 fixes"
        );
    }

    #[test]
    fn zupt_pins_velocity() {
        let mut f = Eskf15::new();
        f.velocity = Vector3::new(3.0, -2.0, 1.0);
        for _ in 0..20 {
            f.update_zupt(0.02);
        }
        assert!(f.velocity.norm() < 0.05, "ZUPT must drive velocity to zero");
    }

    #[test]
    fn stationary_drift_is_bounded_with_aiding() {
        // Blueprint invariant: bounded drift under aiding.
        let mut f = Eskf15::new();
        let truth = Vector3::zeros();
        f.update_position(truth, 1.0);
        let (a, w) = stationary_imu();
        for i in 0..600 {
            f.predict(a, w, 0.1); // one minute of IMU
            if i % 10 == 0 {
                f.update_position(truth, 3.0); // 1 Hz GNSS
                f.update_zupt(0.05);
            }
        }
        assert!(
            f.position.norm() < 2.0,
            "aided stationary drift must stay bounded, got {}",
            f.position.norm()
        );
    }

    /// Classic INS observability: when stationary with only position/ZUPT
    /// aiding, horizontal (x/y) accel biases are indistinguishable from a
    /// small tilt — only the vertical bias is separately observable, because
    /// it conflicts directly with the known gravity magnitude. The filter
    /// must estimate a z-bias; expecting x-bias here would be physically
    /// wrong (it needs attitude aiding or maneuvers to become observable).
    #[test]
    fn vertical_accel_bias_observable_when_stationary() {
        let mut f = Eskf15::new();
        // The IMU over-reads specific force by +0.3 m/s² on z.
        let bias = Vector3::new(0.0, 0.0, 0.3);
        let (a0, w) = stationary_imu();
        for _ in 0..400 {
            f.predict(a0 + bias, w, 0.05);
            f.update_position(Vector3::zeros(), 1.0);
            f.update_zupt(0.02);
        }
        assert!(
            (f.accel_bias.z - 0.3).abs() < 0.05,
            "vertical accel bias must be estimated, got {}",
            f.accel_bias.z
        );
    }

    #[test]
    fn nis_gate_flags_outliers() {
        let mut f = Eskf15::new();
        for _ in 0..30 {
            f.update_position(Vector3::zeros(), 1.0);
            f.update_zupt(0.05);
        }
        let nominal = f.update_position(Vector3::new(0.5, 0.0, 0.0), 1.0);
        let outlier = f.update_position(Vector3::new(500.0, 0.0, 0.0), 1.0);
        assert!(
            outlier > nominal * 100.0,
            "outlier NIS ({outlier:.1}) must dwarf nominal ({nominal:.3})"
        );
    }

    #[test]
    fn attitude_stays_normalized() {
        let mut f = Eskf15::new();
        let (a, _) = stationary_imu();
        for _ in 0..1000 {
            f.predict(a, Vector3::new(0.1, -0.05, 0.2), 0.01);
        }
        assert_relative_eq!(f.attitude.norm(), 1.0, epsilon = 1e-9);
        assert!(f.heading_rad().is_finite());
    }
}
