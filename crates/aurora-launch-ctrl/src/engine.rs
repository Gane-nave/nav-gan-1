/// Launch control: RPM, clutch, traction, timing, mode
/// Phase 944

#[derive(Debug, Clone)]
pub struct LaunchCtrl {
    pub rpm_ok: bool,
    pub clutch_ok: bool,
    pub traction_ok: bool,
    pub timing_ok: bool,
    pub mode_ok: bool,
}

impl Default for LaunchCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl LaunchCtrl {
    pub fn new() -> Self {
        Self {
            rpm_ok: true,
            clutch_ok: true,
            traction_ok: true,
            timing_ok: true,
            mode_ok: true,
        }
    }

    pub fn preparation_ok(&self) -> bool {
        self.rpm_ok && self.clutch_ok && self.mode_ok
    }

    pub fn execution_ok(&self) -> bool {
        self.traction_ok && self.timing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.preparation_ok() && self.execution_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.rpm_ok || !self.clutch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rpm_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preparation() {
        let c = LaunchCtrl::new();
        assert!(c.preparation_ok());
    }

    #[test]
    fn test_execution() {
        let c = LaunchCtrl::new();
        assert!(c.execution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LaunchCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = LaunchCtrl::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_rpm() {
        let mut c = LaunchCtrl::new();
        c.rpm_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = LaunchCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
