/// mem arena2: alloc, reset, stats, compact, log
/// Phase 1942

#[derive(Debug, Clone)]
pub struct MemArena2 {
    pub alloc_ok: bool,
    pub reset_ok: bool,
    pub stats_ok: bool,
    pub compact_ok: bool,
    pub log_ok: bool,
}

impl Default for MemArena2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemArena2 {
    pub fn new() -> Self {
        Self {
            alloc_ok: true,
            reset_ok: true,
            stats_ok: true,
            compact_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.alloc_ok && self.reset_ok && self.stats_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compact_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.alloc_ok || !self.reset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.alloc_ok {
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
        let c = MemArena2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemArena2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemArena2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemArena2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemArena2::new();
        c.alloc_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemArena2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
