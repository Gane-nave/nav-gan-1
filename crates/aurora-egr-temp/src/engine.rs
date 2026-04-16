/// EGR temperature sensor: thermocouple, delta, range
/// Phase 594

#[derive(Debug, Clone)]
pub struct EgrTemp {
    pub temp_c: f64,
    pub max_temp_c: f64,
    pub thermocouple_ok: bool,
    pub in_range: bool,
    pub wiring_ok: bool,
}

impl Default for EgrTemp {
    fn default() -> Self {
        Self::new()
    }
}

impl EgrTemp {
    pub fn new() -> Self {
        Self {
            temp_c: 250.0,
            max_temp_c: 500.0,
            thermocouple_ok: true,
            in_range: true,
            wiring_ok: true,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn sensor_ok(&self) -> bool {
        self.thermocouple_ok && self.in_range && self.wiring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.temp_ok() && self.sensor_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.thermocouple_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.thermocouple_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp() {
        let c = EgrTemp::new();
        assert!(c.temp_ok());
    }

    #[test]
    fn test_sensor() {
        let c = EgrTemp::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EgrTemp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = EgrTemp::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_thermocouple() {
        let mut c = EgrTemp::new();
        c.thermocouple_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = EgrTemp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
