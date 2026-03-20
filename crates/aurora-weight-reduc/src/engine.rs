/// Weight reduction: carbon, alloy, composite, hollow, thin
/// Phase 959

#[derive(Debug, Clone)]
pub struct WeightReduc {
    pub carbon_ok: bool,
    pub alloy_ok: bool,
    pub composite_ok: bool,
    pub hollow_ok: bool,
    pub thin_ok: bool,
}

impl Default for WeightReduc {
    fn default() -> Self {
        Self::new()
    }
}

impl WeightReduc {
    pub fn new() -> Self {
        Self {
            carbon_ok: true,
            alloy_ok: true,
            composite_ok: true,
            hollow_ok: true,
            thin_ok: true,
        }
    }

    pub fn material_ok(&self) -> bool {
        self.carbon_ok && self.alloy_ok && self.composite_ok
    }

    pub fn design_ok(&self) -> bool {
        self.hollow_ok && self.thin_ok
    }

    pub fn all_ok(&self) -> bool {
        self.material_ok() && self.design_ok()
    }

    pub fn needs_review(&self) -> bool {
        !self.carbon_ok || !self.alloy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.carbon_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material() {
        let c = WeightReduc::new();
        assert!(c.material_ok());
    }

    #[test]
    fn test_design() {
        let c = WeightReduc::new();
        assert!(c.design_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WeightReduc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_review() {
        let c = WeightReduc::new();
        assert!(!c.needs_review());
    }

    #[test]
    fn test_carbon() {
        let mut c = WeightReduc::new();
        c.carbon_ok = false;
        assert!(c.needs_review());
    }

    #[test]
    fn test_health() {
        let c = WeightReduc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
