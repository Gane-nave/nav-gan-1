/// Brake rotor: thickness, runout, crack, heat spots
/// Phase 654

#[derive(Debug, Clone)]
pub struct BrakeRotor {
    pub thickness_ok: bool,
    pub runout_ok: bool,
    pub cracked: bool,
    pub heat_spots: bool,
    pub surface_ok: bool,
}

impl Default for BrakeRotor {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeRotor {
    pub fn new() -> Self {
        Self {
            thickness_ok: true,
            runout_ok: true,
            cracked: false,
            heat_spots: false,
            surface_ok: true,
        }
    }

    pub fn measurement_ok(&self) -> bool {
        self.thickness_ok && self.runout_ok
    }

    pub fn surface_good(&self) -> bool {
        self.surface_ok && !self.heat_spots
    }

    pub fn all_ok(&self) -> bool {
        self.measurement_ok() && self.surface_good() && !self.cracked
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || !self.thickness_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement() {
        let c = BrakeRotor::new();
        assert!(c.measurement_ok());
    }

    #[test]
    fn test_surface() {
        let c = BrakeRotor::new();
        assert!(c.surface_good());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeRotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeRotor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_crack() {
        let mut c = BrakeRotor::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeRotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
