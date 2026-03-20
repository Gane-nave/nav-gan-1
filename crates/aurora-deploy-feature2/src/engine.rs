/// deploy feature2: create, deploy, test, merge, log
/// Phase 2124

#[derive(Debug, Clone)]
pub struct DeployFeature2 {
    pub create_ok: bool,
    pub deploy_ok: bool,
    pub test_ok: bool,
    pub merge_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployFeature2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployFeature2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            deploy_ok: true,
            test_ok: true,
            merge_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.deploy_ok && self.test_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.merge_ok && self.log_ok
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
        let c = DeployFeature2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployFeature2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployFeature2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployFeature2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployFeature2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployFeature2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
