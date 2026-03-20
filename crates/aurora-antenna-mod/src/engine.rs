/// Antenna module: AM/FM, GPS, cellular, satellite
/// Phase 560

#[derive(Debug, Clone)]
pub struct AntennaModule {
    pub am_fm_ok: bool,
    pub gps_ok: bool,
    pub cellular_ok: bool,
    pub satellite_ok: bool,
    pub cable_ok: bool,
}

impl Default for AntennaModule {
    fn default() -> Self {
        Self::new()
    }
}

impl AntennaModule {
    pub fn new() -> Self {
        Self {
            am_fm_ok: true,
            gps_ok: true,
            cellular_ok: true,
            satellite_ok: true,
            cable_ok: true,
        }
    }

    pub fn radio_ok(&self) -> bool {
        self.am_fm_ok && self.cable_ok
    }

    pub fn nav_ok(&self) -> bool {
        self.gps_ok && self.satellite_ok
    }

    pub fn all_ok(&self) -> bool {
        self.radio_ok() && self.nav_ok() && self.cellular_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.cable_ok || !self.gps_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cable_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio() {
        let c = AntennaModule::new();
        assert!(c.radio_ok());
    }

    #[test]
    fn test_nav() {
        let c = AntennaModule::new();
        assert!(c.nav_ok());
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
    fn test_cable() {
        let mut c = AntennaModule::new();
        c.cable_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AntennaModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
