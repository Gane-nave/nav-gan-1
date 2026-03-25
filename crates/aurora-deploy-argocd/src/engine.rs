/// deploy argocd: sync, prune, rollback, health, log
/// Phase 2118

#[derive(Debug, Clone)]
pub struct DeployArgocd {
    pub sync_ok: bool,
    pub prune_ok: bool,
    pub rollback_ok: bool,
    pub health_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployArgocd {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployArgocd {
    pub fn new() -> Self {
        Self {
            sync_ok: true,
            prune_ok: true,
            rollback_ok: true,
            health_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sync_ok && self.prune_ok && self.rollback_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.health_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sync_ok || !self.prune_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sync_ok {
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
        let c = DeployArgocd::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployArgocd::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployArgocd::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployArgocd::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployArgocd::new();
        c.sync_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployArgocd::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
