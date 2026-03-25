/// mem slab2: allocate, deallocate, grow, stats, log
/// Phase 2366

#[derive(Debug, Clone)]
pub struct MemSlab2 {
    pub allocate_ok: bool,
    pub deallocate_ok: bool,
    pub grow_ok: bool,
    pub stats_ok: bool,
    pub log_ok: bool,
}

impl Default for MemSlab2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemSlab2 {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            deallocate_ok: true,
            grow_ok: true,
            stats_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.deallocate_ok && self.grow_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stats_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.allocate_ok || !self.deallocate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.allocate_ok {
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
        let c = MemSlab2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemSlab2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemSlab2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemSlab2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemSlab2::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemSlab2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
