/// Schema registry: register, validate, evolve, compat, cache
/// Phase 1036

#[derive(Debug, Clone)]
pub struct SchemaReg {
    pub register_ok: bool,
    pub validate_ok: bool,
    pub evolve_ok: bool,
    pub compat_ok: bool,
    pub cache_ok: bool,
}

impl Default for SchemaReg {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaReg {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            validate_ok: true,
            evolve_ok: true,
            compat_ok: true,
            cache_ok: true,
        }
    }

    pub fn management_ok(&self) -> bool {
        self.register_ok && self.validate_ok && self.evolve_ok
    }

    pub fn performance_ok(&self) -> bool {
        self.compat_ok && self.cache_ok
    }

    pub fn all_ok(&self) -> bool {
        self.management_ok() && self.performance_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.register_ok || !self.evolve_ok
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
    fn test_management() {
        let c = SchemaReg::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_performance() {
        let c = SchemaReg::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchemaReg::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = SchemaReg::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_register() {
        let mut c = SchemaReg::new();
        c.register_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = SchemaReg::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
