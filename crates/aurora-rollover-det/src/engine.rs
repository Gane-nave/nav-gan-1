/// Rollover detection: roll angle, roll rate, stability threshold
/// Phase 228

#[derive(Debug, Clone)]
pub struct RolloverDetector {
    pub roll_angle_deg: f64,
    pub roll_rate_dps: f64,
    pub lateral_accel_g: f64,
    pub threshold_deg: f64,
    pub curtain_airbag_armed: bool,
}

impl Default for RolloverDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl RolloverDetector {
    pub fn new() -> Self {
        Self {
            roll_angle_deg: 0.5,
            roll_rate_dps: 0.0,
            lateral_accel_g: 0.05,
            threshold_deg: 45.0,
            curtain_airbag_armed: true,
        }
    }

    pub fn rollover_imminent(&self) -> bool {
        self.roll_angle_deg.abs() > self.threshold_deg
    }

    pub fn high_roll_rate(&self) -> bool {
        self.roll_rate_dps.abs() > 75.0
    }

    pub fn deploy_curtains(&self) -> bool {
        self.curtain_airbag_armed && self.rollover_imminent()
    }

    pub fn stability_margin_deg(&self) -> f64 {
        (self.threshold_deg - self.roll_angle_deg.abs()).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.curtain_airbag_armed {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_rollover() {
        let r = RolloverDetector::new();
        assert!(!r.rollover_imminent());
    }

    #[test]
    fn test_no_high_rate() {
        let r = RolloverDetector::new();
        assert!(!r.high_roll_rate());
    }

    #[test]
    fn test_no_deploy() {
        let r = RolloverDetector::new();
        assert!(!r.deploy_curtains());
    }

    #[test]
    fn test_margin() {
        let r = RolloverDetector::new();
        assert!(r.stability_margin_deg() > 40.0);
    }

    #[test]
    fn test_rollover() {
        let mut r = RolloverDetector::new();
        r.roll_angle_deg = 50.0;
        assert!(r.deploy_curtains());
    }

    #[test]
    fn test_health() {
        let r = RolloverDetector::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
