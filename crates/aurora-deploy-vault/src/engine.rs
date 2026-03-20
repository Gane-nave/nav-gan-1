/// deploy vault: seal, unseal, read, write, log
/// Phase 1607

#[derive(Debug, Clone)]
pub struct DeployVault {
    pub seal_ok: bool,
    pub unseal_ok: bool,
    pub read_ok: bool,
    pub write_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployVault {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployVault {
    pub fn new() -> Self {
        Self {
            seal_ok: true,
            unseal_ok: true,
            read_ok: true,
            write_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.seal_ok && self.unseal_ok && self.read_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.write_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.seal_ok || !self.unseal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.seal_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DeployVault::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployVault::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployVault::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployVault::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployVault::new();
        c.seal_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployVault::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
