/// runtime semaphore2: acquire, release, permits, close, log
/// Phase 1803

#[derive(Debug, Clone)]
pub struct RuntimeSemaphore2 {
    pub acquire_ok: bool,
    pub release_ok: bool,
    pub permits_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeSemaphore2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeSemaphore2 {
    pub fn new() -> Self {
        Self {
            acquire_ok: true,
            release_ok: true,
            permits_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.acquire_ok && self.release_ok && self.permits_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
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
        let c = RuntimeSemaphore2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeSemaphore2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeSemaphore2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeSemaphore2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeSemaphore2::new();
        c.acquire_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeSemaphore2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
