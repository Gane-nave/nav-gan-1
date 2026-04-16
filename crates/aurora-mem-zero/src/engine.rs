/// mem zero: allocate, zero, check, free, log
/// Phase 2377

#[derive(Debug, Clone)]
pub struct MemZero {
    pub allocate_ok: bool,
    pub zero_ok: bool,
    pub check_ok: bool,
    pub free_ok: bool,
    pub log_ok: bool,
}

impl Default for MemZero {
    fn default() -> Self {
        Self::new()
    }
}

impl MemZero {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            zero_ok: true,
            check_ok: true,
            free_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.zero_ok && self.check_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.free_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.allocate_ok || !self.zero_ok
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
        let c = MemZero::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemZero::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemZero::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemZero::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemZero::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemZero::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
