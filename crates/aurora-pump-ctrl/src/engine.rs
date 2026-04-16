/// pump ctrl: prime, run, regulate, monitor, stop
/// Phase 1152

#[derive(Debug, Clone)]
pub struct PumpCtrl {
    pub prime_ok: bool,
    pub run_ok: bool,
    pub regulate_ok: bool,
    pub monitor_ok: bool,
    pub stop_ok: bool,
}

impl Default for PumpCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl PumpCtrl {
    pub fn new() -> Self {
        Self {
            prime_ok: true,
            run_ok: true,
            regulate_ok: true,
            monitor_ok: true,
            stop_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.prime_ok && self.run_ok && self.regulate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.stop_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.prime_ok || !self.run_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.prime_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = PumpCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PumpCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PumpCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PumpCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PumpCtrl::new();
        c.prime_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PumpCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
