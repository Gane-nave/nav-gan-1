/// Weld simulation: spot, seam, laser, robot, quality
/// Phase 966

#[derive(Debug, Clone)]
pub struct WeldSim {
    pub spot_ok: bool,
    pub seam_ok: bool,
    pub laser_ok: bool,
    pub robot_ok: bool,
    pub quality_ok: bool,
}

impl Default for WeldSim {
    fn default() -> Self {
        Self::new()
    }
}

impl WeldSim {
    pub fn new() -> Self {
        Self {
            spot_ok: true,
            seam_ok: true,
            laser_ok: true,
            robot_ok: true,
            quality_ok: true,
        }
    }

    pub fn process_ok(&self) -> bool {
        self.spot_ok && self.seam_ok && self.laser_ok
    }

    pub fn automation_ok(&self) -> bool {
        self.robot_ok && self.quality_ok
    }

    pub fn all_ok(&self) -> bool {
        self.process_ok() && self.automation_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.robot_ok || !self.laser_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.robot_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let c = WeldSim::new();
        assert!(c.process_ok());
    }

    #[test]
    fn test_automation() {
        let c = WeldSim::new();
        assert!(c.automation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WeldSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = WeldSim::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_robot() {
        let mut c = WeldSim::new();
        c.robot_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = WeldSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
