/// Evasive steering: path planning, torque overlay, stability
/// Phase 841

#[derive(Debug, Clone)]
pub struct EvasiveSteer {
    pub planning_ok: bool,
    pub torque_ok: bool,
    pub stability_ok: bool,
    pub sensor_ok: bool,
    pub calibration_ok: bool,
}

impl Default for EvasiveSteer {
    fn default() -> Self {
        Self::new()
    }
}

impl EvasiveSteer {
    pub fn new() -> Self {
        Self {
            planning_ok: true,
            torque_ok: true,
            stability_ok: true,
            sensor_ok: true,
            calibration_ok: true,
        }
    }

    pub fn control_ok(&self) -> bool {
        self.planning_ok && self.torque_ok && self.stability_ok
    }

    pub fn system_ok(&self) -> bool {
        self.sensor_ok && self.calibration_ok
    }

    pub fn all_ok(&self) -> bool {
        self.control_ok() && self.system_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibration_ok || !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.planning_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control() {
        let c = EvasiveSteer::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_system() {
        let c = EvasiveSteer::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EvasiveSteer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = EvasiveSteer::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = EvasiveSteer::new();
        c.calibration_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = EvasiveSteer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
