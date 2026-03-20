/// deploy artifact: build, sign, store, distribute, log
/// Phase 1597

#[derive(Debug, Clone)]
pub struct DeployArtifact {
    pub build_ok: bool,
    pub sign_ok: bool,
    pub store_ok: bool,
    pub distribute_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployArtifact {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployArtifact {
    pub fn new() -> Self {
        Self {
            build_ok: true,
            sign_ok: true,
            store_ok: true,
            distribute_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.build_ok && self.sign_ok && self.store_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.distribute_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.build_ok || !self.sign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.build_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DeployArtifact::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployArtifact::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployArtifact::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployArtifact::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployArtifact::new();
        c.build_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployArtifact::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
