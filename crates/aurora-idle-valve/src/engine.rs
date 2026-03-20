/// Idle air control valve: stepper, position, bypass
/// Phase 600

#[derive(Debug, Clone)]
pub struct IdleValve {
    pub position_pct: f64,
    pub stepper_ok: bool,
    pub bypass_ok: bool,
    pub signal_ok: bool,
    pub calibrated: bool,
}

impl Default for IdleValve {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleValve {
    pub fn new() -> Self {
        Self {
            position_pct: 25.0,
            stepper_ok: true,
            bypass_ok: true,
            signal_ok: true,
            calibrated: true,
        }
    }

    pub fn position_ok(&self) -> bool {
        self.position_pct >= 0.0 && self.position_pct <= 100.0
    }

    pub fn system_ok(&self) -> bool {
        self.stepper_ok && self.bypass_ok && self.signal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.position_ok() && self.system_ok() && self.calibrated
    }

    pub fn needs_cleaning(&self) -> bool {
        !self.stepper_ok || !self.bypass_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.stepper_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let c = IdleValve::new();
        assert!(c.position_ok());
    }

    #[test]
    fn test_system() {
        let c = IdleValve::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IdleValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_clean() {
        let c = IdleValve::new();
        assert!(!c.needs_cleaning());
    }

    #[test]
    fn test_stepper() {
        let mut c = IdleValve::new();
        c.stepper_ok = false;
        assert!(c.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let c = IdleValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
