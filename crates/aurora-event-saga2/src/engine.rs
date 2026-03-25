/// event saga2: begin, step, compensate, complete, log
/// Phase 1863

#[derive(Debug, Clone)]
pub struct EventSaga2 {
    pub begin_ok: bool,
    pub step_ok: bool,
    pub compensate_ok: bool,
    pub complete_ok: bool,
    pub log_ok: bool,
}

impl Default for EventSaga2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSaga2 {
    pub fn new() -> Self {
        Self {
            begin_ok: true,
            step_ok: true,
            compensate_ok: true,
            complete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.begin_ok && self.step_ok && self.compensate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.begin_ok || !self.step_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.begin_ok {
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
        let c = EventSaga2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventSaga2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventSaga2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventSaga2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventSaga2::new();
        c.begin_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventSaga2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
