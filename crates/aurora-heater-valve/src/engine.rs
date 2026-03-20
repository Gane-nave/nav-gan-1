/// Heater control valve: actuator, blend, position
/// Phase 626

#[derive(Debug, Clone)]
pub struct HeaterValve {
    pub actuator_ok: bool,
    pub blend_ok: bool,
    pub position_ok: bool,
    pub seal_ok: bool,
    pub cable_ok: bool,
}

impl Default for HeaterValve {
    fn default() -> Self {
        Self::new()
    }
}

impl HeaterValve {
    pub fn new() -> Self {
        Self {
            actuator_ok: true,
            blend_ok: true,
            position_ok: true,
            seal_ok: true,
            cable_ok: true,
        }
    }

    pub fn valve_ok(&self) -> bool {
        self.actuator_ok && self.position_ok
    }

    pub fn control_ok(&self) -> bool {
        self.blend_ok && self.cable_ok
    }

    pub fn all_ok(&self) -> bool {
        self.valve_ok() && self.control_ok() && self.seal_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.actuator_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valve() {
        let c = HeaterValve::new();
        assert!(c.valve_ok());
    }

    #[test]
    fn test_control() {
        let c = HeaterValve::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeaterValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HeaterValve::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_actuator() {
        let mut c = HeaterValve::new();
        c.actuator_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HeaterValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
