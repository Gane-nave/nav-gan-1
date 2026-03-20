/// Oil temperature sensor: thermistor, range, response
/// Phase 690

#[derive(Debug, Clone)]
pub struct OilTempSensor {
    pub thermistor_ok: bool,
    pub range_ok: bool,
    pub response_ok: bool,
    pub wiring_ok: bool,
    pub calibrated: bool,
}

impl Default for OilTempSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl OilTempSensor {
    pub fn new() -> Self {
        Self {
            thermistor_ok: true,
            range_ok: true,
            response_ok: true,
            wiring_ok: true,
            calibrated: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.thermistor_ok && self.range_ok && self.response_ok
    }

    pub fn circuit_ok(&self) -> bool {
        self.wiring_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.circuit_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.thermistor_ok || !self.range_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.thermistor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = OilTempSensor::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_circuit() {
        let c = OilTempSensor::new();
        assert!(c.circuit_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilTempSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = OilTempSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_thermistor() {
        let mut c = OilTempSensor::new();
        c.thermistor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = OilTempSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
