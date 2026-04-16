/// aurora-cloud-cache: cloud cache
/// Phase 2549

#[derive(Debug, Clone)]
pub struct CloudCache {
    pub set_ok: bool,
    pub get_ok: bool,
    pub expire_ok: bool,
    pub cluster_ok: bool,
    pub monitor_ok: bool,
}

impl Default for CloudCache {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudCache {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            get_ok: true,
            expire_ok: true,
            cluster_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.get_ok && self.expire_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cluster_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = CloudCache::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudCache::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudCache::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudCache::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudCache::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudCache::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = CloudCache::default();
        assert!(c.all_ok());
    }
}
