/// map search: index, query, rank, suggest, log
/// Phase 1432

#[derive(Debug, Clone)]
pub struct MapSearch2 {
    pub index_ok: bool,
    pub query_ok: bool,
    pub rank_ok: bool,
    pub suggest_ok: bool,
    pub log_ok: bool,
}

impl Default for MapSearch2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MapSearch2 {
    pub fn new() -> Self {
        Self {
            index_ok: true,
            query_ok: true,
            rank_ok: true,
            suggest_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.index_ok && self.query_ok && self.rank_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.suggest_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.index_ok || !self.query_ok
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
    fn test_primary() {
        let c = MapSearch2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapSearch2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapSearch2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapSearch2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapSearch2::new();
        c.index_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapSearch2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
