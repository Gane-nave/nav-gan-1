/// battery mgr: charge, discharge, balance, protect, report
/// Phase 1140

#[derive(Debug, Clone)]
pub struct BatteryMgr {
    pub charge_ok: bool,
    pub discharge_ok: bool,
    pub balance_ok: bool,
    pub protect_ok: bool,
    pub report_ok: bool,
}

impl Default for BatteryMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl BatteryMgr {
    pub fn new() -> Self {
        Self {
            charge_ok: true,
            discharge_ok: true,
            balance_ok: true,
            protect_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.charge_ok && self.discharge_ok && self.balance_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.protect_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.charge_ok || !self.discharge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.charge_ok {
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
        let c = BatteryMgr::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BatteryMgr::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BatteryMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BatteryMgr::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BatteryMgr::new();
        c.charge_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BatteryMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
