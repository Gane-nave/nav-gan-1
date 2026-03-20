/// ml deploy: package, upload, serve, monitor, log
/// Phase 1469

#[derive(Debug, Clone)]
pub struct MlDeploy {
    pub package_ok: bool,
    pub upload_ok: bool,
    pub serve_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for MlDeploy {
    fn default() -> Self {
        Self::new()
    }
}

impl MlDeploy {
    pub fn new() -> Self {
        Self {
            package_ok: true,
            upload_ok: true,
            serve_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.package_ok && self.upload_ok && self.serve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.package_ok || !self.upload_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.package_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MlDeploy::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlDeploy::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlDeploy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlDeploy::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlDeploy::new();
        c.package_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlDeploy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
