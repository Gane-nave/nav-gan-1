/// Sealant monitoring: seam sealant, underbody sealant, cavity wax
/// Phase 399

#[derive(Debug, Clone)]
pub struct SealantMon {
    pub coverage_pct: f64,
    pub adhesion_ok: bool,
    pub cured: bool,
    pub cracked: bool,
    pub age_years: f64,
}

impl Default for SealantMon {
    fn default() -> Self {
        Self::new()
    }
}

impl SealantMon {
    pub fn new() -> Self {
        Self {
            coverage_pct: 98.0,
            adhesion_ok: true,
            cured: true,
            cracked: false,
            age_years: 1.0,
        }
    }

    pub fn effective(&self) -> bool {
        self.coverage_pct > 90.0 && self.adhesion_ok && !self.cracked
    }

    pub fn watertight(&self) -> bool {
        self.coverage_pct > 95.0 && !self.cracked
    }

    pub fn needs_reseal(&self) -> bool {
        self.cracked || self.coverage_pct < 80.0
    }

    pub fn aging(&self) -> bool {
        self.age_years > 10.0
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 10.0;
        }
        if !self.adhesion_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective() {
        let s = SealantMon::new();
        assert!(s.effective());
    }

    #[test]
    fn test_watertight() {
        let s = SealantMon::new();
        assert!(s.watertight());
    }

    #[test]
    fn test_no_reseal() {
        let s = SealantMon::new();
        assert!(!s.needs_reseal());
    }

    #[test]
    fn test_not_aging() {
        let s = SealantMon::new();
        assert!(!s.aging());
    }

    #[test]
    fn test_cracked() {
        let mut s = SealantMon::new();
        s.cracked = true;
        assert!(s.needs_reseal());
    }

    #[test]
    fn test_health() {
        let s = SealantMon::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
