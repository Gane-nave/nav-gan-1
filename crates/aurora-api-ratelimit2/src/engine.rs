/// api ratelimit2: check, limit, quota, reset, log
/// Phase 1848

#[derive(Debug, Clone)]
pub struct ApiRatelimit2 {
    pub check_ok: bool,
    pub limit_ok: bool,
    pub quota_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiRatelimit2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiRatelimit2 {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            limit_ok: true,
            quota_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.limit_ok && self.quota_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.limit_ok
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
        let c = ApiRatelimit2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiRatelimit2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiRatelimit2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiRatelimit2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiRatelimit2::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiRatelimit2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
