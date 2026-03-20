/// batt manage: charge, discharge, balance, protect, log
/// Phase 1356

#[derive(Debug, Clone)]
pub struct BattManage {
    pub charge_ok: bool,
    pub discharge_ok: bool,
    pub balance_ok: bool,
    pub protect_ok: bool,
    pub log_ok: bool,
}

impl Default for BattManage {
    fn default() -> Self {
        Self::new()
    }
}

impl BattManage {
    pub fn new() -> Self {
        Self {
            charge_ok: true,
            discharge_ok: true,
            balance_ok: true,
            protect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.charge_ok && self.discharge_ok && self.balance_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.protect_ok && self.log_ok
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
        let c = BattManage::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BattManage::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BattManage::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BattManage::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BattManage::new();
        c.charge_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BattManage::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
