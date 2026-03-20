/// Engine coolant temp sensor: thermistor, range, accuracy
/// Phase 589

#[derive(Debug, Clone)]
pub struct EctSensor {
    pub temp_c: f64,
    pub thermistor_ok: bool,
    pub in_range: bool,
    pub accurate: bool,
    pub wiring_ok: bool,
}

impl Default for EctSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl EctSensor {
    pub fn new() -> Self {
        Self {
            temp_c: 85.0,
            thermistor_ok: true,
            in_range: true,
            accurate: true,
            wiring_ok: true,
        }
    }

    pub fn reading_ok(&self) -> bool {
        self.thermistor_ok && self.in_range
    }

    pub fn accuracy_ok(&self) -> bool {
        self.accurate && self.wiring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.reading_ok() && self.accuracy_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.thermistor_ok || !self.wiring_ok
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
    fn test_reading() {
        let c = EctSensor::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_accuracy() {
        let c = EctSensor::new();
        assert!(c.accuracy_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EctSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = EctSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_thermistor() {
        let mut c = EctSensor::new();
        c.thermistor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = EctSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
