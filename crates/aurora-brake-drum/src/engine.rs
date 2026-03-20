/// Brake drum: diameter, roundness, crack, scoring
/// Phase 655

#[derive(Debug, Clone)]
pub struct BrakeDrum {
    pub diameter_ok: bool,
    pub roundness_ok: bool,
    pub cracked: bool,
    pub scored: bool,
    pub lip_ok: bool,
}

impl Default for BrakeDrum {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeDrum {
    pub fn new() -> Self {
        Self {
            diameter_ok: true,
            roundness_ok: true,
            cracked: false,
            scored: false,
            lip_ok: true,
        }
    }

    pub fn geometry_ok(&self) -> bool {
        self.diameter_ok && self.roundness_ok
    }

    pub fn surface_good(&self) -> bool {
        !self.scored && self.lip_ok
    }

    pub fn all_ok(&self) -> bool {
        self.geometry_ok() && self.surface_good() && !self.cracked
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || !self.diameter_ok
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
    fn test_geometry() {
        let c = BrakeDrum::new();
        assert!(c.geometry_ok());
    }

    #[test]
    fn test_surface() {
        let c = BrakeDrum::new();
        assert!(c.surface_good());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeDrum::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeDrum::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_crack() {
        let mut c = BrakeDrum::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeDrum::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
