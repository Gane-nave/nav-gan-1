/// Sound deadening: constrained layer damping, mass loading, absorption
/// Phase 363

#[derive(Debug, Clone)]
pub struct SoundDead {
    pub coverage_pct: f64,
    pub thickness_mm: f64,
    pub material_ok: bool,
    pub adhered: bool,
    pub cabin_db: f64,
}

impl Default for SoundDead {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundDead {
    pub fn new() -> Self {
        Self {
            coverage_pct: 85.0,
            thickness_mm: 3.0,
            material_ok: true,
            adhered: true,
            cabin_db: 38.0,
        }
    }

    pub fn effective(&self) -> bool {
        self.coverage_pct > 70.0 && self.material_ok && self.adhered
    }

    pub fn cabin_quiet(&self) -> bool {
        self.cabin_db < 45.0
    }

    pub fn premium_level(&self) -> bool {
        self.cabin_db < 35.0
    }

    pub fn needs_repair(&self) -> bool {
        !self.adhered || !self.material_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.adhered {
            return 30.0;
        }
        if !self.material_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective() {
        let s = SoundDead::new();
        assert!(s.effective());
    }

    #[test]
    fn test_quiet() {
        let s = SoundDead::new();
        assert!(s.cabin_quiet());
    }

    #[test]
    fn test_not_premium() {
        let s = SoundDead::new();
        assert!(!s.premium_level());
    }

    #[test]
    fn test_no_repair() {
        let s = SoundDead::new();
        assert!(!s.needs_repair());
    }

    #[test]
    fn test_detached() {
        let mut s = SoundDead::new();
        s.adhered = false;
        assert!(s.needs_repair());
    }

    #[test]
    fn test_health() {
        let s = SoundDead::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
