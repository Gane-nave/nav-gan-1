/// Elevation/terrain profile engine: gradient analysis, energy estimation, hill assists.
#[derive(Debug, Clone, PartialEq)]
pub struct ElevationPoint {
    pub distance_m: f64,
    pub altitude_m: f64,
}

impl ElevationPoint {
    pub fn new(distance_m: f64, altitude_m: f64) -> Self {
        Self {
            distance_m,
            altitude_m,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElevationProfile {
    pub points: Vec<ElevationPoint>,
}

impl Default for ElevationProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl ElevationProfile {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point(&mut self, distance_m: f64, altitude_m: f64) {
        self.points
            .push(ElevationPoint::new(distance_m, altitude_m));
    }

    pub fn total_distance(&self) -> f64 {
        self.points.last().map(|p| p.distance_m).unwrap_or(0.0)
    }

    pub fn min_altitude(&self) -> f64 {
        self.points
            .iter()
            .map(|p| p.altitude_m)
            .fold(f64::INFINITY, f64::min)
    }

    pub fn max_altitude(&self) -> f64 {
        self.points
            .iter()
            .map(|p| p.altitude_m)
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn elevation_range(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        self.max_altitude() - self.min_altitude()
    }

    pub fn total_ascent(&self) -> f64 {
        self.points
            .windows(2)
            .map(|w| (w[1].altitude_m - w[0].altitude_m).max(0.0))
            .sum()
    }

    pub fn total_descent(&self) -> f64 {
        self.points
            .windows(2)
            .map(|w| (w[0].altitude_m - w[1].altitude_m).max(0.0))
            .sum()
    }

    pub fn gradient_at(&self, index: usize) -> Option<f64> {
        if index + 1 >= self.points.len() {
            return None;
        }
        let dx = self.points[index + 1].distance_m - self.points[index].distance_m;
        if dx.abs() < f64::EPSILON {
            return None;
        }
        let dy = self.points[index + 1].altitude_m - self.points[index].altitude_m;
        Some((dy / dx) * 100.0) // percentage
    }

    pub fn max_gradient(&self) -> f64 {
        (0..self.points.len().saturating_sub(1))
            .filter_map(|i| self.gradient_at(i))
            .fold(0.0_f64, |a, b| a.max(b.abs()))
    }

    pub fn average_gradient(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }
        let first = &self.points[0];
        let last = &self.points[self.points.len() - 1];
        let dx = last.distance_m - first.distance_m;
        if dx.abs() < f64::EPSILON {
            return 0.0;
        }
        ((last.altitude_m - first.altitude_m) / dx) * 100.0
    }

    pub fn segments(&self) -> Vec<GradientSegment> {
        self.points
            .windows(2)
            .filter_map(|w| {
                let dx = w[1].distance_m - w[0].distance_m;
                if dx.abs() < f64::EPSILON {
                    return None;
                }
                let gradient = ((w[1].altitude_m - w[0].altitude_m) / dx) * 100.0;
                Some(GradientSegment {
                    start_m: w[0].distance_m,
                    end_m: w[1].distance_m,
                    gradient_pct: gradient,
                    classification: GradientClass::from_gradient(gradient),
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GradientClass {
    Flat,
    GentleUp,
    ModerateUp,
    SteepUp,
    GentleDown,
    ModerateDown,
    SteepDown,
}

impl GradientClass {
    pub fn from_gradient(pct: f64) -> Self {
        match pct {
            g if g > 8.0 => GradientClass::SteepUp,
            g if g > 4.0 => GradientClass::ModerateUp,
            g if g > 1.0 => GradientClass::GentleUp,
            g if g > -1.0 => GradientClass::Flat,
            g if g > -4.0 => GradientClass::GentleDown,
            g if g > -8.0 => GradientClass::ModerateDown,
            _ => GradientClass::SteepDown,
        }
    }

    pub fn energy_factor(&self) -> f64 {
        match self {
            GradientClass::SteepUp => 1.8,
            GradientClass::ModerateUp => 1.4,
            GradientClass::GentleUp => 1.15,
            GradientClass::Flat => 1.0,
            GradientClass::GentleDown => 0.85,
            GradientClass::ModerateDown => 0.7,
            GradientClass::SteepDown => 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GradientSegment {
    pub start_m: f64,
    pub end_m: f64,
    pub gradient_pct: f64,
    pub classification: GradientClass,
}

impl GradientSegment {
    pub fn length(&self) -> f64 {
        self.end_m - self.start_m
    }

    pub fn energy_cost(&self, base_consumption: f64) -> f64 {
        base_consumption * self.classification.energy_factor() * self.length() / 1000.0
    }
}

#[derive(Debug, Clone)]
pub struct ElevationAnalysis {
    pub total_ascent: f64,
    pub total_descent: f64,
    pub max_gradient_pct: f64,
    pub avg_gradient_pct: f64,
    pub difficulty_score: f64,
    pub energy_factor: f64,
}

impl ElevationAnalysis {
    pub fn from_profile(profile: &ElevationProfile) -> Self {
        let segments = profile.segments();
        let energy_factor = if segments.is_empty() {
            1.0
        } else {
            let total_len: f64 = segments.iter().map(|s| s.length()).sum();
            if total_len < f64::EPSILON {
                1.0
            } else {
                segments
                    .iter()
                    .map(|s| s.classification.energy_factor() * s.length())
                    .sum::<f64>()
                    / total_len
            }
        };

        let ascent = profile.total_ascent();
        let max_grad = profile.max_gradient();
        let difficulty = (ascent / 100.0).min(5.0) * 0.5 + (max_grad / 15.0).min(5.0) * 0.5;

        Self {
            total_ascent: ascent,
            total_descent: profile.total_descent(),
            max_gradient_pct: max_grad,
            avg_gradient_pct: profile.average_gradient(),
            difficulty_score: difficulty.clamp(0.0, 10.0),
            energy_factor,
        }
    }

    pub fn is_hilly(&self) -> bool {
        self.difficulty_score > 3.0
    }

    pub fn needs_hill_assist(&self) -> bool {
        self.max_gradient_pct > 10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_profile() -> ElevationProfile {
        let mut p = ElevationProfile::new();
        p.add_point(0.0, 100.0);
        p.add_point(1000.0, 100.0);
        p.add_point(2000.0, 100.0);
        p
    }

    fn hilly_profile() -> ElevationProfile {
        let mut p = ElevationProfile::new();
        p.add_point(0.0, 0.0);
        p.add_point(500.0, 80.0);
        p.add_point(1000.0, 20.0);
        p.add_point(1500.0, 120.0);
        p.add_point(2000.0, 50.0);
        p
    }

    #[test]
    fn test_flat_profile() {
        let p = flat_profile();
        assert!((p.total_ascent()).abs() < 0.01);
        assert!((p.total_descent()).abs() < 0.01);
        assert!((p.elevation_range()).abs() < 0.01);
    }

    #[test]
    fn test_total_distance() {
        let p = flat_profile();
        assert!((p.total_distance() - 2000.0).abs() < 0.01);
    }

    #[test]
    fn test_hilly_ascent_descent() {
        let p = hilly_profile();
        assert!(p.total_ascent() > 100.0);
        assert!(p.total_descent() > 50.0);
    }

    #[test]
    fn test_min_max_altitude() {
        let p = hilly_profile();
        assert!((p.min_altitude()).abs() < 0.01);
        assert!((p.max_altitude() - 120.0).abs() < 0.01);
    }

    #[test]
    fn test_elevation_range() {
        let p = hilly_profile();
        assert!((p.elevation_range() - 120.0).abs() < 0.01);
    }

    #[test]
    fn test_gradient_at() {
        let mut p = ElevationProfile::new();
        p.add_point(0.0, 0.0);
        p.add_point(100.0, 10.0);
        let g = p.gradient_at(0).unwrap();
        assert!((g - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_gradient_at_out_of_bounds() {
        let p = flat_profile();
        assert!(p.gradient_at(5).is_none());
    }

    #[test]
    fn test_max_gradient() {
        let p = hilly_profile();
        assert!(p.max_gradient() > 5.0);
    }

    #[test]
    fn test_average_gradient() {
        let mut p = ElevationProfile::new();
        p.add_point(0.0, 0.0);
        p.add_point(1000.0, 50.0);
        let avg = p.average_gradient();
        assert!((avg - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_segments() {
        let p = hilly_profile();
        let segs = p.segments();
        assert_eq!(segs.len(), 4);
    }

    #[test]
    fn test_gradient_class_steep_up() {
        assert_eq!(GradientClass::from_gradient(12.0), GradientClass::SteepUp);
    }

    #[test]
    fn test_gradient_class_flat() {
        assert_eq!(GradientClass::from_gradient(0.5), GradientClass::Flat);
    }

    #[test]
    fn test_gradient_class_steep_down() {
        assert_eq!(
            GradientClass::from_gradient(-10.0),
            GradientClass::SteepDown
        );
    }

    #[test]
    fn test_energy_factor_uphill() {
        assert!(GradientClass::SteepUp.energy_factor() > 1.5);
    }

    #[test]
    fn test_energy_factor_downhill() {
        assert!(GradientClass::SteepDown.energy_factor() < 0.6);
    }

    #[test]
    fn test_segment_length() {
        let seg = GradientSegment {
            start_m: 100.0,
            end_m: 500.0,
            gradient_pct: 5.0,
            classification: GradientClass::ModerateUp,
        };
        assert!((seg.length() - 400.0).abs() < 0.01);
    }

    #[test]
    fn test_segment_energy_cost() {
        let seg = GradientSegment {
            start_m: 0.0,
            end_m: 1000.0,
            gradient_pct: 0.0,
            classification: GradientClass::Flat,
        };
        let cost = seg.energy_cost(15.0);
        assert!((cost - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_analysis_flat() {
        let p = flat_profile();
        let a = ElevationAnalysis::from_profile(&p);
        assert!(!a.is_hilly());
        assert!(!a.needs_hill_assist());
        assert!((a.energy_factor - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_analysis_hilly() {
        let p = hilly_profile();
        let a = ElevationAnalysis::from_profile(&p);
        assert!(a.total_ascent > 100.0);
        assert!(a.max_gradient_pct > 5.0);
        assert!(a.difficulty_score > 0.0);
    }

    #[test]
    fn test_empty_profile() {
        let p = ElevationProfile::new();
        assert_eq!(p.total_distance(), 0.0);
        assert_eq!(p.elevation_range(), 0.0);
    }

    #[test]
    fn test_single_point_profile() {
        let mut p = ElevationProfile::new();
        p.add_point(0.0, 100.0);
        assert!((p.average_gradient()).abs() < 0.01);
    }
}
