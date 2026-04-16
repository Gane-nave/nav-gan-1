/// aurora-cov-line: cov line
/// Phase 2539

#[derive(Debug, Clone)]
pub struct CovLine {
    pub instrument_ok: bool,
    pub measure_ok: bool,
    pub report_ok: bool,
    pub threshold_ok: bool,
    pub diff_ok: bool,
}

impl Default for CovLine {
    fn default() -> Self {
        Self::new()
    }
}

impl CovLine {
    pub fn new() -> Self {
        Self {
            instrument_ok: true,
            measure_ok: true,
            report_ok: true,
            threshold_ok: true,
            diff_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.instrument_ok && self.measure_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.threshold_ok && self.diff_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.instrument_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.instrument_ok {
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
        let c = CovLine::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CovLine::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CovLine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CovLine::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CovLine::new();
        c.instrument_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CovLine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = CovLine::default();
        assert!(c.all_ok());
    }
}
