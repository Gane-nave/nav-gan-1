/// Rack and pinion: steering ratio, play measurement, boot condition
/// Phase 223

#[derive(Debug, Clone)]
pub struct RackPinion {
    pub steering_ratio: f64,
    pub center_play_mm: f64,
    pub boot_left_ok: bool,
    pub boot_right_ok: bool,
    pub leak_detected: bool,
    pub mileage_km: f64,
}

impl Default for RackPinion {
    fn default() -> Self {
        Self::new()
    }
}

impl RackPinion {
    pub fn new() -> Self {
        Self {
            steering_ratio: 15.0,
            center_play_mm: 0.5,
            boot_left_ok: true,
            boot_right_ok: true,
            leak_detected: false,
            mileage_km: 80000.0,
        }
    }

    pub fn play_ok(&self) -> bool {
        self.center_play_mm < 2.0
    }

    pub fn boots_ok(&self) -> bool {
        self.boot_left_ok && self.boot_right_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.leak_detected || !self.play_ok() || !self.boots_ok()
    }

    pub fn quick_ratio(&self) -> bool {
        self.steering_ratio < 14.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if self.leak_detected {
            score -= 40.0;
        }
        if !self.play_ok() {
            score -= 25.0;
        }
        if !self.boots_ok() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_ok() {
        let r = RackPinion::new();
        assert!(r.play_ok());
    }

    #[test]
    fn test_boots_ok() {
        let r = RackPinion::new();
        assert!(r.boots_ok());
    }

    #[test]
    fn test_no_replacement() {
        let r = RackPinion::new();
        assert!(!r.needs_replacement());
    }

    #[test]
    fn test_not_quick() {
        let r = RackPinion::new();
        assert!(!r.quick_ratio());
    }

    #[test]
    fn test_leak() {
        let mut r = RackPinion::new();
        r.leak_detected = true;
        assert!(r.needs_replacement());
    }

    #[test]
    fn test_health() {
        let r = RackPinion::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
