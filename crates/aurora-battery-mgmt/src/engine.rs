/// Battery management: SOC, SOH, cell balance, thermal
/// Phase 716

#[derive(Debug, Clone)]
pub struct BatteryMgmt {
    pub soc_ok: bool,
    pub soh_ok: bool,
    pub balance_ok: bool,
    pub thermal_ok: bool,
    pub comm_ok: bool,
}

impl Default for BatteryMgmt {
    fn default() -> Self {
        Self::new()
    }
}

impl BatteryMgmt {
    pub fn new() -> Self {
        Self {
            soc_ok: true,
            soh_ok: true,
            balance_ok: true,
            thermal_ok: true,
            comm_ok: true,
        }
    }

    pub fn state_ok(&self) -> bool {
        self.soc_ok && self.soh_ok
    }

    pub fn management_ok(&self) -> bool {
        self.balance_ok && self.thermal_ok && self.comm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.state_ok() && self.management_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.soc_ok || !self.balance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.soc_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let c = BatteryMgmt::new();
        assert!(c.state_ok());
    }

    #[test]
    fn test_management() {
        let c = BatteryMgmt::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BatteryMgmt::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BatteryMgmt::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_soc() {
        let mut c = BatteryMgmt::new();
        c.soc_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BatteryMgmt::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
