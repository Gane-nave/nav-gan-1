/// event replay: start, pause, seek, complete, log
/// Phase 1870

#[derive(Debug, Clone)]
pub struct EventReplay {
    pub start_ok: bool,
    pub pause_ok: bool,
    pub seek_ok: bool,
    pub complete_ok: bool,
    pub log_ok: bool,
}

impl Default for EventReplay {
    fn default() -> Self {
        Self::new()
    }
}

impl EventReplay {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            pause_ok: true,
            seek_ok: true,
            complete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.pause_ok && self.seek_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.pause_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
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
        let c = EventReplay::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventReplay::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventReplay::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventReplay::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventReplay::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventReplay::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
