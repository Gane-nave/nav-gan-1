/// Acceleration sensor: MEMS, g-force, crash detection
/// Phase 599

#[derive(Debug, Clone)]
pub struct AccelSensor {
    pub g_force: f64,
    pub mems_ok: bool,
    pub range_ok: bool,
    pub crash_detect_ok: bool,
    pub calibrated: bool,
}

impl Default for AccelSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl AccelSensor {
    pub fn new() -> Self {
        Self {
            g_force: 1.0,
            mems_ok: true,
            range_ok: true,
            crash_detect_ok: true,
            calibrated: true,
        }
    }

    pub fn reading_ok(&self) -> bool {
        self.mems_ok && self.range_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.crash_detect_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.reading_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.mems_ok || !self.crash_detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mems_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let c = AccelSensor::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_safety() {
        let c = AccelSensor::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AccelSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AccelSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_mems() {
        let mut c = AccelSensor::new();
        c.mems_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AccelSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
