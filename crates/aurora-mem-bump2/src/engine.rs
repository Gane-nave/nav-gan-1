/// mem bump2: allocate, reset, checkpoint, restore, log
/// Phase 2367

#[derive(Debug, Clone)]
pub struct MemBump2 {
    pub allocate_ok: bool,
    pub reset_ok: bool,
    pub checkpoint_ok: bool,
    pub restore_ok: bool,
    pub log_ok: bool,
}

impl Default for MemBump2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemBump2 {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            reset_ok: true,
            checkpoint_ok: true,
            restore_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.reset_ok && self.checkpoint_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.restore_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.allocate_ok || !self.reset_ok
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
        let c = MemBump2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemBump2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemBump2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemBump2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemBump2::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemBump2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
