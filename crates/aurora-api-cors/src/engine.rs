/// api cors: check, allow, deny, preflight, log
/// Phase 1856

#[derive(Debug, Clone)]
pub struct ApiCors {
    pub check_ok: bool,
    pub allow_ok: bool,
    pub deny_ok: bool,
    pub preflight_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiCors {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiCors {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            allow_ok: true,
            deny_ok: true,
            preflight_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.allow_ok && self.deny_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.preflight_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.allow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = ApiCors::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiCors::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiCors::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiCors::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiCors::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiCors::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
