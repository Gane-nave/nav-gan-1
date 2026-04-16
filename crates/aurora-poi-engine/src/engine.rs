/// POI engine: search, rank, filter, detail, review
/// Phase 1087

#[derive(Debug, Clone)]
pub struct PoiEngine {
    pub search_ok: bool,
    pub rank_ok: bool,
    pub filter_ok: bool,
    pub detail_ok: bool,
    pub review_ok: bool,
}

impl Default for PoiEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PoiEngine {
    pub fn new() -> Self {
        Self {
            search_ok: true,
            rank_ok: true,
            filter_ok: true,
            detail_ok: true,
            review_ok: true,
        }
    }

    pub fn discovery_ok(&self) -> bool {
        self.search_ok && self.rank_ok && self.filter_ok
    }

    pub fn content_ok(&self) -> bool {
        self.detail_ok && self.review_ok
    }

    pub fn all_ok(&self) -> bool {
        self.discovery_ok() && self.content_ok()
    }

    pub fn needs_index(&self) -> bool {
        !self.search_ok || !self.rank_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.search_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery() {
        let c = PoiEngine::new();
        assert!(c.discovery_ok());
    }

    #[test]
    fn test_content() {
        let c = PoiEngine::new();
        assert!(c.content_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PoiEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_index() {
        let c = PoiEngine::new();
        assert!(!c.needs_index());
    }

    #[test]
    fn test_search() {
        let mut c = PoiEngine::new();
        c.search_ok = false;
        assert!(c.needs_index());
    }

    #[test]
    fn test_health() {
        let c = PoiEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
