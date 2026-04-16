/// auth scope: define, check, grant, revoke, log
/// Phase 2070

#[derive(Debug, Clone)]
pub struct AuthScope {
    pub define_ok: bool,
    pub check_ok: bool,
    pub grant_ok: bool,
    pub revoke_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthScope {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthScope {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            check_ok: true,
            grant_ok: true,
            revoke_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.check_ok && self.grant_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.revoke_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = AuthScope::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthScope::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthScope::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthScope::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthScope::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthScope::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
