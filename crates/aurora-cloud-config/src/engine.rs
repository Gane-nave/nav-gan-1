/// cloud config: fetch, parse, validate, apply, log
/// Phase 1449

#[derive(Debug, Clone)]
pub struct CloudConfig {
    pub fetch_ok: bool,
    pub parse_ok: bool,
    pub validate_ok: bool,
    pub apply_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudConfig {
    pub fn new() -> Self {
        Self {
            fetch_ok: true,
            parse_ok: true,
            validate_ok: true,
            apply_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fetch_ok && self.parse_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.apply_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fetch_ok || !self.parse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fetch_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudConfig::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudConfig::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudConfig::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudConfig::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudConfig::new();
        c.fetch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudConfig::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
