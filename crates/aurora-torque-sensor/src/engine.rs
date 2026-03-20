/// Torque sensor: strain gauge, signal, calibration, range
/// Phase 865

#[derive(Debug, Clone)]
pub struct TorqueSensor {
    pub gauge_ok: bool,
    pub signal_ok: bool,
    pub calibration_ok: bool,
    pub range_ok: bool,
    pub drift_free: bool,
}

impl Default for TorqueSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl TorqueSensor {
    pub fn new() -> Self {
        Self {
            gauge_ok: true,
            signal_ok: true,
            calibration_ok: true,
            range_ok: true,
            drift_free: true,
        }
    }

    pub fn measurement_ok(&self) -> bool {
        self.gauge_ok && self.signal_ok && self.range_ok
    }

    pub fn accuracy_ok(&self) -> bool {
        self.calibration_ok && self.drift_free
    }

    pub fn all_ok(&self) -> bool {
        self.measurement_ok() && self.accuracy_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibration_ok || !self.drift_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.gauge_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement() {
        let c = TorqueSensor::new();
        assert!(c.measurement_ok());
    }

    #[test]
    fn test_accuracy() {
        let c = TorqueSensor::new();
        assert!(c.accuracy_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TorqueSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = TorqueSensor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = TorqueSensor::new();
        c.calibration_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = TorqueSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
