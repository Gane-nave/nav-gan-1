/// Voltage monitor: cell, module, pack, fault
/// Phase 867

#[derive(Debug, Clone)]
pub struct VoltageMonitor {
    pub cell_ok: bool,
    pub module_ok: bool,
    pub pack_ok: bool,
    pub fault_ok: bool,
    pub accuracy_ok: bool,
}

impl Default for VoltageMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl VoltageMonitor {
    pub fn new() -> Self {
        Self {
            cell_ok: true,
            module_ok: true,
            pack_ok: true,
            fault_ok: true,
            accuracy_ok: true,
        }
    }

    pub fn monitoring_ok(&self) -> bool {
        self.cell_ok && self.module_ok && self.pack_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.fault_ok && self.accuracy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.monitoring_ok() && self.safety_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.accuracy_ok || !self.cell_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cell_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring() {
        let c = VoltageMonitor::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_safety() {
        let c = VoltageMonitor::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VoltageMonitor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = VoltageMonitor::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_accuracy() {
        let mut c = VoltageMonitor::new();
        c.accuracy_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = VoltageMonitor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
