/// Oil filter: bypass valve, anti-drain, media
/// Phase 571

#[derive(Debug, Clone)]
pub struct OilFilter {
    pub bypass_ok: bool,
    pub anti_drain_ok: bool,
    pub media_ok: bool,
    pub service_km: f64,
    pub max_service_km: f64,
}

impl Default for OilFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl OilFilter {
    pub fn new() -> Self {
        Self {
            bypass_ok: true,
            anti_drain_ok: true,
            media_ok: true,
            service_km: 5000.0,
            max_service_km: 15000.0,
        }
    }

    pub fn valve_ok(&self) -> bool {
        self.bypass_ok && self.anti_drain_ok
    }

    pub fn filter_ok(&self) -> bool {
        self.media_ok
    }

    pub fn all_ok(&self) -> bool {
        self.valve_ok() && self.filter_ok() && self.service_km < self.max_service_km
    }

    pub fn needs_replacement(&self) -> bool {
        self.service_km > self.max_service_km || !self.media_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.media_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valve() {
        let c = OilFilter::new();
        assert!(c.valve_ok());
    }

    #[test]
    fn test_filter() {
        let c = OilFilter::new();
        assert!(c.filter_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = OilFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_worn() {
        let mut c = OilFilter::new();
        c.service_km = 20000.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = OilFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
