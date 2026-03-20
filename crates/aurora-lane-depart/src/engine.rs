/// Lane departure warning: camera, lane model, haptic, audio
/// Phase 736

#[derive(Debug, Clone)]
pub struct LaneDepart {
    pub camera_ok: bool,
    pub lane_model_ok: bool,
    pub haptic_ok: bool,
    pub audio_ok: bool,
    pub calibrated: bool,
}

impl Default for LaneDepart {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneDepart {
    pub fn new() -> Self {
        Self {
            camera_ok: true,
            lane_model_ok: true,
            haptic_ok: true,
            audio_ok: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.camera_ok && self.lane_model_ok && self.calibrated
    }

    pub fn feedback_ok(&self) -> bool {
        self.haptic_ok && self.audio_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.feedback_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.camera_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = LaneDepart::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_feedback() {
        let c = LaneDepart::new();
        assert!(c.feedback_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LaneDepart::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = LaneDepart::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = LaneDepart::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = LaneDepart::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
