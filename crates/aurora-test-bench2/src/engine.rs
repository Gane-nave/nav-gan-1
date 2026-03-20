/// test bench: setup, measure, compare, regress, log
/// Phase 1535

#[derive(Debug, Clone)]
pub struct TestBench2 {
    pub setup_ok: bool,
    pub measure_ok: bool,
    pub compare_ok: bool,
    pub regress_ok: bool,
    pub log_ok: bool,
}

impl Default for TestBench2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TestBench2 {
    pub fn new() -> Self {
        Self {
            setup_ok: true,
            measure_ok: true,
            compare_ok: true,
            regress_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.setup_ok && self.measure_ok && self.compare_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.regress_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.setup_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.setup_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TestBench2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestBench2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestBench2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestBench2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestBench2::new();
        c.setup_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestBench2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
