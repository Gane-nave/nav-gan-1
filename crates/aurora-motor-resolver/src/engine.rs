/// Motor resolver: angle, speed, excitation, fault
/// Phase 864

#[derive(Debug, Clone)]
pub struct MotorResolver {
    pub angle_ok: bool,
    pub speed_ok: bool,
    pub excitation_ok: bool,
    pub fault_ok: bool,
    pub calibration_ok: bool,
}

impl Default for MotorResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl MotorResolver {
    pub fn new() -> Self {
        Self {
            angle_ok: true,
            speed_ok: true,
            excitation_ok: true,
            fault_ok: true,
            calibration_ok: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.angle_ok && self.speed_ok && self.excitation_ok
    }

    pub fn diagnostics_ok(&self) -> bool {
        self.fault_ok && self.calibration_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.diagnostics_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibration_ok || !self.angle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.angle_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = MotorResolver::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_diagnostics() {
        let c = MotorResolver::new();
        assert!(c.diagnostics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MotorResolver::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = MotorResolver::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = MotorResolver::new();
        c.calibration_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = MotorResolver::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
