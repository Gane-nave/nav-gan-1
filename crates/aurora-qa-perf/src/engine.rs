/// aurora-qa-perf: qa perf
/// Phase 2516

#[derive(Debug, Clone)]
pub struct QaPerf {
    pub run_ok: bool,
    pub measure_ok: bool,
    pub baseline_ok: bool,
    pub compare_ok: bool,
    pub report_ok: bool,
}

impl Default for QaPerf {
    fn default() -> Self {
        Self::new()
    }
}

impl QaPerf {
    pub fn new() -> Self {
        Self {
            run_ok: true,
            measure_ok: true,
            baseline_ok: true,
            compare_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.run_ok && self.measure_ok && self.baseline_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compare_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.run_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.run_ok {
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
        let c = QaPerf::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QaPerf::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QaPerf::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QaPerf::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QaPerf::new();
        c.run_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QaPerf::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = QaPerf::default();
        assert!(c.all_ok());
    }
}
