//! Incremental correction — micro-adjustments that blend over time
//! instead of hard position resets.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalCorrector {
    current_lat: f64,
    current_lon: f64,
    target_lat: f64,
    target_lon: f64,
    blend_rate: f64,
    corrections: u64,
    max_step_m: f64,
}

impl IncrementalCorrector {
    pub fn new(blend_rate: f64) -> Self {
        Self {
            current_lat: 0.0,
            current_lon: 0.0,
            target_lat: 0.0,
            target_lon: 0.0,
            blend_rate: blend_rate.clamp(0.01, 1.0),
            corrections: 0,
            max_step_m: 5.0,
        }
    }

    pub fn set_position(&mut self, lat: f64, lon: f64) {
        self.current_lat = lat;
        self.current_lon = lon;
        self.target_lat = lat;
        self.target_lon = lon;
    }

    pub fn set_correction_target(&mut self, lat: f64, lon: f64) {
        self.target_lat = lat;
        self.target_lon = lon;
    }

    /// Apply one step of incremental blending toward the target.
    pub fn step(&mut self) -> (f64, f64) {
        let dlat = self.target_lat - self.current_lat;
        let dlon = self.target_lon - self.current_lon;
        let dist_m = (dlat * dlat + dlon * dlon).sqrt() * 111_111.0;
        let effective_rate = if dist_m > self.max_step_m {
            self.max_step_m / dist_m.max(1e-10)
        } else {
            self.blend_rate
        };
        self.current_lat += dlat * effective_rate;
        self.current_lon += dlon * effective_rate;
        self.corrections += 1;
        (self.current_lat, self.current_lon)
    }

    /// Check if correction has converged (within threshold).
    pub fn has_converged(&self, threshold_m: f64) -> bool {
        let dlat = (self.target_lat - self.current_lat) * 111_111.0;
        let dlon = (self.target_lon - self.current_lon) * 111_111.0;
        (dlat * dlat + dlon * dlon).sqrt() < threshold_m
    }

    pub fn corrections(&self) -> u64 {
        self.corrections
    }
    pub fn position(&self) -> (f64, f64) {
        (self.current_lat, self.current_lon)
    }
}

impl Default for IncrementalCorrector {
    fn default() -> Self {
        Self::new(0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = IncrementalCorrector::new(0.2);
        assert_eq!(c.corrections(), 0);
    }

    #[test]
    fn test_default() {
        let c = IncrementalCorrector::default();
        assert_eq!(c.blend_rate, 0.1);
    }

    #[test]
    fn test_set_position() {
        let mut c = IncrementalCorrector::default();
        c.set_position(32.0, 34.0);
        assert_eq!(c.position(), (32.0, 34.0));
    }

    #[test]
    fn test_converges() {
        let mut c = IncrementalCorrector::new(0.5);
        c.set_position(32.0, 34.0);
        c.set_correction_target(32.0001, 34.0001);
        for _ in 0..20 {
            c.step();
        }
        assert!(c.has_converged(1.0));
    }

    #[test]
    fn test_no_hard_reset() {
        let mut c = IncrementalCorrector::new(0.1);
        c.set_position(32.0, 34.0);
        c.set_correction_target(32.001, 34.001);
        let (lat, _) = c.step();
        assert!((lat - 32.0).abs() < 0.001); // didn't jump all the way
        assert!(lat > 32.0); // but did move toward target
    }

    #[test]
    fn test_large_correction_capped() {
        let mut c = IncrementalCorrector::new(0.5);
        c.set_position(32.0, 34.0);
        c.set_correction_target(33.0, 35.0); // huge correction
        let (lat, _) = c.step();
        let moved_m = (lat - 32.0).abs() * 111_111.0;
        assert!(moved_m <= 6.0); // capped at max_step_m
    }

    #[test]
    fn test_already_at_target() {
        let mut c = IncrementalCorrector::default();
        c.set_position(32.0, 34.0);
        c.set_correction_target(32.0, 34.0);
        assert!(c.has_converged(0.1));
    }

    #[test]
    fn test_corrections_count() {
        let mut c = IncrementalCorrector::default();
        c.set_position(32.0, 34.0);
        c.set_correction_target(32.001, 34.001);
        c.step();
        c.step();
        c.step();
        assert_eq!(c.corrections(), 3);
    }
}
