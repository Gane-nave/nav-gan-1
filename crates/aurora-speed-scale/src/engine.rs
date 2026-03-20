/// Speed-aware scaling: less detail at high speed, more at low speed.
#[derive(Debug, Clone, PartialEq)]
pub enum DetailLevel { Full, High, Medium, Low, Minimal }
#[derive(Debug, Clone)]
pub struct SpeedScaler { pub low_speed_threshold: f64, pub high_speed_threshold: f64 }
impl SpeedScaler {
    pub fn default_scaler() -> Self { Self { low_speed_threshold: 8.0, high_speed_threshold: 30.0 } }
    pub fn detail_level(&self, speed_mps: f64) -> DetailLevel {
        if speed_mps < self.low_speed_threshold * 0.5 { DetailLevel::Full }
        else if speed_mps < self.low_speed_threshold { DetailLevel::High }
        else if speed_mps < self.high_speed_threshold * 0.7 { DetailLevel::Medium }
        else if speed_mps < self.high_speed_threshold { DetailLevel::Low }
        else { DetailLevel::Minimal }
    }
    pub fn poi_visibility(&self, speed_mps: f64) -> f64 { (1.0 - (speed_mps / self.high_speed_threshold).min(1.0)).clamp(0.1, 1.0) }
    pub fn label_scale(&self, speed_mps: f64) -> f64 { (1.0 + (speed_mps / self.high_speed_threshold * 0.5)).clamp(1.0, 1.5) }
    pub fn simplification_factor(&self, speed_mps: f64) -> f64 { (speed_mps / self.high_speed_threshold).clamp(0.0, 1.0) }
    pub fn max_visible_elements(&self, speed_mps: f64) -> usize {
        let base: usize = 20;
        let reduction = (speed_mps / self.high_speed_threshold * 15.0) as usize;
        base.saturating_sub(reduction).max(3)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_parked() { let s = SpeedScaler::default_scaler(); assert_eq!(s.detail_level(0.0), DetailLevel::Full); assert!(s.poi_visibility(0.0) > 0.9); }
    #[test] fn test_highway() { let s = SpeedScaler::default_scaler(); assert_eq!(s.detail_level(35.0), DetailLevel::Minimal); assert!(s.poi_visibility(35.0) < 0.2); }
    #[test] fn test_labels() { let s = SpeedScaler::default_scaler(); assert!(s.label_scale(30.0) > s.label_scale(5.0)); }
    #[test] fn test_elements() { let s = SpeedScaler::default_scaler(); assert!(s.max_visible_elements(0.0) > s.max_visible_elements(30.0)); }
}
