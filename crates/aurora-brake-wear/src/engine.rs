/// Brake wear sensor: pad thickness, wire, connector
/// Phase 667

#[derive(Debug, Clone)]
pub struct BrakeWearSensor {
    pub pad_ok: bool,
    pub wire_ok: bool,
    pub connector_ok: bool,
    pub triggered: bool,
    pub calibrated: bool,
}

impl Default for BrakeWearSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeWearSensor {
    pub fn new() -> Self {
        Self {
            pad_ok: true,
            wire_ok: true,
            connector_ok: true,
            triggered: false,
            calibrated: true,
        }
    }

    pub fn sensor_ok(&self) -> bool {
        self.wire_ok && self.connector_ok
    }

    pub fn reading_ok(&self) -> bool {
        self.pad_ok && !self.triggered && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok() && self.reading_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.triggered || !self.wire_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.triggered { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor() {
        let c = BrakeWearSensor::new();
        assert!(c.sensor_ok());
    }

    #[test]
    fn test_reading() {
        let c = BrakeWearSensor::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeWearSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeWearSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_triggered() {
        let mut c = BrakeWearSensor::new();
        c.triggered = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeWearSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
