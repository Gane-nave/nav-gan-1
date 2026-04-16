/// devops scan3: vulnerability, license, secret, report, log
/// Phase 2171

#[derive(Debug, Clone)]
pub struct DevopsScan3 {
    pub vulnerability_ok: bool,
    pub license_ok: bool,
    pub secret_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsScan3 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsScan3 {
    pub fn new() -> Self {
        Self {
            vulnerability_ok: true,
            license_ok: true,
            secret_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.vulnerability_ok && self.license_ok && self.secret_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.vulnerability_ok || !self.license_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.vulnerability_ok {
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
        let c = DevopsScan3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsScan3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsScan3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsScan3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsScan3::new();
        c.vulnerability_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsScan3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
