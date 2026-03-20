/// Electric fuel pump: motor, check valve, strainer
/// Phase 620

#[derive(Debug, Clone)]
pub struct ElecFuelPump {
    pub motor_ok: bool,
    pub check_valve_ok: bool,
    pub strainer_ok: bool,
    pub pressure_ok: bool,
    pub wiring_ok: bool,
}

impl Default for ElecFuelPump {
    fn default() -> Self {
        Self::new()
    }
}

impl ElecFuelPump {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            check_valve_ok: true,
            strainer_ok: true,
            pressure_ok: true,
            wiring_ok: true,
        }
    }

    pub fn pump_ok(&self) -> bool {
        self.motor_ok && self.check_valve_ok
    }

    pub fn fuel_delivery_ok(&self) -> bool {
        self.pump_ok() && self.strainer_ok && self.pressure_ok
    }

    pub fn all_ok(&self) -> bool {
        self.fuel_delivery_ok() && self.wiring_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.motor_ok || !self.check_valve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pump() {
        let c = ElecFuelPump::new();
        assert!(c.pump_ok());
    }

    #[test]
    fn test_delivery() {
        let c = ElecFuelPump::new();
        assert!(c.fuel_delivery_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ElecFuelPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ElecFuelPump::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_motor() {
        let mut c = ElecFuelPump::new();
        c.motor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ElecFuelPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
