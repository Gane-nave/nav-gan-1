/// Forward collision warning: TTC, radar, camera fusion
/// Phase 735

#[derive(Debug, Clone)]
pub struct CollisionWarn {
    pub ttc_ok: bool,
    pub radar_ok: bool,
    pub camera_ok: bool,
    pub alert_ok: bool,
    pub calibrated: bool,
}

impl Default for CollisionWarn {
    fn default() -> Self {
        Self::new()
    }
}

impl CollisionWarn {
    pub fn new() -> Self {
        Self {
            ttc_ok: true,
            radar_ok: true,
            camera_ok: true,
            alert_ok: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.ttc_ok && self.radar_ok && self.camera_ok
    }

    pub fn warning_ok(&self) -> bool {
        self.alert_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.warning_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.radar_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.radar_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = CollisionWarn::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_warning() {
        let c = CollisionWarn::new();
        assert!(c.warning_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CollisionWarn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = CollisionWarn::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = CollisionWarn::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = CollisionWarn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
