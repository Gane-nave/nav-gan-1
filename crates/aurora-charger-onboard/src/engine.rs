/// charger onboard: connect, negotiate, convert, limit, log
/// Phase 1362

#[derive(Debug, Clone)]
pub struct ChargerOnboard {
    pub connect_ok: bool,
    pub negotiate_ok: bool,
    pub convert_ok: bool,
    pub limit_ok: bool,
    pub log_ok: bool,
}

impl Default for ChargerOnboard {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargerOnboard {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            negotiate_ok: true,
            convert_ok: true,
            limit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.negotiate_ok && self.convert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.limit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.negotiate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ChargerOnboard::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChargerOnboard::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargerOnboard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChargerOnboard::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChargerOnboard::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChargerOnboard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
