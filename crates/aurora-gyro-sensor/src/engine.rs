/// gyro sensor: measure, drift, calibrate, fuse, log
/// Phase 1297

#[derive(Debug, Clone)]
pub struct GyroSensor {
    pub measure_ok: bool,
    pub drift_ok: bool,
    pub calibrate_ok: bool,
    pub fuse_ok: bool,
    pub log_ok: bool,
}

impl Default for GyroSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl GyroSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            drift_ok: true,
            calibrate_ok: true,
            fuse_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.drift_ok && self.calibrate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.fuse_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.drift_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = GyroSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GyroSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GyroSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GyroSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GyroSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GyroSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
