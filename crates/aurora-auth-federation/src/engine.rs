/// auth federation: register, discover, validate, sync, log
/// Phase 2075

#[derive(Debug, Clone)]
pub struct AuthFederation {
    pub register_ok: bool,
    pub discover_ok: bool,
    pub validate_ok: bool,
    pub sync_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthFederation {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthFederation {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            discover_ok: true,
            validate_ok: true,
            sync_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.discover_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sync_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.discover_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = AuthFederation::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthFederation::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthFederation::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthFederation::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthFederation::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthFederation::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
