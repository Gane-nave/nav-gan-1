/// accel sensor: measure, filter, calibrate, alert, log
/// Phase 1296

#[derive(Debug, Clone)]
pub struct AccelSensor {
    pub measure_ok: bool,
    pub filter_ok: bool,
    pub calibrate_ok: bool,
    pub alert_ok: bool,
    pub log_ok: bool,
}

impl Default for AccelSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl AccelSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            filter_ok: true,
            calibrate_ok: true,
            alert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.filter_ok && self.calibrate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AccelSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AccelSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AccelSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AccelSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AccelSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AccelSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
