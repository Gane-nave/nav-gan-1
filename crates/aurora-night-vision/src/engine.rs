/// Night vision: IR camera, pedestrian detect, overlay
/// Phase 741

#[derive(Debug, Clone)]
pub struct NightVision {
    pub ir_camera_ok: bool,
    pub pedestrian_ok: bool,
    pub overlay_ok: bool,
    pub heater_ok: bool,
    pub calibrated: bool,
}

impl Default for NightVision {
    fn default() -> Self {
        Self::new()
    }
}

impl NightVision {
    pub fn new() -> Self {
        Self {
            ir_camera_ok: true,
            pedestrian_ok: true,
            overlay_ok: true,
            heater_ok: true,
            calibrated: true,
        }
    }

    pub fn imaging_ok(&self) -> bool {
        self.ir_camera_ok && self.heater_ok && self.calibrated
    }

    pub fn detection_ok(&self) -> bool {
        self.pedestrian_ok && self.overlay_ok
    }

    pub fn all_ok(&self) -> bool {
        self.imaging_ok() && self.detection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.ir_camera_ok || !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.ir_camera_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imaging() {
        let c = NightVision::new();
        assert!(c.imaging_ok());
    }

    #[test]
    fn test_detection() {
        let c = NightVision::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NightVision::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = NightVision::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ir() {
        let mut c = NightVision::new();
        c.ir_camera_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = NightVision::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
