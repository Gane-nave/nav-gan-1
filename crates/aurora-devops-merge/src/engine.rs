/// devops merge: check, merge, revert, notify, log
/// Phase 2173

#[derive(Debug, Clone)]
pub struct DevopsMerge {
    pub check_ok: bool,
    pub merge_ok: bool,
    pub revert_ok: bool,
    pub notify_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsMerge {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsMerge {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            merge_ok: true,
            revert_ok: true,
            notify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.merge_ok && self.revert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.notify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.merge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = DevopsMerge::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsMerge::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsMerge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsMerge::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsMerge::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsMerge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
