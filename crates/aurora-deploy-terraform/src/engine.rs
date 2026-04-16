/// deploy terraform: plan, apply, destroy, output, log
/// Phase 2115

#[derive(Debug, Clone)]
pub struct DeployTerraform {
    pub plan_ok: bool,
    pub apply_ok: bool,
    pub destroy_ok: bool,
    pub output_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployTerraform {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployTerraform {
    pub fn new() -> Self {
        Self {
            plan_ok: true,
            apply_ok: true,
            destroy_ok: true,
            output_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.plan_ok && self.apply_ok && self.destroy_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.output_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.plan_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.plan_ok {
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
        let c = DeployTerraform::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployTerraform::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployTerraform::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployTerraform::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployTerraform::new();
        c.plan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployTerraform::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
