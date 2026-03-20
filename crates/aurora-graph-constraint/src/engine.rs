/// graph constraint: add, check, remove, list, log
/// Phase 1913

#[derive(Debug, Clone)]
pub struct GraphConstraint {
    pub add_ok: bool,
    pub check_ok: bool,
    pub remove_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphConstraint {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphConstraint {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            check_ok: true,
            remove_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.check_ok && self.remove_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.add_ok {
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
        let c = GraphConstraint::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphConstraint::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphConstraint::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphConstraint::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphConstraint::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphConstraint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
