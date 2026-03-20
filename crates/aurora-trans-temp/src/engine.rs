/// Transmission temperature sensor: fluid temp, range
/// Phase 691

#[derive(Debug, Clone)]
pub struct TransTempSensor {
    pub sensor_ok: bool,
    pub range_ok: bool,
    pub response_ok: bool,
    pub connector_ok: bool,
    pub calibrated: bool,
}

impl Default for TransTempSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl TransTempSensor {
    pub fn new() -> Self {
        Self {
            sensor_ok: true,
            range_ok: true,
            response_ok: true,
            connector_ok: true,
            calibrated: true,
        }
    }

    pub fn measurement_ok(&self) -> bool {
        self.sensor_ok && self.range_ok && self.response_ok
    }

    pub fn connection_ok(&self) -> bool {
        self.connector_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.measurement_ok() && self.connection_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.sensor_ok || !self.range_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement() {
        let c = TransTempSensor::new();
        assert!(c.measurement_ok());
    }

    #[test]
    fn test_connection() {
        let c = TransTempSensor::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransTempSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TransTempSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_sensor() {
        let mut c = TransTempSensor::new();
        c.sensor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TransTempSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
