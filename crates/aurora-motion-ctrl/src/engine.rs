/// Motion control: steer, throttle, brake, gear, stability
/// Phase 1112

#[derive(Debug, Clone)]
pub struct MotionCtrl {
    pub steer_ok: bool,
    pub throttle_ok: bool,
    pub brake_ok: bool,
    pub gear_ok: bool,
    pub stability_ok: bool,
}

impl Default for MotionCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl MotionCtrl {
    pub fn new() -> Self {
        Self {
            steer_ok: true,
            throttle_ok: true,
            brake_ok: true,
            gear_ok: true,
            stability_ok: true,
        }
    }

    pub fn actuation_ok(&self) -> bool {
        self.steer_ok && self.throttle_ok && self.brake_ok
    }

    pub fn management_ok(&self) -> bool {
        self.gear_ok && self.stability_ok
    }

    pub fn all_ok(&self) -> bool {
        self.actuation_ok() && self.management_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.steer_ok || !self.brake_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.steer_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuation() {
        let c = MotionCtrl::new();
        assert!(c.actuation_ok());
    }

    #[test]
    fn test_management() {
        let c = MotionCtrl::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MotionCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = MotionCtrl::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_steer() {
        let mut c = MotionCtrl::new();
        c.steer_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = MotionCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
