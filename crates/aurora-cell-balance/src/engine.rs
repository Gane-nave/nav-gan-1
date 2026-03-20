/// Cell balancing: passive, active, monitoring, SOC
/// Phase 858

#[derive(Debug, Clone)]
pub struct CellBalance {
    pub passive_ok: bool,
    pub active_ok: bool,
    pub monitor_ok: bool,
    pub soc_ok: bool,
    pub voltage_ok: bool,
}

impl Default for CellBalance {
    fn default() -> Self {
        Self::new()
    }
}

impl CellBalance {
    pub fn new() -> Self {
        Self {
            passive_ok: true,
            active_ok: true,
            monitor_ok: true,
            soc_ok: true,
            voltage_ok: true,
        }
    }

    pub fn balancing_ok(&self) -> bool {
        self.passive_ok && self.active_ok
    }

    pub fn tracking_ok(&self) -> bool {
        self.monitor_ok && self.soc_ok && self.voltage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.balancing_ok() && self.tracking_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.passive_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.passive_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balancing() {
        let c = CellBalance::new();
        assert!(c.balancing_ok());
    }

    #[test]
    fn test_tracking() {
        let c = CellBalance::new();
        assert!(c.tracking_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CellBalance::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CellBalance::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_passive() {
        let mut c = CellBalance::new();
        c.passive_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CellBalance::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
