/// graph algo: centrality, community, rank, flow, log
/// Phase 1908

#[derive(Debug, Clone)]
pub struct GraphAlgo {
    pub centrality_ok: bool,
    pub community_ok: bool,
    pub rank_ok: bool,
    pub flow_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphAlgo {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphAlgo {
    pub fn new() -> Self {
        Self {
            centrality_ok: true,
            community_ok: true,
            rank_ok: true,
            flow_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.centrality_ok && self.community_ok && self.rank_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flow_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.centrality_ok || !self.community_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.centrality_ok {
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
        let c = GraphAlgo::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphAlgo::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphAlgo::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphAlgo::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphAlgo::new();
        c.centrality_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphAlgo::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
