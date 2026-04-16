/// graph edge: create, update, delete, query, log
/// Phase 1897

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub create_ok: bool,
    pub update_ok: bool,
    pub delete_ok: bool,
    pub query_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphEdge {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphEdge {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            update_ok: true,
            delete_ok: true,
            query_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.update_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.query_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.update_ok
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
        let c = GraphEdge::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphEdge::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphEdge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphEdge::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphEdge::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphEdge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
