/// Automatic parking: ultrasonic, camera, steering, braking
/// Phase 739

#[derive(Debug, Clone)]
pub struct AutoPark {
    pub ultrasonic_ok: bool,
    pub camera_ok: bool,
    pub steering_ok: bool,
    pub braking_ok: bool,
    pub calibrated: bool,
}

impl Default for AutoPark {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoPark {
    pub fn new() -> Self {
        Self {
            ultrasonic_ok: true,
            camera_ok: true,
            steering_ok: true,
            braking_ok: true,
            calibrated: true,
        }
    }

    pub fn perception_ok(&self) -> bool {
        self.ultrasonic_ok && self.camera_ok && self.calibrated
    }

    pub fn actuation_ok(&self) -> bool {
        self.steering_ok && self.braking_ok
    }

    pub fn all_ok(&self) -> bool {
        self.perception_ok() && self.actuation_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.ultrasonic_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ultrasonic_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perception() {
        let c = AutoPark::new();
        assert!(c.perception_ok());
    }

    #[test]
    fn test_actuation() {
        let c = AutoPark::new();
        assert!(c.actuation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AutoPark::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = AutoPark::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = AutoPark::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = AutoPark::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
