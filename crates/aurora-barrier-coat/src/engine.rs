/// Barrier coating: e-coat, phosphate conversion, corrosion prevention
/// Phase 384

#[derive(Debug, Clone)]
pub struct BarrierCoat {
    pub thickness_um: f64,
    pub min_thickness_um: f64,
    pub coverage_pct: f64,
    pub adhesion_ok: bool,
    pub cured: bool,
}

impl Default for BarrierCoat {
    fn default() -> Self {
        Self::new()
    }
}

impl BarrierCoat {
    pub fn new() -> Self {
        Self {
            thickness_um: 25.0,
            min_thickness_um: 15.0,
            coverage_pct: 99.0,
            adhesion_ok: true,
            cured: true,
        }
    }

    pub fn thickness_ok(&self) -> bool {
        self.thickness_um >= self.min_thickness_um
    }

    pub fn fully_covered(&self) -> bool {
        self.coverage_pct > 98.0
    }

    pub fn effective(&self) -> bool {
        self.thickness_ok() && self.adhesion_ok && self.cured
    }

    pub fn needs_recoat(&self) -> bool {
        !self.thickness_ok() || !self.adhesion_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.adhesion_ok {
            return 0.0;
        }
        if !self.thickness_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thickness() {
        let b = BarrierCoat::new();
        assert!(b.thickness_ok());
    }

    #[test]
    fn test_covered() {
        let b = BarrierCoat::new();
        assert!(b.fully_covered());
    }

    #[test]
    fn test_effective() {
        let b = BarrierCoat::new();
        assert!(b.effective());
    }

    #[test]
    fn test_no_recoat() {
        let b = BarrierCoat::new();
        assert!(!b.needs_recoat());
    }

    #[test]
    fn test_thin() {
        let mut b = BarrierCoat::new();
        b.thickness_um = 10.0;
        assert!(b.needs_recoat());
    }

    #[test]
    fn test_health() {
        let b = BarrierCoat::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
