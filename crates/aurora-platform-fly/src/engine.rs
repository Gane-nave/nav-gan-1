/// platform fly: deploy, scale, config, monitor, log
/// Phase 1820

#[derive(Debug, Clone)]
pub struct PlatformFly {
    pub deploy_ok: bool,
    pub scale_ok: bool,
    pub config_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for PlatformFly {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformFly {
    pub fn new() -> Self {
        Self {
            deploy_ok: true,
            scale_ok: true,
            config_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.deploy_ok && self.scale_ok && self.config_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.deploy_ok || !self.scale_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.deploy_ok {
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
        let c = PlatformFly::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PlatformFly::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PlatformFly::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PlatformFly::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PlatformFly::new();
        c.deploy_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PlatformFly::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
