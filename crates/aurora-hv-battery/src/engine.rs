/// HV battery pack: module, contactor, BMS, cooling
/// Phase 722

#[derive(Debug, Clone)]
pub struct HvBattery {
    pub module_ok: bool,
    pub contactor_ok: bool,
    pub bms_ok: bool,
    pub cooling_ok: bool,
    pub isolation_ok: bool,
}

impl Default for HvBattery {
    fn default() -> Self {
        Self::new()
    }
}

impl HvBattery {
    pub fn new() -> Self {
        Self {
            module_ok: true,
            contactor_ok: true,
            bms_ok: true,
            cooling_ok: true,
            isolation_ok: true,
        }
    }

    pub fn cells_ok(&self) -> bool {
        self.module_ok && self.bms_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.contactor_ok && self.isolation_ok && self.cooling_ok
    }

    pub fn all_ok(&self) -> bool {
        self.cells_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.module_ok || !self.isolation_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.module_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cells() {
        let c = HvBattery::new();
        assert!(c.cells_ok());
    }

    #[test]
    fn test_safety() {
        let c = HvBattery::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HvBattery::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HvBattery::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_module() {
        let mut c = HvBattery::new();
        c.module_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HvBattery::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
