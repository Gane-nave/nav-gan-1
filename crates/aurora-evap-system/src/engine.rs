/// EVAP system: fuel vapor recovery, purge valve, canister
/// Phase 497

#[derive(Debug, Clone)]
pub struct EvapSystem {
    pub canister_load_pct: f64,
    pub purge_valve_ok: bool,
    pub vent_valve_ok: bool,
    pub leak_detected: bool,
    pub pressure_ok: bool,
}

impl Default for EvapSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EvapSystem {
    pub fn new() -> Self {
        Self {
            canister_load_pct: 40.0,
            purge_valve_ok: true,
            vent_valve_ok: true,
            leak_detected: false,
            pressure_ok: true,
        }
    }

    pub fn canister_ok(&self) -> bool {
        self.canister_load_pct < 80.0
    }

    pub fn valves_ok(&self) -> bool {
        self.purge_valve_ok && self.vent_valve_ok
    }

    pub fn all_ok(&self) -> bool {
        self.canister_ok() && self.valves_ok() && !self.leak_detected && self.pressure_ok
    }

    pub fn needs_service(&self) -> bool {
        self.leak_detected || !self.purge_valve_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.leak_detected {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canister() {
        let c = EvapSystem::new();
        assert!(c.canister_ok());
    }

    #[test]
    fn test_valves() {
        let c = EvapSystem::new();
        assert!(c.valves_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EvapSystem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EvapSystem::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = EvapSystem::new();
        c.leak_detected = true;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EvapSystem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
