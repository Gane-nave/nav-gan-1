/// Zigbee hub: coordinator, router, end-device, mesh, bind
/// Phase 986

#[derive(Debug, Clone)]
pub struct ZigbeeHub {
    pub coord_ok: bool,
    pub router_ok: bool,
    pub end_dev_ok: bool,
    pub mesh_ok: bool,
    pub bind_ok: bool,
}

impl Default for ZigbeeHub {
    fn default() -> Self {
        Self::new()
    }
}

impl ZigbeeHub {
    pub fn new() -> Self {
        Self {
            coord_ok: true,
            router_ok: true,
            end_dev_ok: true,
            mesh_ok: true,
            bind_ok: true,
        }
    }

    pub fn network_ok(&self) -> bool {
        self.coord_ok && self.router_ok && self.mesh_ok
    }

    pub fn device_ok(&self) -> bool {
        self.end_dev_ok && self.bind_ok
    }

    pub fn all_ok(&self) -> bool {
        self.network_ok() && self.device_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.coord_ok || !self.mesh_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.coord_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network() {
        let c = ZigbeeHub::new();
        assert!(c.network_ok());
    }

    #[test]
    fn test_device() {
        let c = ZigbeeHub::new();
        assert!(c.device_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ZigbeeHub::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = ZigbeeHub::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_coord() {
        let mut c = ZigbeeHub::new();
        c.coord_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = ZigbeeHub::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
