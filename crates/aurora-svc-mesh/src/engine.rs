/// aurora-svc-mesh: svc mesh
/// Phase 2564

#[derive(Debug, Clone)]
pub struct SvcMesh {
    pub discover_ok: bool,
    pub route_ok: bool,
    pub lb_ok: bool,
    pub secure_ok: bool,
    pub observe_ok: bool,
}

impl Default for SvcMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcMesh {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            route_ok: true,
            lb_ok: true,
            secure_ok: true,
            observe_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.discover_ok && self.route_ok && self.lb_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.secure_ok && self.observe_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.discover_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.discover_ok {
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
        let c = SvcMesh::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcMesh::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcMesh::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcMesh::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcMesh::new();
        c.discover_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcMesh::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcMesh::default();
        assert!(c.all_ok());
    }
}
