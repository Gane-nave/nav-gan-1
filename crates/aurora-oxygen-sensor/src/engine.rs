/// oxygen sensor: measure, voltage, heater, age, log
/// Phase 1382

#[derive(Debug, Clone)]
pub struct OxygenSensor {
    pub measure_ok: bool,
    pub voltage_ok: bool,
    pub heater_ok: bool,
    pub age_ok: bool,
    pub log_ok: bool,
}

impl Default for OxygenSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl OxygenSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            voltage_ok: true,
            heater_ok: true,
            age_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.voltage_ok && self.heater_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.age_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.voltage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = OxygenSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OxygenSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OxygenSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OxygenSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OxygenSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OxygenSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
