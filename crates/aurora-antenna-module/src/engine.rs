/// Antenna module: GPS, cellular, WiFi, Bluetooth
/// Phase 687

#[derive(Debug, Clone)]
pub struct AntennaModule {
    pub gps_ok: bool,
    pub cellular_ok: bool,
    pub wifi_ok: bool,
    pub bluetooth_ok: bool,
    pub ground_ok: bool,
}

impl Default for AntennaModule {
    fn default() -> Self {
        Self::new()
    }
}

impl AntennaModule {
    pub fn new() -> Self {
        Self {
            gps_ok: true,
            cellular_ok: true,
            wifi_ok: true,
            bluetooth_ok: true,
            ground_ok: true,
        }
    }

    pub fn navigation_ok(&self) -> bool {
        self.gps_ok && self.ground_ok
    }

    pub fn connectivity_ok(&self) -> bool {
        self.cellular_ok && self.wifi_ok && self.bluetooth_ok
    }

    pub fn all_ok(&self) -> bool {
        self.navigation_ok() && self.connectivity_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.gps_ok || !self.cellular_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gps_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() {
        let c = AntennaModule::new();
        assert!(c.navigation_ok());
    }

    #[test]
    fn test_connectivity() {
        let c = AntennaModule::new();
        assert!(c.connectivity_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AntennaModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AntennaModule::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_gps() {
        let mut c = AntennaModule::new();
        c.gps_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AntennaModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
