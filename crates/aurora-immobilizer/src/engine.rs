/// Immobilizer: transponder, antenna, ECU authorization
/// Phase 527

#[derive(Debug, Clone)]
pub struct Immobilizer {
    pub key_detected: bool,
    pub transponder_ok: bool,
    pub antenna_ok: bool,
    pub authorized: bool,
    pub ecu_ok: bool,
}

impl Default for Immobilizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Immobilizer {
    pub fn new() -> Self {
        Self {
            key_detected: true,
            transponder_ok: true,
            antenna_ok: true,
            authorized: true,
            ecu_ok: true,
        }
    }

    pub fn key_ok(&self) -> bool {
        self.key_detected && self.transponder_ok
    }

    pub fn comm_ok(&self) -> bool {
        self.antenna_ok && self.ecu_ok
    }

    pub fn all_ok(&self) -> bool {
        self.key_ok() && self.comm_ok() && self.authorized
    }

    pub fn needs_service(&self) -> bool {
        !self.ecu_ok || !self.antenna_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ecu_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let c = Immobilizer::new();
        assert!(c.key_ok());
    }

    #[test]
    fn test_comm() {
        let c = Immobilizer::new();
        assert!(c.comm_ok());
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
    fn test_ecu_fail() {
        let mut c = Immobilizer::new();
        c.ecu_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Immobilizer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
