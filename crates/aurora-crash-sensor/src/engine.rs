/// Crash sensor: accelerometer, threshold, deployment, reset
/// Phase 834

#[derive(Debug, Clone)]
pub struct CrashSensor {
    pub accel_ok: bool,
    pub threshold_ok: bool,
    pub deploy_ok: bool,
    pub reset_ok: bool,
    pub calibration_ok: bool,
}

impl Default for CrashSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashSensor {
    pub fn new() -> Self {
        Self {
            accel_ok: true,
            threshold_ok: true,
            deploy_ok: true,
            reset_ok: true,
            calibration_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.accel_ok && self.threshold_ok && self.calibration_ok
    }

    pub fn response_ok(&self) -> bool {
        self.deploy_ok && self.reset_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.response_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibration_ok || !self.threshold_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.accel_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = CrashSensor::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_response() {
        let c = CrashSensor::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CrashSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibration() {
        let c = CrashSensor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = CrashSensor::new();
        c.calibration_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = CrashSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
