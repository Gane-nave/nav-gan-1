/// rt sync2: execute, wait, notify, cancel, log
/// Phase 2329

#[derive(Debug, Clone)]
pub struct RtSync2 {
    pub execute_ok: bool,
    pub wait_ok: bool,
    pub notify_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for RtSync2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtSync2 {
    pub fn new() -> Self {
        Self {
            execute_ok: true,
            wait_ok: true,
            notify_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.execute_ok && self.wait_ok && self.notify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.execute_ok || !self.wait_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.execute_ok {
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
        let c = RtSync2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtSync2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtSync2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtSync2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtSync2::new();
        c.execute_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtSync2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
