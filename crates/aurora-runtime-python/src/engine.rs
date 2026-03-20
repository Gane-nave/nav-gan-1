/// runtime python: create, execute, import, gc, log
/// Phase 1791

#[derive(Debug, Clone)]
pub struct RuntimePython {
    pub create_ok: bool,
    pub execute_ok: bool,
    pub import_ok: bool,
    pub gc_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimePython {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimePython {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            execute_ok: true,
            import_ok: true,
            gc_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.execute_ok && self.import_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.gc_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.execute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = RuntimePython::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimePython::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimePython::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimePython::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimePython::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimePython::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
