/// Ignition module: driver circuit, dwell, trigger
/// Phase 607

#[derive(Debug, Clone)]
pub struct IgnitionModule {
    pub driver_ok: bool,
    pub dwell_ok: bool,
    pub trigger_ok: bool,
    pub signal_ok: bool,
    pub thermal_ok: bool,
}

impl Default for IgnitionModule {
    fn default() -> Self {
        Self::new()
    }
}

impl IgnitionModule {
    pub fn new() -> Self {
        Self {
            driver_ok: true,
            dwell_ok: true,
            trigger_ok: true,
            signal_ok: true,
            thermal_ok: true,
        }
    }

    pub fn circuit_ok(&self) -> bool {
        self.driver_ok && self.dwell_ok
    }

    pub fn timing_ok(&self) -> bool {
        self.trigger_ok && self.signal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.circuit_ok() && self.timing_ok() && self.thermal_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.driver_ok || !self.trigger_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.driver_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let c = IgnitionModule::new();
        assert!(c.circuit_ok());
    }

    #[test]
    fn test_timing() {
        let c = IgnitionModule::new();
        assert!(c.timing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IgnitionModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = IgnitionModule::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_driver() {
        let mut c = IgnitionModule::new();
        c.driver_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = IgnitionModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
