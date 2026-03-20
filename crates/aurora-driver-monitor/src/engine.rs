/// Driver monitoring: IR camera, eye tracking, fatigue
/// Phase 744

#[derive(Debug, Clone)]
pub struct DriverMonitor {
    pub ir_cam_ok: bool,
    pub eye_track_ok: bool,
    pub fatigue_ok: bool,
    pub alert_ok: bool,
    pub calibrated: bool,
}

impl Default for DriverMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverMonitor {
    pub fn new() -> Self {
        Self {
            ir_cam_ok: true,
            eye_track_ok: true,
            fatigue_ok: true,
            alert_ok: true,
            calibrated: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.ir_cam_ok && self.eye_track_ok && self.calibrated
    }

    pub fn safety_ok(&self) -> bool {
        self.fatigue_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.safety_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.ir_cam_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ir_cam_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = DriverMonitor::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_safety() {
        let c = DriverMonitor::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DriverMonitor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = DriverMonitor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = DriverMonitor::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = DriverMonitor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
