/// runtime v8: create, execute, gc, snapshot, log
/// Phase 1789

#[derive(Debug, Clone)]
pub struct RuntimeV8 {
    pub create_ok: bool,
    pub execute_ok: bool,
    pub gc_ok: bool,
    pub snapshot_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeV8 {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeV8 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            execute_ok: true,
            gc_ok: true,
            snapshot_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.execute_ok && self.gc_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.snapshot_ok && self.log_ok
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
        let c = RuntimeV8::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeV8::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeV8::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeV8::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeV8::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeV8::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
