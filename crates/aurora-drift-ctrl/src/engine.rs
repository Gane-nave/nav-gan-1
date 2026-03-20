/// Drift control: angle, counter-steer, traction, stability
/// Phase 942

#[derive(Debug, Clone)]
pub struct DriftCtrl {
    pub angle_ok: bool,
    pub steer_ok: bool,
    pub traction_ok: bool,
    pub stability_ok: bool,
    pub mode_ok: bool,
}

impl Default for DriftCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl DriftCtrl {
    pub fn new() -> Self {
        Self {
            angle_ok: true,
            steer_ok: true,
            traction_ok: true,
            stability_ok: true,
            mode_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.angle_ok && self.steer_ok
    }

    pub fn control_ok(&self) -> bool {
        self.traction_ok && self.stability_ok && self.mode_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.control_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.angle_ok || !self.steer_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.angle_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = DriftCtrl::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_control() {
        let c = DriftCtrl::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DriftCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = DriftCtrl::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_angle() {
        let mut c = DriftCtrl::new();
        c.angle_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = DriftCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
