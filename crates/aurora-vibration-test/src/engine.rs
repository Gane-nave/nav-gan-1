/// vibration test: excite, measure, analyze, report, log
/// Phase 1396

#[derive(Debug, Clone)]
pub struct VibrationTest {
    pub excite_ok: bool,
    pub measure_ok: bool,
    pub analyze_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for VibrationTest {
    fn default() -> Self {
        Self::new()
    }
}

impl VibrationTest {
    pub fn new() -> Self {
        Self {
            excite_ok: true,
            measure_ok: true,
            analyze_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.excite_ok && self.measure_ok && self.analyze_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.excite_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.excite_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = VibrationTest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VibrationTest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VibrationTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VibrationTest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VibrationTest::new();
        c.excite_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VibrationTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
