/// mem mapped2: map, unmap, sync, protect, log
/// Phase 2370

#[derive(Debug, Clone)]
pub struct MemMapped2 {
    pub map_ok: bool,
    pub unmap_ok: bool,
    pub sync_ok: bool,
    pub protect_ok: bool,
    pub log_ok: bool,
}

impl Default for MemMapped2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemMapped2 {
    pub fn new() -> Self {
        Self {
            map_ok: true,
            unmap_ok: true,
            sync_ok: true,
            protect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.map_ok && self.unmap_ok && self.sync_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.protect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.map_ok || !self.unmap_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.map_ok {
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
        let c = MemMapped2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemMapped2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemMapped2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemMapped2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemMapped2::new();
        c.map_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemMapped2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
