/// GNSS receiver: antenna, LNA, correlator, PVT
/// Phase 704

#[derive(Debug, Clone)]
pub struct GnssReceiver {
    pub antenna_ok: bool,
    pub lna_ok: bool,
    pub correlator_ok: bool,
    pub pvt_ok: bool,
    pub rtcm_ok: bool,
}

impl Default for GnssReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl GnssReceiver {
    pub fn new() -> Self {
        Self {
            antenna_ok: true,
            lna_ok: true,
            correlator_ok: true,
            pvt_ok: true,
            rtcm_ok: true,
        }
    }

    pub fn frontend_ok(&self) -> bool {
        self.antenna_ok && self.lna_ok
    }

    pub fn processing_ok(&self) -> bool {
        self.correlator_ok && self.pvt_ok
    }

    pub fn all_ok(&self) -> bool {
        self.frontend_ok() && self.processing_ok() && self.rtcm_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.antenna_ok || !self.correlator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.antenna_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontend() {
        let c = GnssReceiver::new();
        assert!(c.frontend_ok());
    }

    #[test]
    fn test_processing() {
        let c = GnssReceiver::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GnssReceiver::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = GnssReceiver::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_antenna() {
        let mut c = GnssReceiver::new();
        c.antenna_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = GnssReceiver::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
