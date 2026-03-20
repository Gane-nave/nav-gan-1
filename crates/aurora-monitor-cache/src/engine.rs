/// monitor cache: hit, miss, evict, size, log
/// Phase 1580

#[derive(Debug, Clone)]
pub struct MonitorCache {
    pub hit_ok: bool,
    pub miss_ok: bool,
    pub evict_ok: bool,
    pub size_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorCache {
    pub fn new() -> Self {
        Self {
            hit_ok: true,
            miss_ok: true,
            evict_ok: true,
            size_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.hit_ok && self.miss_ok && self.evict_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.size_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.hit_ok || !self.miss_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hit_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MonitorCache::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorCache::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorCache::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorCache::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorCache::new();
        c.hit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorCache::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
