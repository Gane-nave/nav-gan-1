/// immobilizer: challenge, respond, verify, enable, log
/// Phase 1286

#[derive(Debug, Clone)]
pub struct Immobilizer {
    pub challenge_ok: bool,
    pub respond_ok: bool,
    pub verify_ok: bool,
    pub enable_ok: bool,
    pub log_ok: bool,
}

impl Default for Immobilizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Immobilizer {
    pub fn new() -> Self {
        Self {
            challenge_ok: true,
            respond_ok: true,
            verify_ok: true,
            enable_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.challenge_ok && self.respond_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.enable_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.challenge_ok || !self.respond_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.challenge_ok {
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
        let c = Immobilizer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Immobilizer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Immobilizer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Immobilizer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Immobilizer::new();
        c.challenge_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Immobilizer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
