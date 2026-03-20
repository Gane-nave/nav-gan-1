/// stream dedup: check, window, key, evict, log
/// Phase 1931

#[derive(Debug, Clone)]
pub struct StreamDedup {
    pub check_ok: bool,
    pub window_ok: bool,
    pub key_ok: bool,
    pub evict_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamDedup {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamDedup {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            window_ok: true,
            key_ok: true,
            evict_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.window_ok && self.key_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.evict_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.window_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = StreamDedup::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamDedup::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamDedup::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamDedup::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamDedup::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamDedup::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
