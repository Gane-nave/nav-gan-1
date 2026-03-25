/// HomeKit link: accessory, service, characteristic, pair
/// Phase 988

#[derive(Debug, Clone)]
pub struct HomekitLink {
    pub accessory_ok: bool,
    pub service_ok: bool,
    pub char_ok: bool,
    pub pair_ok: bool,
    pub secure_ok: bool,
}

impl Default for HomekitLink {
    fn default() -> Self {
        Self::new()
    }
}

impl HomekitLink {
    pub fn new() -> Self {
        Self {
            accessory_ok: true,
            service_ok: true,
            char_ok: true,
            pair_ok: true,
            secure_ok: true,
        }
    }

    pub fn device_ok(&self) -> bool {
        self.accessory_ok && self.service_ok && self.char_ok
    }

    pub fn auth_ok(&self) -> bool {
        self.pair_ok && self.secure_ok
    }

    pub fn all_ok(&self) -> bool {
        self.device_ok() && self.auth_ok()
    }

    pub fn needs_pair(&self) -> bool {
        !self.pair_ok || !self.secure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pair_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device() {
        let c = HomekitLink::new();
        assert!(c.device_ok());
    }

    #[test]
    fn test_auth() {
        let c = HomekitLink::new();
        assert!(c.auth_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HomekitLink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_pair() {
        let c = HomekitLink::new();
        assert!(!c.needs_pair());
    }

    #[test]
    fn test_pair() {
        let mut c = HomekitLink::new();
        c.pair_ok = false;
        assert!(c.needs_pair());
    }

    #[test]
    fn test_health() {
        let c = HomekitLink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
