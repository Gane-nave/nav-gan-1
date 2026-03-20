/// cache lfu: get, put, evict, resize, log
/// Phase 1933

#[derive(Debug, Clone)]
pub struct CacheLfu {
    pub get_ok: bool,
    pub put_ok: bool,
    pub evict_ok: bool,
    pub resize_ok: bool,
    pub log_ok: bool,
}

impl Default for CacheLfu {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheLfu {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            put_ok: true,
            evict_ok: true,
            resize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.put_ok && self.evict_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.get_ok || !self.put_ok
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
        let c = CacheLfu::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CacheLfu::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CacheLfu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CacheLfu::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CacheLfu::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CacheLfu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
