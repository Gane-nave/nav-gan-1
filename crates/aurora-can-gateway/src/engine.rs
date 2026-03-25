/// CAN gateway: message routing, filtering, bus isolation
/// Phase 277

#[derive(Debug, Clone)]
pub struct CanGateway {
    pub buses: u8,
    pub messages_per_sec: u32,
    pub filter_count: u16,
    pub error_count: u32,
    pub bus_off: bool,
    pub gateway_ok: bool,
}

impl Default for CanGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl CanGateway {
    pub fn new() -> Self {
        Self {
            buses: 3,
            messages_per_sec: 5000,
            filter_count: 128,
            error_count: 0,
            bus_off: false,
            gateway_ok: true,
        }
    }

    pub fn throughput_ok(&self) -> bool {
        self.messages_per_sec < 10000
    }

    pub fn error_free(&self) -> bool {
        self.error_count == 0
    }

    pub fn bus_healthy(&self) -> bool {
        !self.bus_off && self.gateway_ok
    }

    pub fn overloaded(&self) -> bool {
        self.messages_per_sec > 8000
    }

    pub fn health_score(&self) -> f64 {
        if self.bus_off {
            return 0.0;
        }
        if !self.gateway_ok {
            return 20.0;
        }
        if !self.error_free() {
            return 70.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throughput() {
        let c = CanGateway::new();
        assert!(c.throughput_ok());
    }

    #[test]
    fn test_error_free() {
        let c = CanGateway::new();
        assert!(c.error_free());
    }

    #[test]
    fn test_bus_healthy() {
        let c = CanGateway::new();
        assert!(c.bus_healthy());
    }

    #[test]
    fn test_not_overloaded() {
        let c = CanGateway::new();
        assert!(!c.overloaded());
    }

    #[test]
    fn test_bus_off() {
        let mut c = CanGateway::new();
        c.bus_off = true;
        assert!(!c.bus_healthy());
    }

    #[test]
    fn test_health() {
        let c = CanGateway::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
