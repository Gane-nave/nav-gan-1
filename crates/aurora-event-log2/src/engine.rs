/// aurora-event-log2: event log2
/// Phase 2583

#[derive(Debug, Clone)]
pub struct EventLog2 {
    pub write_ok: bool,
    pub read_ok: bool,
    pub rotate_ok: bool,
    pub archive_ok: bool,
    pub query_ok: bool,
}

impl Default for EventLog2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EventLog2 {
    pub fn new() -> Self {
        Self {
            write_ok: true,
            read_ok: true,
            rotate_ok: true,
            archive_ok: true,
            query_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.write_ok && self.read_ok && self.rotate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.archive_ok && self.query_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.write_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.write_ok {
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
        let c = EventLog2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventLog2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventLog2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventLog2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventLog2::new();
        c.write_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventLog2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventLog2::default();
        assert!(c.all_ok());
    }
}
