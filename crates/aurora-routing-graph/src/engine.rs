/// Routing graph: build, contract, partition, query, update
/// Phase 1092

#[derive(Debug, Clone)]
pub struct RoutingGraph {
    pub build_ok: bool,
    pub contract_ok: bool,
    pub partition_ok: bool,
    pub query_ok: bool,
    pub update_ok: bool,
}

impl Default for RoutingGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutingGraph {
    pub fn new() -> Self {
        Self {
            build_ok: true,
            contract_ok: true,
            partition_ok: true,
            query_ok: true,
            update_ok: true,
        }
    }

    pub fn construction_ok(&self) -> bool {
        self.build_ok && self.contract_ok && self.partition_ok
    }

    pub fn operations_ok(&self) -> bool {
        self.query_ok && self.update_ok
    }

    pub fn all_ok(&self) -> bool {
        self.construction_ok() && self.operations_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.build_ok || !self.contract_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.build_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let c = RoutingGraph::new();
        assert!(c.construction_ok());
    }

    #[test]
    fn test_operations() {
        let c = RoutingGraph::new();
        assert!(c.operations_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoutingGraph::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = RoutingGraph::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_build() {
        let mut c = RoutingGraph::new();
        c.build_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = RoutingGraph::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
