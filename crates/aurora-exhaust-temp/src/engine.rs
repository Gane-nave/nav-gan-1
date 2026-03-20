/// Exhaust gas temperature sensor: thermocouple, range
/// Phase 692

#[derive(Debug, Clone)]
pub struct ExhaustTempSensor {
    pub thermocouple_ok: bool,
    pub range_ok: bool,
    pub response_ok: bool,
    pub sheath_ok: bool,
    pub calibrated: bool,
}

impl Default for ExhaustTempSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustTempSensor {
    pub fn new() -> Self {
        Self {
            thermocouple_ok: true,
            range_ok: true,
            response_ok: true,
            sheath_ok: true,
            calibrated: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.thermocouple_ok && self.range_ok
    }

    pub fn physical_ok(&self) -> bool {
        self.sheath_ok && self.response_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.physical_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.thermocouple_ok || !self.sheath_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.thermocouple_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = ExhaustTempSensor::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_physical() {
        let c = ExhaustTempSensor::new();
        assert!(c.physical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ExhaustTempSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ExhaustTempSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_thermocouple() {
        let mut c = ExhaustTempSensor::new();
        c.thermocouple_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ExhaustTempSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
