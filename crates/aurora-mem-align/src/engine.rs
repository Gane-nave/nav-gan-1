/// mem align: allocate, check, adjust, free, log
/// Phase 2376

#[derive(Debug, Clone)]
pub struct MemAlign {
    pub allocate_ok: bool,
    pub check_ok: bool,
    pub adjust_ok: bool,
    pub free_ok: bool,
    pub log_ok: bool,
}

impl Default for MemAlign {
    fn default() -> Self {
        Self::new()
    }
}

impl MemAlign {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            check_ok: true,
            adjust_ok: true,
            free_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.check_ok && self.adjust_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.free_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.allocate_ok || !self.check_ok
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
        let c = MemAlign::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemAlign::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemAlign::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemAlign::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemAlign::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemAlign::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
