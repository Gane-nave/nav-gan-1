/// emission test: measure, compare, evaluate, report, log
/// Phase 1380

#[derive(Debug, Clone)]
pub struct EmissionTest {
    pub measure_ok: bool,
    pub compare_ok: bool,
    pub evaluate_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for EmissionTest {
    fn default() -> Self {
        Self::new()
    }
}

impl EmissionTest {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            compare_ok: true,
            evaluate_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.compare_ok && self.evaluate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.compare_ok
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
        let c = EmissionTest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EmissionTest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EmissionTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EmissionTest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EmissionTest::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EmissionTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
