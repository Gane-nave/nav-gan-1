/// sync barrier2: wait, reset, count, leader, log
/// Phase 2386

#[derive(Debug, Clone)]
pub struct SyncBarrier2 {
    pub wait_ok: bool,
    pub reset_ok: bool,
    pub count_ok: bool,
    pub leader_ok: bool,
    pub log_ok: bool,
}

impl Default for SyncBarrier2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncBarrier2 {
    pub fn new() -> Self {
        Self {
            wait_ok: true,
            reset_ok: true,
            count_ok: true,
            leader_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.wait_ok && self.reset_ok && self.count_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.leader_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.wait_ok || !self.reset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wait_ok {
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
        let c = SyncBarrier2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SyncBarrier2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SyncBarrier2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SyncBarrier2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SyncBarrier2::new();
        c.wait_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SyncBarrier2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
