/// Adhesive bonding: structural adhesive, crash adhesive, peel strength
/// Phase 398

#[derive(Debug, Clone)]
pub struct AdhesiveBond {
    pub peel_n_mm: f64,
    pub min_peel_n_mm: f64,
    pub shear_mpa: f64,
    pub cured: bool,
    pub coverage_pct: f64,
}

impl Default for AdhesiveBond {
    fn default() -> Self {
        Self::new()
    }
}

impl AdhesiveBond {
    pub fn new() -> Self {
        Self {
            peel_n_mm: 12.0,
            min_peel_n_mm: 8.0,
            shear_mpa: 25.0,
            cured: true,
            coverage_pct: 95.0,
        }
    }

    pub fn peel_ok(&self) -> bool {
        self.peel_n_mm >= self.min_peel_n_mm
    }

    pub fn strength_ok(&self) -> bool {
        self.shear_mpa > 15.0
    }

    pub fn effective(&self) -> bool {
        self.peel_ok() && self.cured && self.coverage_pct > 90.0
    }

    pub fn needs_rebond(&self) -> bool {
        !self.cured || self.coverage_pct < 70.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.cured {
            return 0.0;
        }
        if !self.peel_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peel() {
        let a = AdhesiveBond::new();
        assert!(a.peel_ok());
    }

    #[test]
    fn test_strength() {
        let a = AdhesiveBond::new();
        assert!(a.strength_ok());
    }

    #[test]
    fn test_effective() {
        let a = AdhesiveBond::new();
        assert!(a.effective());
    }

    #[test]
    fn test_no_rebond() {
        let a = AdhesiveBond::new();
        assert!(!a.needs_rebond());
    }

    #[test]
    fn test_uncured() {
        let mut a = AdhesiveBond::new();
        a.cured = false;
        assert!(a.needs_rebond());
    }

    #[test]
    fn test_health() {
        let a = AdhesiveBond::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
