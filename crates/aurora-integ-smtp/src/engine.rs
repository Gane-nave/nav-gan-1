/// integ smtp: connect, send, template, retry, log
/// Phase 2244

#[derive(Debug, Clone)]
pub struct IntegSmtp {
    pub connect_ok: bool,
    pub send_ok: bool,
    pub template_ok: bool,
    pub retry_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegSmtp {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegSmtp {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            send_ok: true,
            template_ok: true,
            retry_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.send_ok && self.template_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retry_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.send_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = IntegSmtp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegSmtp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegSmtp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegSmtp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegSmtp::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegSmtp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
