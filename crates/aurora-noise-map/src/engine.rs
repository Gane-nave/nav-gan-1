/// Noise mapping: ambient noise levels, quiet route planning, noise pollution avoidance.
#[derive(Debug, Clone, PartialEq)]
pub enum NoiseSource {
    Traffic,
    Construction,
    Industrial,
    Airport,
    Railway,
    Nightlife,
    Nature,
    Residential,
}

impl NoiseSource {
    pub fn typical_db(&self) -> f64 {
        match self {
            NoiseSource::Airport => 85.0,
            NoiseSource::Construction => 80.0,
            NoiseSource::Industrial => 75.0,
            NoiseSource::Traffic => 70.0,
            NoiseSource::Railway => 75.0,
            NoiseSource::Nightlife => 65.0,
            NoiseSource::Residential => 45.0,
            NoiseSource::Nature => 35.0,
        }
    }

    pub fn is_intermittent(&self) -> bool {
        matches!(
            self,
            NoiseSource::Airport | NoiseSource::Railway | NoiseSource::Construction
        )
    }

    pub fn health_impact(&self) -> f64 {
        let db = self.typical_db();
        if db > 80.0 {
            0.9
        } else if db > 70.0 {
            0.6
        } else if db > 55.0 {
            0.3
        } else {
            0.1
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoiseZone {
    pub source: NoiseSource,
    pub measured_db: f64,
    pub area_km2: f64,
    pub time_of_day_factor: f64,
}

impl NoiseZone {
    pub fn new(source: NoiseSource) -> Self {
        let db = source.typical_db();
        Self {
            source,
            measured_db: db,
            area_km2: 1.0,
            time_of_day_factor: 1.0,
        }
    }

    pub fn effective_db(&self) -> f64 {
        self.measured_db * self.time_of_day_factor
    }

    pub fn noise_category(&self) -> &str {
        let db = self.effective_db();
        if db > 80.0 {
            "Very Loud"
        } else if db > 65.0 {
            "Loud"
        } else if db > 50.0 {
            "Moderate"
        } else if db > 35.0 {
            "Quiet"
        } else {
            "Very Quiet"
        }
    }

    pub fn exceeds_limit(&self, limit_db: f64) -> bool {
        self.effective_db() > limit_db
    }

    pub fn sleep_disruption_risk(&self) -> f64 {
        let db = self.effective_db();
        if db > 55.0 {
            ((db - 55.0) / 30.0).min(1.0)
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoiseRoute {
    pub zones: Vec<NoiseZone>,
}

impl Default for NoiseRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseRoute {
    pub fn new() -> Self {
        Self { zones: Vec::new() }
    }

    pub fn add_zone(&mut self, z: NoiseZone) {
        self.zones.push(z);
    }

    pub fn average_db(&self) -> f64 {
        if self.zones.is_empty() {
            return 0.0;
        }
        let total: f64 = self.zones.iter().map(|z| z.effective_db()).sum();
        total / self.zones.len() as f64
    }

    pub fn max_db(&self) -> f64 {
        self.zones
            .iter()
            .map(|z| z.effective_db())
            .fold(0.0_f64, f64::max)
    }

    pub fn quiet_zone_count(&self) -> usize {
        self.zones
            .iter()
            .filter(|z| z.effective_db() < 50.0)
            .count()
    }

    pub fn loud_zone_count(&self) -> usize {
        self.zones
            .iter()
            .filter(|z| z.effective_db() > 70.0)
            .count()
    }

    pub fn noise_score(&self) -> f64 {
        let avg = self.average_db();
        (100.0 - avg).clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typical_db() {
        assert!(NoiseSource::Airport.typical_db() > NoiseSource::Nature.typical_db());
    }

    #[test]
    fn test_intermittent() {
        assert!(NoiseSource::Airport.is_intermittent());
        assert!(!NoiseSource::Traffic.is_intermittent());
    }

    #[test]
    fn test_health_impact() {
        assert!(NoiseSource::Airport.health_impact() > NoiseSource::Nature.health_impact());
    }

    #[test]
    fn test_effective_db() {
        let mut z = NoiseZone::new(NoiseSource::Traffic);
        z.time_of_day_factor = 0.5;
        assert!(z.effective_db() < z.measured_db);
    }

    #[test]
    fn test_noise_category() {
        let z = NoiseZone::new(NoiseSource::Airport);
        assert_eq!(z.noise_category(), "Very Loud");
    }

    #[test]
    fn test_quiet_category() {
        let z = NoiseZone::new(NoiseSource::Nature);
        assert_eq!(z.noise_category(), "Very Quiet");
    }

    #[test]
    fn test_exceeds_limit() {
        let z = NoiseZone::new(NoiseSource::Traffic);
        assert!(z.exceeds_limit(60.0));
        assert!(!z.exceeds_limit(80.0));
    }

    #[test]
    fn test_sleep_disruption() {
        let z = NoiseZone::new(NoiseSource::Airport);
        assert!(z.sleep_disruption_risk() > 0.5);
    }

    #[test]
    fn test_no_sleep_disruption() {
        let z = NoiseZone::new(NoiseSource::Nature);
        assert!(z.sleep_disruption_risk() < 0.01);
    }

    #[test]
    fn test_route_average() {
        let mut r = NoiseRoute::new();
        r.add_zone(NoiseZone::new(NoiseSource::Traffic));
        r.add_zone(NoiseZone::new(NoiseSource::Nature));
        let avg = r.average_db();
        assert!(avg > 40.0 && avg < 70.0);
    }

    #[test]
    fn test_route_max() {
        let mut r = NoiseRoute::new();
        r.add_zone(NoiseZone::new(NoiseSource::Traffic));
        r.add_zone(NoiseZone::new(NoiseSource::Airport));
        assert!(r.max_db() > 80.0);
    }

    #[test]
    fn test_quiet_zones() {
        let mut r = NoiseRoute::new();
        r.add_zone(NoiseZone::new(NoiseSource::Nature));
        r.add_zone(NoiseZone::new(NoiseSource::Residential));
        assert_eq!(r.quiet_zone_count(), 2);
    }

    #[test]
    fn test_noise_score() {
        let mut r = NoiseRoute::new();
        r.add_zone(NoiseZone::new(NoiseSource::Nature));
        assert!(r.noise_score() > 50.0);
    }

    #[test]
    fn test_empty_route() {
        let r = NoiseRoute::new();
        assert_eq!(r.average_db(), 0.0);
    }
}
