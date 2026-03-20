/// Immobilizer: transponder, ECU match, relay, antenna ring
/// Phase 733

#[derive(Debug, Clone)]
pub struct Immobilizer {
    pub transponder_ok: bool,
    pub ecu_match_ok: bool,
    pub relay_ok: bool,
    pub antenna_ring_ok: bool,
    pub enabled: bool,
}

impl Default for Immobilizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Immobilizer {
    pub fn new() -> Self {
        Self {
            transponder_ok: true,
            ecu_match_ok: true,
            relay_ok: true,
            antenna_ring_ok: true,
            enabled: true,
        }
    }

    pub fn authentication_ok(&self) -> bool {
        self.transponder_ok && self.ecu_match_ok
    }

    pub fn hardware_ok(&self) -> bool {
        self.relay_ok && self.antenna_ring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.authentication_ok() && self.hardware_ok() && self.enabled
    }

    pub fn needs_service(&self) -> bool {
        !self.transponder_ok || !self.ecu_match_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.transponder_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication() {
        let c = Immobilizer::new();
        assert!(c.authentication_ok());
    }

    #[test]
    fn test_hardware() {
        let c = Immobilizer::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Immobilizer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Immobilizer::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_transponder() {
        let mut c = Immobilizer::new();
        c.transponder_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Immobilizer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
