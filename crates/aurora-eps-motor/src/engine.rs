/// EPS motor: torque sensor, control unit, assist map
/// Phase 638

#[derive(Debug, Clone)]
pub struct EpsMotor {
    pub torque_sensor_ok: bool,
    pub control_ok: bool,
    pub assist_ok: bool,
    pub motor_ok: bool,
    pub calibrated: bool,
}

impl Default for EpsMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl EpsMotor {
    pub fn new() -> Self {
        Self {
            torque_sensor_ok: true,
            control_ok: true,
            assist_ok: true,
            motor_ok: true,
            calibrated: true,
        }
    }

    pub fn sensor_ok(&self) -> bool {
        self.torque_sensor_ok && self.calibrated
    }

    pub fn system_ok(&self) -> bool {
        self.control_ok && self.motor_ok && self.assist_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.torque_sensor_ok || !self.motor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor() {
        let c = EpsMotor::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_system() {
        let c = EpsMotor::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EpsMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EpsMotor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = EpsMotor::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EpsMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
