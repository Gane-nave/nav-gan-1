/// Data catalog: discover, classify, tag, search, govern
/// Phase 1041

#[derive(Debug, Clone)]
pub struct DataCatalog {
    pub discover_ok: bool,
    pub classify_ok: bool,
    pub tag_ok: bool,
    pub search_ok: bool,
    pub govern_ok: bool,
}

impl Default for DataCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl DataCatalog {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            classify_ok: true,
            tag_ok: true,
            search_ok: true,
            govern_ok: true,
        }
    }

    pub fn discovery_ok(&self) -> bool {
        self.discover_ok && self.classify_ok && self.tag_ok
    }

    pub fn access_ok(&self) -> bool {
        self.search_ok && self.govern_ok
    }

    pub fn all_ok(&self) -> bool {
        self.discovery_ok() && self.access_ok()
    }

    pub fn needs_crawl(&self) -> bool {
        !self.discover_ok || !self.classify_ok
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
    fn test_discovery() {
        let c = DataCatalog::new();
        assert!(c.discovery_ok());
    }

    #[test]
    fn test_access() {
        let c = DataCatalog::new();
        assert!(c.access_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataCatalog::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_crawl() {
        let c = DataCatalog::new();
        assert!(!c.needs_crawl());
    }

    #[test]
    fn test_discover() {
        let mut c = DataCatalog::new();
        c.discover_ok = false;
        assert!(c.needs_crawl());
    }

    #[test]
    fn test_health() {
        let c = DataCatalog::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
