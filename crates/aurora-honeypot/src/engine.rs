/// Honeypot: decoy, trap, log, analyze, alert
/// Phase 1011

#[derive(Debug, Clone)]
pub struct Honeypot {
    pub decoy_ok: bool,
    pub trap_ok: bool,
    pub log_ok: bool,
    pub analyze_ok: bool,
    pub alert_ok: bool,
}

impl Default for Honeypot {
    fn default() -> Self {
        Self::new()
    }
}

impl Honeypot {
    pub fn new() -> Self {
        Self {
            decoy_ok: true,
            trap_ok: true,
            log_ok: true,
            analyze_ok: true,
            alert_ok: true,
        }
    }

    pub fn deception_ok(&self) -> bool {
        self.decoy_ok && self.trap_ok
    }

    pub fn intelligence_ok(&self) -> bool {
        self.log_ok && self.analyze_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.deception_ok() && self.intelligence_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.decoy_ok || !self.trap_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.decoy_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deception() {
        let c = Honeypot::new();
        assert!(c.deception_ok());
    }

    #[test]
    fn test_intelligence() {
        let c = Honeypot::new();
        assert!(c.intelligence_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Honeypot::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = Honeypot::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_decoy() {
        let mut c = Honeypot::new();
        c.decoy_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = Honeypot::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
