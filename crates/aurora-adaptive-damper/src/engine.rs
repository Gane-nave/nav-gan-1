/// Adaptive damper: solenoid, fluid, ECU control
/// Phase 647

#[derive(Debug, Clone)]
pub struct AdaptiveDamper {
    pub solenoid_ok: bool,
    pub fluid_ok: bool,
    pub ecu_ok: bool,
    pub response_ok: bool,
    pub calibrated: bool,
}

impl Default for AdaptiveDamper {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveDamper {
    pub fn new() -> Self {
        Self {
            solenoid_ok: true,
            fluid_ok: true,
            ecu_ok: true,
            response_ok: true,
            calibrated: true,
        }
    }

    pub fn actuator_ok(&self) -> bool {
        self.solenoid_ok && self.fluid_ok
    }

    pub fn control_ok(&self) -> bool {
        self.ecu_ok && self.response_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.actuator_ok() && self.control_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.solenoid_ok || !self.ecu_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.solenoid_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuator() {
        let c = AdaptiveDamper::new();
        assert!(c.actuator_ok());
    }

    #[test]
    fn test_control() {
        let c = AdaptiveDamper::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AdaptiveDamper::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AdaptiveDamper::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_solenoid() {
        let mut c = AdaptiveDamper::new();
        c.solenoid_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AdaptiveDamper::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
