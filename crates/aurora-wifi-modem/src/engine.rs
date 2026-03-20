/// wifi modem: scan, connect, transfer, roam, log
/// Phase 1352

#[derive(Debug, Clone)]
pub struct WifiModem {
    pub scan_ok: bool,
    pub connect_ok: bool,
    pub transfer_ok: bool,
    pub roam_ok: bool,
    pub log_ok: bool,
}

impl Default for WifiModem {
    fn default() -> Self {
        Self::new()
    }
}

impl WifiModem {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            connect_ok: true,
            transfer_ok: true,
            roam_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.connect_ok && self.transfer_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.roam_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.connect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok {
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
        let c = WifiModem::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WifiModem::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WifiModem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WifiModem::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WifiModem::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WifiModem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
