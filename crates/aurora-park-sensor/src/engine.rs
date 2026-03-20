/// Parking distance sensor: ultrasonic, range, accuracy
/// Phase 699

#[derive(Debug, Clone)]
pub struct ParkSensor {
    pub ultrasonic_ok: bool,
    pub range_ok: bool,
    pub accuracy_ok: bool,
    pub connector_ok: bool,
    pub calibrated: bool,
}

impl Default for ParkSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkSensor {
    pub fn new() -> Self {
        Self {
            ultrasonic_ok: true,
            range_ok: true,
            accuracy_ok: true,
            connector_ok: true,
            calibrated: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.ultrasonic_ok && self.range_ok && self.accuracy_ok
    }

    pub fn system_ok(&self) -> bool {
        self.connector_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.system_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.ultrasonic_ok || !self.accuracy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ultrasonic_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = ParkSensor::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_system() {
        let c = ParkSensor::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ParkSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ParkSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_ultrasonic() {
        let mut c = ParkSensor::new();
        c.ultrasonic_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ParkSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
