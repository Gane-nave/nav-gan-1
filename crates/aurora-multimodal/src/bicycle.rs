//! Bicycle routing engine.
//!
//! Bike-lane preference, elevation-aware effort estimation,
//! surface suitability, and bike-share station integration.

/// Bicycle route segment.
#[derive(Debug, Clone)]
pub struct BikeSegment {
    pub from_lat: f64,
    pub from_lon: f64,
    pub to_lat: f64,
    pub to_lon: f64,
    pub distance_m: f64,
    pub infrastructure: BikeInfrastructure,
    pub surface: BikeSurface,
    pub elevation_gain_m: f64,
    pub elevation_loss_m: f64,
    pub speed_limit_kmh: Option<f64>,
    pub traffic_density: TrafficDensity,
}

/// Type of cycling infrastructure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BikeInfrastructure {
    /// Physically separated bike lane.
    ProtectedLane,
    /// Painted bike lane on road.
    PaintedLane,
    /// Shared road with sharrows / signage.
    SharedRoad,
    /// Dedicated cycling path (off-road).
    DedicatedPath,
    /// No cycling infrastructure.
    None,
}

impl BikeInfrastructure {
    /// Safety multiplier (lower = safer).
    pub fn safety_factor(self) -> f64 {
        match self {
            Self::DedicatedPath => 1.0,
            Self::ProtectedLane => 1.05,
            Self::PaintedLane => 1.20,
            Self::SharedRoad => 1.40,
            Self::None => 1.70,
        }
    }
}

/// Road surface suitability for cycling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BikeSurface {
    Asphalt,
    Concrete,
    Gravel,
    Cobblestone,
    Dirt,
}

impl BikeSurface {
    /// Speed factor relative to smooth asphalt.
    pub fn speed_factor(self) -> f64 {
        match self {
            Self::Asphalt => 1.0,
            Self::Concrete => 0.95,
            Self::Gravel => 0.70,
            Self::Cobblestone => 0.65,
            Self::Dirt => 0.55,
        }
    }
}

/// Traffic density classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficDensity {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl TrafficDensity {
    /// Stress factor for cycling in traffic.
    pub fn stress_factor(self) -> f64 {
        match self {
            Self::Low => 1.0,
            Self::Medium => 1.15,
            Self::High => 1.35,
            Self::VeryHigh => 1.60,
        }
    }
}

/// Bicycle type affects speed and terrain capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BikeType {
    Road,
    Hybrid,
    Mountain,
    EBike,
    Cargo,
}

impl BikeType {
    /// Base cruising speed in m/s on flat asphalt.
    pub fn base_speed_ms(self) -> f64 {
        match self {
            Self::Road => 7.0,     // ~25 km/h
            Self::Hybrid => 5.5,   // ~20 km/h
            Self::Mountain => 4.5, // ~16 km/h
            Self::EBike => 6.9,    // ~25 km/h (motor-assisted)
            Self::Cargo => 4.0,    // ~14 km/h
        }
    }

    /// Elevation penalty factor (lower = less affected by hills).
    pub fn climb_penalty(self) -> f64 {
        match self {
            Self::EBike => 0.3, // Motor assists climbing
            Self::Road => 1.0,
            Self::Hybrid => 1.0,
            Self::Mountain => 0.8,
            Self::Cargo => 1.3,
        }
    }
}

/// Bike-share station.
#[derive(Debug, Clone)]
pub struct BikeShareStation {
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub available_bikes: u32,
    pub available_docks: u32,
    pub has_ebikes: bool,
}

impl BikeShareStation {
    /// Whether the station can provide a bike.
    pub fn can_rent(&self) -> bool {
        self.available_bikes > 0
    }

    /// Whether the station can accept a return.
    pub fn can_return(&self) -> bool {
        self.available_docks > 0
    }
}

/// Bicycle routing preferences.
#[derive(Debug, Clone)]
pub struct BikePreferences {
    pub bike_type: BikeType,
    /// Prefer dedicated infrastructure even if longer.
    pub prefer_safe_routes: bool,
    /// Maximum acceptable gradient (percent).
    pub max_gradient_pct: f64,
    /// Avoid cobblestone / gravel.
    pub smooth_surface_only: bool,
}

impl Default for BikePreferences {
    fn default() -> Self {
        Self {
            bike_type: BikeType::Hybrid,
            prefer_safe_routes: true,
            max_gradient_pct: 12.0,
            smooth_surface_only: false,
        }
    }
}

/// Bicycle routing engine.
#[derive(Debug)]
pub struct BikeRouter {
    prefs: BikePreferences,
}

impl BikeRouter {
    pub fn new(prefs: BikePreferences) -> Self {
        Self { prefs }
    }

    /// Check if a segment is rideable given preferences.
    pub fn is_rideable(&self, seg: &BikeSegment) -> bool {
        if self.prefs.smooth_surface_only
            && matches!(
                seg.surface,
                BikeSurface::Gravel | BikeSurface::Cobblestone | BikeSurface::Dirt
            )
        {
            return false;
        }
        if seg.distance_m > 0.0 {
            let gradient = (seg.elevation_gain_m / seg.distance_m) * 100.0;
            if gradient > self.prefs.max_gradient_pct {
                return false;
            }
        }
        true
    }

    /// Estimate cycling time in seconds for a segment.
    pub fn estimate_time_s(&self, seg: &BikeSegment) -> f64 {
        let base = self.prefs.bike_type.base_speed_ms() * seg.surface.speed_factor();
        if base <= 0.0 {
            return f64::INFINITY;
        }
        let flat_time = seg.distance_m / base;
        // Climbing penalty
        let climb_s = seg.elevation_gain_m * self.prefs.bike_type.climb_penalty() * 0.5;
        // Descent bonus (capped)
        let descent_bonus = (seg.elevation_loss_m * 0.2).min(flat_time * 0.3);
        (flat_time + climb_s - descent_bonus).max(seg.distance_m / 15.0)
    }

    /// Compute segment cost (lower = better).
    pub fn segment_cost(&self, seg: &BikeSegment) -> f64 {
        let time = self.estimate_time_s(seg);
        let safety = seg.infrastructure.safety_factor();
        let stress = seg.traffic_density.stress_factor();
        let infra_bonus = if self.prefs.prefer_safe_routes {
            safety
        } else {
            1.0
        };
        time * infra_bonus * stress
    }

    /// Total route cost.
    pub fn route_cost(&self, segments: &[BikeSegment]) -> f64 {
        segments.iter().map(|s| self.segment_cost(s)).sum()
    }

    /// Total estimated ride time in seconds.
    pub fn route_time_s(&self, segments: &[BikeSegment]) -> f64 {
        segments.iter().map(|s| self.estimate_time_s(s)).sum()
    }

    /// Total route distance in metres.
    pub fn route_distance_m(segments: &[BikeSegment]) -> f64 {
        segments.iter().map(|s| s.distance_m).sum()
    }

    /// Find the nearest station that can provide a bike.
    pub fn nearest_rental(
        stations: &[BikeShareStation],
        lat: f64,
        lon: f64,
    ) -> Option<&BikeShareStation> {
        stations.iter().filter(|s| s.can_rent()).min_by(|a, b| {
            let da = (a.lat - lat).powi(2) + (a.lon - lon).powi(2);
            let db = (b.lat - lat).powi(2) + (b.lon - lon).powi(2);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_seg(dist: f64, infra: BikeInfrastructure) -> BikeSegment {
        BikeSegment {
            from_lat: 0.0,
            from_lon: 0.0,
            to_lat: 0.0,
            to_lon: 0.01,
            distance_m: dist,
            infrastructure: infra,
            surface: BikeSurface::Asphalt,
            elevation_gain_m: 0.0,
            elevation_loss_m: 0.0,
            speed_limit_kmh: None,
            traffic_density: TrafficDensity::Low,
        }
    }

    #[test]
    fn test_bike_type_speeds() {
        assert!(BikeType::Road.base_speed_ms() > BikeType::Cargo.base_speed_ms());
        assert!(BikeType::EBike.base_speed_ms() > BikeType::Mountain.base_speed_ms());
    }

    #[test]
    fn test_ebike_climb_advantage() {
        assert!(BikeType::EBike.climb_penalty() < BikeType::Road.climb_penalty());
    }

    #[test]
    fn test_infrastructure_safety() {
        assert!(
            BikeInfrastructure::DedicatedPath.safety_factor()
                < BikeInfrastructure::None.safety_factor()
        );
    }

    #[test]
    fn test_flat_time_estimate() {
        let router = BikeRouter::new(BikePreferences {
            bike_type: BikeType::Hybrid,
            ..Default::default()
        });
        let s = flat_seg(550.0, BikeInfrastructure::DedicatedPath);
        let time = router.estimate_time_s(&s);
        // 550m / 5.5m/s = 100s
        assert!((time - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_hill_costs_more() {
        let router = BikeRouter::new(Default::default());
        let flat = flat_seg(1000.0, BikeInfrastructure::PaintedLane);
        let mut hill = flat.clone();
        hill.elevation_gain_m = 50.0;
        assert!(router.estimate_time_s(&hill) > router.estimate_time_s(&flat));
    }

    #[test]
    fn test_safe_route_preference() {
        let router = BikeRouter::new(BikePreferences {
            prefer_safe_routes: true,
            ..Default::default()
        });
        let safe = flat_seg(1000.0, BikeInfrastructure::ProtectedLane);
        let risky = flat_seg(1000.0, BikeInfrastructure::None);
        assert!(router.segment_cost(&safe) < router.segment_cost(&risky));
    }

    #[test]
    fn test_gradient_filter() {
        let router = BikeRouter::new(BikePreferences {
            max_gradient_pct: 10.0,
            ..Default::default()
        });
        let mut steep = flat_seg(100.0, BikeInfrastructure::PaintedLane);
        steep.elevation_gain_m = 15.0; // 15% gradient
        assert!(!router.is_rideable(&steep));

        let mut mild = flat_seg(100.0, BikeInfrastructure::PaintedLane);
        mild.elevation_gain_m = 5.0; // 5% gradient
        assert!(router.is_rideable(&mild));
    }

    #[test]
    fn test_smooth_surface_filter() {
        let router = BikeRouter::new(BikePreferences {
            smooth_surface_only: true,
            ..Default::default()
        });
        let mut gravel = flat_seg(100.0, BikeInfrastructure::DedicatedPath);
        gravel.surface = BikeSurface::Gravel;
        assert!(!router.is_rideable(&gravel));
    }

    #[test]
    fn test_bike_share_nearest() {
        let stations = vec![
            BikeShareStation {
                id: "a".into(),
                name: "A".into(),
                lat: 1.0,
                lon: 1.0,
                available_bikes: 0,
                available_docks: 5,
                has_ebikes: false,
            },
            BikeShareStation {
                id: "b".into(),
                name: "B".into(),
                lat: 0.1,
                lon: 0.1,
                available_bikes: 3,
                available_docks: 2,
                has_ebikes: true,
            },
        ];
        let nearest = BikeRouter::nearest_rental(&stations, 0.0, 0.0).unwrap();
        assert_eq!(nearest.id, "b");
    }

    #[test]
    fn test_route_distance() {
        let segs = vec![
            flat_seg(500.0, BikeInfrastructure::DedicatedPath),
            flat_seg(300.0, BikeInfrastructure::PaintedLane),
        ];
        assert!((BikeRouter::route_distance_m(&segs) - 800.0).abs() < f64::EPSILON);
    }
}
