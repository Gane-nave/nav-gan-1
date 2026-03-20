/// LoRaWAN: gateway, device, join, downlink, uplink
/// Phase 985

#[derive(Debug, Clone)]
pub struct LoraWan {
    pub gateway_ok: bool,
    pub device_ok: bool,
    pub join_ok: bool,
    pub downlink_ok: bool,
    pub uplink_ok: bool,
}

impl Default for LoraWan {
    fn default() -> Self {
        Self::new()
    }
}

impl LoraWan {
    pub fn new() -> Self {
        Self {
            gateway_ok: true,
            device_ok: true,
            join_ok: true,
            downlink_ok: true,
            uplink_ok: true,
        }
    }

    pub fn network_ok(&self) -> bool {
        self.gateway_ok && self.device_ok && self.join_ok
    }

    pub fn data_ok(&self) -> bool {
        self.downlink_ok && self.uplink_ok
    }

    pub fn all_ok(&self) -> bool {
        self.network_ok() && self.data_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.gateway_ok || !self.join_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gateway_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network() {
        let c = LoraWan::new();
        assert!(c.network_ok());
    }

    #[test]
    fn test_data() {
        let c = LoraWan::new();
        assert!(c.data_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LoraWan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = LoraWan::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_gateway() {
        let mut c = LoraWan::new();
        c.gateway_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = LoraWan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
