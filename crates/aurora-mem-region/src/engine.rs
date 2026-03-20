/// mem region: alloc, free, merge, split, log
/// Phase 1945

#[derive(Debug, Clone)]
pub struct MemRegion {
    pub alloc_ok: bool,
    pub free_ok: bool,
    pub merge_ok: bool,
    pub split_ok: bool,
    pub log_ok: bool,
}

impl Default for MemRegion {
    fn default() -> Self {
        Self::new()
    }
}

impl MemRegion {
    pub fn new() -> Self {
        Self {
            alloc_ok: true,
            free_ok: true,
            merge_ok: true,
            split_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.alloc_ok && self.free_ok && self.merge_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.split_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.alloc_ok || !self.free_ok
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
        let c = MemRegion::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemRegion::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemRegion::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemRegion::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemRegion::new();
        c.alloc_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemRegion::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
