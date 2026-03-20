/// comply rde: measure, analyze, report, certify, log
/// Phase 1499

#[derive(Debug, Clone)]
pub struct ComplyRde {
    pub measure_ok: bool,
    pub analyze_ok: bool,
    pub report_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyRde {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyRde {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            analyze_ok: true,
            report_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.analyze_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.analyze_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok {
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
        let c = ComplyRde::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyRde::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyRde::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyRde::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyRde::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyRde::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
