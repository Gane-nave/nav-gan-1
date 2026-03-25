/// Speed advisory engine: optimal speed recommendations based on road, traffic, and conditions.
#[derive(Debug, Clone, PartialEq)]
pub enum SpeedZone {
    Residential,
    Urban,
    Suburban,
    Highway,
    Motorway,
    SchoolZone,
    Construction,
    Parking,
}

impl SpeedZone {
    pub fn base_limit_kmh(&self) -> f64 {
        match self {
            SpeedZone::Residential => 30.0,
            SpeedZone::Urban => 50.0,
            SpeedZone::Suburban => 70.0,
            SpeedZone::Highway => 100.0,
            SpeedZone::Motorway => 130.0,
            SpeedZone::SchoolZone => 20.0,
            SpeedZone::Construction => 40.0,
            SpeedZone::Parking => 10.0,
        }
    }

    pub fn eco_speed_kmh(&self) -> f64 {
        match self {
            SpeedZone::Motorway => 100.0,
            SpeedZone::Highway => 85.0,
            other => other.base_limit_kmh() * 0.9,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrafficDensity {
    Free,
    Light,
    Moderate,
    Heavy,
    Standstill,
}

impl TrafficDensity {
    pub fn speed_factor(&self) -> f64 {
        match self {
            TrafficDensity::Free => 1.0,
            TrafficDensity::Light => 0.85,
            TrafficDensity::Moderate => 0.65,
            TrafficDensity::Heavy => 0.35,
            TrafficDensity::Standstill => 0.05,
        }
    }

    pub fn from_occupancy(occupancy_pct: f64) -> Self {
        match occupancy_pct {
            o if o < 15.0 => TrafficDensity::Free,
            o if o < 35.0 => TrafficDensity::Light,
            o if o < 60.0 => TrafficDensity::Moderate,
            o if o < 85.0 => TrafficDensity::Heavy,
            _ => TrafficDensity::Standstill,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpeedAdvisory {
    pub zone: SpeedZone,
    pub traffic: TrafficDensity,
    pub curvature_radius_m: f64,
    pub gradient_pct: f64,
    pub wet_road: bool,
    pub night_mode: bool,
}

impl SpeedAdvisory {
    pub fn new(zone: SpeedZone) -> Self {
        Self {
            zone,
            traffic: TrafficDensity::Free,
            curvature_radius_m: f64::INFINITY,
            gradient_pct: 0.0,
            wet_road: false,
            night_mode: false,
        }
    }

    pub fn curve_safe_speed(&self) -> f64 {
        if self.curvature_radius_m <= 0.0 || self.curvature_radius_m.is_infinite() {
            return f64::INFINITY;
        }
        let friction = if self.wet_road { 0.5 } else { 0.7 };
        let v_ms = (friction * 9.81 * self.curvature_radius_m).sqrt();
        v_ms * 3.6
    }

    pub fn gradient_adjustment(&self) -> f64 {
        if self.gradient_pct > 6.0 {
            0.85
        } else if self.gradient_pct > 3.0 {
            0.92
        } else if self.gradient_pct < -6.0 {
            0.80
        } else if self.gradient_pct < -3.0 {
            0.90
        } else {
            1.0
        }
    }

    pub fn recommended_speed(&self) -> f64 {
        let base = self.zone.base_limit_kmh();
        let traffic = base * self.traffic.speed_factor();
        let curve = self.curve_safe_speed();
        let gradient_adj = self.gradient_adjustment();
        let night_adj = if self.night_mode { 0.9 } else { 1.0 };
        let wet_adj = if self.wet_road { 0.85 } else { 1.0 };

        let speed = traffic.min(curve) * gradient_adj * night_adj * wet_adj;
        speed.clamp(5.0, base)
    }

    pub fn eco_recommended_speed(&self) -> f64 {
        let eco = self.zone.eco_speed_kmh();
        let recommended = self.recommended_speed();
        recommended.min(eco)
    }

    pub fn is_speed_safe(&self, current_speed_kmh: f64) -> bool {
        current_speed_kmh <= self.recommended_speed() * 1.05
    }

    pub fn speed_excess(&self, current_speed_kmh: f64) -> f64 {
        (current_speed_kmh - self.recommended_speed()).max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct SpeedProfile {
    pub advisories: Vec<SpeedAdvisory>,
}

impl Default for SpeedProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeedProfile {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
        }
    }

    pub fn add_advisory(&mut self, adv: SpeedAdvisory) {
        self.advisories.push(adv);
    }

    pub fn min_recommended(&self) -> f64 {
        self.advisories
            .iter()
            .map(|a| a.recommended_speed())
            .fold(f64::INFINITY, f64::min)
    }

    pub fn max_recommended(&self) -> f64 {
        self.advisories
            .iter()
            .map(|a| a.recommended_speed())
            .fold(0.0_f64, f64::max)
    }

    pub fn average_recommended(&self) -> f64 {
        if self.advisories.is_empty() {
            return 0.0;
        }
        let total: f64 = self.advisories.iter().map(|a| a.recommended_speed()).sum();
        total / self.advisories.len() as f64
    }

    pub fn has_construction_zones(&self) -> bool {
        self.advisories
            .iter()
            .any(|a| a.zone == SpeedZone::Construction)
    }

    pub fn school_zone_count(&self) -> usize {
        self.advisories
            .iter()
            .filter(|a| a.zone == SpeedZone::SchoolZone)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_limits() {
        assert!((SpeedZone::Motorway.base_limit_kmh() - 130.0).abs() < 0.01);
        assert!((SpeedZone::SchoolZone.base_limit_kmh() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_eco_speed() {
        assert!(SpeedZone::Motorway.eco_speed_kmh() < SpeedZone::Motorway.base_limit_kmh());
    }

    #[test]
    fn test_traffic_density_free() {
        assert!((TrafficDensity::Free.speed_factor() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_traffic_density_standstill() {
        assert!(TrafficDensity::Standstill.speed_factor() < 0.1);
    }

    #[test]
    fn test_traffic_from_occupancy() {
        assert_eq!(TrafficDensity::from_occupancy(10.0), TrafficDensity::Free);
        assert_eq!(
            TrafficDensity::from_occupancy(50.0),
            TrafficDensity::Moderate
        );
        assert_eq!(
            TrafficDensity::from_occupancy(90.0),
            TrafficDensity::Standstill
        );
    }

    #[test]
    fn test_recommended_speed_free_motorway() {
        let adv = SpeedAdvisory::new(SpeedZone::Motorway);
        assert!((adv.recommended_speed() - 130.0).abs() < 0.01);
    }

    #[test]
    fn test_recommended_speed_heavy_traffic() {
        let mut adv = SpeedAdvisory::new(SpeedZone::Highway);
        adv.traffic = TrafficDensity::Heavy;
        assert!(adv.recommended_speed() < 50.0);
    }

    #[test]
    fn test_curve_safe_speed() {
        let mut adv = SpeedAdvisory::new(SpeedZone::Highway);
        adv.curvature_radius_m = 50.0;
        assert!(adv.curve_safe_speed() < 80.0);
    }

    #[test]
    fn test_curve_infinite_radius() {
        let adv = SpeedAdvisory::new(SpeedZone::Highway);
        assert!(adv.curve_safe_speed().is_infinite());
    }

    #[test]
    fn test_wet_road_reduces_speed() {
        let dry = SpeedAdvisory::new(SpeedZone::Highway);
        let mut wet = SpeedAdvisory::new(SpeedZone::Highway);
        wet.wet_road = true;
        assert!(wet.recommended_speed() < dry.recommended_speed());
    }

    #[test]
    fn test_night_reduces_speed() {
        let day = SpeedAdvisory::new(SpeedZone::Urban);
        let mut night = SpeedAdvisory::new(SpeedZone::Urban);
        night.night_mode = true;
        assert!(night.recommended_speed() < day.recommended_speed());
    }

    #[test]
    fn test_gradient_steep_up() {
        let mut adv = SpeedAdvisory::new(SpeedZone::Highway);
        adv.gradient_pct = 8.0;
        assert!(adv.gradient_adjustment() < 1.0);
    }

    #[test]
    fn test_speed_safe() {
        let adv = SpeedAdvisory::new(SpeedZone::Urban);
        assert!(adv.is_speed_safe(45.0));
        assert!(!adv.is_speed_safe(60.0));
    }

    #[test]
    fn test_speed_excess() {
        let adv = SpeedAdvisory::new(SpeedZone::Urban);
        assert!(adv.speed_excess(40.0) < 0.01);
        assert!(adv.speed_excess(70.0) > 15.0);
    }

    #[test]
    fn test_eco_speed_capped() {
        let adv = SpeedAdvisory::new(SpeedZone::Motorway);
        assert!(adv.eco_recommended_speed() <= adv.zone.eco_speed_kmh());
    }

    #[test]
    fn test_profile_empty() {
        let p = SpeedProfile::new();
        assert_eq!(p.average_recommended(), 0.0);
        assert!(!p.has_construction_zones());
    }

    #[test]
    fn test_profile_min_max() {
        let mut p = SpeedProfile::new();
        p.add_advisory(SpeedAdvisory::new(SpeedZone::SchoolZone));
        p.add_advisory(SpeedAdvisory::new(SpeedZone::Motorway));
        assert!(p.min_recommended() < 25.0);
        assert!(p.max_recommended() > 120.0);
    }

    #[test]
    fn test_school_zone_count() {
        let mut p = SpeedProfile::new();
        p.add_advisory(SpeedAdvisory::new(SpeedZone::SchoolZone));
        p.add_advisory(SpeedAdvisory::new(SpeedZone::SchoolZone));
        p.add_advisory(SpeedAdvisory::new(SpeedZone::Urban));
        assert_eq!(p.school_zone_count(), 2);
    }

    #[test]
    fn test_speed_floor() {
        let mut adv = SpeedAdvisory::new(SpeedZone::Parking);
        adv.traffic = TrafficDensity::Standstill;
        adv.wet_road = true;
        adv.night_mode = true;
        assert!(adv.recommended_speed() >= 5.0);
    }
}
