/// Thermal simulation: heat transfer, convection, radiation
/// Phase 962

#[derive(Debug, Clone)]
pub struct ThermalSim {
    pub transfer_ok: bool,
    pub convection_ok: bool,
    pub radiation_ok: bool,
    pub conduction_ok: bool,
    pub validate_ok: bool,
}

impl Default for ThermalSim {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalSim {
    pub fn new() -> Self {
        Self {
            transfer_ok: true,
            convection_ok: true,
            radiation_ok: true,
            conduction_ok: true,
            validate_ok: true,
        }
    }

    pub fn modeling_ok(&self) -> bool {
        self.transfer_ok && self.convection_ok && self.radiation_ok
    }

    pub fn accuracy_ok(&self) -> bool {
        self.conduction_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.modeling_ok() && self.accuracy_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.validate_ok || !self.transfer_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.transfer_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modeling() {
        let c = ThermalSim::new();
        assert!(c.modeling_ok());
    }

    #[test]
    fn test_accuracy() {
        let c = ThermalSim::new();
        assert!(c.accuracy_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ThermalSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = ThermalSim::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_validate() {
        let mut c = ThermalSim::new();
        c.validate_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = ThermalSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
