/// baro sensor: measure, altitude, trend, calibrate, log
/// Phase 1299

#[derive(Debug, Clone)]
pub struct BaroSensor {
    pub measure_ok: bool,
    pub altitude_ok: bool,
    pub trend_ok: bool,
    pub calibrate_ok: bool,
    pub log_ok: bool,
}

impl Default for BaroSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl BaroSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            altitude_ok: true,
            trend_ok: true,
            calibrate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.altitude_ok && self.trend_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.calibrate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.altitude_ok
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
        let c = BaroSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BaroSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BaroSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BaroSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BaroSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BaroSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
