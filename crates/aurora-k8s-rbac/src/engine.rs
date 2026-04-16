/// aurora-k8s-rbac: k8s rbac
/// Phase 2563

#[derive(Debug, Clone)]
pub struct K8sRbac {
    pub create_ok: bool,
    pub delete_ok: bool,
    pub bind_ok: bool,
    pub audit_ok: bool,
    pub watch_ok: bool,
}

impl Default for K8sRbac {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sRbac {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            delete_ok: true,
            bind_ok: true,
            audit_ok: true,
            watch_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.delete_ok && self.bind_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.watch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.delete_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = K8sRbac::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = K8sRbac::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = K8sRbac::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = K8sRbac::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = K8sRbac::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = K8sRbac::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = K8sRbac::default();
        assert!(c.all_ok());
    }
}
