/// DNS resolver: query, cache, recurse, validate, zone
/// Phase 1060

#[derive(Debug, Clone)]
pub struct DnsResolver {
    pub query_ok: bool,
    pub cache_ok: bool,
    pub recurse_ok: bool,
    pub validate_ok: bool,
    pub zone_ok: bool,
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsResolver {
    pub fn new() -> Self {
        Self {
            query_ok: true,
            cache_ok: true,
            recurse_ok: true,
            validate_ok: true,
            zone_ok: true,
        }
    }

    pub fn resolution_ok(&self) -> bool {
        self.query_ok && self.cache_ok && self.recurse_ok
    }

    pub fn security_ok(&self) -> bool {
        self.validate_ok && self.zone_ok
    }

    pub fn all_ok(&self) -> bool {
        self.resolution_ok() && self.security_ok()
    }

    pub fn needs_flush(&self) -> bool {
        !self.cache_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.query_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution() {
        let c = DnsResolver::new();
        assert!(c.resolution_ok());
    }

    #[test]
    fn test_security() {
        let c = DnsResolver::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DnsResolver::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_flush() {
        let c = DnsResolver::new();
        assert!(!c.needs_flush());
    }

    #[test]
    fn test_cache() {
        let mut c = DnsResolver::new();
        c.cache_ok = false;
        assert!(c.needs_flush());
    }

    #[test]
    fn test_health() {
        let c = DnsResolver::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
