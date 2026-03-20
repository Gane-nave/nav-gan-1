/// Charge port: connector, locking pin, cooling, comm
/// Phase 556

#[derive(Debug, Clone)]
pub struct ChargePort {
    pub connector_ok: bool,
    pub lock_pin_ok: bool,
    pub cooling_ok: bool,
    pub comm_ok: bool,
    pub sealed: bool,
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
            lock_pin_ok: true,
            cooling_ok: true,
            comm_ok: true,
            sealed: true,
        }
    }

    pub fn electrical_ok(&self) -> bool {
        self.connector_ok && self.comm_ok
    }

    pub fn mechanical_ok(&self) -> bool {
        self.lock_pin_ok && self.sealed
    }

    pub fn all_ok(&self) -> bool {
        self.electrical_ok() && self.mechanical_ok() && self.cooling_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.connector_ok || !self.lock_pin_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connector_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrical() {
        let c = ChargePort::new();
        assert!(c.electrical_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = ChargePort::new();
        assert!(c.mechanical_ok());
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
