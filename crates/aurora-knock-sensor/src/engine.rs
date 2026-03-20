/// Knock sensor: piezoelectric, frequency, threshold
/// Phase 587

#[derive(Debug, Clone)]
pub struct KnockSensor {
    pub piezo_ok: bool,
    pub frequency_ok: bool,
    pub threshold_ok: bool,
    pub signal_ok: bool,
    pub calibrated: bool,
}

impl Default for KnockSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl KnockSensor {
    pub fn new() -> Self {
        Self {
            piezo_ok: true,
            frequency_ok: true,
            threshold_ok: true,
            signal_ok: true,
            calibrated: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.piezo_ok && self.frequency_ok
    }

    pub fn system_ok(&self) -> bool {
        self.detection_ok() && self.threshold_ok && self.signal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && self.calibrated
    }

    pub fn needs_replacement(&self) -> bool {
        !self.piezo_ok || !self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.piezo_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = KnockSensor::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_system() {
        let c = KnockSensor::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KnockSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = KnockSensor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_piezo() {
        let mut c = KnockSensor::new();
        c.piezo_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = KnockSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
