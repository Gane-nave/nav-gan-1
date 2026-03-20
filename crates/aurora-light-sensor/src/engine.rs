/// light sensor: measure, adapt, dim, bright, log
/// Phase 1293

#[derive(Debug, Clone)]
pub struct LightSensor {
    pub measure_ok: bool,
    pub adapt_ok: bool,
    pub dim_ok: bool,
    pub bright_ok: bool,
    pub log_ok: bool,
}

impl Default for LightSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl LightSensor {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            adapt_ok: true,
            dim_ok: true,
            bright_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.adapt_ok && self.dim_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.bright_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.adapt_ok
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
        let c = LightSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LightSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LightSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LightSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LightSensor::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LightSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
