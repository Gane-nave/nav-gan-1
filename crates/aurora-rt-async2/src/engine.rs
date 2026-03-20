/// rt async2: spawn, poll, wake, cancel, log
/// Phase 2328

#[derive(Debug, Clone)]
pub struct RtAsync2 {
    pub spawn_ok: bool,
    pub poll_ok: bool,
    pub wake_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for RtAsync2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtAsync2 {
    pub fn new() -> Self {
        Self {
            spawn_ok: true,
            poll_ok: true,
            wake_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.spawn_ok && self.poll_ok && self.wake_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.spawn_ok || !self.poll_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spawn_ok {
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
        let c = RtAsync2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtAsync2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtAsync2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtAsync2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtAsync2::new();
        c.spawn_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtAsync2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
