/// OTA update: download, verify, install, rollback, schedule
/// Phase 885

#[derive(Debug, Clone)]
pub struct OtaUpdate {
    pub download_ok: bool,
    pub verify_ok: bool,
    pub install_ok: bool,
    pub rollback_ok: bool,
    pub schedule_ok: bool,
}

impl Default for OtaUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl OtaUpdate {
    pub fn new() -> Self {
        Self {
            download_ok: true,
            verify_ok: true,
            install_ok: true,
            rollback_ok: true,
            schedule_ok: true,
        }
    }

    pub fn delivery_ok(&self) -> bool {
        self.download_ok && self.verify_ok
    }

    pub fn deployment_ok(&self) -> bool {
        self.install_ok && self.rollback_ok && self.schedule_ok
    }

    pub fn all_ok(&self) -> bool {
        self.delivery_ok() && self.deployment_ok()
    }

    pub fn needs_retry(&self) -> bool {
        !self.download_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.verify_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery() {
        let c = OtaUpdate::new();
        assert!(c.delivery_ok());
    }

    #[test]
    fn test_deployment() {
        let c = OtaUpdate::new();
        assert!(c.deployment_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtaUpdate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retry() {
        let c = OtaUpdate::new();
        assert!(!c.needs_retry());
    }

    #[test]
    fn test_verify() {
        let mut c = OtaUpdate::new();
        c.verify_ok = false;
        assert!(c.needs_retry());
    }

    #[test]
    fn test_health() {
        let c = OtaUpdate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
