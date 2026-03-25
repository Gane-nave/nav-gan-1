/// Pedestrian detection and safety: crosswalk monitoring, trajectory prediction.
#[derive(Debug, Clone, PartialEq)]
pub enum PedestrianActivity {
    Standing,
    Walking,
    Running,
    Cycling,
    Wheelchair,
    ChildPlaying,
    ElderlySlowWalk,
    GroupCrossing,
}

impl PedestrianActivity {
    pub fn typical_speed_ms(&self) -> f64 {
        match self {
            PedestrianActivity::Standing => 0.0,
            PedestrianActivity::Walking => 1.4,
            PedestrianActivity::Running => 3.5,
            PedestrianActivity::Cycling => 5.0,
            PedestrianActivity::Wheelchair => 1.0,
            PedestrianActivity::ChildPlaying => 2.0,
            PedestrianActivity::ElderlySlowWalk => 0.8,
            PedestrianActivity::GroupCrossing => 1.2,
        }
    }

    pub fn unpredictability(&self) -> f64 {
        match self {
            PedestrianActivity::ChildPlaying => 0.95,
            PedestrianActivity::Running => 0.6,
            PedestrianActivity::GroupCrossing => 0.5,
            PedestrianActivity::Walking => 0.3,
            PedestrianActivity::Cycling => 0.4,
            PedestrianActivity::ElderlySlowWalk => 0.2,
            PedestrianActivity::Standing => 0.4,
            PedestrianActivity::Wheelchair => 0.15,
        }
    }

    pub fn vulnerability(&self) -> f64 {
        match self {
            PedestrianActivity::ChildPlaying => 1.0,
            PedestrianActivity::ElderlySlowWalk => 0.95,
            PedestrianActivity::Wheelchair => 0.9,
            _ => 0.7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pedestrian {
    pub activity: PedestrianActivity,
    pub distance_m: f64,
    pub heading_deg: f64,
    pub near_crosswalk: bool,
}

impl Pedestrian {
    pub fn new(activity: PedestrianActivity, distance_m: f64) -> Self {
        Self {
            activity,
            distance_m,
            heading_deg: 0.0,
            near_crosswalk: false,
        }
    }

    pub fn time_to_collision_sec(&self, vehicle_speed_kmh: f64) -> f64 {
        if vehicle_speed_kmh <= 0.0 {
            return f64::INFINITY;
        }
        self.distance_m / (vehicle_speed_kmh / 3.6)
    }

    pub fn risk_score(&self) -> f64 {
        let dist_risk = (50.0 - self.distance_m).max(0.0) * 2.0;
        let vuln = self.activity.vulnerability() * 20.0;
        let unpredict = self.activity.unpredictability() * 20.0;
        (dist_risk + vuln + unpredict).clamp(0.0, 100.0)
    }

    pub fn safe_vehicle_speed_kmh(&self) -> f64 {
        if self.distance_m < 5.0 {
            return 0.0;
        }
        if self.distance_m < 15.0 {
            return 15.0;
        }
        if self.distance_m < 30.0 {
            return 30.0;
        }
        50.0
    }

    pub fn crossing_time_sec(&self, road_width_m: f64) -> f64 {
        let speed = self.activity.typical_speed_ms();
        if speed <= 0.0 {
            return f64::INFINITY;
        }
        road_width_m / speed
    }

    pub fn is_immediate_threat(&self) -> bool {
        self.distance_m < 20.0 && self.activity.unpredictability() > 0.4
    }
}

#[derive(Debug, Clone)]
pub struct PedestrianScene {
    pub pedestrians: Vec<Pedestrian>,
}

impl Default for PedestrianScene {
    fn default() -> Self {
        Self::new()
    }
}

impl PedestrianScene {
    pub fn new() -> Self {
        Self {
            pedestrians: Vec::new(),
        }
    }

    pub fn add(&mut self, p: Pedestrian) {
        self.pedestrians.push(p);
    }

    pub fn max_risk(&self) -> f64 {
        self.pedestrians
            .iter()
            .map(|p| p.risk_score())
            .fold(0.0_f64, f64::max)
    }

    pub fn immediate_threats(&self) -> usize {
        self.pedestrians
            .iter()
            .filter(|p| p.is_immediate_threat())
            .count()
    }

    pub fn min_safe_speed(&self) -> f64 {
        self.pedestrians
            .iter()
            .map(|p| p.safe_vehicle_speed_kmh())
            .fold(f64::INFINITY, f64::min)
    }

    pub fn nearest_distance(&self) -> f64 {
        self.pedestrians
            .iter()
            .map(|p| p.distance_m)
            .fold(f64::INFINITY, f64::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walking_speed() {
        assert!((PedestrianActivity::Walking.typical_speed_ms() - 1.4).abs() < 0.01);
    }

    #[test]
    fn test_child_unpredictable() {
        assert!(PedestrianActivity::ChildPlaying.unpredictability() > 0.9);
    }

    #[test]
    fn test_vulnerability() {
        assert!(
            PedestrianActivity::ChildPlaying.vulnerability()
                > PedestrianActivity::Walking.vulnerability()
        );
    }

    #[test]
    fn test_ttc() {
        let p = Pedestrian::new(PedestrianActivity::Walking, 36.0);
        let ttc = p.time_to_collision_sec(36.0);
        assert!((ttc - 3.6).abs() < 0.1);
    }

    #[test]
    fn test_risk_close() {
        let close = Pedestrian::new(PedestrianActivity::ChildPlaying, 5.0);
        let far = Pedestrian::new(PedestrianActivity::Walking, 45.0);
        assert!(close.risk_score() > far.risk_score());
    }

    #[test]
    fn test_safe_speed_close() {
        let p = Pedestrian::new(PedestrianActivity::Walking, 3.0);
        assert!(p.safe_vehicle_speed_kmh() < 1.0);
    }

    #[test]
    fn test_crossing_time() {
        let p = Pedestrian::new(PedestrianActivity::Walking, 10.0);
        let t = p.crossing_time_sec(7.0);
        assert!((t - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_immediate_threat() {
        let p = Pedestrian::new(PedestrianActivity::ChildPlaying, 10.0);
        assert!(p.is_immediate_threat());
    }

    #[test]
    fn test_not_threat_far() {
        let p = Pedestrian::new(PedestrianActivity::Walking, 50.0);
        assert!(!p.is_immediate_threat());
    }

    #[test]
    fn test_scene_max_risk() {
        let mut s = PedestrianScene::new();
        s.add(Pedestrian::new(PedestrianActivity::Walking, 30.0));
        s.add(Pedestrian::new(PedestrianActivity::ChildPlaying, 5.0));
        assert!(s.max_risk() > 50.0);
    }

    #[test]
    fn test_scene_threats() {
        let mut s = PedestrianScene::new();
        s.add(Pedestrian::new(PedestrianActivity::ChildPlaying, 10.0));
        s.add(Pedestrian::new(PedestrianActivity::Walking, 50.0));
        assert_eq!(s.immediate_threats(), 1);
    }

    #[test]
    fn test_nearest_distance() {
        let mut s = PedestrianScene::new();
        s.add(Pedestrian::new(PedestrianActivity::Walking, 30.0));
        s.add(Pedestrian::new(PedestrianActivity::Standing, 10.0));
        assert!((s.nearest_distance() - 10.0).abs() < 0.01);
    }
}
