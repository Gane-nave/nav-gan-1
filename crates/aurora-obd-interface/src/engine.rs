/// OBD interface: CAN bus, DTC, readiness, protocol
/// Phase 706

#[derive(Debug, Clone)]
pub struct ObdInterface {
    pub can_ok: bool,
    pub dtc_ok: bool,
    pub readiness_ok: bool,
    pub protocol_ok: bool,
    pub connector_ok: bool,
}

impl Default for ObdInterface {
    fn default() -> Self {
        Self::new()
    }
}

impl ObdInterface {
    pub fn new() -> Self {
        Self {
            can_ok: true,
            dtc_ok: true,
            readiness_ok: true,
            protocol_ok: true,
            connector_ok: true,
        }
    }

    pub fn communication_ok(&self) -> bool {
        self.can_ok && self.protocol_ok
    }

    pub fn diagnostic_ok(&self) -> bool {
        self.dtc_ok && self.readiness_ok && self.connector_ok
    }

    pub fn all_ok(&self) -> bool {
        self.communication_ok() && self.diagnostic_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.can_ok || !self.connector_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.can_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication() {
        let c = ObdInterface::new();
        assert!(c.communication_ok());
    }

    #[test]
    fn test_diagnostic() {
        let c = ObdInterface::new();
        assert!(c.diagnostic_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObdInterface::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ObdInterface::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_can() {
        let mut c = ObdInterface::new();
        c.can_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ObdInterface::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
