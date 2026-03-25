/// integ redis3: get, set, delete, subscribe, log
/// Phase 2252

#[derive(Debug, Clone)]
pub struct IntegRedis3 {
    pub get_ok: bool,
    pub set_ok: bool,
    pub delete_ok: bool,
    pub subscribe_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegRedis3 {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegRedis3 {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            set_ok: true,
            delete_ok: true,
            subscribe_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.set_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.subscribe_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.get_ok || !self.set_ok
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
        let c = IntegRedis3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegRedis3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegRedis3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegRedis3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegRedis3::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegRedis3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
