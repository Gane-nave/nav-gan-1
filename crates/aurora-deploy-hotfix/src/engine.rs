/// deploy hotfix: branch, fix, test, merge, log
/// Phase 1595

#[derive(Debug, Clone)]
pub struct DeployHotfix {
    pub branch_ok: bool,
    pub fix_ok: bool,
    pub test_ok: bool,
    pub merge_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployHotfix {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployHotfix {
    pub fn new() -> Self {
        Self {
            branch_ok: true,
            fix_ok: true,
            test_ok: true,
            merge_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.branch_ok && self.fix_ok && self.test_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.merge_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.branch_ok || !self.fix_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.branch_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DeployHotfix::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployHotfix::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployHotfix::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployHotfix::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployHotfix::new();
        c.branch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployHotfix::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
