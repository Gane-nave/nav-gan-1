/// Matter bridge: thread, wifi, BLE, commissioning, fabric
/// Phase 987

#[derive(Debug, Clone)]
pub struct MatterBridge {
    pub thread_ok: bool,
    pub wifi_ok: bool,
    pub ble_ok: bool,
    pub commission_ok: bool,
    pub fabric_ok: bool,
}

impl Default for MatterBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl MatterBridge {
    pub fn new() -> Self {
        Self {
            thread_ok: true,
            wifi_ok: true,
            ble_ok: true,
            commission_ok: true,
            fabric_ok: true,
        }
    }

    pub fn connectivity_ok(&self) -> bool {
        self.thread_ok && self.wifi_ok && self.ble_ok
    }

    pub fn setup_ok(&self) -> bool {
        self.commission_ok && self.fabric_ok
    }

    pub fn all_ok(&self) -> bool {
        self.connectivity_ok() && self.setup_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.commission_ok || !self.fabric_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.thread_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity() {
        let c = MatterBridge::new();
        assert!(c.connectivity_ok());
    }

    #[test]
    fn test_setup() {
        let c = MatterBridge::new();
        assert!(c.setup_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MatterBridge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = MatterBridge::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_commission() {
        let mut c = MatterBridge::new();
        c.commission_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = MatterBridge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
