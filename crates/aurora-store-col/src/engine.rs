/// store col: append, read, compact, stats, log
/// Phase 1970

#[derive(Debug, Clone)]
pub struct StoreCol {
    pub append_ok: bool,
    pub read_ok: bool,
    pub compact_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreCol {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreCol {
    pub fn new() -> Self {
        Self {
            append_ok: true,
            read_ok: true,
            compact_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.append_ok && self.read_ok && self.compact_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.append_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.append_ok {
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
        let c = StoreCol::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreCol::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreCol::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreCol::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreCol::new();
        c.append_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreCol::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
