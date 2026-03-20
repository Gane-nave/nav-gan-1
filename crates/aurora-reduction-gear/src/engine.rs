/// reduction gear: mesh, ratio, lubricate, cool, check
/// Phase 1222

#[derive(Debug, Clone)]
pub struct ReductionGear {
    pub mesh_ok: bool,
    pub ratio_ok: bool,
    pub lubricate_ok: bool,
    pub cool_ok: bool,
    pub check_ok: bool,
}

impl Default for ReductionGear {
    fn default() -> Self {
        Self::new()
    }
}

impl ReductionGear {
    pub fn new() -> Self {
        Self {
            mesh_ok: true,
            ratio_ok: true,
            lubricate_ok: true,
            cool_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mesh_ok && self.ratio_ok && self.lubricate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cool_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mesh_ok || !self.ratio_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mesh_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ReductionGear::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReductionGear::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReductionGear::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReductionGear::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReductionGear::new();
        c.mesh_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReductionGear::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
