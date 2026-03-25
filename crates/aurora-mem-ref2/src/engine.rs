/// mem ref2: acquire, release, count, weak, log
/// Phase 2373

#[derive(Debug, Clone)]
pub struct MemRef2 {
    pub acquire_ok: bool,
    pub release_ok: bool,
    pub count_ok: bool,
    pub weak_ok: bool,
    pub log_ok: bool,
}

impl Default for MemRef2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemRef2 {
    pub fn new() -> Self {
        Self {
            acquire_ok: true,
            release_ok: true,
            count_ok: true,
            weak_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.acquire_ok && self.release_ok && self.count_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.weak_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.acquire_ok || !self.release_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.acquire_ok {
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
        let c = MemRef2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemRef2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemRef2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemRef2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemRef2::new();
        c.acquire_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemRef2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
