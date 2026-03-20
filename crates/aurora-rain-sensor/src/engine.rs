/// Rain sensor: optical, sensitivity, wiper control
/// Phase 697

#[derive(Debug, Clone)]
pub struct RainSensor {
    pub optical_ok: bool,
    pub sensitivity_ok: bool,
    pub wiper_ctrl_ok: bool,
    pub lens_ok: bool,
    pub calibrated: bool,
}

impl Default for RainSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl RainSensor {
    pub fn new() -> Self {
        Self {
            optical_ok: true,
            sensitivity_ok: true,
            wiper_ctrl_ok: true,
            lens_ok: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.optical_ok && self.sensitivity_ok && self.lens_ok
    }

    pub fn control_ok(&self) -> bool {
        self.wiper_ctrl_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.control_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.optical_ok || !self.lens_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.optical_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = RainSensor::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_control() {
        let c = RainSensor::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RainSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RainSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_optical() {
        let mut c = RainSensor::new();
        c.optical_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RainSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
