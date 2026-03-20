/// mag sensor: measure, heading, calibrate, filter, log
/// Phase 1298

#[derive(Debug, Clone)]
pub struct MagSensor {
    pub measure_ok: bool,
    pub heading_ok: bool,
    pub calibrate_ok: bool,
    pub filter_ok: bool,
    pub log_ok: bool,
}

impl Default for MagSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl MagSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            heading_ok: true,
            calibrate_ok: true,
            filter_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.heading_ok && self.calibrate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.filter_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.heading_ok
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
        let c = MagSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MagSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MagSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MagSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MagSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MagSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
