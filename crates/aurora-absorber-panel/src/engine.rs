/// Absorber panel: porous material, micro-perforated, broadband absorption
/// Phase 382

#[derive(Debug, Clone)]
pub struct AbsorberPanel {
    pub absorption_coeff: f64,
    pub thickness_mm: f64,
    pub coverage_pct: f64,
    pub condition_ok: bool,
    pub moisture_ok: bool,
}

impl Default for AbsorberPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsorberPanel {
    pub fn new() -> Self {
        Self {
            absorption_coeff: 0.7,
            thickness_mm: 25.0,
            coverage_pct: 80.0,
            condition_ok: true,
            moisture_ok: true,
        }
    }

    pub fn effective(&self) -> bool {
        self.absorption_coeff > 0.5 && self.condition_ok
    }

    pub fn high_performance(&self) -> bool {
        self.absorption_coeff > 0.85
    }

    pub fn needs_replacement(&self) -> bool {
        !self.condition_ok || !self.moisture_ok
    }

    pub fn coverage_ok(&self) -> bool {
        self.coverage_pct > 60.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.condition_ok {
            return 0.0;
        }
        if !self.moisture_ok {
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
        let a = AbsorberPanel::new();
        assert!(a.effective());
    }

    #[test]
    fn test_not_high() {
        let a = AbsorberPanel::new();
        assert!(!a.high_performance());
    }

    #[test]
    fn test_no_replace() {
        let a = AbsorberPanel::new();
        assert!(!a.needs_replacement());
    }

    #[test]
    fn test_coverage() {
        let a = AbsorberPanel::new();
        assert!(a.coverage_ok());
    }

    #[test]
    fn test_wet() {
        let mut a = AbsorberPanel::new();
        a.moisture_ok = false;
        assert!(a.needs_replacement());
    }

    #[test]
    fn test_health() {
        let a = AbsorberPanel::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
