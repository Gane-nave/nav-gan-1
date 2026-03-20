/// LDAP client: bind, search, modify, add, compare
/// Phase 1061

#[derive(Debug, Clone)]
pub struct LdapClient {
    pub bind_ok: bool,
    pub search_ok: bool,
    pub modify_ok: bool,
    pub add_ok: bool,
    pub compare_ok: bool,
}

impl Default for LdapClient {
    fn default() -> Self {
        Self::new()
    }
}

impl LdapClient {
    pub fn new() -> Self {
        Self {
            bind_ok: true,
            search_ok: true,
            modify_ok: true,
            add_ok: true,
            compare_ok: true,
        }
    }

    pub fn auth_ok(&self) -> bool {
        self.bind_ok && self.search_ok
    }

    pub fn operations_ok(&self) -> bool {
        self.modify_ok && self.add_ok && self.compare_ok
    }

    pub fn all_ok(&self) -> bool {
        self.auth_ok() && self.operations_ok()
    }

    pub fn needs_rebind(&self) -> bool {
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
    fn test_auth() {
        let c = LdapClient::new();
        assert!(c.auth_ok());
    }

    #[test]
    fn test_operations() {
        let c = LdapClient::new();
        assert!(c.operations_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LdapClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebind() {
        let c = LdapClient::new();
        assert!(!c.needs_rebind());
    }

    #[test]
    fn test_bind() {
        let mut c = LdapClient::new();
        c.bind_ok = false;
        assert!(c.needs_rebind());
    }

    #[test]
    fn test_health() {
        let c = LdapClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
