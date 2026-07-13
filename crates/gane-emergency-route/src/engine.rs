/// Emergency routing engine: priority routing for emergency vehicles and situations.
#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyType {
    Fire,
    Medical,
    Police,
    HazMat,
    Search,
    NaturalDisaster,
    CivilUnrest,
    MilitaryConvoy,
}

impl EmergencyType {
    pub fn priority_level(&self) -> u8 {
        match self {
            EmergencyType::Medical => 10,
            EmergencyType::Fire => 9,
            EmergencyType::HazMat => 8,
            EmergencyType::NaturalDisaster => 8,
            EmergencyType::Police => 7,
            EmergencyType::Search => 6,
            EmergencyType::MilitaryConvoy => 5,
            EmergencyType::CivilUnrest => 4,
        }
    }

    pub fn speed_override_kmh(&self) -> f64 {
        match self {
            EmergencyType::Medical => 160.0,
            EmergencyType::Fire => 140.0,
            EmergencyType::Police => 180.0,
            EmergencyType::HazMat => 80.0,
            EmergencyType::Search => 60.0,
            EmergencyType::NaturalDisaster => 100.0,
            EmergencyType::CivilUnrest => 120.0,
            EmergencyType::MilitaryConvoy => 70.0,
        }
    }

    pub fn requires_escort(&self) -> bool {
        matches!(self, EmergencyType::HazMat | EmergencyType::MilitaryConvoy)
    }

    pub fn requires_siren(&self) -> bool {
        matches!(
            self,
            EmergencyType::Medical | EmergencyType::Fire | EmergencyType::Police
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoadClearance {
    FullClosure,
    LaneClosure,
    TrafficSignalPreempt,
    AdvisoryOnly,
    NoClearance,
}

impl RoadClearance {
    pub fn delay_reduction_pct(&self) -> f64 {
        match self {
            RoadClearance::FullClosure => 95.0,
            RoadClearance::LaneClosure => 70.0,
            RoadClearance::TrafficSignalPreempt => 50.0,
            RoadClearance::AdvisoryOnly => 15.0,
            RoadClearance::NoClearance => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmergencySegment {
    pub distance_km: f64,
    pub normal_speed_kmh: f64,
    pub clearance: RoadClearance,
    pub blocked: bool,
    pub detour_km: f64,
}

impl EmergencySegment {
    pub fn new(distance_km: f64, normal_speed_kmh: f64) -> Self {
        Self {
            distance_km,
            normal_speed_kmh,
            clearance: RoadClearance::NoClearance,
            blocked: false,
            detour_km: 0.0,
        }
    }

    pub fn effective_distance(&self) -> f64 {
        if self.blocked {
            self.detour_km
        } else {
            self.distance_km
        }
    }

    pub fn normal_time_min(&self) -> f64 {
        if self.normal_speed_kmh <= 0.0 {
            return f64::INFINITY;
        }
        (self.effective_distance() / self.normal_speed_kmh) * 60.0
    }

    pub fn emergency_time_min(&self, emergency: &EmergencyType) -> f64 {
        let speed = emergency
            .speed_override_kmh()
            .min(self.normal_speed_kmh * 1.5);
        let reduction = self.clearance.delay_reduction_pct() / 100.0;
        let base_time = (self.effective_distance() / speed) * 60.0;
        base_time * (1.0 - reduction * 0.3)
    }

    pub fn time_saved_min(&self, emergency: &EmergencyType) -> f64 {
        (self.normal_time_min() - self.emergency_time_min(emergency)).max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct EmergencyRoute {
    pub emergency_type: EmergencyType,
    pub segments: Vec<EmergencySegment>,
    pub dispatch_time_sec: f64,
}

impl EmergencyRoute {
    pub fn new(emergency_type: EmergencyType) -> Self {
        Self {
            emergency_type,
            segments: Vec::new(),
            dispatch_time_sec: 30.0,
        }
    }

    pub fn add_segment(&mut self, seg: EmergencySegment) {
        self.segments.push(seg);
    }

    pub fn total_distance_km(&self) -> f64 {
        self.segments.iter().map(|s| s.effective_distance()).sum()
    }

    pub fn normal_eta_min(&self) -> f64 {
        let travel: f64 = self.segments.iter().map(|s| s.normal_time_min()).sum();
        travel + self.dispatch_time_sec / 60.0
    }

    pub fn emergency_eta_min(&self) -> f64 {
        let travel: f64 = self
            .segments
            .iter()
            .map(|s| s.emergency_time_min(&self.emergency_type))
            .sum();
        travel + self.dispatch_time_sec / 60.0
    }

    pub fn total_time_saved_min(&self) -> f64 {
        (self.normal_eta_min() - self.emergency_eta_min()).max(0.0)
    }

    pub fn blocked_segments(&self) -> usize {
        self.segments.iter().filter(|s| s.blocked).count()
    }

    pub fn route_feasible(&self) -> bool {
        !self
            .segments
            .iter()
            .any(|s| s.blocked && s.detour_km <= 0.0)
    }

    pub fn priority(&self) -> u8 {
        self.emergency_type.priority_level()
    }

    pub fn requires_escort(&self) -> bool {
        self.emergency_type.requires_escort()
    }

    pub fn golden_hour_feasible(&self) -> bool {
        self.emergency_eta_min() <= 60.0
    }

    pub fn critical_response_feasible(&self) -> bool {
        self.emergency_eta_min() <= 8.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_levels() {
        assert_eq!(EmergencyType::Medical.priority_level(), 10);
        assert!(EmergencyType::Medical.priority_level() > EmergencyType::Police.priority_level());
    }

    #[test]
    fn test_speed_overrides() {
        assert!(
            EmergencyType::Police.speed_override_kmh() > EmergencyType::HazMat.speed_override_kmh()
        );
    }

    #[test]
    fn test_requires_escort() {
        assert!(EmergencyType::HazMat.requires_escort());
        assert!(EmergencyType::MilitaryConvoy.requires_escort());
        assert!(!EmergencyType::Medical.requires_escort());
    }

    #[test]
    fn test_requires_siren() {
        assert!(EmergencyType::Medical.requires_siren());
        assert!(!EmergencyType::Search.requires_siren());
    }

    #[test]
    fn test_clearance_reduction() {
        assert!(RoadClearance::FullClosure.delay_reduction_pct() > 90.0);
        assert!(RoadClearance::NoClearance.delay_reduction_pct() < 0.01);
    }

    #[test]
    fn test_segment_normal_time() {
        let seg = EmergencySegment::new(10.0, 60.0);
        assert!((seg.normal_time_min() - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_segment_blocked_detour() {
        let mut seg = EmergencySegment::new(5.0, 50.0);
        seg.blocked = true;
        seg.detour_km = 8.0;
        assert!((seg.effective_distance() - 8.0).abs() < 0.01);
    }

    #[test]
    fn test_emergency_faster() {
        let seg = EmergencySegment::new(10.0, 60.0);
        let normal = seg.normal_time_min();
        let emergency = seg.emergency_time_min(&EmergencyType::Medical);
        assert!(emergency < normal);
    }

    #[test]
    fn test_time_saved_positive() {
        let seg = EmergencySegment::new(10.0, 60.0);
        assert!(seg.time_saved_min(&EmergencyType::Fire) > 0.0);
    }

    #[test]
    fn test_route_total_distance() {
        let mut route = EmergencyRoute::new(EmergencyType::Medical);
        route.add_segment(EmergencySegment::new(5.0, 60.0));
        route.add_segment(EmergencySegment::new(3.0, 50.0));
        assert!((route.total_distance_km() - 8.0).abs() < 0.01);
    }

    #[test]
    fn test_route_emergency_eta() {
        let mut route = EmergencyRoute::new(EmergencyType::Medical);
        route.add_segment(EmergencySegment::new(5.0, 80.0));
        assert!(route.emergency_eta_min() < route.normal_eta_min());
    }

    #[test]
    fn test_route_feasible() {
        let mut route = EmergencyRoute::new(EmergencyType::Fire);
        route.add_segment(EmergencySegment::new(5.0, 60.0));
        assert!(route.route_feasible());
    }

    #[test]
    fn test_route_infeasible_blocked() {
        let mut route = EmergencyRoute::new(EmergencyType::Fire);
        let mut seg = EmergencySegment::new(5.0, 60.0);
        seg.blocked = true;
        seg.detour_km = 0.0;
        route.add_segment(seg);
        assert!(!route.route_feasible());
    }

    #[test]
    fn test_blocked_with_detour_feasible() {
        let mut route = EmergencyRoute::new(EmergencyType::Fire);
        let mut seg = EmergencySegment::new(5.0, 60.0);
        seg.blocked = true;
        seg.detour_km = 7.0;
        route.add_segment(seg);
        assert!(route.route_feasible());
    }

    #[test]
    fn test_golden_hour() {
        let mut route = EmergencyRoute::new(EmergencyType::Medical);
        route.add_segment(EmergencySegment::new(5.0, 80.0));
        assert!(route.golden_hour_feasible());
    }

    #[test]
    fn test_critical_response_short() {
        let mut route = EmergencyRoute::new(EmergencyType::Medical);
        route.add_segment(EmergencySegment::new(2.0, 80.0));
        assert!(route.critical_response_feasible());
    }

    #[test]
    fn test_priority() {
        let route = EmergencyRoute::new(EmergencyType::Medical);
        assert_eq!(route.priority(), 10);
    }

    #[test]
    fn test_time_saved() {
        let mut route = EmergencyRoute::new(EmergencyType::Police);
        route.add_segment(EmergencySegment::new(10.0, 60.0));
        assert!(route.total_time_saved_min() > 0.0);
    }
}
