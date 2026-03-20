/// Ignition coil: spark energy, dwell time, resistance
/// Phase 518

#[derive(Debug, Clone)]
pub struct IgnitionCoil {
    pub spark_energy_mj: f64,
    pub min_energy_mj: f64,
    pub primary_ohms: f64,
    pub secondary_kohms: f64,
    pub misfiring: bool,
}

impl Default for IgnitionCoil {
    fn default() -> Self {
        Self::new()
    }
}

impl IgnitionCoil {
    pub fn new() -> Self {
        Self {
            spark_energy_mj: 40.0,
            min_energy_mj: 20.0,
            primary_ohms: 0.5,
            secondary_kohms: 8.0,
            misfiring: false,
        }
    }

    pub fn energy_ok(&self) -> bool {
        self.spark_energy_mj > self.min_energy_mj
    }

    pub fn resistance_ok(&self) -> bool {
        self.primary_ohms > 0.3 && self.primary_ohms < 1.0
    }

    pub fn all_ok(&self) -> bool {
        self.energy_ok() && self.resistance_ok() && !self.misfiring
    }

    pub fn needs_replacement(&self) -> bool {
        self.misfiring || !self.energy_ok()
    }

    pub fn health_score(&self) -> f64 {
        if self.misfiring { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy() {
        let c = IgnitionCoil::new();
        assert!(c.energy_ok());
    }

    #[test]
    fn test_resistance() {
        let c = IgnitionCoil::new();
        assert!(c.resistance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IgnitionCoil::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = IgnitionCoil::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_misfire() {
        let mut c = IgnitionCoil::new();
        c.misfiring = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = IgnitionCoil::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
