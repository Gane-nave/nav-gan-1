/// K8s operator: reconcile, watch, status, finalize, webhook
/// Phase 1069

#[derive(Debug, Clone)]
pub struct K8sOperator {
    pub reconcile_ok: bool,
    pub watch_ok: bool,
    pub status_ok: bool,
    pub finalize_ok: bool,
    pub webhook_ok: bool,
}

impl Default for K8sOperator {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sOperator {
    pub fn new() -> Self {
        Self {
            reconcile_ok: true,
            watch_ok: true,
            status_ok: true,
            finalize_ok: true,
            webhook_ok: true,
        }
    }

    pub fn control_ok(&self) -> bool {
        self.reconcile_ok && self.watch_ok && self.status_ok
    }

    pub fn lifecycle_ok(&self) -> bool {
        self.finalize_ok && self.webhook_ok
    }

    pub fn all_ok(&self) -> bool {
        self.control_ok() && self.lifecycle_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.reconcile_ok || !self.watch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.reconcile_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control() {
        let c = K8sOperator::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_lifecycle() {
        let c = K8sOperator::new();
        assert!(c.lifecycle_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = K8sOperator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = K8sOperator::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_reconcile() {
        let mut c = K8sOperator::new();
        c.reconcile_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = K8sOperator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
