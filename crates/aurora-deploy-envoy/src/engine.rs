/// deploy envoy: config, route, filter, cluster, log
/// Phase 1605

#[derive(Debug, Clone)]
pub struct DeployEnvoy {
    pub config_ok: bool,
    pub route_ok: bool,
    pub filter_ok: bool,
    pub cluster_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployEnvoy {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployEnvoy {
    pub fn new() -> Self {
        Self {
            config_ok: true,
            route_ok: true,
            filter_ok: true,
            cluster_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.config_ok && self.route_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cluster_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.config_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.config_ok {
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
        let c = DeployEnvoy::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployEnvoy::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployEnvoy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployEnvoy::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployEnvoy::new();
        c.config_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployEnvoy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
