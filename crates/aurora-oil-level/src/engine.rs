/// Oil level sensor: probe, heater, signal
/// Phase 694

#[derive(Debug, Clone)]
pub struct OilLevel {
    pub probe_ok: bool,
    pub heater_ok: bool,
    pub signal_ok: bool,
    pub connector_ok: bool,
    pub calibrated: bool,
}

impl Default for OilLevel {
    fn default() -> Self {
        Self::new()
    }
}

impl OilLevel {
    pub fn new() -> Self {
        Self {
            probe_ok: true,
            heater_ok: true,
            signal_ok: true,
            connector_ok: true,
            calibrated: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.probe_ok && self.heater_ok && self.signal_ok
    }

    pub fn connection_ok(&self) -> bool {
        self.connector_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.connection_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.probe_ok || !self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.probe_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = OilLevel::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_connection() {
        let c = OilLevel::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilLevel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = OilLevel::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_probe() {
        let mut c = OilLevel::new();
        c.probe_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = OilLevel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
