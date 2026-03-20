/// comply pci: scope, assess, remediate, certify, log
/// Phase 1492

#[derive(Debug, Clone)]
pub struct ComplyPci {
    pub scope_ok: bool,
    pub assess_ok: bool,
    pub remediate_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyPci {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyPci {
    pub fn new() -> Self {
        Self {
            scope_ok: true,
            assess_ok: true,
            remediate_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scope_ok && self.assess_ok && self.remediate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scope_ok || !self.assess_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scope_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ComplyPci::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyPci::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyPci::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyPci::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyPci::new();
        c.scope_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyPci::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
