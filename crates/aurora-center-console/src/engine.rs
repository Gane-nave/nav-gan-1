/// Center console: storage, armrest, USB, wireless charge
/// Phase 756

#[derive(Debug, Clone)]
pub struct CenterConsole {
    pub storage_ok: bool,
    pub armrest_ok: bool,
    pub usb_ok: bool,
    pub wireless_ok: bool,
    pub hinge_ok: bool,
}

impl Default for CenterConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl CenterConsole {
    pub fn new() -> Self {
        Self {
            storage_ok: true,
            armrest_ok: true,
            usb_ok: true,
            wireless_ok: true,
            hinge_ok: true,
        }
    }

    pub fn compartment_ok(&self) -> bool {
        self.storage_ok && self.armrest_ok && self.hinge_ok
    }

    pub fn connectivity_ok(&self) -> bool {
        self.usb_ok && self.wireless_ok
    }

    pub fn all_ok(&self) -> bool {
        self.compartment_ok() && self.connectivity_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.usb_ok || !self.hinge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hinge_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compartment() {
        let c = CenterConsole::new();
        assert!(c.compartment_ok());
    }

    #[test]
    fn test_connectivity() {
        let c = CenterConsole::new();
        assert!(c.connectivity_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CenterConsole::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CenterConsole::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_hinge() {
        let mut c = CenterConsole::new();
        c.hinge_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CenterConsole::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
