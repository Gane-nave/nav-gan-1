/// comply fmvss: assess, test, document, certify, log
/// Phase 1496

#[derive(Debug, Clone)]
pub struct ComplyFmvss {
    pub assess_ok: bool,
    pub test_ok: bool,
    pub document_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyFmvss {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyFmvss {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            test_ok: true,
            document_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.test_ok && self.document_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
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
        let c = ComplyFmvss::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyFmvss::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyFmvss::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyFmvss::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyFmvss::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyFmvss::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
