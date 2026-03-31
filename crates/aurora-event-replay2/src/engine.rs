/// aurora-event-replay2: event replay2
/// Phase 2580

#[derive(Debug, Clone)]
pub struct EventReplay2 {
    pub seek_ok: bool,
    pub play_ok: bool,
    pub speed_ok: bool,
    pub filter_ok: bool,
    pub export_ok: bool,
}

impl Default for EventReplay2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EventReplay2 {
    pub fn new() -> Self {
        Self {
            seek_ok: true,
            play_ok: true,
            speed_ok: true,
            filter_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.seek_ok && self.play_ok && self.speed_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.filter_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.seek_ok || !self.play_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.seek_ok {
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
        let c = EventReplay2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventReplay2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventReplay2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventReplay2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventReplay2::new();
        c.seek_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventReplay2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventReplay2::default();
        assert!(c.all_ok());
    }
}
