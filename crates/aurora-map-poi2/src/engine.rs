/// map poi: fetch, filter, rank, display, log
/// Phase 1438

#[derive(Debug, Clone)]
pub struct MapPoi2 {
    pub fetch_ok: bool,
    pub filter_ok: bool,
    pub rank_ok: bool,
    pub display_ok: bool,
    pub log_ok: bool,
}

impl Default for MapPoi2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MapPoi2 {
    pub fn new() -> Self {
        Self {
            fetch_ok: true,
            filter_ok: true,
            rank_ok: true,
            display_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fetch_ok && self.filter_ok && self.rank_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.display_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fetch_ok || !self.filter_ok
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
        let c = MapPoi2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapPoi2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapPoi2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapPoi2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapPoi2::new();
        c.fetch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapPoi2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
