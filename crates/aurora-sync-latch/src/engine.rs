/// sync latch: countdown, wait, reset, check, log
/// Phase 2388

#[derive(Debug, Clone)]
pub struct SyncLatch {
    pub countdown_ok: bool,
    pub wait_ok: bool,
    pub reset_ok: bool,
    pub check_ok: bool,
    pub log_ok: bool,
}

impl Default for SyncLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncLatch {
    pub fn new() -> Self {
        Self {
            countdown_ok: true,
            wait_ok: true,
            reset_ok: true,
            check_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.countdown_ok && self.wait_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.check_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.countdown_ok || !self.wait_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.countdown_ok {
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
        let c = SyncLatch::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SyncLatch::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SyncLatch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SyncLatch::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SyncLatch::new();
        c.countdown_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SyncLatch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
