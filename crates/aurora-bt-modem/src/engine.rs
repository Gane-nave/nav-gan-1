/// bt modem: scan, pair, connect, transfer, log
/// Phase 1353

#[derive(Debug, Clone)]
pub struct BtModem {
    pub scan_ok: bool,
    pub pair_ok: bool,
    pub connect_ok: bool,
    pub transfer_ok: bool,
    pub log_ok: bool,
}

impl Default for BtModem {
    fn default() -> Self {
        Self::new()
    }
}

impl BtModem {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            pair_ok: true,
            connect_ok: true,
            transfer_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.pair_ok && self.connect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.transfer_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.pair_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BtModem::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BtModem::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BtModem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BtModem::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BtModem::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BtModem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
