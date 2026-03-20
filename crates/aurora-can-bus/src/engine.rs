/// CAN bus: termination, baud rate, error frame, load
/// Phase 727

#[derive(Debug, Clone)]
pub struct CanBus {
    pub termination_ok: bool,
    pub baud_ok: bool,
    pub error_free: bool,
    pub load_ok: bool,
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
            termination_ok: true,
            baud_ok: true,
            error_free: true,
            load_ok: true,
            shielding_ok: true,
        }
    }

    pub fn signal_ok(&self) -> bool {
        self.termination_ok && self.baud_ok && self.shielding_ok
    }

    pub fn health_ok(&self) -> bool {
        self.error_free && self.load_ok
    }

    pub fn all_ok(&self) -> bool {
        self.signal_ok() && self.health_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.termination_ok || !self.error_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.termination_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        let c = CanBus::new();
        assert!(c.signal_ok());
    }

    #[test]
    fn test_health_chk() {
        let c = CanBus::new();
        assert!(c.health_ok());
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
    fn test_termination() {
        let mut c = CanBus::new();
        c.termination_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CanBus::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
