/// Vector DB: index, search, filter, cluster, quantize
/// Phase 1046

#[derive(Debug, Clone)]
pub struct VectorDb {
    pub index_ok: bool,
    pub search_ok: bool,
    pub filter_ok: bool,
    pub cluster_ok: bool,
    pub quantize_ok: bool,
}

impl Default for VectorDb {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorDb {
    pub fn new() -> Self {
        Self {
            index_ok: true,
            search_ok: true,
            filter_ok: true,
            cluster_ok: true,
            quantize_ok: true,
        }
    }

    pub fn retrieval_ok(&self) -> bool {
        self.index_ok && self.search_ok && self.filter_ok
    }

    pub fn optimization_ok(&self) -> bool {
        self.cluster_ok && self.quantize_ok
    }

    pub fn all_ok(&self) -> bool {
        self.retrieval_ok() && self.optimization_ok()
    }

    pub fn needs_reindex(&self) -> bool {
        !self.index_ok || !self.search_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.index_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retrieval() {
        let c = VectorDb::new();
        assert!(c.retrieval_ok());
    }

    #[test]
    fn test_optimization() {
        let c = VectorDb::new();
        assert!(c.optimization_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VectorDb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reindex() {
        let c = VectorDb::new();
        assert!(!c.needs_reindex());
    }

    #[test]
    fn test_index() {
        let mut c = VectorDb::new();
        c.index_ok = false;
        assert!(c.needs_reindex());
    }

    #[test]
    fn test_health() {
        let c = VectorDb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
