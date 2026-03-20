/// comply wltp: measure, calculate, report, certify, log
/// Phase 1498

#[derive(Debug, Clone)]
pub struct ComplyWltp {
    pub measure_ok: bool,
    pub calculate_ok: bool,
    pub report_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyWltp {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyWltp {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            calculate_ok: true,
            report_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.calculate_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.calculate_ok
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
        let c = ComplyWltp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyWltp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyWltp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyWltp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyWltp::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyWltp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
