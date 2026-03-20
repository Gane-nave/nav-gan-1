/// queue delay2: schedule, poll, cancel, size, log
/// Phase 1917

#[derive(Debug, Clone)]
pub struct QueueDelay2 {
    pub schedule_ok: bool,
    pub poll_ok: bool,
    pub cancel_ok: bool,
    pub size_ok: bool,
    pub log_ok: bool,
}

impl Default for QueueDelay2 {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueDelay2 {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            poll_ok: true,
            cancel_ok: true,
            size_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.poll_ok && self.cancel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.size_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.poll_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schedule_ok {
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
        let c = QueueDelay2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = QueueDelay2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QueueDelay2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = QueueDelay2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = QueueDelay2::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = QueueDelay2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
