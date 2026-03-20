/// Rain sensor: optical, sensitivity, wiper auto mode
/// Phase 561

#[derive(Debug, Clone)]
pub struct RainSensor {
    pub sensitivity_pct: f64,
    pub optical_ok: bool,
    pub lens_clean: bool,
    pub auto_mode: bool,
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
            sensitivity_pct: 70.0,
            optical_ok: true,
            lens_clean: true,
            auto_mode: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.optical_ok && self.lens_clean
    }

    pub fn system_ok(&self) -> bool {
        self.detection_ok() && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && self.auto_mode
    }

    pub fn needs_service(&self) -> bool {
        !self.optical_ok || !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.optical_ok { return 20.0; }
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
    fn test_system() {
        let c = RainSensor::new();
        assert!(c.system_ok());
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
