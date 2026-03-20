/// HV interlock: connector, loop, monitoring, fault
/// Phase 860

#[derive(Debug, Clone)]
pub struct HvInterlock {
    pub connector_ok: bool,
    pub loop_ok: bool,
    pub monitor_ok: bool,
    pub fault_ok: bool,
    pub seal_ok: bool,
}

impl Default for HvInterlock {
    fn default() -> Self {
        Self::new()
    }
}

impl HvInterlock {
    pub fn new() -> Self {
        Self {
            connector_ok: true,
            loop_ok: true,
            monitor_ok: true,
            fault_ok: true,
            seal_ok: true,
        }
    }

    pub fn circuit_ok(&self) -> bool {
        self.connector_ok && self.loop_ok && self.seal_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.monitor_ok && self.fault_ok
    }

    pub fn all_ok(&self) -> bool {
        self.circuit_ok() && self.protection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.loop_ok || !self.connector_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.loop_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let c = HvInterlock::new();
        assert!(c.circuit_ok());
    }

    #[test]
    fn test_protection() {
        let c = HvInterlock::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HvInterlock::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HvInterlock::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_loop() {
        let mut c = HvInterlock::new();
        c.loop_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HvInterlock::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
