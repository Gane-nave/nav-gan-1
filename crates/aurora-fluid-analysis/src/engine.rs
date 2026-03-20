/// Fluid analysis: viscosity, contamination, TBN, TAN
/// Phase 819

#[derive(Debug, Clone)]
pub struct FluidAnalysis {
    pub viscosity_ok: bool,
    pub contam_ok: bool,
    pub tbn_ok: bool,
    pub tan_ok: bool,
    pub sample_ok: bool,
}

impl Default for FluidAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl FluidAnalysis {
    pub fn new() -> Self {
        Self {
            viscosity_ok: true,
            contam_ok: true,
            tbn_ok: true,
            tan_ok: true,
            sample_ok: true,
        }
    }

    pub fn condition_ok(&self) -> bool {
        self.viscosity_ok && self.contam_ok
    }

    pub fn chemistry_ok(&self) -> bool {
        self.tbn_ok && self.tan_ok && self.sample_ok
    }

    pub fn all_ok(&self) -> bool {
        self.condition_ok() && self.chemistry_ok()
    }

    pub fn needs_change(&self) -> bool {
        !self.viscosity_ok || !self.contam_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.viscosity_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition() {
        let c = FluidAnalysis::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_chemistry() {
        let c = FluidAnalysis::new();
        assert!(c.chemistry_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FluidAnalysis::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_change() {
        let c = FluidAnalysis::new();
        assert!(!c.needs_change());
    }

    #[test]
    fn test_viscosity() {
        let mut c = FluidAnalysis::new();
        c.viscosity_ok = false;
        assert!(c.needs_change());
    }

    #[test]
    fn test_health() {
        let c = FluidAnalysis::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
