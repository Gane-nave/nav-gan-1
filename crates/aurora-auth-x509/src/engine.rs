/// auth x509: verify, extract, validate, chain, log
/// Phase 2066

#[derive(Debug, Clone)]
pub struct AuthX509 {
    pub verify_ok: bool,
    pub extract_ok: bool,
    pub validate_ok: bool,
    pub chain_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthX509 {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthX509 {
    pub fn new() -> Self {
        Self {
            verify_ok: true,
            extract_ok: true,
            validate_ok: true,
            chain_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.verify_ok && self.extract_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.chain_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.verify_ok || !self.extract_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.verify_ok {
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
        let c = AuthX509::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthX509::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthX509::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthX509::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthX509::new();
        c.verify_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthX509::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
