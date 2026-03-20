/// EV charger port: inlet, lock, LED, temp sensor
/// Phase 852

#[derive(Debug, Clone)]
pub struct EvChargerPort {
    pub inlet_ok: bool,
    pub lock_ok: bool,
    pub led_ok: bool,
    pub temp_ok: bool,
    pub seal_ok: bool,
}

impl Default for EvChargerPort {
    fn default() -> Self {
        Self::new()
    }
}

impl EvChargerPort {
    pub fn new() -> Self {
        Self {
            inlet_ok: true,
            lock_ok: true,
            led_ok: true,
            temp_ok: true,
            seal_ok: true,
        }
    }

    pub fn connection_ok(&self) -> bool {
        self.inlet_ok && self.lock_ok && self.seal_ok
    }

    pub fn indicators_ok(&self) -> bool {
        self.led_ok && self.temp_ok
    }

    pub fn all_ok(&self) -> bool {
        self.connection_ok() && self.indicators_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.inlet_ok || !self.lock_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inlet_ok {
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
        let c = EvChargerPort::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_indicators() {
        let c = EvChargerPort::new();
        assert!(c.indicators_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EvChargerPort::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EvChargerPort::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_inlet() {
        let mut c = EvChargerPort::new();
        c.inlet_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EvChargerPort::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
