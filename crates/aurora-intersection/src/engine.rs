/// Intersection navigation: signal timing, right-of-way, conflict detection.
#[derive(Debug, Clone, PartialEq)]
pub enum SignalState {
    Green,
    Yellow,
    Red,
    FlashingYellow,
    FlashingRed,
    Off,
}

impl SignalState {
    pub fn can_proceed(&self) -> bool {
        matches!(self, SignalState::Green | SignalState::FlashingYellow)
    }

    pub fn must_stop(&self) -> bool {
        matches!(self, SignalState::Red | SignalState::FlashingRed)
    }

    pub fn caution_required(&self) -> bool {
        matches!(
            self,
            SignalState::Yellow | SignalState::FlashingYellow | SignalState::Off
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntersectionType {
    Signalized,
    FourWayStop,
    TwoWayStop,
    Yield,
    Uncontrolled,
    Roundabout,
    Diverge,
    Merge,
}

impl IntersectionType {
    pub fn complexity_score(&self) -> f64 {
        match self {
            IntersectionType::Signalized => 3.0,
            IntersectionType::FourWayStop => 4.0,
            IntersectionType::TwoWayStop => 2.0,
            IntersectionType::Yield => 2.5,
            IntersectionType::Uncontrolled => 5.0,
            IntersectionType::Roundabout => 3.5,
            IntersectionType::Diverge => 1.5,
            IntersectionType::Merge => 2.0,
        }
    }

    pub fn avg_delay_sec(&self) -> f64 {
        match self {
            IntersectionType::Signalized => 25.0,
            IntersectionType::FourWayStop => 15.0,
            IntersectionType::TwoWayStop => 8.0,
            IntersectionType::Yield => 5.0,
            IntersectionType::Uncontrolled => 3.0,
            IntersectionType::Roundabout => 12.0,
            IntersectionType::Diverge => 2.0,
            IntersectionType::Merge => 6.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub intersection_type: IntersectionType,
    pub signal: Option<SignalState>,
    pub num_approaches: u8,
    pub has_turn_lane: bool,
    pub pedestrian_crossing: bool,
    pub speed_limit_kmh: f64,
}

impl Intersection {
    pub fn new(intersection_type: IntersectionType) -> Self {
        Self {
            intersection_type,
            signal: None,
            num_approaches: 4,
            has_turn_lane: false,
            pedestrian_crossing: false,
            speed_limit_kmh: 50.0,
        }
    }

    pub fn risk_score(&self) -> f64 {
        let base = self.intersection_type.complexity_score() * 10.0;
        let ped = if self.pedestrian_crossing { 15.0 } else { 0.0 };
        let approaches = (self.num_approaches as f64 - 2.0) * 5.0;
        (base + ped + approaches).clamp(0.0, 100.0)
    }

    pub fn expected_delay_sec(&self) -> f64 {
        let base = self.intersection_type.avg_delay_sec();
        let turn_bonus = if self.has_turn_lane { -3.0 } else { 5.0 };
        (base + turn_bonus).max(0.0)
    }

    pub fn approach_speed_kmh(&self) -> f64 {
        let factor = match &self.signal {
            Some(SignalState::Green) => 0.9,
            Some(SignalState::Yellow) => 0.5,
            Some(SignalState::Red) => 0.0,
            _ => 0.6,
        };
        self.speed_limit_kmh * factor
    }

    pub fn can_proceed(&self) -> bool {
        match &self.signal {
            Some(s) => s.can_proceed(),
            None => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntersectionRoute {
    pub intersections: Vec<Intersection>,
}

impl Default for IntersectionRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl IntersectionRoute {
    pub fn new() -> Self {
        Self {
            intersections: Vec::new(),
        }
    }

    pub fn add(&mut self, i: Intersection) {
        self.intersections.push(i);
    }

    pub fn total_delay_sec(&self) -> f64 {
        self.intersections
            .iter()
            .map(|i| i.expected_delay_sec())
            .sum()
    }

    pub fn total_delay_min(&self) -> f64 {
        self.total_delay_sec() / 60.0
    }

    pub fn highest_risk(&self) -> f64 {
        self.intersections
            .iter()
            .map(|i| i.risk_score())
            .fold(0.0_f64, f64::max)
    }

    pub fn signalized_count(&self) -> usize {
        self.intersections
            .iter()
            .filter(|i| i.intersection_type == IntersectionType::Signalized)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_proceed() {
        assert!(SignalState::Green.can_proceed());
        assert!(!SignalState::Red.can_proceed());
    }

    #[test]
    fn test_signal_stop() {
        assert!(SignalState::Red.must_stop());
        assert!(!SignalState::Green.must_stop());
    }

    #[test]
    fn test_complexity() {
        assert!(
            IntersectionType::Uncontrolled.complexity_score()
                > IntersectionType::Diverge.complexity_score()
        );
    }

    #[test]
    fn test_intersection_risk() {
        let i = Intersection::new(IntersectionType::Uncontrolled);
        assert!(i.risk_score() > 40.0);
    }

    #[test]
    fn test_delay() {
        let i = Intersection::new(IntersectionType::Signalized);
        assert!(i.expected_delay_sec() > 20.0);
    }

    #[test]
    fn test_approach_green() {
        let mut i = Intersection::new(IntersectionType::Signalized);
        i.signal = Some(SignalState::Green);
        assert!(i.approach_speed_kmh() > 40.0);
    }

    #[test]
    fn test_approach_red() {
        let mut i = Intersection::new(IntersectionType::Signalized);
        i.signal = Some(SignalState::Red);
        assert!(i.approach_speed_kmh() < 0.01);
    }

    #[test]
    fn test_can_proceed_no_signal() {
        let i = Intersection::new(IntersectionType::Uncontrolled);
        assert!(i.can_proceed());
    }

    #[test]
    fn test_route_delay() {
        let mut r = IntersectionRoute::new();
        r.add(Intersection::new(IntersectionType::Signalized));
        r.add(Intersection::new(IntersectionType::FourWayStop));
        assert!(r.total_delay_sec() > 30.0);
    }

    #[test]
    fn test_signalized_count() {
        let mut r = IntersectionRoute::new();
        r.add(Intersection::new(IntersectionType::Signalized));
        r.add(Intersection::new(IntersectionType::Yield));
        assert_eq!(r.signalized_count(), 1);
    }

    #[test]
    fn test_ped_crossing_risk() {
        let mut i = Intersection::new(IntersectionType::Signalized);
        let r1 = i.risk_score();
        i.pedestrian_crossing = true;
        assert!(i.risk_score() > r1);
    }

    #[test]
    fn test_turn_lane_reduces_delay() {
        let mut i1 = Intersection::new(IntersectionType::Signalized);
        let d1 = i1.expected_delay_sec();
        i1.has_turn_lane = true;
        assert!(i1.expected_delay_sec() < d1);
    }
}
