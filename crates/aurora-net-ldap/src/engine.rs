/// net ldap: connect, bind, search, modify, log
/// Phase 1553

#[derive(Debug, Clone)]
pub struct NetLdap {
    pub connect_ok: bool,
    pub bind_ok: bool,
    pub search_ok: bool,
    pub modify_ok: bool,
    pub log_ok: bool,
}

impl Default for NetLdap {
    fn default() -> Self {
        Self::new()
    }
}

impl NetLdap {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            bind_ok: true,
            search_ok: true,
            modify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.bind_ok && self.search_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.modify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.bind_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = NetLdap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetLdap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetLdap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetLdap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetLdap::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetLdap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
