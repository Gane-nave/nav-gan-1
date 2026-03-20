/// Dealer network: locate, inventory, service, appointment
/// Phase 972

#[derive(Debug, Clone)]
pub struct DealerNet {
    pub locate_ok: bool,
    pub inventory_ok: bool,
    pub service_ok: bool,
    pub appoint_ok: bool,
    pub rating_ok: bool,
}

impl Default for DealerNet {
    fn default() -> Self {
        Self::new()
    }
}

impl DealerNet {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            inventory_ok: true,
            service_ok: true,
            appoint_ok: true,
            rating_ok: true,
        }
    }

    pub fn search_ok(&self) -> bool {
        self.locate_ok && self.inventory_ok && self.rating_ok
    }

    pub fn booking_ok(&self) -> bool {
        self.service_ok && self.appoint_ok
    }

    pub fn all_ok(&self) -> bool {
        self.search_ok() && self.booking_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.locate_ok || !self.inventory_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.locate_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let c = DealerNet::new();
        assert!(c.search_ok());
    }

    #[test]
    fn test_booking() {
        let c = DealerNet::new();
        assert!(c.booking_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DealerNet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = DealerNet::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_locate() {
        let mut c = DealerNet::new();
        c.locate_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = DealerNet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
