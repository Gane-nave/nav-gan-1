/// Lane assist: detection, departure, centering, change
/// Phase 926

#[derive(Debug, Clone)]
pub struct LaneAssist {
    pub detect_ok: bool,
    pub departure_ok: bool,
    pub center_ok: bool,
    pub change_ok: bool,
    pub camera_ok: bool,
}

impl Default for LaneAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneAssist {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            departure_ok: true,
            center_ok: true,
            change_ok: true,
            camera_ok: true,
        }
    }

    pub fn perception_ok(&self) -> bool {
        self.detect_ok && self.camera_ok
    }

    pub fn control_ok(&self) -> bool {
        self.departure_ok && self.center_ok && self.change_ok
    }

    pub fn all_ok(&self) -> bool {
        self.perception_ok() && self.control_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.camera_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perception() {
        let c = LaneAssist::new();
        assert!(c.perception_ok());
    }

    #[test]
    fn test_control() {
        let c = LaneAssist::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LaneAssist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = LaneAssist::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_camera() {
        let mut c = LaneAssist::new();
        c.camera_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = LaneAssist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
