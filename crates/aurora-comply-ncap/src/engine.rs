/// comply ncap: assess, test, score, report, log
/// Phase 1497

#[derive(Debug, Clone)]
pub struct ComplyNcap {
    pub assess_ok: bool,
    pub test_ok: bool,
    pub score_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyNcap {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyNcap {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            test_ok: true,
            score_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.test_ok && self.score_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assess_ok || !self.test_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assess_ok {
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
        let c = ComplyNcap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyNcap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyNcap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyNcap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyNcap::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyNcap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
