/// cache read: get, load, refresh, prefetch, log
/// Phase 1937

#[derive(Debug, Clone)]
pub struct CacheRead {
    pub get_ok: bool,
    pub load_ok: bool,
    pub refresh_ok: bool,
    pub prefetch_ok: bool,
    pub log_ok: bool,
}

impl Default for CacheRead {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheRead {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            load_ok: true,
            refresh_ok: true,
            prefetch_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.load_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.prefetch_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.get_ok || !self.load_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.get_ok {
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
        let c = CacheRead::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CacheRead::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CacheRead::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CacheRead::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CacheRead::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CacheRead::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
