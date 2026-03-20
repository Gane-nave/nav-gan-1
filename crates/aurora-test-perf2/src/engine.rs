/// test perf2: benchmark, profile, compare, report, log
/// Phase 2097

#[derive(Debug, Clone)]
pub struct TestPerf2 {
    pub benchmark_ok: bool,
    pub profile_ok: bool,
    pub compare_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for TestPerf2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TestPerf2 {
    pub fn new() -> Self {
        Self {
            benchmark_ok: true,
            profile_ok: true,
            compare_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.benchmark_ok && self.profile_ok && self.compare_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.benchmark_ok || !self.profile_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.benchmark_ok {
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
        let c = TestPerf2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestPerf2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestPerf2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestPerf2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestPerf2::new();
        c.benchmark_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestPerf2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
