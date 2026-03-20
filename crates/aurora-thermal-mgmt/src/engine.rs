/// Thermal management: battery cooling/heating, motor cooling, cabin HVAC
/// Phase 299

#[derive(Debug, Clone)]
pub struct ThermalManagement {
    pub battery_temp_c: f64,
    pub motor_temp_c: f64,
    pub inverter_temp_c: f64,
    pub coolant_temp_c: f64,
    pub pump_active: bool,
    pub chiller_active: bool,
}

impl Default for ThermalManagement {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalManagement {
    pub fn new() -> Self {
        Self {
            battery_temp_c: 25.0,
            motor_temp_c: 40.0,
            inverter_temp_c: 45.0,
            coolant_temp_c: 30.0,
            pump_active: true,
            chiller_active: false,
        }
    }

    pub fn all_temps_ok(&self) -> bool {
        self.battery_temp_c < 45.0 && self.motor_temp_c < 120.0 && self.inverter_temp_c < 130.0
    }

    pub fn needs_cooling(&self) -> bool {
        self.battery_temp_c > 35.0 || self.motor_temp_c > 100.0
    }

    pub fn needs_heating(&self) -> bool {
        self.battery_temp_c < 5.0
    }

    pub fn max_temp(&self) -> f64 {
        self.battery_temp_c
            .max(self.motor_temp_c)
            .max(self.inverter_temp_c)
    }

    pub fn health_score(&self) -> f64 {
        if !self.all_temps_ok() {
            return 20.0;
        }
        if self.needs_cooling() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temps_ok() {
        let t = ThermalManagement::new();
        assert!(t.all_temps_ok());
    }

    #[test]
    fn test_no_cooling() {
        let t = ThermalManagement::new();
        assert!(!t.needs_cooling());
    }

    #[test]
    fn test_no_heating() {
        let t = ThermalManagement::new();
        assert!(!t.needs_heating());
    }

    #[test]
    fn test_max_temp() {
        let t = ThermalManagement::new();
        assert!((t.max_temp() - 45.0).abs() < 0.1);
    }

    #[test]
    fn test_hot_battery() {
        let mut t = ThermalManagement::new();
        t.battery_temp_c = 50.0;
        assert!(!t.all_temps_ok());
    }

    #[test]
    fn test_health() {
        let t = ThermalManagement::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
