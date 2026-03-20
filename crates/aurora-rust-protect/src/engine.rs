/// Rust protection: underbody wax, cavity wax, sacrificial anode
/// Phase 386

#[derive(Debug, Clone)]
pub struct RustProtect {
    pub wax_coverage_pct: f64,
    pub cavity_treated: bool,
    pub anode_ok: bool,
    pub age_years: f64,
    pub max_age_years: f64,
}

impl Default for RustProtect {
    fn default() -> Self {
        Self::new()
    }
}

impl RustProtect {
    pub fn new() -> Self {
        Self {
            wax_coverage_pct: 95.0,
            cavity_treated: true,
            anode_ok: true,
            age_years: 1.0,
            max_age_years: 5.0,
        }
    }

    pub fn coverage_ok(&self) -> bool {
        self.wax_coverage_pct > 80.0
    }

    pub fn all_ok(&self) -> bool {
        self.coverage_ok() && self.cavity_treated && self.anode_ok
    }

    pub fn needs_renewal(&self) -> bool {
        self.age_years > self.max_age_years || self.wax_coverage_pct < 60.0
    }

    pub fn remaining_life_pct(&self) -> f64 {
        if self.max_age_years <= 0.0 {
            return 0.0;
        }
        ((1.0 - self.age_years / self.max_age_years) * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_renewal() {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage() {
        let r = RustProtect::new();
        assert!(r.coverage_ok());
    }

    #[test]
    fn test_all_ok() {
        let r = RustProtect::new();
        assert!(r.all_ok());
    }

    #[test]
    fn test_no_renewal() {
        let r = RustProtect::new();
        assert!(!r.needs_renewal());
    }

    #[test]
    fn test_life() {
        let r = RustProtect::new();
        assert!(r.remaining_life_pct() > 70.0);
    }

    #[test]
    fn test_old() {
        let mut r = RustProtect::new();
        r.age_years = 6.0;
        assert!(r.needs_renewal());
    }

    #[test]
    fn test_health() {
        let r = RustProtect::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
