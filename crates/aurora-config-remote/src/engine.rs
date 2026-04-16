/// config remote: fetch, cache, watch, fallback, log
/// Phase 1772

#[derive(Debug, Clone)]
pub struct ConfigRemote {
    pub fetch_ok: bool,
    pub cache_ok: bool,
    pub watch_ok: bool,
    pub fallback_ok: bool,
    pub log_ok: bool,
}

impl Default for ConfigRemote {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigRemote {
    pub fn new() -> Self {
        Self {
            fetch_ok: true,
            cache_ok: true,
            watch_ok: true,
            fallback_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fetch_ok && self.cache_ok && self.watch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.fallback_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fetch_ok || !self.cache_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fetch_ok {
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
        let c = ConfigRemote::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ConfigRemote::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConfigRemote::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ConfigRemote::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ConfigRemote::new();
        c.fetch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ConfigRemote::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
