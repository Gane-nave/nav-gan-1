//! Latency compensation — corrects timing gaps between measurement
//! and display, extrapolates position forward.

use serde::{Deserialize, Serialize};

/// Compensates for sensor-to-display latency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyCompensator {
    /// Last known position (lat, lon).
    last_pos: (f64, f64),
    /// Last known velocity (m/s north, m/s east).
    last_vel: (f64, f64),
    /// Last known heading (radians).
    last_heading_rad: f64,
    /// Timestamp of last measurement (ms).
    last_ts_ms: u64,
    /// Estimated pipeline latency (ms).
    pipeline_latency_ms: u64,
    /// Extrapolation count.
    extrapolations: u64,
}

impl LatencyCompensator {
    pub fn new(pipeline_latency_ms: u64) -> Self {
        Self {
            last_pos: (0.0, 0.0),
            last_vel: (0.0, 0.0),
            last_heading_rad: 0.0,
            last_ts_ms: 0,
            pipeline_latency_ms,
            extrapolations: 0,
        }
    }

    /// Update with a new measurement.
    pub fn update(
        &mut self,
        lat: f64,
        lon: f64,
        vel_n: f64,
        vel_e: f64,
        heading_rad: f64,
        ts_ms: u64,
    ) {
        self.last_pos = (lat, lon);
        self.last_vel = (vel_n, vel_e);
        self.last_heading_rad = heading_rad;
        self.last_ts_ms = ts_ms;
    }

    /// Extrapolate position to compensate for latency.
    pub fn compensated_position(&mut self, display_ts_ms: u64) -> (f64, f64, f64) {
        let dt_s = (display_ts_ms.saturating_sub(self.last_ts_ms) + self.pipeline_latency_ms)
            as f64
            / 1000.0;
        let dt_s = dt_s.min(3.0); // cap at 3 seconds
        let dlat = self.last_vel.0 * dt_s / 111_111.0;
        let dlon =
            self.last_vel.1 * dt_s / (111_111.0 * self.last_pos.0.to_radians().cos().max(0.01));
        self.extrapolations += 1;
        (
            self.last_pos.0 + dlat,
            self.last_pos.1 + dlon,
            self.last_heading_rad,
        )
    }

    /// Blend GNSS (slow, accurate) with IMU (fast, drifty).
    pub fn blend_gnss_imu(
        &self,
        gnss_lat: f64,
        gnss_lon: f64,
        gnss_age_ms: u64,
        imu_lat: f64,
        imu_lon: f64,
    ) -> (f64, f64) {
        let gnss_weight = 1.0 / (1.0 + gnss_age_ms as f64 / 500.0);
        let imu_weight = 1.0 - gnss_weight;
        (
            gnss_lat * gnss_weight + imu_lat * imu_weight,
            gnss_lon * gnss_weight + imu_lon * imu_weight,
        )
    }

    pub fn extrapolations(&self) -> u64 {
        self.extrapolations
    }
    pub fn pipeline_latency_ms(&self) -> u64 {
        self.pipeline_latency_ms
    }
    pub fn set_pipeline_latency(&mut self, ms: u64) {
        self.pipeline_latency_ms = ms;
    }
}

impl Default for LatencyCompensator {
    fn default() -> Self {
        Self::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = LatencyCompensator::new(100);
        assert_eq!(c.pipeline_latency_ms(), 100);
    }

    #[test]
    fn test_default() {
        let c = LatencyCompensator::default();
        assert_eq!(c.extrapolations(), 0);
    }

    #[test]
    fn test_update() {
        let mut c = LatencyCompensator::default();
        c.update(32.0, 34.0, 10.0, 0.0, 0.0, 1000);
        assert_eq!(c.last_pos.0, 32.0);
    }

    #[test]
    fn test_compensated_moves_forward() {
        let mut c = LatencyCompensator::new(100);
        c.update(32.0, 34.0, 10.0, 0.0, 0.0, 1000);
        let (lat, _, _) = c.compensated_position(1200);
        assert!(lat > 32.0); // moved north
    }

    #[test]
    fn test_extrapolation_capped() {
        let mut c = LatencyCompensator::new(0);
        c.update(32.0, 34.0, 100.0, 0.0, 0.0, 0);
        let (lat, _, _) = c.compensated_position(100_000); // huge gap
        let max_move = 100.0 * 3.0 / 111_111.0; // 3 sec cap
        assert!((lat - 32.0) <= max_move + 1e-6);
    }

    #[test]
    fn test_blend_fresh_gnss() {
        let c = LatencyCompensator::default();
        let (lat, _) = c.blend_gnss_imu(32.0, 34.0, 0, 32.1, 34.1);
        assert!((lat - 32.0).abs() < 0.01); // fresh GNSS dominates
    }

    #[test]
    fn test_blend_stale_gnss() {
        let c = LatencyCompensator::default();
        let (lat, _) = c.blend_gnss_imu(32.0, 34.0, 5000, 32.1, 34.1);
        assert!(lat > 32.05); // stale GNSS, IMU dominates
    }

    #[test]
    fn test_set_latency() {
        let mut c = LatencyCompensator::default();
        c.set_pipeline_latency(200);
        assert_eq!(c.pipeline_latency_ms(), 200);
    }
}
