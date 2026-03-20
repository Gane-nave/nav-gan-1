/// Hub cap: retention, finish, center logo, ventilation
/// Phase 809

#[derive(Debug, Clone)]
pub struct HubCap {
    pub retention_ok: bool,
    pub finish_ok: bool,
    pub logo_ok: bool,
    pub vent_ok: bool,
    pub fit_ok: bool,
}

impl Default for HubCap {
    fn default() -> Self {
        Self::new()
    }
}

impl HubCap {
    pub fn new() -> Self {
        Self {
            retention_ok: true,
            finish_ok: true,
            logo_ok: true,
            vent_ok: true,
            fit_ok: true,
        }
    }

    pub fn attachment_ok(&self) -> bool {
        self.retention_ok && self.fit_ok
    }

    pub fn appearance_ok(&self) -> bool {
        self.finish_ok && self.logo_ok && self.vent_ok
    }

    pub fn all_ok(&self) -> bool {
        self.attachment_ok() && self.appearance_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.retention_ok || !self.finish_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.retention_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment() {
        let c = HubCap::new();
        assert!(c.attachment_ok());
    }

    #[test]
    fn test_appearance() {
        let c = HubCap::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HubCap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = HubCap::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_retention() {
        let mut c = HubCap::new();
        c.retention_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = HubCap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
