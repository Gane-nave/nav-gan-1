/// deploy feature: flag, enable, disable, cleanup, log
/// Phase 1594

#[derive(Debug, Clone)]
pub struct DeployFeature {
    pub flag_ok: bool,
    pub enable_ok: bool,
    pub disable_ok: bool,
    pub cleanup_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployFeature {
    pub fn new() -> Self {
        Self {
            flag_ok: true,
            enable_ok: true,
            disable_ok: true,
            cleanup_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.flag_ok && self.enable_ok && self.disable_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cleanup_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.flag_ok || !self.enable_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flag_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DeployFeature::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployFeature::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployFeature::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployFeature::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployFeature::new();
        c.flag_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployFeature::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
