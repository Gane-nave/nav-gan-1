/// Zinc layer: galvanization, hot-dip, electro-galvanized coating
/// Phase 387

#[derive(Debug, Clone)]
pub struct ZincLayer {
    pub thickness_um: f64,
    pub min_thickness_um: f64,
    pub uniformity_pct: f64,
    pub adhesion_ok: bool,
    pub passivated: bool,
}

impl Default for ZincLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZincLayer {
    pub fn new() -> Self {
        Self {
            thickness_um: 12.0,
            min_thickness_um: 7.0,
            uniformity_pct: 95.0,
            adhesion_ok: true,
            passivated: true,
        }
    }

    pub fn thickness_ok(&self) -> bool {
        self.thickness_um >= self.min_thickness_um
    }

    pub fn uniform(&self) -> bool {
        self.uniformity_pct > 85.0
    }

    pub fn effective(&self) -> bool {
        self.thickness_ok() && self.adhesion_ok && self.passivated
    }

    pub fn protection_years(&self) -> f64 {
        self.thickness_um * 2.0
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
        let z = ZincLayer::new();
        assert!(z.thickness_ok());
    }

    #[test]
    fn test_uniform() {
        let z = ZincLayer::new();
        assert!(z.uniform());
    }

    #[test]
    fn test_effective() {
        let z = ZincLayer::new();
        assert!(z.effective());
    }

    #[test]
    fn test_protection() {
        let z = ZincLayer::new();
        assert!(z.protection_years() > 20.0);
    }

    #[test]
    fn test_thin() {
        let mut z = ZincLayer::new();
        z.thickness_um = 5.0;
        assert!(!z.thickness_ok());
    }

    #[test]
    fn test_health() {
        let z = ZincLayer::new();
        assert!((z.health_score() - 100.0).abs() < 0.1);
    }
}
