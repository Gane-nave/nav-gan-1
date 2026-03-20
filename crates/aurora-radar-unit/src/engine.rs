/// Radar unit: transmitter, receiver, processor, alignment
/// Phase 702

#[derive(Debug, Clone)]
pub struct RadarUnit {
    pub transmitter_ok: bool,
    pub receiver_ok: bool,
    pub processor_ok: bool,
    pub aligned: bool,
    pub calibrated: bool,
}

impl Default for RadarUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl RadarUnit {
    pub fn new() -> Self {
        Self {
            transmitter_ok: true,
            receiver_ok: true,
            processor_ok: true,
            aligned: true,
            calibrated: true,
        }
    }

    pub fn rf_ok(&self) -> bool {
        self.transmitter_ok && self.receiver_ok
    }

    pub fn system_ok(&self) -> bool {
        self.processor_ok && self.aligned && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.rf_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.transmitter_ok || !self.aligned
    }

    pub fn health_score(&self) -> f64 {
        if !self.transmitter_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rf() {
        let c = RadarUnit::new();
        assert!(c.rf_ok());
    }

    #[test]
    fn test_system() {
        let c = RadarUnit::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RadarUnit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RadarUnit::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_transmitter() {
        let mut c = RadarUnit::new();
        c.transmitter_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RadarUnit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
