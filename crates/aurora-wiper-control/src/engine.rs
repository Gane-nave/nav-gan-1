/// Wiper control: rain detection, speed adjustment, washer fluid management
/// Phase 135

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WiperSpeed {
    Off,
    Intermittent,
    Low,
    Medium,
    High,
}

impl WiperSpeed {
    pub fn cycles_per_min(&self) -> u32 {
        match self {
            WiperSpeed::Off => 0,
            WiperSpeed::Intermittent => 10,
            WiperSpeed::Low => 30,
            WiperSpeed::Medium => 45,
            WiperSpeed::High => 60,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RainIntensity {
    None,
    Light,
    Moderate,
    Heavy,
    Torrential,
}

impl RainIntensity {
    pub fn recommended_speed(&self) -> WiperSpeed {
        match self {
            RainIntensity::None => WiperSpeed::Off,
            RainIntensity::Light => WiperSpeed::Intermittent,
            RainIntensity::Moderate => WiperSpeed::Low,
            RainIntensity::Heavy => WiperSpeed::Medium,
            RainIntensity::Torrential => WiperSpeed::High,
        }
    }
    pub fn visibility_reduction_pct(&self) -> f64 {
        match self {
            RainIntensity::None => 0.0,
            RainIntensity::Light => 10.0,
            RainIntensity::Moderate => 25.0,
            RainIntensity::Heavy => 50.0,
            RainIntensity::Torrential => 75.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WiperSystem {
    pub current_speed: WiperSpeed,
    pub rain_intensity: RainIntensity,
    pub washer_fluid_pct: f64,
    pub blade_wear_pct: f64,
    pub auto_mode: bool,
}

impl WiperSystem {
    pub fn new() -> Self {
        Self {
            current_speed: WiperSpeed::Off,
            rain_intensity: RainIntensity::None,
            washer_fluid_pct: 100.0,
            blade_wear_pct: 0.0,
            auto_mode: true,
        }
    }
    pub fn recommended_speed(&self) -> WiperSpeed {
        self.rain_intensity.recommended_speed()
    }
    pub fn needs_speed_change(&self) -> bool {
        self.auto_mode && self.current_speed != self.recommended_speed()
    }
    pub fn washer_fluid_low(&self) -> bool {
        self.washer_fluid_pct < 15.0
    }
    pub fn blades_need_replacement(&self) -> bool {
        self.blade_wear_pct > 80.0
    }
    pub fn visibility_score(&self) -> f64 {
        let rain_penalty = self.rain_intensity.visibility_reduction_pct();
        let wiper_benefit = if self.current_speed != WiperSpeed::Off {
            rain_penalty * 0.6
        } else {
            0.0
        };
        let blade_penalty = self.blade_wear_pct * 0.1;
        (100.0 - rain_penalty + wiper_benefit - blade_penalty).clamp(0.0, 100.0)
    }
    pub fn safe_speed_factor(&self) -> f64 {
        match self.rain_intensity {
            RainIntensity::None => 1.0,
            RainIntensity::Light => 0.9,
            RainIntensity::Moderate => 0.8,
            RainIntensity::Heavy => 0.65,
            RainIntensity::Torrential => 0.5,
        }
    }
    pub fn washer_uses_remaining(&self) -> u32 {
        (self.washer_fluid_pct / 2.0) as u32
    }
}

impl Default for WiperSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cycles() {
        assert_eq!(WiperSpeed::High.cycles_per_min(), 60);
    }
    #[test]
    fn test_rain_speed() {
        assert_eq!(RainIntensity::Heavy.recommended_speed(), WiperSpeed::Medium);
    }
    #[test]
    fn test_visibility_reduction() {
        assert!(RainIntensity::Torrential.visibility_reduction_pct() > 50.0);
    }
    #[test]
    fn test_needs_change() {
        let mut w = WiperSystem::new();
        w.rain_intensity = RainIntensity::Heavy;
        assert!(w.needs_speed_change());
    }
    #[test]
    fn test_no_change() {
        let w = WiperSystem::new();
        assert!(!w.needs_speed_change());
    }
    #[test]
    fn test_fluid_low() {
        let mut w = WiperSystem::new();
        w.washer_fluid_pct = 10.0;
        assert!(w.washer_fluid_low());
    }
    #[test]
    fn test_blade_replace() {
        let mut w = WiperSystem::new();
        w.blade_wear_pct = 90.0;
        assert!(w.blades_need_replacement());
    }
    #[test]
    fn test_visibility_clear() {
        let w = WiperSystem::new();
        assert!(w.visibility_score() > 90.0);
    }
    #[test]
    fn test_safe_speed() {
        let mut w = WiperSystem::new();
        w.rain_intensity = RainIntensity::Torrential;
        assert!(w.safe_speed_factor() <= 0.5);
    }
    #[test]
    fn test_washer_uses() {
        let w = WiperSystem::new();
        assert_eq!(w.washer_uses_remaining(), 50);
    }
}
