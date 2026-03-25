/// cloud deploy: package, upload, activate, verify, log
/// Phase 1450

#[derive(Debug, Clone)]
pub struct CloudDeploy {
    pub package_ok: bool,
    pub upload_ok: bool,
    pub activate_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudDeploy {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudDeploy {
    pub fn new() -> Self {
        Self {
            package_ok: true,
            upload_ok: true,
            activate_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.package_ok && self.upload_ok && self.activate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.package_ok || !self.upload_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.package_ok {
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
        let c = CloudDeploy::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudDeploy::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudDeploy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudDeploy::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudDeploy::new();
        c.package_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudDeploy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
