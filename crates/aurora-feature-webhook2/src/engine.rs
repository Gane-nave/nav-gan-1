/// feature webhook2: register, fire, retry, delete, log
/// Phase 1786

#[derive(Debug, Clone)]
pub struct FeatureWebhook2 {
    pub register_ok: bool,
    pub fire_ok: bool,
    pub retry_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for FeatureWebhook2 {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureWebhook2 {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            fire_ok: true,
            retry_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.fire_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.fire_ok
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
    fn test_primary() {
        let c = FeatureWebhook2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FeatureWebhook2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FeatureWebhook2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FeatureWebhook2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FeatureWebhook2::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FeatureWebhook2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
