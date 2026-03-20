/// evap test: seal, pressurize, measure, evaluate, log
/// Phase 1384

#[derive(Debug, Clone)]
pub struct EvapTest {
    pub seal_ok: bool,
    pub pressurize_ok: bool,
    pub measure_ok: bool,
    pub evaluate_ok: bool,
    pub log_ok: bool,
}

impl Default for EvapTest {
    fn default() -> Self {
        Self::new()
    }
}

impl EvapTest {
    pub fn new() -> Self {
        Self {
            seal_ok: true,
            pressurize_ok: true,
            measure_ok: true,
            evaluate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.seal_ok && self.pressurize_ok && self.measure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.evaluate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.seal_ok || !self.pressurize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.seal_ok {
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
        let c = EvapTest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EvapTest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EvapTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EvapTest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EvapTest::new();
        c.seal_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EvapTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
