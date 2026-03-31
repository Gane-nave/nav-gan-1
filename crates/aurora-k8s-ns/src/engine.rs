/// aurora-k8s-ns: k8s ns
/// Phase 2557

#[derive(Debug, Clone)]
pub struct K8sNs {
    pub create_ok: bool,
    pub delete_ok: bool,
    pub list_ok: bool,
    pub label_ok: bool,
    pub quota_ok: bool,
}

impl Default for K8sNs {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sNs {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            delete_ok: true,
            list_ok: true,
            label_ok: true,
            quota_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.delete_ok && self.list_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.quota_ok
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
        let c = K8sNs::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = K8sNs::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = K8sNs::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = K8sNs::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = K8sNs::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = K8sNs::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = K8sNs::default();
        assert!(c.all_ok());
    }
}
