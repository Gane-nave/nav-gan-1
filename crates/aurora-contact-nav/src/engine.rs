/// Contact navigation: address book, quick dial, send ETA
/// Phase 910

#[derive(Debug, Clone)]
pub struct ContactNav {
    pub address_ok: bool,
    pub dial_ok: bool,
    pub eta_ok: bool,
    pub sync_ok: bool,
    pub privacy_ok: bool,
}

impl Default for ContactNav {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactNav {
    pub fn new() -> Self {
        Self {
            address_ok: true,
            dial_ok: true,
            eta_ok: true,
            sync_ok: true,
            privacy_ok: true,
        }
    }

    pub fn contact_ok(&self) -> bool {
        self.address_ok && self.dial_ok && self.sync_ok
    }

    pub fn sharing_ok(&self) -> bool {
        self.eta_ok && self.privacy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.contact_ok() && self.sharing_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.sync_ok || !self.address_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.address_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact() {
        let c = ContactNav::new();
        assert!(c.contact_ok());
    }

    #[test]
    fn test_sharing() {
        let c = ContactNav::new();
        assert!(c.sharing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ContactNav::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = ContactNav::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_sync() {
        let mut c = ContactNav::new();
        c.sync_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = ContactNav::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
