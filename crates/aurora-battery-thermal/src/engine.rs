/// Battery thermal management: heating, cooling, insulation
/// Phase 857

#[derive(Debug, Clone)]
pub struct BatteryThermal {
    pub heating_ok: bool,
    pub cooling_ok: bool,
    pub insulation_ok: bool,
    pub sensor_ok: bool,
    pub control_ok: bool,
}

impl Default for BatteryThermal {
    fn default() -> Self {
        Self::new()
    }
}

impl BatteryThermal {
    pub fn new() -> Self {
        Self {
            heating_ok: true,
            cooling_ok: true,
            insulation_ok: true,
            sensor_ok: true,
            control_ok: true,
        }
    }

    pub fn temp_control_ok(&self) -> bool {
        self.heating_ok && self.cooling_ok && self.control_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.insulation_ok && self.sensor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.temp_control_ok() && self.protection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.cooling_ok || !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cooling_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_control() {
        let c = BatteryThermal::new();
        assert!(c.temp_control_ok());
    }

    #[test]
    fn test_protection() {
        let c = BatteryThermal::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BatteryThermal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BatteryThermal::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_cooling() {
        let mut c = BatteryThermal::new();
        c.cooling_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BatteryThermal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
