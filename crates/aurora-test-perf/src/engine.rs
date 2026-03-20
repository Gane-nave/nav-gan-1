/// test perf: baseline, run, measure, compare, log
/// Phase 1521

#[derive(Debug, Clone)]
pub struct TestPerf {
    pub baseline_ok: bool,
    pub run_ok: bool,
    pub measure_ok: bool,
    pub compare_ok: bool,
    pub log_ok: bool,
}

impl Default for TestPerf {
    fn default() -> Self {
        Self::new()
    }
}

impl TestPerf {
    pub fn new() -> Self {
        Self {
            baseline_ok: true,
            run_ok: true,
            measure_ok: true,
            compare_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.baseline_ok && self.run_ok && self.measure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compare_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.baseline_ok || !self.run_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.baseline_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TestPerf::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestPerf::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestPerf::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestPerf::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestPerf::new();
        c.baseline_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestPerf::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
