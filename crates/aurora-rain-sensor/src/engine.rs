/// rain sensor: detect, measure, adapt, wipe, log
/// Phase 1292

#[derive(Debug, Clone)]
pub struct RainSensor {
    pub detect_ok: bool,
    pub measure_ok: bool,
    pub adapt_ok: bool,
    pub wipe_ok: bool,
    pub log_ok: bool,
}

impl Default for RainSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl RainSensor {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            measure_ok: true,
            adapt_ok: true,
            wipe_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.measure_ok && self.adapt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.wipe_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RainSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RainSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RainSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RainSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RainSensor::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RainSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
