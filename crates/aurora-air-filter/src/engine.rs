/// Air filter: filtration efficiency, restriction, service
/// Phase 508

#[derive(Debug, Clone)]
pub struct AirFilter {
    pub restriction_kpa: f64,
    pub max_restriction_kpa: f64,
    pub efficiency_pct: f64,
    pub service_km: f64,
    pub max_service_km: f64,
}

impl Default for AirFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl AirFilter {
    pub fn new() -> Self {
        Self {
            restriction_kpa: 1.5,
            max_restriction_kpa: 5.0,
            efficiency_pct: 99.0,
            service_km: 5000.0,
            max_service_km: 30000.0,
        }
    }

    pub fn restriction_ok(&self) -> bool {
        self.restriction_kpa < self.max_restriction_kpa
    }

    pub fn efficient(&self) -> bool {
        self.efficiency_pct > 95.0
    }

    pub fn all_ok(&self) -> bool {
        self.restriction_ok() && self.efficient()
    }

    pub fn needs_replacement(&self) -> bool {
        self.restriction_kpa > self.max_restriction_kpa || self.service_km > self.max_service_km
    }

    pub fn health_score(&self) -> f64 {
        if self.restriction_kpa > self.max_restriction_kpa {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restriction() {
        let c = AirFilter::new();
        assert!(c.restriction_ok());
    }

    #[test]
    fn test_efficient() {
        let c = AirFilter::new();
        assert!(c.efficient());
    }

    #[test]
    fn test_all_ok() {
        let c = AirFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = AirFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_high_restrict() {
        let mut c = AirFilter::new();
        c.restriction_kpa = 6.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = AirFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
