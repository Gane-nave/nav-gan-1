/// endurance test: cycle, monitor, measure, report, log
/// Phase 1398

#[derive(Debug, Clone)]
pub struct EnduranceTest {
    pub cycle_ok: bool,
    pub monitor_ok: bool,
    pub measure_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for EnduranceTest {
    fn default() -> Self {
        Self::new()
    }
}

impl EnduranceTest {
    pub fn new() -> Self {
        Self {
            cycle_ok: true,
            monitor_ok: true,
            measure_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.cycle_ok && self.monitor_ok && self.measure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.cycle_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cycle_ok {
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
        let c = EnduranceTest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EnduranceTest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EnduranceTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EnduranceTest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EnduranceTest::new();
        c.cycle_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EnduranceTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
