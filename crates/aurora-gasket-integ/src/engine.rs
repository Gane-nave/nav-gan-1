/// Gasket integrity: compression, leak detection, material degradation
/// Phase 400

#[derive(Debug, Clone)]
pub struct GasketInteg {
    pub compression_pct: f64,
    pub leak_detected: bool,
    pub material_ok: bool,
    pub torque_ok: bool,
    pub age_years: f64,
}

impl Default for GasketInteg {
    fn default() -> Self {
        Self::new()
    }
}

impl GasketInteg {
    pub fn new() -> Self {
        Self {
            compression_pct: 85.0,
            leak_detected: false,
            material_ok: true,
            torque_ok: true,
            age_years: 2.0,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        !self.leak_detected && self.compression_pct > 60.0
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.material_ok && self.torque_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.leak_detected || self.compression_pct < 40.0
    }

    pub fn remaining_life_pct(&self) -> f64 {
        self.compression_pct.clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.leak_detected {
            return 0.0;
        }
        if !self.material_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let g = GasketInteg::new();
        assert!(g.sealing_ok());
    }

    #[test]
    fn test_all_ok() {
        let g = GasketInteg::new();
        assert!(g.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let g = GasketInteg::new();
        assert!(!g.needs_replacement());
    }

    #[test]
    fn test_life() {
        let g = GasketInteg::new();
        assert!(g.remaining_life_pct() > 80.0);
    }

    #[test]
    fn test_leak() {
        let mut g = GasketInteg::new();
        g.leak_detected = true;
        assert!(g.needs_replacement());
    }

    #[test]
    fn test_health() {
        let g = GasketInteg::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
