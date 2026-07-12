/// Adaptive suspension: ride comfort, road adaptation, load compensation.
#[derive(Debug, Clone, PartialEq)]
pub enum SuspensionMode {
    Comfort,
    Normal,
    Sport,
    OffRoad,
    Snow,
    AutoAdapt,
}

impl SuspensionMode {
    pub fn damping_factor(&self) -> f64 {
        match self {
            SuspensionMode::Comfort => 0.3,
            SuspensionMode::Normal => 0.5,
            SuspensionMode::Sport => 0.8,
            SuspensionMode::OffRoad => 0.4,
            SuspensionMode::Snow => 0.35,
            SuspensionMode::AutoAdapt => 0.5,
        }
    }

    pub fn ride_height_mm(&self) -> f64 {
        match self {
            SuspensionMode::Comfort => 150.0,
            SuspensionMode::Normal => 140.0,
            SuspensionMode::Sport => 120.0,
            SuspensionMode::OffRoad => 200.0,
            SuspensionMode::Snow => 180.0,
            SuspensionMode::AutoAdapt => 150.0,
        }
    }

    pub fn comfort_score(&self) -> f64 {
        match self {
            SuspensionMode::Comfort => 95.0,
            SuspensionMode::Normal => 75.0,
            SuspensionMode::AutoAdapt => 85.0,
            SuspensionMode::OffRoad => 60.0,
            SuspensionMode::Snow => 70.0,
            SuspensionMode::Sport => 50.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoadCondition {
    Smooth,
    Rough,
    Potholed,
    SpeedBump,
    Gravel,
    Cobblestone,
}

impl RoadCondition {
    pub fn recommended_mode(&self) -> SuspensionMode {
        match self {
            RoadCondition::Smooth => SuspensionMode::Normal,
            RoadCondition::Rough => SuspensionMode::Comfort,
            RoadCondition::Potholed => SuspensionMode::OffRoad,
            RoadCondition::SpeedBump => SuspensionMode::Comfort,
            RoadCondition::Gravel => SuspensionMode::OffRoad,
            RoadCondition::Cobblestone => SuspensionMode::Comfort,
        }
    }

    pub fn impact_severity(&self) -> f64 {
        match self {
            RoadCondition::Smooth => 0.1,
            RoadCondition::Cobblestone => 0.4,
            RoadCondition::Rough => 0.5,
            RoadCondition::SpeedBump => 0.6,
            RoadCondition::Gravel => 0.7,
            RoadCondition::Potholed => 0.9,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SuspensionState {
    pub mode: SuspensionMode,
    pub load_kg: f64,
    pub speed_kmh: f64,
    pub road: RoadCondition,
}

impl SuspensionState {
    pub fn new(mode: SuspensionMode) -> Self {
        Self {
            mode,
            load_kg: 200.0,
            speed_kmh: 60.0,
            road: RoadCondition::Smooth,
        }
    }

    pub fn effective_damping(&self) -> f64 {
        let base = self.mode.damping_factor();
        let load_adj = (self.load_kg / 1000.0).min(0.2);
        let speed_adj = (self.speed_kmh / 200.0).min(0.15);
        (base + load_adj + speed_adj).clamp(0.1, 1.0)
    }

    pub fn comfort_index(&self) -> f64 {
        let mode_comfort = self.mode.comfort_score();
        let road_penalty = self.road.impact_severity() * 30.0;
        let speed_penalty = (self.speed_kmh / 200.0) * 10.0;
        (mode_comfort - road_penalty - speed_penalty).clamp(0.0, 100.0)
    }

    pub fn should_adapt(&self) -> bool {
        self.mode == SuspensionMode::AutoAdapt
    }

    pub fn optimal_mode(&self) -> SuspensionMode {
        self.road.recommended_mode()
    }

    pub fn is_overloaded(&self) -> bool {
        self.load_kg > 500.0
    }

    pub fn ground_clearance_mm(&self) -> f64 {
        let base = self.mode.ride_height_mm();
        let load_sag = (self.load_kg / 50.0).min(30.0);
        (base - load_sag).max(80.0)
    }

    pub fn bottoming_risk(&self) -> f64 {
        let clearance = self.ground_clearance_mm();
        let severity = self.road.impact_severity();
        let risk = severity * 50.0 + (150.0 - clearance).max(0.0) * 0.5;
        risk.clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damping_sport() {
        assert!(SuspensionMode::Sport.damping_factor() > SuspensionMode::Comfort.damping_factor());
    }

    #[test]
    fn test_ride_height() {
        assert!(SuspensionMode::OffRoad.ride_height_mm() > SuspensionMode::Sport.ride_height_mm());
    }

    #[test]
    fn test_comfort_score() {
        assert!(SuspensionMode::Comfort.comfort_score() > SuspensionMode::Sport.comfort_score());
    }

    #[test]
    fn test_recommended_mode() {
        assert_eq!(
            RoadCondition::Potholed.recommended_mode(),
            SuspensionMode::OffRoad
        );
    }

    #[test]
    fn test_impact_severity() {
        assert!(
            RoadCondition::Potholed.impact_severity() > RoadCondition::Smooth.impact_severity()
        );
    }

    #[test]
    fn test_effective_damping() {
        let s = SuspensionState::new(SuspensionMode::Sport);
        assert!(s.effective_damping() > 0.5);
    }

    #[test]
    fn test_comfort_smooth() {
        let s = SuspensionState::new(SuspensionMode::Comfort);
        assert!(s.comfort_index() > 60.0);
    }

    #[test]
    fn test_comfort_rough() {
        let mut s = SuspensionState::new(SuspensionMode::Sport);
        s.road = RoadCondition::Potholed;
        assert!(s.comfort_index() < 40.0);
    }

    #[test]
    fn test_auto_adapt() {
        let s = SuspensionState::new(SuspensionMode::AutoAdapt);
        assert!(s.should_adapt());
    }

    #[test]
    fn test_overloaded() {
        let mut s = SuspensionState::new(SuspensionMode::Normal);
        s.load_kg = 600.0;
        assert!(s.is_overloaded());
    }

    #[test]
    fn test_ground_clearance() {
        let s = SuspensionState::new(SuspensionMode::OffRoad);
        assert!(s.ground_clearance_mm() > 150.0);
    }

    #[test]
    fn test_bottoming_risk_smooth() {
        let s = SuspensionState::new(SuspensionMode::OffRoad);
        assert!(s.bottoming_risk() < 20.0);
    }

    #[test]
    fn test_bottoming_risk_potholed() {
        let mut s = SuspensionState::new(SuspensionMode::Sport);
        s.road = RoadCondition::Potholed;
        assert!(s.bottoming_risk() > 30.0);
    }
}
