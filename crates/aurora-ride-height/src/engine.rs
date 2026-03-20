/// ride height: measure, adjust, level, store, report
/// Phase 1211

#[derive(Debug, Clone)]
pub struct RideHeight {
    pub measure_ok: bool,
    pub adjust_ok: bool,
    pub level_ok: bool,
    pub store_ok: bool,
    pub report_ok: bool,
}

impl Default for RideHeight {
    fn default() -> Self {
        Self::new()
    }
}

impl RideHeight {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            adjust_ok: true,
            level_ok: true,
            store_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.adjust_ok && self.level_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.store_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.adjust_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RideHeight::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RideHeight::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RideHeight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RideHeight::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RideHeight::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RideHeight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
