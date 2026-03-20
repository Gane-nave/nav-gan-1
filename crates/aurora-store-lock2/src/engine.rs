/// store lock2: acquire, release, renew, check, log
/// Phase 1983

#[derive(Debug, Clone)]
pub struct StoreLock2 {
    pub acquire_ok: bool,
    pub release_ok: bool,
    pub renew_ok: bool,
    pub check_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreLock2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreLock2 {
    pub fn new() -> Self {
        Self {
            acquire_ok: true,
            release_ok: true,
            renew_ok: true,
            check_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.acquire_ok && self.release_ok && self.renew_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.check_ok && self.log_ok
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
        let c = StoreLock2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreLock2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreLock2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreLock2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreLock2::new();
        c.acquire_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreLock2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
