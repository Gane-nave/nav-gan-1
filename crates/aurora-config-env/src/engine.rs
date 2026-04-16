/// config env: load, get, set, validate, log
/// Phase 1770

#[derive(Debug, Clone)]
pub struct ConfigEnv {
    pub load_ok: bool,
    pub get_ok: bool,
    pub set_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for ConfigEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigEnv {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            get_ok: true,
            set_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.get_ok && self.set_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = ConfigEnv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ConfigEnv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConfigEnv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ConfigEnv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ConfigEnv::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ConfigEnv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
