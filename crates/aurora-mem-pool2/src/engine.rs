/// mem pool2: allocate, deallocate, grow, shrink, log
/// Phase 2364

#[derive(Debug, Clone)]
pub struct MemPool2 {
    pub allocate_ok: bool,
    pub deallocate_ok: bool,
    pub grow_ok: bool,
    pub shrink_ok: bool,
    pub log_ok: bool,
}

impl Default for MemPool2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemPool2 {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            deallocate_ok: true,
            grow_ok: true,
            shrink_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.deallocate_ok && self.grow_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.shrink_ok && self.log_ok
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
        let c = MemPool2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemPool2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemPool2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemPool2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemPool2::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemPool2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
