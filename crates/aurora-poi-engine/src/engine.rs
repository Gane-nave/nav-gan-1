/// POI engine: search, category, rating, hours, photos
/// Phase 907

#[derive(Debug, Clone)]
pub struct PoiEngine {
    pub search_ok: bool,
    pub category_ok: bool,
    pub rating_ok: bool,
    pub hours_ok: bool,
    pub photos_ok: bool,
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
            category_ok: true,
            rating_ok: true,
            hours_ok: true,
            photos_ok: true,
        }
    }

    pub fn discovery_ok(&self) -> bool {
        self.search_ok && self.category_ok
    }

    pub fn detail_ok(&self) -> bool {
        self.rating_ok && self.hours_ok && self.photos_ok
    }

    pub fn all_ok(&self) -> bool {
        self.discovery_ok() && self.detail_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.hours_ok || !self.rating_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.search_ok { return 10.0; }
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
    fn test_detail() {
        let c = PoiEngine::new();
        assert!(c.detail_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PoiEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = PoiEngine::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_hours() {
        let mut c = PoiEngine::new();
        c.hours_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = PoiEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
