/// Charge port: connector, latch, light, communication
/// Phase 686

#[derive(Debug, Clone)]
pub struct ChargePort {
    pub connector_ok: bool,
    pub latch_ok: bool,
    pub light_ok: bool,
    pub comm_ok: bool,
    pub seal_ok: bool,
}

impl Default for ChargePort {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargePort {
    pub fn new() -> Self {
        Self {
            connector_ok: true,
            latch_ok: true,
            light_ok: true,
            comm_ok: true,
            seal_ok: true,
        }
    }

    pub fn connection_ok(&self) -> bool {
        self.connector_ok && self.latch_ok
    }

    pub fn interface_ok(&self) -> bool {
        self.light_ok && self.comm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.connection_ok() && self.interface_ok() && self.seal_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.connector_ok || !self.comm_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connector_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let c = ChargePort::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_interface() {
        let c = ChargePort::new();
        assert!(c.interface_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargePort::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ChargePort::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_connector() {
        let mut c = ChargePort::new();
        c.connector_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ChargePort::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
