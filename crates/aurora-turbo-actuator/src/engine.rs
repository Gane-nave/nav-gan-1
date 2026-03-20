/// Turbo actuator: electronic, vacuum, position
/// Phase 610

#[derive(Debug, Clone)]
pub struct TurboActuator {
    pub electronic_ok: bool,
    pub vacuum_ok: bool,
    pub position_ok: bool,
    pub signal_ok: bool,
    pub calibrated: bool,
}

impl Default for TurboActuator {
    fn default() -> Self {
        Self::new()
    }
}

impl TurboActuator {
    pub fn new() -> Self {
        Self {
            electronic_ok: true,
            vacuum_ok: true,
            position_ok: true,
            signal_ok: true,
            calibrated: true,
        }
    }

    pub fn actuator_ok(&self) -> bool {
        self.electronic_ok && self.vacuum_ok
    }

    pub fn control_ok(&self) -> bool {
        self.position_ok && self.signal_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.actuator_ok() && self.control_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.electronic_ok || !self.position_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.electronic_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuator() {
        let c = TurboActuator::new();
        assert!(c.actuator_ok());
    }

    #[test]
    fn test_control() {
        let c = TurboActuator::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TurboActuator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TurboActuator::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_electronic() {
        let mut c = TurboActuator::new();
        c.electronic_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TurboActuator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
