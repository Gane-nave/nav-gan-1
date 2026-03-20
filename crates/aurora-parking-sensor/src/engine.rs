/// Parking sensor: ultrasonic, calibration, coverage
/// Phase 544

#[derive(Debug, Clone)]
pub struct ParkingSensor {
    pub sensor_count: u32,
    pub failed_count: u32,
    pub calibrated: bool,
    pub range_cm: f64,
    pub min_range_cm: f64,
}

impl Default for ParkingSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkingSensor {
    pub fn new() -> Self {
        Self {
            sensor_count: 8,
            failed_count: 0,
            calibrated: true,
            range_cm: 150.0,
            min_range_cm: 20.0,
        }
    }

    pub fn all_sensors_ok(&self) -> bool {
        self.failed_count == 0
    }

    pub fn range_ok(&self) -> bool {
        self.range_cm > self.min_range_cm
    }

    pub fn all_ok(&self) -> bool {
        self.all_sensors_ok() && self.range_ok() && self.calibrated
    }

    pub fn needs_service(&self) -> bool {
        self.failed_count > 0 || !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if self.failed_count > 0 { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensors() {
        let c = ParkingSensor::new();
        assert!(c.all_sensors_ok());
    }

    #[test]
    fn test_range() {
        let c = ParkingSensor::new();
        assert!(c.range_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ParkingSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ParkingSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_failed() {
        let mut c = ParkingSensor::new();
        c.failed_count = 2;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ParkingSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
