/// Exhaust gasket: manifold seal, flange, heat shield
/// Phase 580

#[derive(Debug, Clone)]
pub struct ExhaustGasket {
    pub manifold_seal_ok: bool,
    pub flange_ok: bool,
    pub heat_shield_ok: bool,
    pub leak_free: bool,
    pub bolts_ok: bool,
}

impl Default for ExhaustGasket {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustGasket {
    pub fn new() -> Self {
        Self {
            manifold_seal_ok: true,
            flange_ok: true,
            heat_shield_ok: true,
            leak_free: true,
            bolts_ok: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        self.manifold_seal_ok && self.flange_ok && self.leak_free
    }

    pub fn protection_ok(&self) -> bool {
        self.heat_shield_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.protection_ok() && self.bolts_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.manifold_seal_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = ExhaustGasket::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_protection() {
        let c = ExhaustGasket::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ExhaustGasket::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ExhaustGasket::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_leak() {
        let mut c = ExhaustGasket::new();
        c.leak_free = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ExhaustGasket::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
