/// valid ip2: parse, classify, range, subnet, log
/// Phase 2081

#[derive(Debug, Clone)]
pub struct ValidIp2 {
    pub parse_ok: bool,
    pub classify_ok: bool,
    pub range_ok: bool,
    pub subnet_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidIp2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidIp2 {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            classify_ok: true,
            range_ok: true,
            subnet_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.classify_ok && self.range_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.subnet_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = ValidIp2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidIp2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidIp2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidIp2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidIp2::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidIp2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
