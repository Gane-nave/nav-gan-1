/// deploy flux: reconcile, suspend, resume, status, log
/// Phase 2119

#[derive(Debug, Clone)]
pub struct DeployFlux {
    pub reconcile_ok: bool,
    pub suspend_ok: bool,
    pub resume_ok: bool,
    pub status_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployFlux {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployFlux {
    pub fn new() -> Self {
        Self {
            reconcile_ok: true,
            suspend_ok: true,
            resume_ok: true,
            status_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.reconcile_ok && self.suspend_ok && self.resume_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.status_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.reconcile_ok || !self.suspend_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.reconcile_ok {
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
        let c = DeployFlux::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployFlux::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployFlux::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployFlux::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployFlux::new();
        c.reconcile_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployFlux::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
