/// ml registry: register, version, stage, promote, log
/// Phase 1473

#[derive(Debug, Clone)]
pub struct MlRegistry {
    pub register_ok: bool,
    pub version_ok: bool,
    pub stage_ok: bool,
    pub promote_ok: bool,
    pub log_ok: bool,
}

impl Default for MlRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MlRegistry {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            version_ok: true,
            stage_ok: true,
            promote_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.version_ok && self.stage_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.promote_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.version_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MlRegistry::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlRegistry::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlRegistry::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlRegistry::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlRegistry::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlRegistry::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
