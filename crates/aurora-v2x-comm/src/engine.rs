/// V2X communication: beacon, negotiate, coordinate, warn, log
/// Phase 1101

#[derive(Debug, Clone)]
pub struct V2xComm {
    pub beacon_ok: bool,
    pub negotiate_ok: bool,
    pub coordinate_ok: bool,
    pub warn_ok: bool,
    pub log_ok: bool,
}

impl Default for V2xComm {
    fn default() -> Self {
        Self::new()
    }
}

impl V2xComm {
    pub fn new() -> Self {
        Self {
            beacon_ok: true,
            negotiate_ok: true,
            coordinate_ok: true,
            warn_ok: true,
            log_ok: true,
        }
    }

    pub fn communication_ok(&self) -> bool {
        self.beacon_ok && self.negotiate_ok && self.coordinate_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.warn_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.communication_ok() && self.safety_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.beacon_ok || !self.negotiate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.beacon_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication() {
        let c = V2xComm::new();
        assert!(c.communication_ok());
    }

    #[test]
    fn test_safety() {
        let c = V2xComm::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2xComm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = V2xComm::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_beacon() {
        let mut c = V2xComm::new();
        c.beacon_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = V2xComm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
