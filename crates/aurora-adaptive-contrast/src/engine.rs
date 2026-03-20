/// Adaptive contrast: real-time adjustment for sun, night, rain conditions.
#[derive(Debug, Clone, PartialEq)]
pub enum LightingCondition { BrightSun, Daylight, Overcast, Dusk, Night, Tunnel }
#[derive(Debug, Clone)]
pub struct ContrastConfig {
    pub condition: LightingCondition, pub ambient_lux: f64,
    pub foreground_brightness: f64, pub background_brightness: f64, pub text_contrast_ratio: f64,
}
impl ContrastConfig {
    pub fn for_condition(cond: LightingCondition, lux: f64) -> Self {
        match cond {
            LightingCondition::BrightSun => Self { condition: cond, ambient_lux: lux, foreground_brightness: 1.0, background_brightness: 0.95, text_contrast_ratio: 7.0 },
            LightingCondition::Daylight => Self { condition: cond, ambient_lux: lux, foreground_brightness: 0.9, background_brightness: 0.85, text_contrast_ratio: 5.0 },
            LightingCondition::Overcast => Self { condition: cond, ambient_lux: lux, foreground_brightness: 0.85, background_brightness: 0.75, text_contrast_ratio: 4.5 },
            LightingCondition::Dusk => Self { condition: cond, ambient_lux: lux, foreground_brightness: 0.7, background_brightness: 0.3, text_contrast_ratio: 4.5 },
            LightingCondition::Night => Self { condition: cond, ambient_lux: lux, foreground_brightness: 0.5, background_brightness: 0.1, text_contrast_ratio: 4.5 },
            LightingCondition::Tunnel => Self { condition: cond, ambient_lux: lux, foreground_brightness: 0.6, background_brightness: 0.15, text_contrast_ratio: 5.0 },
        }
    }
    pub fn meets_wcag_aa(&self) -> bool { self.text_contrast_ratio >= 4.5 }
    pub fn meets_wcag_aaa(&self) -> bool { self.text_contrast_ratio >= 7.0 }
    pub fn compute_from_lux(lux: f64) -> LightingCondition {
        if lux > 50000.0 { LightingCondition::BrightSun }
        else if lux > 10000.0 { LightingCondition::Daylight }
        else if lux > 1000.0 { LightingCondition::Overcast }
        else if lux > 50.0 { LightingCondition::Dusk }
        else { LightingCondition::Night }
    }
    pub fn interpolate_brightness(lux: f64) -> f64 { (lux.ln().max(0.0) / 12.0).clamp(0.1, 1.0) }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_bright_sun() { let c = ContrastConfig::for_condition(LightingCondition::BrightSun, 80000.0); assert!(c.meets_wcag_aaa()); }
    #[test] fn test_night() { let c = ContrastConfig::for_condition(LightingCondition::Night, 10.0); assert!(c.meets_wcag_aa()); assert!(c.background_brightness < 0.2); }
    #[test] fn test_lux_detect() { assert_eq!(ContrastConfig::compute_from_lux(80000.0), LightingCondition::BrightSun); assert_eq!(ContrastConfig::compute_from_lux(5.0), LightingCondition::Night); }
    #[test] fn test_interpolate() { assert!(ContrastConfig::interpolate_brightness(50000.0) > ContrastConfig::interpolate_brightness(10.0)); }
}
