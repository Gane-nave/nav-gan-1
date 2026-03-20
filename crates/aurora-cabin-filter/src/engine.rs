/// Cabin air filter: particulate filtering, replacement interval, air quality
/// Phase 233

#[derive(Debug, Clone)]
pub struct CabinFilter {
    pub life_remaining_pct: f64,
    pub mileage_since_change_km: f64,
    pub change_interval_km: f64,
    pub pm25_inside: f64,
    pub pm25_outside: f64,
    pub activated_carbon: bool,
}

impl Default for CabinFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl CabinFilter {
    pub fn new() -> Self {
        Self {
            life_remaining_pct: 70.0,
            mileage_since_change_km: 9000.0,
            change_interval_km: 30000.0,
            pm25_inside: 10.0,
            pm25_outside: 35.0,
            activated_carbon: true,
        }
    }

    pub fn needs_replacement(&self) -> bool {
        self.life_remaining_pct < 10.0 || self.mileage_since_change_km > self.change_interval_km
    }

    pub fn filtering_efficiency(&self) -> f64 {
        if self.pm25_outside <= 0.0 {
            return 100.0;
        }
        ((1.0 - self.pm25_inside / self.pm25_outside) * 100.0).clamp(0.0, 100.0)
    }

    pub fn air_quality_good(&self) -> bool {
        self.pm25_inside < 25.0
    }

    pub fn km_until_change(&self) -> f64 {
        (self.change_interval_km - self.mileage_since_change_km).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_replacement() {
            return 10.0;
        }
        self.life_remaining_pct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_replacement() {
        let c = CabinFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_efficiency() {
        let c = CabinFilter::new();
        assert!(c.filtering_efficiency() > 70.0);
    }

    #[test]
    fn test_air_quality() {
        let c = CabinFilter::new();
        assert!(c.air_quality_good());
    }

    #[test]
    fn test_km_until() {
        let c = CabinFilter::new();
        assert!(c.km_until_change() > 20000.0);
    }

    #[test]
    fn test_old_filter() {
        let mut c = CabinFilter::new();
        c.life_remaining_pct = 5.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CabinFilter::new();
        assert!((c.health_score() - 70.0).abs() < 0.1);
    }
}
