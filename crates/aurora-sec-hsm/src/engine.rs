/// sec hsm: init, sign, verify, wrap, log
/// Phase 2049

#[derive(Debug, Clone)]
pub struct SecHsm {
    pub init_ok: bool,
    pub sign_ok: bool,
    pub verify_ok: bool,
    pub wrap_ok: bool,
    pub log_ok: bool,
}

impl Default for SecHsm {
    fn default() -> Self {
        Self::new()
    }
}

impl SecHsm {
    pub fn new() -> Self {
        Self {
            init_ok: true,
            sign_ok: true,
            verify_ok: true,
            wrap_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.init_ok && self.sign_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.wrap_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.init_ok || !self.sign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.init_ok {
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
        let c = SecHsm::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecHsm::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecHsm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecHsm::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecHsm::new();
        c.init_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecHsm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
