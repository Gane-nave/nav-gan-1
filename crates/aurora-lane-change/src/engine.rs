/// Lane change assistance: gap detection, merge timing, safety scoring.
#[derive(Debug, Clone, PartialEq)]
pub enum LanePosition {
    Left,
    Center,
    Right,
    HovLane,
    EmergencyLane,
    BusLane,
}

impl LanePosition {
    pub fn lane_index(&self) -> u8 {
        match self {
            LanePosition::EmergencyLane => 0,
            LanePosition::Left => 1,
            LanePosition::Center => 2,
            LanePosition::Right => 3,
            LanePosition::HovLane => 4,
            LanePosition::BusLane => 5,
        }
    }

    pub fn is_restricted(&self) -> bool {
        matches!(
            self,
            LanePosition::HovLane | LanePosition::BusLane | LanePosition::EmergencyLane
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeDirection {
    Left,
    Right,
    DoubleLeft,
    DoubleRight,
}

impl ChangeDirection {
    pub fn lanes_crossed(&self) -> u8 {
        match self {
            ChangeDirection::Left | ChangeDirection::Right => 1,
            ChangeDirection::DoubleLeft | ChangeDirection::DoubleRight => 2,
        }
    }

    pub fn risk_multiplier(&self) -> f64 {
        match self {
            ChangeDirection::Left | ChangeDirection::Right => 1.0,
            ChangeDirection::DoubleLeft | ChangeDirection::DoubleRight => 2.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GapAssessment {
    pub front_gap_m: f64,
    pub rear_gap_m: f64,
    pub target_speed_kmh: f64,
    pub current_speed_kmh: f64,
}

impl GapAssessment {
    pub fn new(front_gap: f64, rear_gap: f64, target_speed: f64, current_speed: f64) -> Self {
        Self {
            front_gap_m: front_gap,
            rear_gap_m: rear_gap,
            target_speed_kmh: target_speed,
            current_speed_kmh: current_speed,
        }
    }

    pub fn min_safe_gap(&self) -> f64 {
        let speed_ms = self.current_speed_kmh / 3.6;
        (speed_ms * 2.0).max(10.0)
    }

    pub fn is_safe(&self) -> bool {
        let min = self.min_safe_gap();
        self.front_gap_m >= min && self.rear_gap_m >= min
    }

    pub fn safety_score(&self) -> f64 {
        let min = self.min_safe_gap();
        let front_ratio = (self.front_gap_m / min).min(2.0);
        let rear_ratio = (self.rear_gap_m / min).min(2.0);
        ((front_ratio + rear_ratio) / 4.0 * 100.0).clamp(0.0, 100.0)
    }

    pub fn speed_diff_kmh(&self) -> f64 {
        (self.target_speed_kmh - self.current_speed_kmh).abs()
    }

    pub fn merge_window_sec(&self) -> f64 {
        if self.current_speed_kmh <= 0.0 {
            return f64::INFINITY;
        }
        let speed_ms = self.current_speed_kmh / 3.6;
        self.rear_gap_m / speed_ms
    }
}

#[derive(Debug, Clone)]
pub struct LaneChangePlan {
    pub current: LanePosition,
    pub target: LanePosition,
    pub direction: ChangeDirection,
    pub gap: GapAssessment,
}

impl LaneChangePlan {
    pub fn new(
        current: LanePosition,
        target: LanePosition,
        direction: ChangeDirection,
        gap: GapAssessment,
    ) -> Self {
        Self {
            current,
            target,
            direction,
            gap,
        }
    }

    pub fn overall_risk(&self) -> f64 {
        let base = if self.gap.is_safe() { 20.0 } else { 80.0 };
        (base * self.direction.risk_multiplier()).min(100.0)
    }

    pub fn is_recommended(&self) -> bool {
        self.gap.is_safe() && !self.target.is_restricted() && self.overall_risk() < 60.0
    }

    pub fn estimated_duration_sec(&self) -> f64 {
        3.0 * self.direction.lanes_crossed() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lane_index() {
        assert_eq!(LanePosition::Left.lane_index(), 1);
        assert_eq!(LanePosition::Right.lane_index(), 3);
    }

    #[test]
    fn test_restricted_lanes() {
        assert!(LanePosition::HovLane.is_restricted());
        assert!(!LanePosition::Center.is_restricted());
    }

    #[test]
    fn test_lanes_crossed() {
        assert_eq!(ChangeDirection::Left.lanes_crossed(), 1);
        assert_eq!(ChangeDirection::DoubleRight.lanes_crossed(), 2);
    }

    #[test]
    fn test_gap_safe() {
        let g = GapAssessment::new(50.0, 50.0, 100.0, 80.0);
        assert!(g.is_safe());
    }

    #[test]
    fn test_gap_unsafe() {
        let g = GapAssessment::new(5.0, 5.0, 100.0, 80.0);
        assert!(!g.is_safe());
    }

    #[test]
    fn test_safety_score_range() {
        let g = GapAssessment::new(30.0, 30.0, 80.0, 60.0);
        let s = g.safety_score();
        assert!((0.0..=100.0).contains(&s));
    }

    #[test]
    fn test_merge_window() {
        let g = GapAssessment::new(50.0, 50.0, 80.0, 72.0);
        assert!(g.merge_window_sec() > 0.0);
    }

    #[test]
    fn test_speed_diff() {
        let g = GapAssessment::new(50.0, 50.0, 100.0, 80.0);
        assert!((g.speed_diff_kmh() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_plan_recommended() {
        let g = GapAssessment::new(60.0, 60.0, 80.0, 80.0);
        let plan = LaneChangePlan::new(
            LanePosition::Center,
            LanePosition::Right,
            ChangeDirection::Right,
            g,
        );
        assert!(plan.is_recommended());
    }

    #[test]
    fn test_plan_restricted_target() {
        let g = GapAssessment::new(60.0, 60.0, 80.0, 80.0);
        let plan = LaneChangePlan::new(
            LanePosition::Center,
            LanePosition::BusLane,
            ChangeDirection::Right,
            g,
        );
        assert!(!plan.is_recommended());
    }

    #[test]
    fn test_double_lane_duration() {
        let g = GapAssessment::new(60.0, 60.0, 80.0, 80.0);
        let plan = LaneChangePlan::new(
            LanePosition::Left,
            LanePosition::Right,
            ChangeDirection::DoubleRight,
            g,
        );
        assert!((plan.estimated_duration_sec() - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_overall_risk_safe() {
        let g = GapAssessment::new(60.0, 60.0, 80.0, 80.0);
        let plan = LaneChangePlan::new(
            LanePosition::Center,
            LanePosition::Right,
            ChangeDirection::Right,
            g,
        );
        assert!(plan.overall_risk() < 50.0);
    }

    #[test]
    fn test_overall_risk_double() {
        let g = GapAssessment::new(60.0, 60.0, 80.0, 80.0);
        let plan = LaneChangePlan::new(
            LanePosition::Left,
            LanePosition::Right,
            ChangeDirection::DoubleRight,
            g,
        );
        assert!(plan.overall_risk() > 40.0);
    }
}
