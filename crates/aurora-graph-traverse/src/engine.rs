/// graph traverse: bfs, dfs, dijkstra, astar, log
/// Phase 1899

#[derive(Debug, Clone)]
pub struct GraphTraverse {
    pub bfs_ok: bool,
    pub dfs_ok: bool,
    pub dijkstra_ok: bool,
    pub astar_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphTraverse {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphTraverse {
    pub fn new() -> Self {
        Self {
            bfs_ok: true,
            dfs_ok: true,
            dijkstra_ok: true,
            astar_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.bfs_ok && self.dfs_ok && self.dijkstra_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.astar_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.bfs_ok || !self.dfs_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bfs_ok {
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
        let c = GraphTraverse::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphTraverse::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphTraverse::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphTraverse::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphTraverse::new();
        c.bfs_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphTraverse::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
