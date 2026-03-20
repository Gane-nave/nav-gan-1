/// Pollen filter: HEPA, activated carbon, airflow
/// Phase 631

#[derive(Debug, Clone)]
pub struct PollenFilter {
    pub hepa_ok: bool,
    pub carbon_ok: bool,
    pub airflow_ok: bool,
    pub service_km: f64,
    pub max_service_km: f64,
}

impl Default for PollenFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl PollenFilter {
    pub fn new() -> Self {
        Self {
            hepa_ok: true,
            carbon_ok: true,
            airflow_ok: true,
            service_km: 8000.0,
            max_service_km: 20000.0,
        }
    }

    pub fn filtration_ok(&self) -> bool {
        self.hepa_ok && self.carbon_ok
    }

    pub fn flow_ok(&self) -> bool {
        self.airflow_ok
    }

    pub fn all_ok(&self) -> bool {
        self.filtration_ok() && self.flow_ok() && self.service_km < self.max_service_km
    }

    pub fn needs_replacement(&self) -> bool {
        self.service_km > self.max_service_km || !self.hepa_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hepa_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filtration() {
        let c = PollenFilter::new();
        assert!(c.filtration_ok());
    }

    #[test]
    fn test_flow() {
        let c = PollenFilter::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PollenFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = PollenFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_worn() {
        let mut c = PollenFilter::new();
        c.service_km = 25000.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = PollenFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
