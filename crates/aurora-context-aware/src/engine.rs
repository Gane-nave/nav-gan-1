//! Context awareness — adapts navigation parameters based on
//! environment (city/highway), mode (driving/walking), and conditions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigationMode {
    Driving,
    Walking,
    Cycling,
    Transit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentType {
    Urban,
    Suburban,
    Highway,
    Rural,
    Tunnel,
    Indoor,
    Parking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeContext {
    PeakHours,
    OffPeak,
    Night,
    Weekend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextParams {
    pub gnss_sample_rate_hz: f64,
    pub fusion_weight_gnss: f64,
    pub fusion_weight_imu: f64,
    pub reroute_aggressiveness: f64,
    pub map_match_strictness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEngine {
    mode: NavigationMode,
    environment: EnvironmentType,
    time_ctx: TimeContext,
    speed_kmh: f64,
    interference_level: f64,
    params: ContextParams,
}

impl ContextEngine {
    pub fn new() -> Self {
        let mut s = Self {
            mode: NavigationMode::Driving,
            environment: EnvironmentType::Urban,
            time_ctx: TimeContext::OffPeak,
            speed_kmh: 0.0,
            interference_level: 0.0,
            params: ContextParams {
                gnss_sample_rate_hz: 1.0,
                fusion_weight_gnss: 0.7,
                fusion_weight_imu: 0.3,
                reroute_aggressiveness: 0.5,
                map_match_strictness: 0.8,
            },
        };
        s.recalculate();
        s
    }

    pub fn set_mode(&mut self, mode: NavigationMode) {
        self.mode = mode;
        self.recalculate();
    }
    pub fn set_environment(&mut self, env: EnvironmentType) {
        self.environment = env;
        self.recalculate();
    }
    pub fn set_time_context(&mut self, tc: TimeContext) {
        self.time_ctx = tc;
        self.recalculate();
    }
    pub fn set_speed(&mut self, kmh: f64) {
        self.speed_kmh = kmh;
        self.recalculate();
    }
    pub fn set_interference(&mut self, level: f64) {
        self.interference_level = level.clamp(0.0, 1.0);
        self.recalculate();
    }

    fn recalculate(&mut self) {
        // GNSS sample rate: higher in complex areas, lower at standstill
        self.params.gnss_sample_rate_hz = match self.environment {
            EnvironmentType::Urban | EnvironmentType::Parking => 5.0,
            EnvironmentType::Highway => {
                if self.speed_kmh > 80.0 {
                    2.0
                } else {
                    1.0
                }
            }
            EnvironmentType::Tunnel | EnvironmentType::Indoor => 0.1,
            _ => 1.0,
        };
        // Fusion weights: trust IMU more in interference
        self.params.fusion_weight_imu = (0.3 + self.interference_level * 0.5).min(0.9);
        self.params.fusion_weight_gnss = 1.0 - self.params.fusion_weight_imu;
        // Reroute aggressiveness: more aggressive in peak hours
        self.params.reroute_aggressiveness = match self.time_ctx {
            TimeContext::PeakHours => 0.8,
            TimeContext::Night => 0.2,
            _ => 0.5,
        };
        // Map match strictness: looser in walking mode
        self.params.map_match_strictness = match self.mode {
            NavigationMode::Walking => 0.3,
            NavigationMode::Cycling => 0.5,
            _ => 0.8,
        };
    }

    pub fn mode(&self) -> NavigationMode {
        self.mode
    }
    pub fn environment(&self) -> EnvironmentType {
        self.environment
    }
    pub fn params(&self) -> &ContextParams {
        &self.params
    }
}

impl Default for ContextEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let e = ContextEngine::new();
        assert_eq!(e.mode(), NavigationMode::Driving);
    }

    #[test]
    fn test_default() {
        let e = ContextEngine::default();
        assert_eq!(e.environment(), EnvironmentType::Urban);
    }

    #[test]
    fn test_urban_high_sample() {
        let e = ContextEngine::new();
        assert!(e.params().gnss_sample_rate_hz >= 5.0);
    }

    #[test]
    fn test_tunnel_low_sample() {
        let mut e = ContextEngine::new();
        e.set_environment(EnvironmentType::Tunnel);
        assert!(e.params().gnss_sample_rate_hz < 1.0);
    }

    #[test]
    fn test_interference_shifts_weights() {
        let mut e = ContextEngine::new();
        e.set_interference(0.8);
        assert!(e.params().fusion_weight_imu > 0.5);
    }

    #[test]
    fn test_peak_hours_aggressive() {
        let mut e = ContextEngine::new();
        e.set_time_context(TimeContext::PeakHours);
        assert!(e.params().reroute_aggressiveness > 0.7);
    }

    #[test]
    fn test_walking_loose_match() {
        let mut e = ContextEngine::new();
        e.set_mode(NavigationMode::Walking);
        assert!(e.params().map_match_strictness < 0.5);
    }

    #[test]
    fn test_mode_set() {
        let mut e = ContextEngine::new();
        e.set_mode(NavigationMode::Cycling);
        assert_eq!(e.mode(), NavigationMode::Cycling);
    }
}
