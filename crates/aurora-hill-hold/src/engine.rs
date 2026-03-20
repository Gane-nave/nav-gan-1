/// Hill hold assist: incline sensor, brake hold, release logic
/// Phase 671

#[derive(Debug, Clone)]
pub struct HillHold {
    pub incline_ok: bool,
    pub brake_hold_ok: bool,
    pub release_ok: bool,
    pub ecu_ok: bool,
    pub enabled: bool,
}

impl Default for HillHold {
    fn default() -> Self {
        Self::new()
    }
}

impl HillHold {
    pub fn new() -> Self {
        Self {
            incline_ok: true,
            brake_hold_ok: true,
            release_ok: true,
            ecu_ok: true,
            enabled: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.incline_ok && self.ecu_ok
    }

    pub fn hold_ok(&self) -> bool {
        self.brake_hold_ok && self.release_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.hold_ok() && self.enabled
    }

    pub fn needs_service(&self) -> bool {
        !self.incline_ok || !self.ecu_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ecu_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = HillHold::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_hold() {
        let c = HillHold::new();
        assert!(c.hold_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HillHold::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HillHold::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_incline() {
        let mut c = HillHold::new();
        c.incline_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HillHold::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
