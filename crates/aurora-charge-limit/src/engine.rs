/// Charge limit: max SOC, min SOC, profile, override
/// Phase 879

#[derive(Debug, Clone)]
pub struct ChargeLimit {
    pub max_soc_ok: bool,
    pub min_soc_ok: bool,
    pub profile_ok: bool,
    pub override_ok: bool,
    pub longevity_ok: bool,
}

impl Default for ChargeLimit {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargeLimit {
    pub fn new() -> Self {
        Self {
            max_soc_ok: true,
            min_soc_ok: true,
            profile_ok: true,
            override_ok: true,
            longevity_ok: true,
        }
    }

    pub fn limits_ok(&self) -> bool {
        self.max_soc_ok && self.min_soc_ok && self.longevity_ok
    }

    pub fn management_ok(&self) -> bool {
        self.profile_ok && self.override_ok
    }

    pub fn all_ok(&self) -> bool {
        self.limits_ok() && self.management_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.max_soc_ok || !self.profile_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.max_soc_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limits() {
        let c = ChargeLimit::new();
        assert!(c.limits_ok());
    }

    #[test]
    fn test_management() {
        let c = ChargeLimit::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargeLimit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = ChargeLimit::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_max_soc() {
        let mut c = ChargeLimit::new();
        c.max_soc_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = ChargeLimit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
