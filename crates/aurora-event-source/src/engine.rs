/// aurora-event-source: event source
/// Phase 2576

#[derive(Debug, Clone)]
pub struct EventSource {
    pub poll_ok: bool,
    pub push_ok: bool,
    pub transform_ok: bool,
    pub filter_ok: bool,
    pub monitor_ok: bool,
}

impl Default for EventSource {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSource {
    pub fn new() -> Self {
        Self {
            poll_ok: true,
            push_ok: true,
            transform_ok: true,
            filter_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.poll_ok && self.push_ok && self.transform_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.filter_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.poll_ok || !self.push_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.poll_ok {
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
        let c = EventSource::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventSource::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventSource::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventSource::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventSource::new();
        c.poll_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventSource::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventSource::default();
        assert!(c.all_ok());
    }
}
