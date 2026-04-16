/// rt timer2: schedule, fire, cancel, reset, log
/// Phase 2336

#[derive(Debug, Clone)]
pub struct RtTimer2 {
    pub schedule_ok: bool,
    pub fire_ok: bool,
    pub cancel_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for RtTimer2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtTimer2 {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            fire_ok: true,
            cancel_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.fire_ok && self.cancel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.fire_ok
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
        let c = RtTimer2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtTimer2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtTimer2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtTimer2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtTimer2::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtTimer2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
