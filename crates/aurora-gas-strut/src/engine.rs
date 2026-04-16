/// Gas strut: gas spring, hood/trunk lift, force curve, seal integrity
/// Phase 413

#[derive(Debug, Clone)]
pub struct GasStrut {
    pub force_n: f64,
    pub min_force_n: f64,
    pub extended: bool,
    pub seal_ok: bool,
    pub age_years: f64,
}

impl Default for GasStrut {
    fn default() -> Self {
        Self::new()
    }
}

impl GasStrut {
    pub fn new() -> Self {
        Self {
            force_n: 350.0,
            min_force_n: 200.0,
            extended: true,
            seal_ok: true,
            age_years: 2.0,
        }
    }

    pub fn force_ok(&self) -> bool {
        self.force_n >= self.min_force_n
    }

    pub fn holds_open(&self) -> bool {
        self.force_ok() && self.seal_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.force_ok() || !self.seal_ok
    }

    pub fn remaining_life_pct(&self) -> f64 {
        let max_age = 8.0;
        ((1.0 - self.age_years / max_age) * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.seal_ok {
            return 0.0;
        }
        if !self.force_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_force() {
        let g = GasStrut::new();
        assert!(g.force_ok());
    }

    #[test]
    fn test_holds() {
        let g = GasStrut::new();
        assert!(g.holds_open());
    }

    #[test]
    fn test_no_replace() {
        let g = GasStrut::new();
        assert!(!g.needs_replacement());
    }

    #[test]
    fn test_life() {
        let g = GasStrut::new();
        assert!(g.remaining_life_pct() > 70.0);
    }

    #[test]
    fn test_weak() {
        let mut g = GasStrut::new();
        g.force_n = 100.0;
        assert!(g.needs_replacement());
    }

    #[test]
    fn test_health() {
        let g = GasStrut::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
