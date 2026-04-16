/// aurora-cov-function: cov function
/// Phase 2542

#[derive(Debug, Clone)]
pub struct CovFunction {
    pub instrument_ok: bool,
    pub measure_ok: bool,
    pub report_ok: bool,
    pub threshold_ok: bool,
    pub diff_ok: bool,
}

impl Default for CovFunction {
    fn default() -> Self {
        Self::new()
    }
}

impl CovFunction {
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
        let c = CovFunction::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CovFunction::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CovFunction::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CovFunction::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CovFunction::new();
        c.instrument_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CovFunction::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = CovFunction::default();
        assert!(c.all_ok());
    }
}
