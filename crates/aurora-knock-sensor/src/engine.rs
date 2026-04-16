/// knock sensor: detect, retard, filter, adapt, check
/// Phase 1244

#[derive(Debug, Clone)]
pub struct KnockSensor {
    pub detect_ok: bool,
    pub retard_ok: bool,
    pub filter_ok: bool,
    pub adapt_ok: bool,
    pub check_ok: bool,
}

impl Default for KnockSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl KnockSensor {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            retard_ok: true,
            filter_ok: true,
            adapt_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.retard_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.retard_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = KnockSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = KnockSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KnockSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = KnockSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = KnockSensor::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = KnockSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
