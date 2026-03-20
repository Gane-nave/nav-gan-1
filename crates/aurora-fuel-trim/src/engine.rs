/// fuel trim: measure, short, long, adapt, log
/// Phase 1381

#[derive(Debug, Clone)]
pub struct FuelTrim {
    pub measure_ok: bool,
    pub short_ok: bool,
    pub long_ok: bool,
    pub adapt_ok: bool,
    pub log_ok: bool,
}

impl Default for FuelTrim {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelTrim {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            short_ok: true,
            long_ok: true,
            adapt_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.short_ok && self.long_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.short_ok
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
        let c = FuelTrim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuelTrim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelTrim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuelTrim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuelTrim::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuelTrim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
