/// super cap: charge, boost, discharge, balance, monitor
/// Phase 1146

#[derive(Debug, Clone)]
pub struct SuperCap {
    pub charge_ok: bool,
    pub boost_ok: bool,
    pub discharge_ok: bool,
    pub balance_ok: bool,
    pub monitor_ok: bool,
}

impl Default for SuperCap {
    fn default() -> Self {
        Self::new()
    }
}

impl SuperCap {
    pub fn new() -> Self {
        Self {
            charge_ok: true,
            boost_ok: true,
            discharge_ok: true,
            balance_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.charge_ok && self.boost_ok && self.discharge_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.balance_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.charge_ok || !self.boost_ok
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
        let c = SuperCap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SuperCap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SuperCap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SuperCap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SuperCap::new();
        c.charge_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SuperCap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
