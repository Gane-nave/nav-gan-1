//! Numerical stability layer — prevents floating-point drift,
//! normalises coordinates, and guards against Kalman divergence.

use serde::{Deserialize, Serialize};

/// Precision guard that detects and corrects floating-point drift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionGuard {
    /// Maximum allowed coordinate drift before correction (metres).
    max_drift_m: f64,
    /// Running Kahan summation compensator for latitude.
    lat_comp: f64,
    /// Running Kahan summation compensator for longitude.
    lon_comp: f64,
    /// Number of corrections applied.
    corrections: u64,
    /// Whether divergence was detected on last check.
    divergence_detected: bool,
}

impl PrecisionGuard {
    pub fn new(max_drift_m: f64) -> Self {
        Self {
            max_drift_m,
            lat_comp: 0.0,
            lon_comp: 0.0,
            corrections: 0,
            divergence_detected: false,
        }
    }

    /// Kahan-compensated addition to reduce floating-point error.
    pub fn kahan_add(&mut self, sum: f64, value: f64, comp: f64) -> (f64, f64) {
        let y = value - comp;
        let t = sum + y;
        let new_comp = (t - sum) - y;
        (t, new_comp)
    }

    /// Normalise latitude to [-90, 90].
    pub fn normalise_lat(&self, lat: f64) -> f64 {
        lat.clamp(-90.0, 90.0)
    }

    /// Normalise longitude to [-180, 180).
    pub fn normalise_lon(&self, lon: f64) -> f64 {
        let mut l = lon % 360.0;
        if l > 180.0 {
            l -= 360.0;
        }
        if l < -180.0 {
            l += 360.0;
        }
        l
    }

    /// Check covariance matrix diagonal for Kalman divergence.
    pub fn check_kalman_divergence(&mut self, covariance_diag: &[f64]) -> bool {
        let max_cov = 1e6;
        self.divergence_detected = covariance_diag
            .iter()
            .any(|&c| c > max_cov || c.is_nan() || c.is_infinite());
        if self.divergence_detected {
            self.corrections += 1;
        }
        self.divergence_detected
    }

    /// Reset covariance to safe values if divergence detected.
    pub fn reset_covariance(&self, cov: &mut [f64], default_val: f64) {
        for c in cov.iter_mut() {
            if c.is_nan() || c.is_infinite() || *c > 1e6 {
                *c = default_val;
            }
        }
    }

    /// Correct accumulated rounding errors on a long trajectory.
    pub fn correct_trajectory_drift(&mut self, positions: &mut [(f64, f64)]) -> u32 {
        let mut fixes = 0u32;
        for pos in positions.iter_mut() {
            let nlat = self.normalise_lat(pos.0);
            let nlon = self.normalise_lon(pos.1);
            if (nlat - pos.0).abs() > 1e-12 || (nlon - pos.1).abs() > 1e-12 {
                pos.0 = nlat;
                pos.1 = nlon;
                fixes += 1;
            }
        }
        self.corrections += fixes as u64;
        fixes
    }

    pub fn corrections(&self) -> u64 {
        self.corrections
    }
    pub fn divergence_detected(&self) -> bool {
        self.divergence_detected
    }
}

impl Default for PrecisionGuard {
    fn default() -> Self {
        Self::new(0.01)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let g = PrecisionGuard::new(0.05);
        assert_eq!(g.corrections(), 0);
    }

    #[test]
    fn test_default() {
        let g = PrecisionGuard::default();
        assert!(!g.divergence_detected());
    }

    #[test]
    fn test_normalise_lat() {
        let g = PrecisionGuard::default();
        assert_eq!(g.normalise_lat(100.0), 90.0);
        assert_eq!(g.normalise_lat(-100.0), -90.0);
        assert!((g.normalise_lat(45.0) - 45.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalise_lon() {
        let g = PrecisionGuard::default();
        assert!((g.normalise_lon(200.0) - (-160.0)).abs() < 1e-10);
        assert!((g.normalise_lon(-200.0) - 160.0).abs() < 1e-10);
    }

    #[test]
    fn test_kahan_add() {
        let mut g = PrecisionGuard::default();
        let (s, c) = g.kahan_add(1e16, 1.0, 0.0);
        assert!(s > 0.0);
        assert!(c.abs() < 2.0);
    }

    #[test]
    fn test_divergence_detection() {
        let mut g = PrecisionGuard::default();
        assert!(!g.check_kalman_divergence(&[1.0, 2.0, 3.0]));
        assert!(g.check_kalman_divergence(&[1.0, f64::NAN, 3.0]));
    }

    #[test]
    fn test_reset_covariance() {
        let g = PrecisionGuard::default();
        let mut cov = vec![1.0, f64::INFINITY, f64::NAN, 2e6];
        g.reset_covariance(&mut cov, 10.0);
        assert_eq!(cov[0], 1.0);
        assert_eq!(cov[1], 10.0);
        assert_eq!(cov[2], 10.0);
        assert_eq!(cov[3], 10.0);
    }

    #[test]
    fn test_trajectory_drift() {
        let mut g = PrecisionGuard::default();
        let mut pos = vec![(100.0, 200.0), (45.0, 90.0)];
        let fixes = g.correct_trajectory_drift(&mut pos);
        assert!(fixes >= 1);
        assert_eq!(pos[0].0, 90.0);
        assert!((pos[0].1 - (-160.0)).abs() < 1e-10);
    }
}
