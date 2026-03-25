/// net coap: request, observe, discover, cache, log
/// Phase 1541

#[derive(Debug, Clone)]
pub struct NetCoap {
    pub request_ok: bool,
    pub observe_ok: bool,
    pub discover_ok: bool,
    pub cache_ok: bool,
    pub log_ok: bool,
}

impl Default for NetCoap {
    fn default() -> Self {
        Self::new()
    }
}

impl NetCoap {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            observe_ok: true,
            discover_ok: true,
            cache_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.observe_ok && self.discover_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cache_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.observe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = NetCoap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetCoap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetCoap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetCoap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetCoap::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetCoap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
