/// EGR valve: actuator, position, carbon buildup
/// Phase 602

#[derive(Debug, Clone)]
pub struct EgrValve {
    pub actuator_ok: bool,
    pub position_ok: bool,
    pub carbon_free: bool,
    pub signal_ok: bool,
    pub flow_ok: bool,
}

impl Default for EgrValve {
    fn default() -> Self {
        Self::new()
    }
}

impl EgrValve {
    pub fn new() -> Self {
        Self {
            actuator_ok: true,
            position_ok: true,
            carbon_free: true,
            signal_ok: true,
            flow_ok: true,
        }
    }

    pub fn valve_ok(&self) -> bool {
        self.actuator_ok && self.position_ok
    }

    pub fn clean(&self) -> bool {
        self.carbon_free && self.flow_ok
    }

    pub fn all_ok(&self) -> bool {
        self.valve_ok() && self.clean() && self.signal_ok
    }

    pub fn needs_cleaning(&self) -> bool {
        !self.carbon_free || !self.flow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valve() {
        let c = EgrValve::new();
        assert!(c.valve_ok());
    }

    #[test]
    fn test_clean() {
        let c = EgrValve::new();
        assert!(c.clean());
    }

    #[test]
    fn test_all_ok() {
        let c = EgrValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_clean() {
        let c = EgrValve::new();
        assert!(!c.needs_cleaning());
    }

    #[test]
    fn test_carbon() {
        let mut c = EgrValve::new();
        c.carbon_free = false;
        assert!(c.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let c = EgrValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
