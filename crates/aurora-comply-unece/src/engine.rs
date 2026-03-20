/// comply unece: assess, implement, validate, certify, log
/// Phase 1495

#[derive(Debug, Clone)]
pub struct ComplyUnece {
    pub assess_ok: bool,
    pub implement_ok: bool,
    pub validate_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyUnece {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyUnece {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            implement_ok: true,
            validate_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.implement_ok && self.validate_ok
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
        let c = ComplyUnece::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyUnece::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyUnece::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyUnece::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyUnece::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyUnece::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
