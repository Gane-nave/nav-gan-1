/// CAN bus: communication, termination, error frames
/// Phase 524

#[derive(Debug, Clone)]
pub struct CanBus {
    pub baud_rate_kbps: u32,
    pub error_count: u32,
    pub termination_ok: bool,
    pub bus_load_pct: f64,
    pub shielding_ok: bool,
}

impl Default for CanBus {
    fn default() -> Self {
        Self::new()
    }
}

impl CanBus {
    pub fn new() -> Self {
        Self {
            baud_rate_kbps: 500,
            error_count: 0,
            termination_ok: true,
            bus_load_pct: 35.0,
            shielding_ok: true,
        }
    }

    pub fn no_errors(&self) -> bool {
        self.error_count == 0
    }

    pub fn load_ok(&self) -> bool {
        self.bus_load_pct < 70.0
    }

    pub fn all_ok(&self) -> bool {
        self.no_errors() && self.load_ok() && self.termination_ok && self.shielding_ok
    }

    pub fn needs_service(&self) -> bool {
        self.error_count > 10 || !self.termination_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.error_count > 10 { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_errors() {
        let c = CanBus::new();
        assert!(c.no_errors());
    }

    #[test]
    fn test_load() {
        let c = CanBus::new();
        assert!(c.load_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CanBus::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CanBus::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_errors() {
        let mut c = CanBus::new();
        c.error_count = 15;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CanBus::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
