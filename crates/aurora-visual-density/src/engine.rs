/// Adaptive information density: adjust based on driving state and complexity.
#[derive(Debug, Clone, PartialEq)]
pub enum DensityLevel { Maximum, High, Medium, Low, Critical }
#[derive(Debug, Clone)]
pub struct DensityConfig { pub level: DensityLevel, pub max_elements: usize, pub max_labels: usize, pub show_secondary_roads: bool, pub show_pois: bool, pub show_buildings: bool }
impl DensityConfig {
    pub fn for_level(level: DensityLevel) -> Self {
        match level {
            DensityLevel::Maximum => Self { level, max_elements: 50, max_labels: 20, show_secondary_roads: true, show_pois: true, show_buildings: true },
            DensityLevel::High => Self { level, max_elements: 30, max_labels: 12, show_secondary_roads: true, show_pois: true, show_buildings: false },
            DensityLevel::Medium => Self { level, max_elements: 15, max_labels: 6, show_secondary_roads: true, show_pois: false, show_buildings: false },
            DensityLevel::Low => Self { level, max_elements: 8, max_labels: 3, show_secondary_roads: false, show_pois: false, show_buildings: false },
            DensityLevel::Critical => Self { level, max_elements: 3, max_labels: 1, show_secondary_roads: false, show_pois: false, show_buildings: false },
        }
    }
    pub fn from_complexity(complexity: f64) -> DensityLevel {
        if complexity < 0.2 { DensityLevel::Maximum }
        else if complexity < 0.4 { DensityLevel::High }
        else if complexity < 0.6 { DensityLevel::Medium }
        else if complexity < 0.8 { DensityLevel::Low }
        else { DensityLevel::Critical }
    }
    pub fn visual_weight(&self) -> f64 { self.max_elements as f64 / 50.0 }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_max() { let c = DensityConfig::for_level(DensityLevel::Maximum); assert!(c.show_buildings); assert_eq!(c.max_elements, 50); }
    #[test] fn test_critical() { let c = DensityConfig::for_level(DensityLevel::Critical); assert!(!c.show_pois); assert_eq!(c.max_elements, 3); }
    #[test] fn test_from_complexity() { assert_eq!(DensityConfig::from_complexity(0.1), DensityLevel::Maximum); assert_eq!(DensityConfig::from_complexity(0.9), DensityLevel::Critical); }
    #[test] fn test_weight() { let max = DensityConfig::for_level(DensityLevel::Maximum); let crit = DensityConfig::for_level(DensityLevel::Critical); assert!(max.visual_weight() > crit.visual_weight()); }
}
