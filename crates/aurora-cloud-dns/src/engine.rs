/// cloud dns: resolve, cache, update, propagate, log
/// Phase 1458

#[derive(Debug, Clone)]
pub struct CloudDns {
    pub resolve_ok: bool,
    pub cache_ok: bool,
    pub update_ok: bool,
    pub propagate_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudDns {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudDns {
    pub fn new() -> Self {
        Self {
            resolve_ok: true,
            cache_ok: true,
            update_ok: true,
            propagate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.resolve_ok && self.cache_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.propagate_ok && self.log_ok
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
        let c = CloudDns::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudDns::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudDns::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudDns::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudDns::new();
        c.resolve_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudDns::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
