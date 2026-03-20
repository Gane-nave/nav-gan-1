/// Yaw rate sensor: gyroscope, drift, calibration
/// Phase 598

#[derive(Debug, Clone)]
pub struct YawSensor {
    pub yaw_rate_dps: f64,
    pub gyro_ok: bool,
    pub drift_ok: bool,
    pub calibrated: bool,
    pub signal_ok: bool,
}

impl Default for YawSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl YawSensor {
    pub fn new() -> Self {
        Self {
            yaw_rate_dps: 0.0,
            gyro_ok: true,
            drift_ok: true,
            calibrated: true,
            signal_ok: true,
        }
    }

    pub fn reading_ok(&self) -> bool {
        self.gyro_ok && self.signal_ok
    }

    pub fn stability_ok(&self) -> bool {
        self.drift_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.reading_ok() && self.stability_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.drift_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gyro_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let c = YawSensor::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_stability() {
        let c = YawSensor::new();
        assert!(c.stability_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = YawSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calib() {
        let c = YawSensor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_drift() {
        let mut c = YawSensor::new();
        c.drift_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = YawSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
