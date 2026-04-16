/// aurora-event-store2: event store2
/// Phase 2582

#[derive(Debug, Clone)]
pub struct EventStore2 {
    pub append_ok: bool,
    pub read_ok: bool,
    pub snapshot_ok: bool,
    pub compact_ok: bool,
    pub archive_ok: bool,
}

impl Default for EventStore2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EventStore2 {
    pub fn new() -> Self {
        Self {
            append_ok: true,
            read_ok: true,
            snapshot_ok: true,
            compact_ok: true,
            archive_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.append_ok && self.read_ok && self.snapshot_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compact_ok && self.archive_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.append_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.append_ok {
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
        let c = EventStore2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventStore2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventStore2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventStore2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventStore2::new();
        c.append_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventStore2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventStore2::default();
        assert!(c.all_ok());
    }
}
