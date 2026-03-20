/// Auto high beam: camera, light sensor, glare detect
/// Phase 746

#[derive(Debug, Clone)]
pub struct AutoHighbeam {
    pub camera_ok: bool,
    pub light_sensor_ok: bool,
    pub glare_ok: bool,
    pub actuator_ok: bool,
    pub calibrated: bool,
}

impl Default for AutoHighbeam {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoHighbeam {
    pub fn new() -> Self {
        Self {
            camera_ok: true,
            light_sensor_ok: true,
            glare_ok: true,
            actuator_ok: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.camera_ok && self.light_sensor_ok && self.glare_ok
    }

    pub fn control_ok(&self) -> bool {
        self.actuator_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.control_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.camera_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = AutoHighbeam::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_control() {
        let c = AutoHighbeam::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AutoHighbeam::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = AutoHighbeam::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = AutoHighbeam::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = AutoHighbeam::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
