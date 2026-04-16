/// aurora-k8s-cm: k8s cm
/// Phase 2559

#[derive(Debug, Clone)]
pub struct K8sCm {
    pub create_ok: bool,
    pub delete_ok: bool,
    pub update_ok: bool,
    pub watch_ok: bool,
    pub apply_ok: bool,
}

impl Default for K8sCm {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sCm {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            delete_ok: true,
            update_ok: true,
            watch_ok: true,
            apply_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.delete_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.watch_ok && self.apply_ok
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
        let c = K8sCm::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = K8sCm::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = K8sCm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = K8sCm::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = K8sCm::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = K8sCm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = K8sCm::default();
        assert!(c.all_ok());
    }
}
