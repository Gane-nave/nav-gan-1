/// Damping mat: viscoelastic, constrained layer, panel treatment
/// Phase 383

#[derive(Debug, Clone)]
pub struct DampingMat {
    pub loss_factor: f64,
    pub thickness_mm: f64,
    pub coverage_pct: f64,
    pub adhered: bool,
    pub temp_range_ok: bool,
}

impl Default for DampingMat {
    fn default() -> Self {
        Self::new()
    }
}

impl DampingMat {
    pub fn new() -> Self {
        Self {
            loss_factor: 0.3,
            thickness_mm: 2.0,
            coverage_pct: 70.0,
            adhered: true,
            temp_range_ok: true,
        }
    }

    pub fn effective(&self) -> bool {
        self.loss_factor > 0.2 && self.adhered
    }

    pub fn high_damping(&self) -> bool {
        self.loss_factor > 0.5
    }

    pub fn needs_replacement(&self) -> bool {
        !self.adhered || !self.temp_range_ok
    }

    pub fn coverage_ok(&self) -> bool {
        self.coverage_pct > 50.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.adhered {
            return 0.0;
        }
        if !self.temp_range_ok {
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
        let d = DampingMat::new();
        assert!(d.effective());
    }

    #[test]
    fn test_not_high() {
        let d = DampingMat::new();
        assert!(!d.high_damping());
    }

    #[test]
    fn test_no_replace() {
        let d = DampingMat::new();
        assert!(!d.needs_replacement());
    }

    #[test]
    fn test_coverage() {
        let d = DampingMat::new();
        assert!(d.coverage_ok());
    }

    #[test]
    fn test_detached() {
        let mut d = DampingMat::new();
        d.adhered = false;
        assert!(d.needs_replacement());
    }

    #[test]
    fn test_health() {
        let d = DampingMat::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
