/// Fuel filter: flow rate, restriction, water separator
/// Phase 572

#[derive(Debug, Clone)]
pub struct FuelFilter {
    pub flow_ok: bool,
    pub restriction_ok: bool,
    pub water_sep_ok: bool,
    pub service_km: f64,
    pub max_service_km: f64,
}

impl Default for FuelFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelFilter {
    pub fn new() -> Self {
        Self {
            flow_ok: true,
            restriction_ok: true,
            water_sep_ok: true,
            service_km: 10000.0,
            max_service_km: 40000.0,
        }
    }

    pub fn filtration_ok(&self) -> bool {
        self.flow_ok && self.restriction_ok
    }

    pub fn separator_ok(&self) -> bool {
        self.water_sep_ok
    }

    pub fn all_ok(&self) -> bool {
        self.filtration_ok() && self.separator_ok() && self.service_km < self.max_service_km
    }

    pub fn needs_replacement(&self) -> bool {
        self.service_km > self.max_service_km || !self.flow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flow_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filtration() {
        let c = FuelFilter::new();
        assert!(c.filtration_ok());
    }

    #[test]
    fn test_separator() {
        let c = FuelFilter::new();
        assert!(c.separator_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = FuelFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_worn() {
        let mut c = FuelFilter::new();
        c.service_km = 50000.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = FuelFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
