/// Active aero: wing, splitter, diffuser, drag, downforce
/// Phase 947

#[derive(Debug, Clone)]
pub struct ActiveAero {
    pub wing_ok: bool,
    pub splitter_ok: bool,
    pub diffuser_ok: bool,
    pub drag_ok: bool,
    pub downforce_ok: bool,
}

impl Default for ActiveAero {
    fn default() -> Self {
        Self::new()
    }
}

impl ActiveAero {
    pub fn new() -> Self {
        Self {
            wing_ok: true,
            splitter_ok: true,
            diffuser_ok: true,
            drag_ok: true,
            downforce_ok: true,
        }
    }

    pub fn front_ok(&self) -> bool {
        self.splitter_ok && self.diffuser_ok
    }

    pub fn rear_ok(&self) -> bool {
        self.wing_ok && self.drag_ok && self.downforce_ok
    }

    pub fn all_ok(&self) -> bool {
        self.front_ok() && self.rear_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.wing_ok || !self.splitter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wing_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_front() {
        let c = ActiveAero::new();
        assert!(c.front_ok());
    }

    #[test]
    fn test_rear() {
        let c = ActiveAero::new();
        assert!(c.rear_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ActiveAero::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = ActiveAero::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_wing() {
        let mut c = ActiveAero::new();
        c.wing_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = ActiveAero::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
