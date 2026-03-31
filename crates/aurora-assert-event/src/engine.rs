/// aurora-assert-event: assert event
/// Phase 2511

#[derive(Debug, Clone)]
pub struct AssertEvent {
    pub type_ok: bool,
    pub payload_ok: bool,
    pub order_ok: bool,
    pub timing_ok: bool,
    pub source_ok: bool,
}

impl Default for AssertEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertEvent {
    pub fn new() -> Self {
        Self {
            type_ok: true,
            payload_ok: true,
            order_ok: true,
            timing_ok: true,
            source_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.type_ok && self.payload_ok && self.order_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.timing_ok && self.source_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.type_ok || !self.payload_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.type_ok {
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
        let c = AssertEvent::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertEvent::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertEvent::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertEvent::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertEvent::new();
        c.type_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertEvent::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertEvent::default();
        assert!(c.all_ok());
    }
}
