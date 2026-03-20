/// deploy ansible: playbook, role, inventory, vault, log
/// Phase 2116

#[derive(Debug, Clone)]
pub struct DeployAnsible {
    pub playbook_ok: bool,
    pub role_ok: bool,
    pub inventory_ok: bool,
    pub vault_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployAnsible {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployAnsible {
    pub fn new() -> Self {
        Self {
            playbook_ok: true,
            role_ok: true,
            inventory_ok: true,
            vault_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.playbook_ok && self.role_ok && self.inventory_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.vault_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.playbook_ok || !self.role_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.playbook_ok {
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
        let c = DeployAnsible::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployAnsible::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployAnsible::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployAnsible::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployAnsible::new();
        c.playbook_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployAnsible::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
