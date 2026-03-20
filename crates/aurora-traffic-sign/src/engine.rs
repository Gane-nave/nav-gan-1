/// Traffic sign recognition: camera, classifier, display
/// Phase 745

#[derive(Debug, Clone)]
pub struct TrafficSign {
    pub camera_ok: bool,
    pub classifier_ok: bool,
    pub display_ok: bool,
    pub database_ok: bool,
    pub calibrated: bool,
}

impl Default for TrafficSign {
    fn default() -> Self {
        Self::new()
    }
}

impl TrafficSign {
    pub fn new() -> Self {
        Self {
            camera_ok: true,
            classifier_ok: true,
            display_ok: true,
            database_ok: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.camera_ok && self.classifier_ok && self.calibrated
    }

    pub fn output_ok(&self) -> bool {
        self.display_ok && self.database_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.output_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.camera_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = TrafficSign::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_output() {
        let c = TrafficSign::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrafficSign::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = TrafficSign::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = TrafficSign::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = TrafficSign::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
