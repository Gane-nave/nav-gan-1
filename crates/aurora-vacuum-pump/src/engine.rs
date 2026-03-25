/// Vacuum pump: vane, check valve, brake booster supply
/// Phase 621

#[derive(Debug, Clone)]
pub struct VacuumPump {
    pub vane_ok: bool,
    pub check_valve_ok: bool,
    pub vacuum_ok: bool,
    pub noise_ok: bool,
    pub oil_ok: bool,
}

impl Default for VacuumPump {
    fn default() -> Self {
        Self::new()
    }
}

impl VacuumPump {
    pub fn new() -> Self {
        Self {
            vane_ok: true,
            check_valve_ok: true,
            vacuum_ok: true,
            noise_ok: true,
            oil_ok: true,
        }
    }

    pub fn pump_ok(&self) -> bool {
        self.vane_ok && self.check_valve_ok
    }

    pub fn brake_supply_ok(&self) -> bool {
        self.vacuum_ok && self.pump_ok()
    }

    pub fn all_ok(&self) -> bool {
        self.brake_supply_ok() && self.noise_ok && self.oil_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.vane_ok || !self.check_valve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.vane_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pump() {
        let c = VacuumPump::new();
        assert!(c.pump_ok());
    }

    #[test]
    fn test_brake() {
        let c = VacuumPump::new();
        assert!(c.brake_supply_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VacuumPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = VacuumPump::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_vane() {
        let mut c = VacuumPump::new();
        c.vane_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = VacuumPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
