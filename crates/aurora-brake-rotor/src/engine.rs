/// Brake rotor: thickness, runout, heat cracks
/// Phase 485

#[derive(Debug, Clone)]
pub struct BrakeRotor {
    pub thickness_mm: f64,
    pub min_thickness_mm: f64,
    pub runout_mm: f64,
    pub max_runout_mm: f64,
    pub cracked: bool,
}

impl Default for BrakeRotor {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeRotor {
    pub fn new() -> Self {
        Self {
            thickness_mm: 28.0,
            min_thickness_mm: 22.0,
            runout_mm: 0.02,
            max_runout_mm: 0.05,
            cracked: false,
        }
    }

    pub fn thickness_ok(&self) -> bool {
        self.thickness_mm > self.min_thickness_mm
    }

    pub fn runout_ok(&self) -> bool {
        self.runout_mm < self.max_runout_mm
    }

    pub fn all_ok(&self) -> bool {
        self.thickness_ok() && self.runout_ok() && !self.cracked
    }

    pub fn needs_replacement(&self) -> bool {
        !self.thickness_ok() || self.cracked
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
    fn test_thickness() {
        let c = BrakeRotor::new();
        assert!(c.thickness_ok());
    }

    #[test]
    fn test_runout() {
        let c = BrakeRotor::new();
        assert!(c.runout_ok());
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
    fn test_cracked() {
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
