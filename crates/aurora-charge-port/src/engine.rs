/// Charge port: door actuator, latch, illumination, lock
/// Phase 292

#[derive(Debug, Clone)]
pub struct ChargePort {
    pub door_open: bool,
    pub locked: bool,
    pub connector_inserted: bool,
    pub illuminated: bool,
    pub latch_ok: bool,
    pub actuator_ok: bool,
}

impl Default for ChargePort {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargePort {
    pub fn new() -> Self {
        Self {
            door_open: false,
            locked: true,
            connector_inserted: false,
            illuminated: false,
            latch_ok: true,
            actuator_ok: true,
        }
    }

    pub fn ready_to_charge(&self) -> bool {
        self.door_open && self.connector_inserted && self.latch_ok
    }

    pub fn can_open(&self) -> bool {
        !self.locked && self.actuator_ok
    }

    pub fn can_remove_connector(&self) -> bool {
        !self.locked && self.connector_inserted
    }

    pub fn secure(&self) -> bool {
        self.locked && self.connector_inserted
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok {
            return 20.0;
        }
        if !self.latch_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_ready() {
        let c = ChargePort::new();
        assert!(!c.ready_to_charge());
    }

    #[test]
    fn test_cannot_open() {
        let c = ChargePort::new();
        assert!(!c.can_open());
    }

    #[test]
    fn test_no_remove() {
        let c = ChargePort::new();
        assert!(!c.can_remove_connector());
    }

    #[test]
    fn test_not_secure() {
        let c = ChargePort::new();
        assert!(!c.secure());
    }

    #[test]
    fn test_open() {
        let mut c = ChargePort::new();
        c.locked = false;
        assert!(c.can_open());
    }

    #[test]
    fn test_health() {
        let c = ChargePort::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
