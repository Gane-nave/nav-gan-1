/// Rollover sensor: gyroscope, angle, threshold, trigger
/// Phase 835

#[derive(Debug, Clone)]
pub struct RolloverSensor {
    pub gyro_ok: bool,
    pub angle_ok: bool,
    pub threshold_ok: bool,
    pub trigger_ok: bool,
    pub calibration_ok: bool,
}

impl Default for RolloverSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl RolloverSensor {
    pub fn new() -> Self {
        Self {
            gyro_ok: true,
            angle_ok: true,
            threshold_ok: true,
            trigger_ok: true,
            calibration_ok: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.gyro_ok && self.angle_ok && self.calibration_ok
    }

    pub fn response_ok(&self) -> bool {
        self.threshold_ok && self.trigger_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.response_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibration_ok || !self.gyro_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gyro_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = RolloverSensor::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_response() {
        let c = RolloverSensor::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RolloverSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibration() {
        let c = RolloverSensor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = RolloverSensor::new();
        c.calibration_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = RolloverSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
