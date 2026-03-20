/// map reverse: locate, resolve, format, cache, log
/// Phase 1434

#[derive(Debug, Clone)]
pub struct MapReverse {
    pub locate_ok: bool,
    pub resolve_ok: bool,
    pub format_ok: bool,
    pub cache_ok: bool,
    pub log_ok: bool,
}

impl Default for MapReverse {
    fn default() -> Self {
        Self::new()
    }
}

impl MapReverse {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            resolve_ok: true,
            format_ok: true,
            cache_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.locate_ok && self.resolve_ok && self.format_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cache_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.locate_ok || !self.resolve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.locate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MapReverse::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapReverse::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapReverse::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapReverse::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapReverse::new();
        c.locate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapReverse::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
