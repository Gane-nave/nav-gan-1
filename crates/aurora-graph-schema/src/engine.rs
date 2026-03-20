/// graph schema: define, migrate, validate, export, log
/// Phase 1903

#[derive(Debug, Clone)]
pub struct GraphSchema {
    pub define_ok: bool,
    pub migrate_ok: bool,
    pub validate_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphSchema {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            migrate_ok: true,
            validate_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.migrate_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.migrate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = GraphSchema::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphSchema::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphSchema::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphSchema::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphSchema::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphSchema::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
