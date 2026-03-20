/// Differential fluid: viscosity, level, metallic content
/// Phase 569

#[derive(Debug, Clone)]
pub struct DiffFluid {
    pub viscosity_ok: bool,
    pub level_ok: bool,
    pub metallic_ppm: f64,
    pub max_metallic_ppm: f64,
    pub color_ok: bool,
}

impl Default for DiffFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffFluid {
    pub fn new() -> Self {
        Self {
            viscosity_ok: true,
            level_ok: true,
            metallic_ppm: 30.0,
            max_metallic_ppm: 100.0,
            color_ok: true,
        }
    }

    pub fn contamination_ok(&self) -> bool {
        self.metallic_ppm < self.max_metallic_ppm
    }

    pub fn fluid_ok(&self) -> bool {
        self.viscosity_ok && self.level_ok && self.color_ok
    }

    pub fn all_ok(&self) -> bool {
        self.contamination_ok() && self.fluid_ok()
    }

    pub fn needs_change(&self) -> bool {
        self.metallic_ppm > self.max_metallic_ppm || !self.level_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.metallic_ppm > self.max_metallic_ppm {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contamination() {
        let c = DiffFluid::new();
        assert!(c.contamination_ok());
    }

    #[test]
    fn test_fluid() {
        let c = DiffFluid::new();
        assert!(c.fluid_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DiffFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_change() {
        let c = DiffFluid::new();
        assert!(!c.needs_change());
    }

    #[test]
    fn test_metallic() {
        let mut c = DiffFluid::new();
        c.metallic_ppm = 150.0;
        assert!(c.needs_change());
    }

    #[test]
    fn test_health() {
        let c = DiffFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
