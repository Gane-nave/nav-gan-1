/// emc test: radiate, conduct, measure, evaluate, log
/// Phase 1400

#[derive(Debug, Clone)]
pub struct EmcTest {
    pub radiate_ok: bool,
    pub conduct_ok: bool,
    pub measure_ok: bool,
    pub evaluate_ok: bool,
    pub log_ok: bool,
}

impl Default for EmcTest {
    fn default() -> Self {
        Self::new()
    }
}

impl EmcTest {
    pub fn new() -> Self {
        Self {
            radiate_ok: true,
            conduct_ok: true,
            measure_ok: true,
            evaluate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.radiate_ok && self.conduct_ok && self.measure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.evaluate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.radiate_ok || !self.conduct_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.radiate_ok {
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
        let c = EmcTest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EmcTest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EmcTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EmcTest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EmcTest::new();
        c.radiate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EmcTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
