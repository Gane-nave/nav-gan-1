/// GitOps controller: sync, diff, prune, health, rollback
/// Phase 1074

#[derive(Debug, Clone)]
pub struct GitopsCtrl {
    pub sync_ok: bool,
    pub diff_ok: bool,
    pub prune_ok: bool,
    pub health_ok: bool,
    pub rollback_ok: bool,
}

impl Default for GitopsCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl GitopsCtrl {
    pub fn new() -> Self {
        Self {
            sync_ok: true,
            diff_ok: true,
            prune_ok: true,
            health_ok: true,
            rollback_ok: true,
        }
    }

    pub fn synchronization_ok(&self) -> bool {
        self.sync_ok && self.diff_ok && self.prune_ok
    }

    pub fn observability_ok(&self) -> bool {
        self.health_ok && self.rollback_ok
    }

    pub fn all_ok(&self) -> bool {
        self.synchronization_ok() && self.observability_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.sync_ok || !self.diff_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sync_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synchronization() {
        let c = GitopsCtrl::new();
        assert!(c.synchronization_ok());
    }

    #[test]
    fn test_observability() {
        let c = GitopsCtrl::new();
        assert!(c.observability_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GitopsCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = GitopsCtrl::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_sync() {
        let mut c = GitopsCtrl::new();
        c.sync_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = GitopsCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
