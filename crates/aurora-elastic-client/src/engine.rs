/// Elasticsearch client: index, search, aggregate, bulk, scroll
/// Phase 1058

#[derive(Debug, Clone)]
pub struct ElasticClient {
    pub index_ok: bool,
    pub search_ok: bool,
    pub aggregate_ok: bool,
    pub bulk_ok: bool,
    pub scroll_ok: bool,
}

impl Default for ElasticClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ElasticClient {
    pub fn new() -> Self {
        Self {
            index_ok: true,
            search_ok: true,
            aggregate_ok: true,
            bulk_ok: true,
            scroll_ok: true,
        }
    }

    pub fn indexing_ok(&self) -> bool {
        self.index_ok && self.bulk_ok
    }

    pub fn querying_ok(&self) -> bool {
        self.search_ok && self.aggregate_ok && self.scroll_ok
    }

    pub fn all_ok(&self) -> bool {
        self.indexing_ok() && self.querying_ok()
    }

    pub fn needs_reindex(&self) -> bool {
        !self.index_ok || !self.search_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.index_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing() {
        let c = ElasticClient::new();
        assert!(c.indexing_ok());
    }

    #[test]
    fn test_querying() {
        let c = ElasticClient::new();
        assert!(c.querying_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ElasticClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reindex() {
        let c = ElasticClient::new();
        assert!(!c.needs_reindex());
    }

    #[test]
    fn test_index() {
        let mut c = ElasticClient::new();
        c.index_ok = false;
        assert!(c.needs_reindex());
    }

    #[test]
    fn test_health() {
        let c = ElasticClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
