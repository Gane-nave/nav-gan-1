/// monitor thread: count, state, contention, deadlock, log
/// Phase 1577

#[derive(Debug, Clone)]
pub struct MonitorThread {
    pub count_ok: bool,
    pub state_ok: bool,
    pub contention_ok: bool,
    pub deadlock_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorThread {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorThread {
    pub fn new() -> Self {
        Self {
            count_ok: true,
            state_ok: true,
            contention_ok: true,
            deadlock_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.count_ok && self.state_ok && self.contention_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.deadlock_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.count_ok || !self.state_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.count_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MonitorThread::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorThread::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorThread::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorThread::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorThread::new();
        c.count_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorThread::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
