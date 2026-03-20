/// Heat pump: refrigerant cycle, COP, reversing valve
/// Phase 635

#[derive(Debug, Clone)]
pub struct HeatPump {
    pub cycle_ok: bool,
    pub cop_value: f64,
    pub reversing_ok: bool,
    pub defrost_ok: bool,
    pub efficiency_ok: bool,
}

impl Default for HeatPump {
    fn default() -> Self {
        Self::new()
    }
}

impl HeatPump {
    pub fn new() -> Self {
        Self {
            cycle_ok: true,
            cop_value: 3.5,
            reversing_ok: true,
            defrost_ok: true,
            efficiency_ok: true,
        }
    }

    pub fn performance_ok(&self) -> bool {
        self.cop_value > 2.0 && self.efficiency_ok
    }

    pub fn system_ok(&self) -> bool {
        self.cycle_ok && self.reversing_ok && self.defrost_ok
    }

    pub fn all_ok(&self) -> bool {
        self.performance_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.cycle_ok || !self.reversing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cycle_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance() {
        let c = HeatPump::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_system() {
        let c = HeatPump::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeatPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HeatPump::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_cycle() {
        let mut c = HeatPump::new();
        c.cycle_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HeatPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
