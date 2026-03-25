/// comply ccpa: disclose, optout, delete, verify, log
/// Phase 1489

#[derive(Debug, Clone)]
pub struct ComplyCcpa {
    pub disclose_ok: bool,
    pub optout_ok: bool,
    pub delete_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyCcpa {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyCcpa {
    pub fn new() -> Self {
        Self {
            disclose_ok: true,
            optout_ok: true,
            delete_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.disclose_ok && self.optout_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.disclose_ok || !self.optout_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.disclose_ok {
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
        let c = ComplyCcpa::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyCcpa::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyCcpa::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyCcpa::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyCcpa::new();
        c.disclose_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyCcpa::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
