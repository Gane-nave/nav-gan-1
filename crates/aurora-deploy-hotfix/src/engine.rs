/// deploy hotfix: create, deploy, verify, close, log
/// Phase 2125

#[derive(Debug, Clone)]
pub struct DeployHotfix {
    pub create_ok: bool,
    pub deploy_ok: bool,
    pub verify_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployHotfix {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployHotfix {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            deploy_ok: true,
            verify_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.deploy_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.deploy_ok
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
        let c = DeployHotfix::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployHotfix::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployHotfix::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployHotfix::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployHotfix::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployHotfix::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
