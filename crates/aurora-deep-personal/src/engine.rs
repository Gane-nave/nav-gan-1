/// Deep personalization: driving style, confidence, route preferences, history.
#[derive(Debug, Clone, PartialEq)]
pub enum DrivingStyle { Calm, Normal, Sporty, Cautious }
#[derive(Debug, Clone)]
pub struct DriverProfile {
    pub driving_style: DrivingStyle, pub confidence_level: f64, pub prefers_highways: bool,
    pub prefers_scenic: bool, pub risk_tolerance: f64, pub avg_speed_ratio: f64,
    pub history_trips: u32, pub preferred_departure_hour: f64,
}
impl Default for DriverProfile {
    fn default() -> Self { Self::new() }
}
impl DriverProfile {
    pub fn new() -> Self { Self { driving_style: DrivingStyle::Normal, confidence_level: 0.5, prefers_highways: true, prefers_scenic: false, risk_tolerance: 0.5, avg_speed_ratio: 1.0, history_trips: 0, preferred_departure_hour: 8.0 } }
    pub fn personalization_depth(&self) -> f64 { (self.history_trips as f64 / 100.0).min(1.0) }
    pub fn route_aggressiveness(&self) -> f64 {
        let style = match self.driving_style { DrivingStyle::Sporty => 0.8, DrivingStyle::Normal => 0.5, DrivingStyle::Calm => 0.3, DrivingStyle::Cautious => 0.1 };
        (style * 0.5 + self.risk_tolerance.clamp(0.0, 1.0) * 0.5).clamp(0.0, 1.0)
    }
    pub fn speed_adjustment(&self) -> f64 { self.avg_speed_ratio.clamp(0.7, 1.3) }
    pub fn instruction_verbosity(&self) -> f64 { (1.0 - self.confidence_level.clamp(0.0, 1.0)).clamp(0.2, 1.0) }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let p = DriverProfile::new(); assert_eq!(p.driving_style, DrivingStyle::Normal); assert_eq!(p.personalization_depth(), 0.0); }
    #[test] fn test_sporty() { let p = DriverProfile { driving_style: DrivingStyle::Sporty, risk_tolerance: 0.8, ..DriverProfile::new() }; assert!(p.route_aggressiveness() > 0.5); }
    #[test] fn test_confident() { let p = DriverProfile { confidence_level: 0.9, ..DriverProfile::new() }; assert!(p.instruction_verbosity() < 0.5); }
    #[test] fn test_deep() { let p = DriverProfile { history_trips: 200, ..DriverProfile::new() }; assert!((p.personalization_depth()-1.0).abs()<0.01); }
}
