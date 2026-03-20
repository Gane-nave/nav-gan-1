//! Pedestrian routing engine.
//!
//! Sidewalk-aware routing with accessibility support, elevation
//! penalties, crossing safety scoring, and indoor navigation hints.

/// Pedestrian route segment.
#[derive(Debug, Clone)]
pub struct PedestrianSegment {
    pub from_lat: f64,
    pub from_lon: f64,
    pub to_lat: f64,
    pub to_lon: f64,
    pub distance_m: f64,
    pub surface: SurfaceType,
    pub has_sidewalk: bool,
    pub crossing: Option<CrossingType>,
    pub elevation_gain_m: f64,
    pub accessibility: AccessibilityLevel,
}

/// Walking surface type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceType {
    Paved,
    Gravel,
    Dirt,
    Cobblestone,
    Stairs,
    Ramp,
    Indoor,
}

impl SurfaceType {
    /// Speed multiplier relative to paved sidewalk (1.0).
    pub fn speed_factor(self) -> f64 {
        match self {
            Self::Paved | Self::Indoor => 1.0,
            Self::Cobblestone => 0.85,
            Self::Gravel => 0.80,
            Self::Dirt => 0.75,
            Self::Ramp => 0.70,
            Self::Stairs => 0.50,
        }
    }
}

/// Crossing type at intersections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossingType {
    SignalControlled,
    Zebra,
    Unmarked,
    Underpass,
    Overpass,
}

impl CrossingType {
    /// Safety score 0.0 (dangerous) .. 1.0 (safest).
    pub fn safety_score(self) -> f64 {
        match self {
            Self::Underpass | Self::Overpass => 1.0,
            Self::SignalControlled => 0.90,
            Self::Zebra => 0.70,
            Self::Unmarked => 0.30,
        }
    }

    /// Estimated crossing delay in seconds.
    pub fn avg_delay_s(self) -> f64 {
        match self {
            Self::SignalControlled => 30.0,
            Self::Zebra => 10.0,
            Self::Unmarked => 5.0,
            Self::Underpass | Self::Overpass => 45.0,
        }
    }
}

/// Accessibility level of a path segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessibilityLevel {
    /// Fully wheelchair accessible.
    Full,
    /// Accessible with minor difficulty.
    Moderate,
    /// Stairs or obstacles present.
    Limited,
    /// Not accessible for mobility-impaired users.
    None,
}

/// Pedestrian routing preferences.
#[derive(Debug, Clone)]
pub struct PedestrianPreferences {
    /// Average walking speed in m/s (default ~1.4 m/s ≈ 5 km/h).
    pub base_speed_ms: f64,
    /// Require wheelchair-accessible routes.
    pub wheelchair: bool,
    /// Prefer shaded / covered paths when available.
    pub prefer_shade: bool,
    /// Avoid stairs.
    pub avoid_stairs: bool,
    /// Maximum acceptable elevation gain per km.
    pub max_elevation_per_km: f64,
}

impl Default for PedestrianPreferences {
    fn default() -> Self {
        Self {
            base_speed_ms: 1.4,
            wheelchair: false,
            prefer_shade: false,
            avoid_stairs: false,
            max_elevation_per_km: 80.0,
        }
    }
}

/// Pedestrian routing engine.
#[derive(Debug)]
pub struct PedestrianRouter {
    prefs: PedestrianPreferences,
}

impl PedestrianRouter {
    pub fn new(prefs: PedestrianPreferences) -> Self {
        Self { prefs }
    }

    /// Check if a segment is passable given current preferences.
    pub fn is_passable(&self, seg: &PedestrianSegment) -> bool {
        if self.prefs.wheelchair && seg.accessibility > AccessibilityLevel::Moderate {
            return false;
        }
        if self.prefs.avoid_stairs && seg.surface == SurfaceType::Stairs {
            return false;
        }
        if !seg.has_sidewalk && seg.surface != SurfaceType::Indoor {
            // Allow but penalise — not a hard filter
        }
        true
    }

    /// Estimate walking time for a segment in seconds.
    pub fn estimate_time_s(&self, seg: &PedestrianSegment) -> f64 {
        let speed = self.prefs.base_speed_ms * seg.surface.speed_factor();
        if speed <= 0.0 {
            return f64::INFINITY;
        }
        let walk_time = seg.distance_m / speed;
        let crossing_delay = seg.crossing.map(|c| c.avg_delay_s()).unwrap_or(0.0);
        // Elevation penalty: +10% time per 10 m gain
        let elev_factor = 1.0 + (seg.elevation_gain_m / 10.0) * 0.10;
        walk_time * elev_factor + crossing_delay
    }

    /// Compute a cost for the segment (lower = better).
    pub fn segment_cost(&self, seg: &PedestrianSegment) -> f64 {
        let time = self.estimate_time_s(seg);
        let safety_penalty = seg.crossing.map(|c| 1.0 - c.safety_score()).unwrap_or(0.0);
        let sidewalk_penalty = if seg.has_sidewalk { 0.0 } else { 0.3 };
        // Cost = time * (1 + penalties)
        time * (1.0 + safety_penalty + sidewalk_penalty)
    }

    /// Compute total route cost for a sequence of segments.
    pub fn route_cost(&self, segments: &[PedestrianSegment]) -> f64 {
        segments.iter().map(|s| self.segment_cost(s)).sum()
    }

    /// Total estimated walk time in seconds.
    pub fn route_time_s(&self, segments: &[PedestrianSegment]) -> f64 {
        segments.iter().map(|s| self.estimate_time_s(s)).sum()
    }

    /// Total route distance in metres.
    pub fn route_distance_m(segments: &[PedestrianSegment]) -> f64 {
        segments.iter().map(|s| s.distance_m).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(
        distance: f64,
        surface: SurfaceType,
        crossing: Option<CrossingType>,
    ) -> PedestrianSegment {
        PedestrianSegment {
            from_lat: 0.0,
            from_lon: 0.0,
            to_lat: 0.0,
            to_lon: 0.01,
            distance_m: distance,
            surface,
            has_sidewalk: true,
            crossing,
            elevation_gain_m: 0.0,
            accessibility: AccessibilityLevel::Full,
        }
    }

    #[test]
    fn test_surface_speed_factors() {
        assert!((SurfaceType::Paved.speed_factor() - 1.0).abs() < f64::EPSILON);
        assert!(SurfaceType::Stairs.speed_factor() < SurfaceType::Gravel.speed_factor());
        assert!(SurfaceType::Indoor.speed_factor() >= 1.0);
    }

    #[test]
    fn test_crossing_safety() {
        assert!(CrossingType::Underpass.safety_score() > CrossingType::Unmarked.safety_score());
        assert!(CrossingType::SignalControlled.safety_score() > CrossingType::Zebra.safety_score());
    }

    #[test]
    fn test_crossing_delay() {
        assert!(CrossingType::Underpass.avg_delay_s() > CrossingType::Zebra.avg_delay_s());
    }

    #[test]
    fn test_estimate_time_flat() {
        let router = PedestrianRouter::new(PedestrianPreferences::default());
        let s = seg(140.0, SurfaceType::Paved, None);
        let time = router.estimate_time_s(&s);
        // 140m / 1.4m/s = 100s, no elevation or crossing
        assert!((time - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_estimate_time_with_elevation() {
        let router = PedestrianRouter::new(PedestrianPreferences::default());
        let mut s = seg(140.0, SurfaceType::Paved, None);
        s.elevation_gain_m = 10.0;
        let time = router.estimate_time_s(&s);
        // 100s * 1.10 = 110s
        assert!((time - 110.0).abs() < 0.01);
    }

    #[test]
    fn test_wheelchair_filter() {
        let router = PedestrianRouter::new(PedestrianPreferences {
            wheelchair: true,
            ..Default::default()
        });
        let mut s = seg(100.0, SurfaceType::Stairs, None);
        s.accessibility = AccessibilityLevel::None;
        assert!(!router.is_passable(&s));

        let s2 = seg(100.0, SurfaceType::Paved, None);
        assert!(router.is_passable(&s2));
    }

    #[test]
    fn test_avoid_stairs() {
        let router = PedestrianRouter::new(PedestrianPreferences {
            avoid_stairs: true,
            ..Default::default()
        });
        let s = seg(50.0, SurfaceType::Stairs, None);
        assert!(!router.is_passable(&s));
    }

    #[test]
    fn test_route_cost_prefers_sidewalk() {
        let router = PedestrianRouter::new(PedestrianPreferences::default());
        let s1 = seg(100.0, SurfaceType::Paved, None);
        let mut s2 = seg(100.0, SurfaceType::Paved, None);
        s2.has_sidewalk = false;
        assert!(router.segment_cost(&s1) < router.segment_cost(&s2));
    }

    #[test]
    fn test_route_distance() {
        let segs = vec![
            seg(100.0, SurfaceType::Paved, None),
            seg(200.0, SurfaceType::Gravel, None),
        ];
        assert!((PedestrianRouter::route_distance_m(&segs) - 300.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_crossing_adds_delay() {
        let router = PedestrianRouter::new(PedestrianPreferences::default());
        let s_no = seg(100.0, SurfaceType::Paved, None);
        let s_cross = seg(
            100.0,
            SurfaceType::Paved,
            Some(CrossingType::SignalControlled),
        );
        assert!(router.estimate_time_s(&s_cross) > router.estimate_time_s(&s_no));
    }
}
