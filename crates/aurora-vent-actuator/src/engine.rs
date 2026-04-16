/// Vent actuator: blend door, mode door, zone control
/// Phase 446

#[derive(Debug, Clone)]
pub struct VentActuator {
    pub position_pct: f64,
    pub target_pct: f64,
    pub motor_ok: bool,
    pub gear_ok: bool,
    pub calibrated: bool,
}

impl Default for VentActuator {
    fn default() -> Self {
        Self::new()
    }
}

impl VentActuator {
    pub fn new() -> Self {
        Self {
            position_pct: 50.0,
            target_pct: 50.0,
            motor_ok: true,
            gear_ok: true,
            calibrated: true,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.position_pct - self.target_pct).abs() < 5.0
    }

    pub fn all_ok(&self) -> bool {
        self.motor_ok && self.gear_ok && self.calibrated
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.gear_ok
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if !self.gear_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target() {
        let v = VentActuator::new();
        assert!(v.at_target());
    }

    #[test]
    fn test_all_ok() {
        let v = VentActuator::new();
        assert!(v.all_ok());
    }

    #[test]
    fn test_no_service() {
        let v = VentActuator::new();
        assert!(!v.needs_service());
    }

    #[test]
    fn test_calibrated() {
        let v = VentActuator::new();
        assert!(!v.needs_calibration());
    }

    #[test]
    fn test_bad_motor() {
        let mut v = VentActuator::new();
        v.motor_ok = false;
        assert!(v.needs_service());
    }

    #[test]
    fn test_health() {
        let v = VentActuator::new();
        assert!((v.health_score() - 100.0).abs() < 0.1);
    }
}
