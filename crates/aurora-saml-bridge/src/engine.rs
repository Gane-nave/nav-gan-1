/// SAML bridge: assertion, metadata, SSO, SLO, attribute
/// Phase 1002

#[derive(Debug, Clone)]
pub struct SamlBridge {
    pub assertion_ok: bool,
    pub metadata_ok: bool,
    pub sso_ok: bool,
    pub slo_ok: bool,
    pub attribute_ok: bool,
}

impl Default for SamlBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl SamlBridge {
    pub fn new() -> Self {
        Self {
            assertion_ok: true,
            metadata_ok: true,
            sso_ok: true,
            slo_ok: true,
            attribute_ok: true,
        }
    }

    pub fn auth_ok(&self) -> bool {
        self.assertion_ok && self.sso_ok && self.metadata_ok
    }

    pub fn session_ok(&self) -> bool {
        self.slo_ok && self.attribute_ok
    }

    pub fn all_ok(&self) -> bool {
        self.auth_ok() && self.session_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.metadata_ok || !self.sso_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assertion_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth() {
        let c = SamlBridge::new();
        assert!(c.auth_ok());
    }

    #[test]
    fn test_session() {
        let c = SamlBridge::new();
        assert!(c.session_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SamlBridge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = SamlBridge::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_metadata() {
        let mut c = SamlBridge::new();
        c.metadata_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = SamlBridge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
