/// Air purifier: HEPA, ionizer, PM2.5 sensor, AQI display
/// Phase 902

#[derive(Debug, Clone)]
pub struct AirPurifier {
    pub hepa_ok: bool,
    pub ionizer_ok: bool,
    pub pm25_ok: bool,
    pub aqi_ok: bool,
    pub filter_ok: bool,
}

impl Default for AirPurifier {
    fn default() -> Self {
        Self::new()
    }
}

impl AirPurifier {
    pub fn new() -> Self {
        Self {
            hepa_ok: true,
            ionizer_ok: true,
            pm25_ok: true,
            aqi_ok: true,
            filter_ok: true,
        }
    }

    pub fn purification_ok(&self) -> bool {
        self.hepa_ok && self.ionizer_ok && self.filter_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.pm25_ok && self.aqi_ok
    }

    pub fn all_ok(&self) -> bool {
        self.purification_ok() && self.monitoring_ok()
    }

    pub fn needs_filter(&self) -> bool {
        !self.filter_ok || !self.hepa_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.filter_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purification() {
        let c = AirPurifier::new();
        assert!(c.purification_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = AirPurifier::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AirPurifier::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_filter() {
        let c = AirPurifier::new();
        assert!(!c.needs_filter());
    }

    #[test]
    fn test_filter() {
        let mut c = AirPurifier::new();
        c.filter_ok = false;
        assert!(c.needs_filter());
    }

    #[test]
    fn test_health() {
        let c = AirPurifier::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
