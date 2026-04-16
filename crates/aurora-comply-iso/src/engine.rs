/// comply iso: assess, implement, audit, certify, log
/// Phase 1490

#[derive(Debug, Clone)]
pub struct ComplyIso {
    pub assess_ok: bool,
    pub implement_ok: bool,
    pub audit_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyIso {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyIso {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            implement_ok: true,
            audit_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.implement_ok && self.audit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assess_ok || !self.implement_ok
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
        let c = ComplyIso::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyIso::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyIso::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyIso::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyIso::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyIso::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
