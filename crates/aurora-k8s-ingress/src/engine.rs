/// aurora-k8s-ingress: k8s ingress
/// Phase 2561

#[derive(Debug, Clone)]
pub struct K8sIngress {
    pub create_ok: bool,
    pub delete_ok: bool,
    pub update_ok: bool,
    pub tls_ok: bool,
    pub watch_ok: bool,
}

impl Default for K8sIngress {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sIngress {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            delete_ok: true,
            update_ok: true,
            tls_ok: true,
            watch_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.delete_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tls_ok && self.watch_ok
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
        let c = K8sIngress::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = K8sIngress::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = K8sIngress::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = K8sIngress::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = K8sIngress::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = K8sIngress::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = K8sIngress::default();
        assert!(c.all_ok());
    }
}
