/// aurora-svc-discovery: svc discovery
/// Phase 2566

#[derive(Debug, Clone)]
pub struct SvcDiscovery {
    pub resolve_ok: bool,
    pub cache_ok: bool,
    pub refresh_ok: bool,
    pub failover_ok: bool,
    pub watch_ok: bool,
}

impl Default for SvcDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcDiscovery {
    pub fn new() -> Self {
        Self {
            resolve_ok: true,
            cache_ok: true,
            refresh_ok: true,
            failover_ok: true,
            watch_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.resolve_ok && self.cache_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.failover_ok && self.watch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.resolve_ok || !self.cache_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.resolve_ok {
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
        let c = SvcDiscovery::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcDiscovery::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcDiscovery::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcDiscovery::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcDiscovery::new();
        c.resolve_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcDiscovery::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcDiscovery::default();
        assert!(c.all_ok());
    }
}
