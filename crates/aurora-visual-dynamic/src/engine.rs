/// State-based visual system: different designs per driving state.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum DrivingMode { Normal, Approaching, Turning, Highway, Emergency, Night, Parking }
#[derive(Debug, Clone)]
pub struct VisualConfig {
    pub mode: DrivingMode, pub color_scheme: String, pub font_scale: f64,
    pub map_zoom: f64, pub show_speed: bool, pub show_eta: bool,
    pub arrow_size: f64, pub info_density: f64,
}
impl VisualConfig {
    pub fn for_mode(mode: DrivingMode) -> Self {
        match mode {
            DrivingMode::Normal => Self { mode, color_scheme: "day".into(), font_scale: 1.0, map_zoom: 15.0, show_speed: true, show_eta: true, arrow_size: 1.0, info_density: 0.7 },
            DrivingMode::Approaching => Self { mode, color_scheme: "day".into(), font_scale: 1.2, map_zoom: 17.0, show_speed: true, show_eta: false, arrow_size: 1.5, info_density: 0.4 },
            DrivingMode::Turning => Self { mode, color_scheme: "day".into(), font_scale: 1.4, map_zoom: 18.0, show_speed: false, show_eta: false, arrow_size: 2.0, info_density: 0.2 },
            DrivingMode::Highway => Self { mode, color_scheme: "day".into(), font_scale: 1.1, map_zoom: 13.0, show_speed: true, show_eta: true, arrow_size: 0.8, info_density: 0.5 },
            DrivingMode::Emergency => Self { mode, color_scheme: "emergency".into(), font_scale: 1.5, map_zoom: 16.0, show_speed: true, show_eta: false, arrow_size: 2.5, info_density: 0.1 },
            DrivingMode::Night => Self { mode, color_scheme: "night".into(), font_scale: 1.1, map_zoom: 15.0, show_speed: true, show_eta: true, arrow_size: 1.2, info_density: 0.5 },
            DrivingMode::Parking => Self { mode, color_scheme: "day".into(), font_scale: 1.0, map_zoom: 19.0, show_speed: false, show_eta: false, arrow_size: 0.5, info_density: 0.8 },
        }
    }
    pub fn cognitive_load(&self) -> f64 { self.info_density * self.font_scale * 0.5 }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_normal() { let c = VisualConfig::for_mode(DrivingMode::Normal); assert!(c.show_speed); assert!(c.show_eta); }
    #[test] fn test_turning() { let c = VisualConfig::for_mode(DrivingMode::Turning); assert!(!c.show_speed); assert!(c.arrow_size > 1.0); assert!(c.info_density < 0.5); }
    #[test] fn test_emergency() { let c = VisualConfig::for_mode(DrivingMode::Emergency); assert_eq!(c.color_scheme, "emergency"); assert!(c.info_density < 0.2); }
    #[test] fn test_night() { let c = VisualConfig::for_mode(DrivingMode::Night); assert_eq!(c.color_scheme, "night"); }
    #[test] fn test_cognitive() { let c = VisualConfig::for_mode(DrivingMode::Turning); assert!(c.cognitive_load() < VisualConfig::for_mode(DrivingMode::Parking).cognitive_load()); }
}
