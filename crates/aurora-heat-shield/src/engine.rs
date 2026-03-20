/// Heat shield: material, mounting, coverage, reflect
/// Phase 767

#[derive(Debug, Clone)]
pub struct HeatShield {
    pub material_ok: bool,
    pub mounting_ok: bool,
    pub coverage_ok: bool,
    pub reflect_ok: bool,
    pub gap_ok: bool,
}

impl Default for HeatShield {
    fn default() -> Self {
        Self::new()
    }
}

impl HeatShield {
    pub fn new() -> Self {
        Self {
            material_ok: true,
            mounting_ok: true,
            coverage_ok: true,
            reflect_ok: true,
            gap_ok: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.material_ok && self.reflect_ok && self.coverage_ok
    }

    pub fn install_ok(&self) -> bool {
        self.mounting_ok && self.gap_ok
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.install_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.material_ok || !self.mounting_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.material_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = HeatShield::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_install() {
        let c = HeatShield::new();
        assert!(c.install_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeatShield::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = HeatShield::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_material() {
        let mut c = HeatShield::new();
        c.material_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = HeatShield::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
