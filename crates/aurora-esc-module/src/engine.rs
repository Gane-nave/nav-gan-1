/// ESC module: yaw sensor, steering angle, lateral accel
/// Phase 660

#[derive(Debug, Clone)]
pub struct EscModule {
    pub yaw_ok: bool,
    pub steering_angle_ok: bool,
    pub lateral_ok: bool,
    pub control_ok: bool,
    pub calibrated: bool,
}

impl Default for EscModule {
    fn default() -> Self {
        Self::new()
    }
}

impl EscModule {
    pub fn new() -> Self {
        Self {
            yaw_ok: true,
            steering_angle_ok: true,
            lateral_ok: true,
            control_ok: true,
            calibrated: true,
        }
    }

    pub fn sensors_ok(&self) -> bool {
        self.yaw_ok && self.steering_angle_ok && self.lateral_ok
    }

    pub fn system_ok(&self) -> bool {
        self.control_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensors_ok() && self.system_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.yaw_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.control_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensors() {
        let c = EscModule::new();
        assert!(c.sensors_ok());
    }

    #[test]
    fn test_system() {
        let c = EscModule::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EscModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = EscModule::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = EscModule::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = EscModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
