/// Fuel pressure sensor: rail, pump, regulator
/// Phase 592

#[derive(Debug, Clone)]
pub struct FuelPressure {
    pub rail_bar: f64,
    pub target_bar: f64,
    pub pump_ok: bool,
    pub regulator_ok: bool,
    pub sensor_ok: bool,
}

impl Default for FuelPressure {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelPressure {
    pub fn new() -> Self {
        Self {
            rail_bar: 3.5,
            target_bar: 3.5,
            pump_ok: true,
            regulator_ok: true,
            sensor_ok: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        (self.rail_bar - self.target_bar).abs() < 0.5
    }

    pub fn system_ok(&self) -> bool {
        self.pump_ok && self.regulator_ok && self.sensor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.pump_ok || !self.regulator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pump_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = FuelPressure::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_system() {
        let c = FuelPressure::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelPressure::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FuelPressure::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_pump() {
        let mut c = FuelPressure::new();
        c.pump_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FuelPressure::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
