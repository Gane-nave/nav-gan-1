/// map tile: fetch, cache, render, stitch, log
/// Phase 1428

#[derive(Debug, Clone)]
pub struct MapTile {
    pub fetch_ok: bool,
    pub cache_ok: bool,
    pub render_ok: bool,
    pub stitch_ok: bool,
    pub log_ok: bool,
}

impl Default for MapTile {
    fn default() -> Self {
        Self::new()
    }
}

impl MapTile {
    pub fn new() -> Self {
        Self {
            fetch_ok: true,
            cache_ok: true,
            render_ok: true,
            stitch_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fetch_ok && self.cache_ok && self.render_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stitch_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fetch_ok || !self.cache_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fetch_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MapTile::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapTile::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapTile::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapTile::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapTile::new();
        c.fetch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapTile::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
