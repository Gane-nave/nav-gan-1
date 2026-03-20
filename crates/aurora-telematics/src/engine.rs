/// Telematics: cellular connectivity, data upload, remote diagnostics
/// Phase 282

#[derive(Debug, Clone)]
pub struct Telematics {
    pub cellular_connected: bool,
    pub signal_bars: u8,
    pub data_usage_mb: f64,
    pub gps_fix: bool,
    pub modem_ok: bool,
    pub sim_ok: bool,
}

impl Default for Telematics {
    fn default() -> Self {
        Self::new()
    }
}

impl Telematics {
    pub fn new() -> Self {
        Self {
            cellular_connected: true,
            signal_bars: 4,
            data_usage_mb: 50.0,
            gps_fix: true,
            modem_ok: true,
            sim_ok: true,
        }
    }

    pub fn online(&self) -> bool {
        self.cellular_connected && self.modem_ok && self.sim_ok
    }

    pub fn signal_strong(&self) -> bool {
        self.signal_bars >= 3
    }

    pub fn can_upload(&self) -> bool {
        self.online() && self.signal_strong()
    }

    pub fn needs_service(&self) -> bool {
        !self.modem_ok || !self.sim_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.modem_ok {
            return 0.0;
        }
        if !self.sim_ok {
            return 20.0;
        }
        if !self.cellular_connected {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_online() {
        let t = Telematics::new();
        assert!(t.online());
    }

    #[test]
    fn test_signal() {
        let t = Telematics::new();
        assert!(t.signal_strong());
    }

    #[test]
    fn test_upload() {
        let t = Telematics::new();
        assert!(t.can_upload());
    }

    #[test]
    fn test_no_service() {
        let t = Telematics::new();
        assert!(!t.needs_service());
    }

    #[test]
    fn test_offline() {
        let mut t = Telematics::new();
        t.cellular_connected = false;
        assert!(!t.online());
    }

    #[test]
    fn test_health() {
        let t = Telematics::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
