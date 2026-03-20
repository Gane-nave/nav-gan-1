/// Digital key: NFC, BLE, UWB, sharing, backup
/// Phase 887

#[derive(Debug, Clone)]
pub struct DigitalKey {
    pub nfc_ok: bool,
    pub ble_ok: bool,
    pub uwb_ok: bool,
    pub sharing_ok: bool,
    pub backup_ok: bool,
}

impl Default for DigitalKey {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalKey {
    pub fn new() -> Self {
        Self {
            nfc_ok: true,
            ble_ok: true,
            uwb_ok: true,
            sharing_ok: true,
            backup_ok: true,
        }
    }

    pub fn access_ok(&self) -> bool {
        self.nfc_ok && self.ble_ok && self.uwb_ok
    }

    pub fn features_ok(&self) -> bool {
        self.sharing_ok && self.backup_ok
    }

    pub fn all_ok(&self) -> bool {
        self.access_ok() && self.features_ok()
    }

    pub fn needs_pairing(&self) -> bool {
        !self.ble_ok || !self.uwb_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ble_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        let c = DigitalKey::new();
        assert!(c.access_ok());
    }

    #[test]
    fn test_features() {
        let c = DigitalKey::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DigitalKey::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_pairing() {
        let c = DigitalKey::new();
        assert!(!c.needs_pairing());
    }

    #[test]
    fn test_ble() {
        let mut c = DigitalKey::new();
        c.ble_ok = false;
        assert!(c.needs_pairing());
    }

    #[test]
    fn test_health() {
        let c = DigitalKey::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
