/// aurora-event-schema: event schema
/// Phase 2581

#[derive(Debug, Clone)]
pub struct EventSchema {
    pub register_ok: bool,
    pub validate_ok: bool,
    pub evolve_ok: bool,
    pub compare_ok: bool,
    pub export_ok: bool,
}

impl Default for EventSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSchema {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            validate_ok: true,
            evolve_ok: true,
            compare_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.validate_ok && self.evolve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compare_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = EventSchema::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventSchema::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventSchema::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventSchema::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventSchema::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventSchema::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EventSchema::default();
        assert!(c.all_ok());
    }
}
