/// valid email2: syntax, domain, deliverable, normalize, log
/// Phase 2079

#[derive(Debug, Clone)]
pub struct ValidEmail2 {
    pub syntax_ok: bool,
    pub domain_ok: bool,
    pub deliverable_ok: bool,
    pub normalize_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidEmail2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidEmail2 {
    pub fn new() -> Self {
        Self {
            syntax_ok: true,
            domain_ok: true,
            deliverable_ok: true,
            normalize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.syntax_ok && self.domain_ok && self.deliverable_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.normalize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.syntax_ok || !self.domain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.syntax_ok {
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
        let c = ValidEmail2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidEmail2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidEmail2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidEmail2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidEmail2::new();
        c.syntax_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidEmail2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
