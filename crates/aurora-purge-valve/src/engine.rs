/// EVAP purge valve: solenoid, seal, flow control
/// Phase 601

#[derive(Debug, Clone)]
pub struct PurgeValve {
    pub solenoid_ok: bool,
    pub seal_ok: bool,
    pub flow_ok: bool,
    pub signal_ok: bool,
    pub leak_free: bool,
}

impl Default for PurgeValve {
    fn default() -> Self {
        Self::new()
    }
}

impl PurgeValve {
    pub fn new() -> Self {
        Self {
            solenoid_ok: true,
            seal_ok: true,
            flow_ok: true,
            signal_ok: true,
            leak_free: true,
        }
    }

    pub fn valve_ok(&self) -> bool {
        self.solenoid_ok && self.seal_ok
    }

    pub fn flow_good(&self) -> bool {
        self.flow_ok && self.leak_free
    }

    pub fn all_ok(&self) -> bool {
        self.valve_ok() && self.flow_good() && self.signal_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.solenoid_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.solenoid_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valve() {
        let c = PurgeValve::new();
        assert!(c.valve_ok());
    }

    #[test]
    fn test_flow() {
        let c = PurgeValve::new();
        assert!(c.flow_good());
    }

    #[test]
    fn test_all_ok() {
        let c = PurgeValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = PurgeValve::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_solenoid() {
        let mut c = PurgeValve::new();
        c.solenoid_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = PurgeValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
