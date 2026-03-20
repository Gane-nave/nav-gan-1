/// Brake line: corrosion, flare, fitting, bracket
/// Phase 666

#[derive(Debug, Clone)]
pub struct BrakeLine {
    pub corrosion_free: bool,
    pub flare_ok: bool,
    pub fitting_ok: bool,
    pub bracket_ok: bool,
    pub leak_free: bool,
}

impl Default for BrakeLine {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeLine {
    pub fn new() -> Self {
        Self {
            corrosion_free: true,
            flare_ok: true,
            fitting_ok: true,
            bracket_ok: true,
            leak_free: true,
        }
    }

    pub fn tube_ok(&self) -> bool {
        self.corrosion_free && self.flare_ok
    }

    pub fn mounting_ok(&self) -> bool {
        self.fitting_ok && self.bracket_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tube_ok() && self.mounting_ok() && self.leak_free
    }

    pub fn needs_replacement(&self) -> bool {
        !self.corrosion_free || !self.flare_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.corrosion_free {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tube() {
        let c = BrakeLine::new();
        assert!(c.tube_ok());
    }

    #[test]
    fn test_mounting() {
        let c = BrakeLine::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeLine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeLine::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_corrosion() {
        let mut c = BrakeLine::new();
        c.corrosion_free = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeLine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
