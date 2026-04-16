/// mem cache3: get, put, evict, stats, log
/// Phase 2371

#[derive(Debug, Clone)]
pub struct MemCache3 {
    pub get_ok: bool,
    pub put_ok: bool,
    pub evict_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for MemCache3 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemCache3 {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            put_ok: true,
            evict_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.put_ok && self.evict_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
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
        let c = MemCache3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemCache3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemCache3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemCache3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemCache3::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemCache3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
