/// auth ldap2: bind, search, modify, unbind, log
/// Phase 2063

#[derive(Debug, Clone)]
pub struct AuthLdap2 {
    pub bind_ok: bool,
    pub search_ok: bool,
    pub modify_ok: bool,
    pub unbind_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthLdap2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthLdap2 {
    pub fn new() -> Self {
        Self {
            bind_ok: true,
            search_ok: true,
            modify_ok: true,
            unbind_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.bind_ok && self.search_ok && self.modify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.unbind_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.bind_ok || !self.search_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bind_ok {
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
        let c = AuthLdap2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthLdap2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthLdap2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthLdap2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthLdap2::new();
        c.bind_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthLdap2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
