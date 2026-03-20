/// key prog: read, clone, program, validate, log
/// Phase 1391

#[derive(Debug, Clone)]
pub struct KeyProg {
    pub read_ok: bool,
    pub clone_ok: bool,
    pub program_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for KeyProg {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyProg {
    pub fn new() -> Self {
        Self {
            read_ok: true,
            clone_ok: true,
            program_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.read_ok && self.clone_ok && self.program_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.read_ok || !self.clone_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.read_ok {
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
        let c = KeyProg::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = KeyProg::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KeyProg::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = KeyProg::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = KeyProg::new();
        c.read_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = KeyProg::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
