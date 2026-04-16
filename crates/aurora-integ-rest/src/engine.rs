/// integ rest: get, post, put, delete, log
/// Phase 1644

#[derive(Debug, Clone)]
pub struct IntegRest {
    pub get_ok: bool,
    pub post_ok: bool,
    pub put_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegRest {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegRest {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            post_ok: true,
            put_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.post_ok && self.put_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.get_ok || !self.post_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.get_ok {
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
        let c = IntegRest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegRest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegRest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegRest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegRest::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegRest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
