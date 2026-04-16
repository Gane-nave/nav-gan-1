/// Telematics unit: modem, SIM, GPS, crash detect
/// Phase 707

#[derive(Debug, Clone)]
pub struct Telematics {
    pub modem_ok: bool,
    pub sim_ok: bool,
    pub gps_ok: bool,
    pub crash_ok: bool,
    pub ota_ok: bool,
}

impl Default for Telematics {
    fn default() -> Self {
        Self::new()
    }
}

impl Telematics {
    pub fn new() -> Self {
        Self {
            modem_ok: true,
            sim_ok: true,
            gps_ok: true,
            crash_ok: true,
            ota_ok: true,
        }
    }

    pub fn connectivity_ok(&self) -> bool {
        self.modem_ok && self.sim_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.crash_ok && self.gps_ok
    }

    pub fn all_ok(&self) -> bool {
        self.connectivity_ok() && self.safety_ok() && self.ota_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.modem_ok || !self.sim_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.modem_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity() {
        let c = Telematics::new();
        assert!(c.connectivity_ok());
    }

    #[test]
    fn test_safety() {
        let c = Telematics::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Telematics::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Telematics::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_modem() {
        let mut c = Telematics::new();
        c.modem_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Telematics::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
