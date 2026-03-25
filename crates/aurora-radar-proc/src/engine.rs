/// Radar processing: adaptive cruise, collision avoidance, cross-traffic alert
/// Phase 183

#[derive(Debug, Clone)]
pub struct RadarTarget {
    pub distance_m: f64,
    pub speed_kmh: f64,
    pub angle_deg: f64,
    pub rcs_dbsm: f64,
}

impl RadarTarget {
    pub fn closing_speed_kmh(&self, ego_speed_kmh: f64) -> f64 {
        ego_speed_kmh - self.speed_kmh
    }

    pub fn time_to_collision_s(&self, ego_speed_kmh: f64) -> f64 {
        let closing = self.closing_speed_kmh(ego_speed_kmh);
        if closing <= 0.0 {
            return f64::MAX;
        }
        self.distance_m / (closing / 3.6)
    }

    pub fn is_large_vehicle(&self) -> bool {
        self.rcs_dbsm > 20.0
    }
}

#[derive(Debug, Clone)]
pub struct RadarSystem {
    pub active: bool,
    pub range_m: f64,
    pub targets: Vec<RadarTarget>,
    pub ego_speed_kmh: f64,
}

impl Default for RadarSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RadarSystem {
    pub fn new() -> Self {
        Self {
            active: true,
            range_m: 250.0,
            targets: Vec::new(),
            ego_speed_kmh: 0.0,
        }
    }

    pub fn closest_target_m(&self) -> f64 {
        self.targets
            .iter()
            .map(|t| t.distance_m)
            .fold(f64::MAX, f64::min)
    }

    pub fn collision_warning(&self) -> bool {
        self.targets
            .iter()
            .any(|t| t.time_to_collision_s(self.ego_speed_kmh) < 3.0)
    }

    pub fn target_count(&self) -> usize {
        self.targets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closing_speed() {
        let t = RadarTarget {
            distance_m: 50.0,
            speed_kmh: 80.0,
            angle_deg: 0.0,
            rcs_dbsm: 15.0,
        };
        assert!((t.closing_speed_kmh(100.0) - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_ttc() {
        let t = RadarTarget {
            distance_m: 50.0,
            speed_kmh: 80.0,
            angle_deg: 0.0,
            rcs_dbsm: 15.0,
        };
        assert!(t.time_to_collision_s(100.0) > 5.0);
    }

    #[test]
    fn test_large_vehicle() {
        let t = RadarTarget {
            distance_m: 100.0,
            speed_kmh: 60.0,
            angle_deg: 0.0,
            rcs_dbsm: 25.0,
        };
        assert!(t.is_large_vehicle());
    }

    #[test]
    fn test_empty_system() {
        let s = RadarSystem::new();
        assert_eq!(s.target_count(), 0);
    }

    #[test]
    fn test_no_collision() {
        let s = RadarSystem::new();
        assert!(!s.collision_warning());
    }

    #[test]
    fn test_closest_empty() {
        let s = RadarSystem::new();
        assert_eq!(s.closest_target_m(), f64::MAX);
    }

    #[test]
    fn test_closest_with_targets() {
        let mut s = RadarSystem::new();
        s.targets.push(RadarTarget {
            distance_m: 30.0,
            speed_kmh: 60.0,
            angle_deg: 0.0,
            rcs_dbsm: 10.0,
        });
        assert!((s.closest_target_m() - 30.0).abs() < 0.1);
    }
}
