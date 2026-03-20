/// propshaft: transmit, balance, joint, phase, check
/// Phase 1219

#[derive(Debug, Clone)]
pub struct Propshaft {
    pub transmit_ok: bool,
    pub balance_ok: bool,
    pub joint_ok: bool,
    pub phase_ok: bool,
    pub check_ok: bool,
}

impl Default for Propshaft {
    fn default() -> Self {
        Self::new()
    }
}

impl Propshaft {
    pub fn new() -> Self {
        Self {
            transmit_ok: true,
            balance_ok: true,
            joint_ok: true,
            phase_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.transmit_ok && self.balance_ok && self.joint_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.phase_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.transmit_ok || !self.balance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.transmit_ok {
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
        let c = Propshaft::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Propshaft::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Propshaft::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Propshaft::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Propshaft::new();
        c.transmit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Propshaft::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
