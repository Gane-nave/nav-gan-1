/// Data mesh: domain, product, catalog, governance, federate
/// Phase 1037

#[derive(Debug, Clone)]
pub struct DataMesh {
    pub domain_ok: bool,
    pub product_ok: bool,
    pub catalog_ok: bool,
    pub governance_ok: bool,
    pub federate_ok: bool,
}

impl Default for DataMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl DataMesh {
    pub fn new() -> Self {
        Self {
            domain_ok: true,
            product_ok: true,
            catalog_ok: true,
            governance_ok: true,
            federate_ok: true,
        }
    }

    pub fn topology_ok(&self) -> bool {
        self.domain_ok && self.product_ok && self.catalog_ok
    }

    pub fn control_ok(&self) -> bool {
        self.governance_ok && self.federate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.topology_ok() && self.control_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.domain_ok || !self.governance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.domain_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology() {
        let c = DataMesh::new();
        assert!(c.topology_ok());
    }

    #[test]
    fn test_control() {
        let c = DataMesh::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataMesh::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = DataMesh::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_domain() {
        let mut c = DataMesh::new();
        c.domain_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = DataMesh::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
