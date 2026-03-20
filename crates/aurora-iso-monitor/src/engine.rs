/// Isolation monitor: resistance, leakage, Y-cap, fault
/// Phase 861

#[derive(Debug, Clone)]
pub struct IsoMonitor {
    pub resistance_ok: bool,
    pub leakage_ok: bool,
    pub y_cap_ok: bool,
    pub fault_ok: bool,
    pub calibration_ok: bool,
}

impl Default for IsoMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl IsoMonitor {
    pub fn new() -> Self {
        Self {
            resistance_ok: true,
            leakage_ok: true,
            y_cap_ok: true,
            fault_ok: true,
            calibration_ok: true,
        }
    }

    pub fn isolation_ok(&self) -> bool {
        self.resistance_ok && self.leakage_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.y_cap_ok && self.fault_ok && self.calibration_ok
    }

    pub fn all_ok(&self) -> bool {
        self.isolation_ok() && self.protection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.resistance_ok || !self.leakage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.resistance_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolation() {
        let c = IsoMonitor::new();
        assert!(c.isolation_ok());
    }

    #[test]
    fn test_protection() {
        let c = IsoMonitor::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IsoMonitor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = IsoMonitor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_resistance() {
        let mut c = IsoMonitor::new();
        c.resistance_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = IsoMonitor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
