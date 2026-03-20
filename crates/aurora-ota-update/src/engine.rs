/// ota update: check, download, verify, install, rollback
/// Phase 1133

#[derive(Debug, Clone)]
pub struct OtaUpdate {
    pub check_ok: bool,
    pub download_ok: bool,
    pub verify_ok: bool,
    pub install_ok: bool,
    pub rollback_ok: bool,
}

impl Default for OtaUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl OtaUpdate {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            download_ok: true,
            verify_ok: true,
            install_ok: true,
            rollback_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.download_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.install_ok && self.rollback_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.download_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = OtaUpdate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtaUpdate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtaUpdate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtaUpdate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtaUpdate::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtaUpdate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
