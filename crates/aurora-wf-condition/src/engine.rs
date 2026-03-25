/// wf condition: evaluate, branch, merge, default, log
/// Phase 2222

#[derive(Debug, Clone)]
pub struct WfCondition {
    pub evaluate_ok: bool,
    pub branch_ok: bool,
    pub merge_ok: bool,
    pub default_ok: bool,
    pub log_ok: bool,
}

impl Default for WfCondition {
    fn default() -> Self {
        Self::new()
    }
}

impl WfCondition {
    pub fn new() -> Self {
        Self {
            evaluate_ok: true,
            branch_ok: true,
            merge_ok: true,
            default_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.evaluate_ok && self.branch_ok && self.merge_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.default_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.evaluate_ok || !self.branch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.evaluate_ok {
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
        let c = WfCondition::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfCondition::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfCondition::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfCondition::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfCondition::new();
        c.evaluate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfCondition::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
