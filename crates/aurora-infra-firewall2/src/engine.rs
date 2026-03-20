/// infra firewall2: create, update, delete, audit, log
/// Phase 2136

#[derive(Debug, Clone)]
pub struct InfraFirewall2 {
    pub create_ok: bool,
    pub update_ok: bool,
    pub delete_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraFirewall2 {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraFirewall2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            update_ok: true,
            delete_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.update_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.update_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = InfraFirewall2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraFirewall2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraFirewall2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraFirewall2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraFirewall2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraFirewall2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
