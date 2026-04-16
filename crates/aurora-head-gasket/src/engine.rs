/// Head gasket: compression, coolant leak, oil leak
/// Phase 579

#[derive(Debug, Clone)]
pub struct HeadGasket {
    pub compression_ok: bool,
    pub coolant_leak: bool,
    pub oil_leak: bool,
    pub fire_ring_ok: bool,
    pub torque_ok: bool,
}

impl Default for HeadGasket {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadGasket {
    pub fn new() -> Self {
        Self {
            compression_ok: true,
            coolant_leak: false,
            oil_leak: false,
            fire_ring_ok: true,
            torque_ok: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        !self.coolant_leak && !self.oil_leak
    }

    pub fn structural_ok(&self) -> bool {
        self.fire_ring_ok && self.torque_ok && self.compression_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.structural_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.coolant_leak || self.oil_leak
    }

    pub fn health_score(&self) -> f64 {
        if self.coolant_leak {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = HeadGasket::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_structural() {
        let c = HeadGasket::new();
        assert!(c.structural_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeadGasket::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = HeadGasket::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_leak() {
        let mut c = HeadGasket::new();
        c.coolant_leak = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = HeadGasket::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
