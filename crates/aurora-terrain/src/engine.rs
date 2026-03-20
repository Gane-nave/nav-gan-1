/// Terrain classification engine: surface type detection, traction estimation, route safety.
#[derive(Debug, Clone, PartialEq)]
pub enum TerrainType {
    Paved,
    Gravel,
    Dirt,
    Sand,
    Mud,
    Snow,
    Ice,
    Flooded,
    Rocky,
    Grass,
}

impl TerrainType {
    pub fn traction_coefficient(&self) -> f64 {
        match self {
            TerrainType::Paved => 0.9,
            TerrainType::Gravel => 0.6,
            TerrainType::Dirt => 0.55,
            TerrainType::Sand => 0.4,
            TerrainType::Mud => 0.3,
            TerrainType::Snow => 0.25,
            TerrainType::Ice => 0.1,
            TerrainType::Flooded => 0.35,
            TerrainType::Rocky => 0.5,
            TerrainType::Grass => 0.45,
        }
    }

    pub fn max_safe_speed_kmh(&self) -> f64 {
        match self {
            TerrainType::Paved => 130.0,
            TerrainType::Gravel => 60.0,
            TerrainType::Dirt => 50.0,
            TerrainType::Sand => 30.0,
            TerrainType::Mud => 20.0,
            TerrainType::Snow => 40.0,
            TerrainType::Ice => 20.0,
            TerrainType::Flooded => 25.0,
            TerrainType::Rocky => 35.0,
            TerrainType::Grass => 40.0,
        }
    }

    pub fn is_offroad(&self) -> bool {
        !matches!(self, TerrainType::Paved)
    }

    pub fn risk_level(&self) -> f64 {
        1.0 - self.traction_coefficient()
    }
}

#[derive(Debug, Clone)]
pub struct TerrainSegment {
    pub terrain: TerrainType,
    pub start_m: f64,
    pub end_m: f64,
    pub slope_pct: f64,
    pub moisture: f64,
}

impl TerrainSegment {
    pub fn new(terrain: TerrainType, start_m: f64, end_m: f64) -> Self {
        Self {
            terrain,
            start_m,
            end_m,
            slope_pct: 0.0,
            moisture: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.end_m - self.start_m).max(0.0)
    }

    pub fn effective_traction(&self) -> f64 {
        let base = self.terrain.traction_coefficient();
        let moisture_penalty = self.moisture.clamp(0.0, 1.0) * 0.3;
        let slope_penalty = (self.slope_pct.abs() / 100.0).min(0.2);
        (base - moisture_penalty - slope_penalty).clamp(0.05, 1.0)
    }

    pub fn braking_distance(&self, speed_kmh: f64) -> f64 {
        let v = speed_kmh / 3.6;
        let traction = self.effective_traction();
        if traction < f64::EPSILON {
            return f64::INFINITY;
        }
        (v * v) / (2.0 * 9.81 * traction)
    }

    pub fn traversal_difficulty(&self) -> f64 {
        let terrain_score = self.terrain.risk_level();
        let slope_score = (self.slope_pct.abs() / 15.0).min(1.0);
        let moisture_score = self.moisture.clamp(0.0, 1.0);
        (terrain_score * 0.5 + slope_score * 0.3 + moisture_score * 0.2).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct TerrainRoute {
    pub segments: Vec<TerrainSegment>,
}

impl Default for TerrainRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainRoute {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, segment: TerrainSegment) {
        self.segments.push(segment);
    }

    pub fn total_length(&self) -> f64 {
        self.segments.iter().map(|s| s.length()).sum()
    }

    pub fn offroad_percentage(&self) -> f64 {
        let total = self.total_length();
        if total < f64::EPSILON {
            return 0.0;
        }
        let offroad: f64 = self
            .segments
            .iter()
            .filter(|s| s.terrain.is_offroad())
            .map(|s| s.length())
            .sum();
        (offroad / total) * 100.0
    }

    pub fn min_traction(&self) -> f64 {
        self.segments
            .iter()
            .map(|s| s.effective_traction())
            .fold(f64::INFINITY, f64::min)
    }

    pub fn worst_segment(&self) -> Option<&TerrainSegment> {
        self.segments.iter().max_by(|a, b| {
            a.traversal_difficulty()
                .partial_cmp(&b.traversal_difficulty())
                .unwrap()
        })
    }

    pub fn average_difficulty(&self) -> f64 {
        if self.segments.is_empty() {
            return 0.0;
        }
        let total: f64 = self.segments.iter().map(|s| s.traversal_difficulty()).sum();
        total / self.segments.len() as f64
    }

    pub fn requires_4wd(&self) -> bool {
        self.segments.iter().any(|s| {
            s.effective_traction() < 0.4
                || s.terrain == TerrainType::Mud
                || s.terrain == TerrainType::Sand
        })
    }

    pub fn max_safe_speed(&self) -> f64 {
        self.segments
            .iter()
            .map(|s| s.terrain.max_safe_speed_kmh())
            .fold(f64::INFINITY, f64::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paved_traction() {
        assert!((TerrainType::Paved.traction_coefficient() - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_ice_traction() {
        assert!((TerrainType::Ice.traction_coefficient() - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_offroad() {
        assert!(!TerrainType::Paved.is_offroad());
        assert!(TerrainType::Dirt.is_offroad());
        assert!(TerrainType::Ice.is_offroad());
    }

    #[test]
    fn test_risk_level() {
        assert!(TerrainType::Ice.risk_level() > TerrainType::Paved.risk_level());
    }

    #[test]
    fn test_segment_length() {
        let s = TerrainSegment::new(TerrainType::Paved, 100.0, 500.0);
        assert!((s.length() - 400.0).abs() < 0.01);
    }

    #[test]
    fn test_segment_negative_length() {
        let s = TerrainSegment::new(TerrainType::Paved, 500.0, 100.0);
        assert!((s.length()).abs() < 0.01);
    }

    #[test]
    fn test_effective_traction_dry() {
        let s = TerrainSegment::new(TerrainType::Paved, 0.0, 100.0);
        assert!(s.effective_traction() > 0.8);
    }

    #[test]
    fn test_effective_traction_wet() {
        let mut s = TerrainSegment::new(TerrainType::Paved, 0.0, 100.0);
        s.moisture = 1.0;
        assert!(s.effective_traction() < s.terrain.traction_coefficient());
    }

    #[test]
    fn test_braking_distance_paved() {
        let s = TerrainSegment::new(TerrainType::Paved, 0.0, 100.0);
        let dist = s.braking_distance(100.0);
        assert!(dist > 20.0 && dist < 100.0);
    }

    #[test]
    fn test_braking_distance_ice() {
        let s = TerrainSegment::new(TerrainType::Ice, 0.0, 100.0);
        let dist = s.braking_distance(100.0);
        assert!(dist > 200.0); // much longer on ice
    }

    #[test]
    fn test_traversal_difficulty() {
        let paved = TerrainSegment::new(TerrainType::Paved, 0.0, 100.0);
        let ice = TerrainSegment::new(TerrainType::Ice, 0.0, 100.0);
        assert!(ice.traversal_difficulty() > paved.traversal_difficulty());
    }

    #[test]
    fn test_route_total_length() {
        let mut r = TerrainRoute::new();
        r.add_segment(TerrainSegment::new(TerrainType::Paved, 0.0, 1000.0));
        r.add_segment(TerrainSegment::new(TerrainType::Gravel, 1000.0, 2000.0));
        assert!((r.total_length() - 2000.0).abs() < 0.01);
    }

    #[test]
    fn test_offroad_percentage() {
        let mut r = TerrainRoute::new();
        r.add_segment(TerrainSegment::new(TerrainType::Paved, 0.0, 500.0));
        r.add_segment(TerrainSegment::new(TerrainType::Dirt, 500.0, 1000.0));
        assert!((r.offroad_percentage() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_route() {
        let r = TerrainRoute::new();
        assert_eq!(r.total_length(), 0.0);
        assert_eq!(r.offroad_percentage(), 0.0);
        assert_eq!(r.average_difficulty(), 0.0);
    }

    #[test]
    fn test_requires_4wd() {
        let mut r = TerrainRoute::new();
        r.add_segment(TerrainSegment::new(TerrainType::Mud, 0.0, 500.0));
        assert!(r.requires_4wd());
    }

    #[test]
    fn test_paved_no_4wd() {
        let mut r = TerrainRoute::new();
        r.add_segment(TerrainSegment::new(TerrainType::Paved, 0.0, 500.0));
        assert!(!r.requires_4wd());
    }

    #[test]
    fn test_max_safe_speed() {
        let mut r = TerrainRoute::new();
        r.add_segment(TerrainSegment::new(TerrainType::Paved, 0.0, 500.0));
        r.add_segment(TerrainSegment::new(TerrainType::Ice, 500.0, 1000.0));
        assert!((r.max_safe_speed() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_worst_segment() {
        let mut r = TerrainRoute::new();
        r.add_segment(TerrainSegment::new(TerrainType::Paved, 0.0, 500.0));
        r.add_segment(TerrainSegment::new(TerrainType::Ice, 500.0, 1000.0));
        let worst = r.worst_segment().unwrap();
        assert_eq!(worst.terrain, TerrainType::Ice);
    }

    #[test]
    fn test_slope_affects_traction() {
        let mut flat = TerrainSegment::new(TerrainType::Gravel, 0.0, 100.0);
        flat.slope_pct = 0.0;
        let mut steep = TerrainSegment::new(TerrainType::Gravel, 0.0, 100.0);
        steep.slope_pct = 15.0;
        assert!(steep.effective_traction() < flat.effective_traction());
    }

    #[test]
    fn test_min_traction() {
        let mut r = TerrainRoute::new();
        r.add_segment(TerrainSegment::new(TerrainType::Paved, 0.0, 500.0));
        r.add_segment(TerrainSegment::new(TerrainType::Ice, 500.0, 1000.0));
        assert!(r.min_traction() < 0.2);
    }
}
