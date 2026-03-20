/// Hill assist: grade, hold, launch, descent control
/// Phase 937

#[derive(Debug, Clone)]
pub struct HillAssist {
    pub grade_ok: bool,
    pub hold_ok: bool,
    pub launch_ok: bool,
    pub descent_ok: bool,
    pub sensor_ok: bool,
}

impl Default for HillAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl HillAssist {
    pub fn new() -> Self {
        Self {
            grade_ok: true,
            hold_ok: true,
            launch_ok: true,
            descent_ok: true,
            sensor_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.grade_ok && self.sensor_ok
    }

    pub fn control_ok(&self) -> bool {
        self.hold_ok && self.launch_ok && self.descent_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.control_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.sensor_ok || !self.grade_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = HillAssist::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_control() {
        let c = HillAssist::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HillAssist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = HillAssist::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_sensor() {
        let mut c = HillAssist::new();
        c.sensor_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = HillAssist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
