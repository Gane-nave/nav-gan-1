/// Wireless charging: coil, alignment, efficiency, foreign obj
/// Phase 855

#[derive(Debug, Clone)]
pub struct WirelessCharge {
    pub coil_ok: bool,
    pub alignment_ok: bool,
    pub efficiency_ok: bool,
    pub fod_ok: bool,
    pub comm_ok: bool,
}

impl Default for WirelessCharge {
    fn default() -> Self {
        Self::new()
    }
}

impl WirelessCharge {
    pub fn new() -> Self {
        Self {
            coil_ok: true,
            alignment_ok: true,
            efficiency_ok: true,
            fod_ok: true,
            comm_ok: true,
        }
    }

    pub fn transfer_ok(&self) -> bool {
        self.coil_ok && self.alignment_ok && self.efficiency_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.fod_ok && self.comm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.transfer_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.coil_ok || !self.alignment_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.coil_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let c = WirelessCharge::new();
        assert!(c.transfer_ok());
    }

    #[test]
    fn test_safety() {
        let c = WirelessCharge::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WirelessCharge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = WirelessCharge::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_coil() {
        let mut c = WirelessCharge::new();
        c.coil_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = WirelessCharge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
