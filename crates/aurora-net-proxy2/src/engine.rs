/// net proxy2: forward, filter, cache, balance, log
/// Phase 2263

#[derive(Debug, Clone)]
pub struct NetProxy2 {
    pub forward_ok: bool,
    pub filter_ok: bool,
    pub cache_ok: bool,
    pub balance_ok: bool,
    pub log_ok: bool,
}

impl Default for NetProxy2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetProxy2 {
    pub fn new() -> Self {
        Self {
            forward_ok: true,
            filter_ok: true,
            cache_ok: true,
            balance_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.forward_ok && self.filter_ok && self.cache_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.balance_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.forward_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.forward_ok {
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
        let c = NetProxy2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetProxy2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetProxy2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetProxy2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetProxy2::new();
        c.forward_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetProxy2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
