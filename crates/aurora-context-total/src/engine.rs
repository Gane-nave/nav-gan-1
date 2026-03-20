/// Context totality: combine location, time, habits, driving state, weather, traffic.
#[derive(Debug, Clone, PartialEq)]
pub enum DrivingState { Parked, Urban, Highway, Rural, Tunnel, Parking }
#[derive(Debug, Clone, PartialEq)]
pub enum WeatherCondition { Clear, Rain, Snow, Fog, Storm }
#[derive(Debug, Clone)]
pub struct TotalContext {
    pub lat: f64, pub lon: f64, pub speed_mps: f64, pub heading_deg: f64,
    pub hour: f64, pub day_of_week: u8, pub driving_state: DrivingState,
    pub weather: WeatherCondition, pub traffic_congestion: f64,
    pub cognitive_load: f64, pub fatigue_level: f64,
}
impl TotalContext {
    pub fn complexity_score(&self) -> f64 {
        let speed = (self.speed_mps / 40.0).min(1.0) * 0.15;
        let weather = match self.weather { WeatherCondition::Clear => 0.0, WeatherCondition::Rain => 0.3, WeatherCondition::Snow => 0.6, WeatherCondition::Fog => 0.7, WeatherCondition::Storm => 0.9 } * 0.2;
        let traffic = self.traffic_congestion.clamp(0.0, 1.0) * 0.2;
        let cognitive = self.cognitive_load.clamp(0.0, 1.0) * 0.2;
        let fatigue = self.fatigue_level.clamp(0.0, 1.0) * 0.15;
        let state = match self.driving_state { DrivingState::Highway => 0.3, DrivingState::Urban => 0.6, DrivingState::Tunnel => 0.7, DrivingState::Parking => 0.5, _ => 0.2 } * 0.1;
        (speed + weather + traffic + cognitive + fatigue + state).clamp(0.0, 1.0)
    }
    pub fn requires_simplified_ui(&self) -> bool { self.complexity_score() > 0.6 }
    pub fn is_night(&self) -> bool { self.hour < 6.0 || self.hour > 20.0 }
    pub fn recommended_info_density(&self) -> f64 { (1.0 - self.complexity_score()).clamp(0.2, 1.0) }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn base() -> TotalContext { TotalContext { lat: 32.0, lon: 34.0, speed_mps: 15.0, heading_deg: 90.0, hour: 14.0, day_of_week: 2, driving_state: DrivingState::Urban, weather: WeatherCondition::Clear, traffic_congestion: 0.3, cognitive_load: 0.3, fatigue_level: 0.1 } }
    #[test] fn test_normal() { assert!(!base().requires_simplified_ui()); }
    #[test] fn test_complex() { let mut c = base(); c.weather = WeatherCondition::Storm; c.traffic_congestion = 0.9; c.cognitive_load = 0.8; assert!(c.requires_simplified_ui()); }
    #[test] fn test_night() { let mut c = base(); c.hour = 23.0; assert!(c.is_night()); }
    #[test] fn test_density() { assert!(base().recommended_info_density() > 0.0); }
}
