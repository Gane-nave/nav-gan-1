/// aurora-assert-sensor: assert sensor
/// Phase 2507

#[derive(Debug, Clone)]
pub struct AssertSensor {
    pub value_ok: bool,
    pub range_ok: bool,
    pub rate_ok: bool,
    pub drift_ok: bool,
    pub outlier_ok: bool,
}

impl Default for AssertSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertSensor {
    pub fn new() -> Self {
        Self {
            value_ok: true,
            range_ok: true,
            rate_ok: true,
            drift_ok: true,
            outlier_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.value_ok && self.range_ok && self.rate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.drift_ok && self.outlier_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.value_ok || !self.range_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.value_ok {
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
        let c = AssertSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertSensor::new();
        c.value_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertSensor::default();
        assert!(c.all_ok());
    }
}
