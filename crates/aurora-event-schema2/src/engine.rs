/// event schema2: register, validate, evolve, compat, log
/// Phase 1868

#[derive(Debug, Clone)]
pub struct EventSchema2 {
    pub register_ok: bool,
    pub validate_ok: bool,
    pub evolve_ok: bool,
    pub compat_ok: bool,
    pub log_ok: bool,
}

impl Default for EventSchema2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSchema2 {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            validate_ok: true,
            evolve_ok: true,
            compat_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.validate_ok && self.evolve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compat_ok && self.log_ok
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
        let c = EventSchema2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventSchema2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventSchema2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventSchema2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventSchema2::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventSchema2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
