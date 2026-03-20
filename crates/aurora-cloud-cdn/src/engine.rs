/// cloud cdn: cache, purge, route, optimize, log
/// Phase 1459

#[derive(Debug, Clone)]
pub struct CloudCdn {
    pub cache_ok: bool,
    pub purge_ok: bool,
    pub route_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudCdn {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudCdn {
    pub fn new() -> Self {
        Self {
            cache_ok: true,
            purge_ok: true,
            route_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.cache_ok && self.purge_ok && self.route_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.cache_ok || !self.purge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cache_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudCdn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudCdn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudCdn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudCdn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudCdn::new();
        c.cache_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudCdn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
