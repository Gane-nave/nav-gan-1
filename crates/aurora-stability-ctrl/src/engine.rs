/// Electronic stability control: yaw rate, lateral acceleration, brake vectoring
/// Phase 153

#[derive(Debug, Clone)]
pub struct StabilitySystem {
    pub enabled: bool,
    pub yaw_rate_deg_s: f64,
    pub target_yaw_deg_s: f64,
    pub lateral_accel_g: f64,
    pub max_lateral_g: f64,
    pub interventions: u64,
}

impl Default for StabilitySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl StabilitySystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            yaw_rate_deg_s: 0.0,
            target_yaw_deg_s: 0.0,
            lateral_accel_g: 0.0,
            max_lateral_g: 0.8,
            interventions: 0,
        }
    }

    pub fn yaw_error(&self) -> f64 {
        (self.yaw_rate_deg_s - self.target_yaw_deg_s).abs()
    }

    pub fn is_oversteering(&self) -> bool {
        self.yaw_rate_deg_s.abs() > self.target_yaw_deg_s.abs() * 1.3
    }

    pub fn is_understeering(&self) -> bool {
        self.target_yaw_deg_s.abs() > 5.0
            && self.yaw_rate_deg_s.abs() < self.target_yaw_deg_s.abs() * 0.7
    }

    pub fn needs_intervention(&self) -> bool {
        self.enabled
            && (self.is_oversteering()
                || self.is_understeering()
                || self.lateral_accel_g > self.max_lateral_g)
    }

    pub fn stability_score(&self) -> f64 {
        let yaw_score = (1.0 - self.yaw_error() / 30.0).clamp(0.0, 1.0) * 100.0;
        let lat_score = (1.0 - self.lateral_accel_g / self.max_lateral_g).clamp(0.0, 1.0) * 100.0;
        yaw_score * 0.6 + lat_score * 0.4
    }

    pub fn apply_correction(&mut self) {
        if self.needs_intervention() {
            self.interventions += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaw_error() {
        let mut s = StabilitySystem::new();
        s.yaw_rate_deg_s = 10.0;
        s.target_yaw_deg_s = 5.0;
        assert!((s.yaw_error() - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_oversteer() {
        let mut s = StabilitySystem::new();
        s.yaw_rate_deg_s = 20.0;
        s.target_yaw_deg_s = 10.0;
        assert!(s.is_oversteering());
    }

    #[test]
    fn test_understeer() {
        let mut s = StabilitySystem::new();
        s.yaw_rate_deg_s = 2.0;
        s.target_yaw_deg_s = 10.0;
        assert!(s.is_understeering());
    }

    #[test]
    fn test_stable() {
        let s = StabilitySystem::new();
        assert!(!s.needs_intervention());
    }

    #[test]
    fn test_intervention_oversteer() {
        let mut s = StabilitySystem::new();
        s.yaw_rate_deg_s = 25.0;
        s.target_yaw_deg_s = 10.0;
        assert!(s.needs_intervention());
    }

    #[test]
    fn test_lateral_g() {
        let mut s = StabilitySystem::new();
        s.lateral_accel_g = 0.9;
        assert!(s.needs_intervention());
    }

    #[test]
    fn test_stability_score() {
        let s = StabilitySystem::new();
        assert!(s.stability_score() > 90.0);
    }

    #[test]
    fn test_correction() {
        let mut s = StabilitySystem::new();
        s.yaw_rate_deg_s = 20.0;
        s.target_yaw_deg_s = 5.0;
        s.apply_correction();
        assert_eq!(s.interventions, 1);
    }
}
