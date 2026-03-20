/// VVT solenoid: oil control, timing advance, filter
/// Phase 605

#[derive(Debug, Clone)]
pub struct VvtSolenoid {
    pub oil_ctrl_ok: bool,
    pub timing_ok: bool,
    pub filter_ok: bool,
    pub signal_ok: bool,
    pub response_ok: bool,
}

impl Default for VvtSolenoid {
    fn default() -> Self {
        Self::new()
    }
}

impl VvtSolenoid {
    pub fn new() -> Self {
        Self {
            oil_ctrl_ok: true,
            timing_ok: true,
            filter_ok: true,
            signal_ok: true,
            response_ok: true,
        }
    }

    pub fn solenoid_ok(&self) -> bool {
        self.oil_ctrl_ok && self.signal_ok
    }

    pub fn system_ok(&self) -> bool {
        self.solenoid_ok() && self.timing_ok && self.filter_ok
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && self.response_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.oil_ctrl_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.oil_ctrl_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solenoid() {
        let c = VvtSolenoid::new();
        assert!(c.solenoid_ok());
    }

    #[test]
    fn test_system() {
        let c = VvtSolenoid::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VvtSolenoid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = VvtSolenoid::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_oil() {
        let mut c = VvtSolenoid::new();
        c.oil_ctrl_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = VvtSolenoid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
