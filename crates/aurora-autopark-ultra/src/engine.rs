/// Autopark ultrasonic: sensor array, slot detect, path plan
/// Phase 888

#[derive(Debug, Clone)]
pub struct AutoparkUltra {
    pub sensor_ok: bool,
    pub slot_ok: bool,
    pub path_ok: bool,
    pub steering_ok: bool,
    pub brake_ok: bool,
}

impl Default for AutoparkUltra {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoparkUltra {
    pub fn new() -> Self {
        Self {
            sensor_ok: true,
            slot_ok: true,
            path_ok: true,
            steering_ok: true,
            brake_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.sensor_ok && self.slot_ok
    }

    pub fn control_ok(&self) -> bool {
        self.path_ok && self.steering_ok && self.brake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.control_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.sensor_ok || !self.path_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = AutoparkUltra::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_control() {
        let c = AutoparkUltra::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AutoparkUltra::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = AutoparkUltra::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_sensor() {
        let mut c = AutoparkUltra::new();
        c.sensor_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = AutoparkUltra::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
