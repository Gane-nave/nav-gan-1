/// Terraform module: plan, apply, destroy, import, state
/// Phase 1071

#[derive(Debug, Clone)]
pub struct TerraformMod {
    pub plan_ok: bool,
    pub apply_ok: bool,
    pub destroy_ok: bool,
    pub import_ok: bool,
    pub state_ok: bool,
}

impl Default for TerraformMod {
    fn default() -> Self {
        Self::new()
    }
}

impl TerraformMod {
    pub fn new() -> Self {
        Self {
            plan_ok: true,
            apply_ok: true,
            destroy_ok: true,
            import_ok: true,
            state_ok: true,
        }
    }

    pub fn provisioning_ok(&self) -> bool {
        self.plan_ok && self.apply_ok && self.destroy_ok
    }

    pub fn management_ok(&self) -> bool {
        self.import_ok && self.state_ok
    }

    pub fn all_ok(&self) -> bool {
        self.provisioning_ok() && self.management_ok()
    }

    pub fn needs_plan(&self) -> bool {
        !self.plan_ok || !self.state_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.plan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provisioning() {
        let c = TerraformMod::new();
        assert!(c.provisioning_ok());
    }

    #[test]
    fn test_management() {
        let c = TerraformMod::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TerraformMod::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_plan() {
        let c = TerraformMod::new();
        assert!(!c.needs_plan());
    }

    #[test]
    fn test_plan() {
        let mut c = TerraformMod::new();
        c.plan_ok = false;
        assert!(c.needs_plan());
    }

    #[test]
    fn test_health() {
        let c = TerraformMod::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
