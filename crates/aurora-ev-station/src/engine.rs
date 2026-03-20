/// EV station finder: location, connector, power, available
/// Phase 925

#[derive(Debug, Clone)]
pub struct EvStation {
    pub location_ok: bool,
    pub connector_ok: bool,
    pub power_ok: bool,
    pub available_ok: bool,
    pub payment_ok: bool,
}

impl Default for EvStation {
    fn default() -> Self {
        Self::new()
    }
}

impl EvStation {
    pub fn new() -> Self {
        Self {
            location_ok: true,
            connector_ok: true,
            power_ok: true,
            available_ok: true,
            payment_ok: true,
        }
    }

    pub fn discovery_ok(&self) -> bool {
        self.location_ok && self.connector_ok && self.power_ok
    }

    pub fn service_ok(&self) -> bool {
        self.available_ok && self.payment_ok
    }

    pub fn all_ok(&self) -> bool {
        self.discovery_ok() && self.service_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.location_ok || !self.available_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.location_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery() {
        let c = EvStation::new();
        assert!(c.discovery_ok());
    }

    #[test]
    fn test_service() {
        let c = EvStation::new();
        assert!(c.service_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EvStation::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = EvStation::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_location() {
        let mut c = EvStation::new();
        c.location_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = EvStation::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
