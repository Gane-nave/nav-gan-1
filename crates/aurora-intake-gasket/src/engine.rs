/// Intake gasket: vacuum seal, coolant passage, oil seal
/// Phase 581

#[derive(Debug, Clone)]
pub struct IntakeGasket {
    pub vacuum_seal_ok: bool,
    pub coolant_pass_ok: bool,
    pub oil_seal_ok: bool,
    pub surface_ok: bool,
    pub torque_ok: bool,
}

impl Default for IntakeGasket {
    fn default() -> Self {
        Self::new()
    }
}

impl IntakeGasket {
    pub fn new() -> Self {
        Self {
            vacuum_seal_ok: true,
            coolant_pass_ok: true,
            oil_seal_ok: true,
            surface_ok: true,
            torque_ok: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        self.vacuum_seal_ok && self.coolant_pass_ok && self.oil_seal_ok
    }

    pub fn mounting_ok(&self) -> bool {
        self.surface_ok && self.torque_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.mounting_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.vacuum_seal_ok || !self.coolant_pass_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.vacuum_seal_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = IntakeGasket::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_mounting() {
        let c = IntakeGasket::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntakeGasket::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = IntakeGasket::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_vacuum() {
        let mut c = IntakeGasket::new();
        c.vacuum_seal_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = IntakeGasket::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
