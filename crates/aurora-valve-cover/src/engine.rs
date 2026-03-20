/// Valve cover: gasket, PCV, breather, bolts
/// Phase 577

#[derive(Debug, Clone)]
pub struct ValveCover {
    pub gasket_ok: bool,
    pub pcv_ok: bool,
    pub breather_ok: bool,
    pub bolts_ok: bool,
    pub leak_free: bool,
}

impl Default for ValveCover {
    fn default() -> Self {
        Self::new()
    }
}

impl ValveCover {
    pub fn new() -> Self {
        Self {
            gasket_ok: true,
            pcv_ok: true,
            breather_ok: true,
            bolts_ok: true,
            leak_free: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        self.gasket_ok && self.leak_free
    }

    pub fn ventilation_ok(&self) -> bool {
        self.pcv_ok && self.breather_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.ventilation_ok() && self.bolts_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.gasket_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.gasket_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = ValveCover::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_ventilation() {
        let c = ValveCover::new();
        assert!(c.ventilation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValveCover::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ValveCover::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_gasket() {
        let mut c = ValveCover::new();
        c.gasket_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ValveCover::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
