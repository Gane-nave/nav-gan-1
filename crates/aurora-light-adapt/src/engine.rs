/// Light adaptation: ambient light sensing, display brightness, tunnel transitions.
#[derive(Debug, Clone, PartialEq)]
pub enum LightCondition {
    BrightSun,
    Overcast,
    Dusk,
    Night,
    TunnelEntry,
    TunnelExit,
    Fog,
    Rain,
    Snow,
}

impl LightCondition {
    pub fn ambient_lux(&self) -> f64 {
        match self {
            LightCondition::BrightSun => 100000.0,
            LightCondition::Overcast => 10000.0,
            LightCondition::Dusk => 400.0,
            LightCondition::Night => 1.0,
            LightCondition::TunnelEntry => 50.0,
            LightCondition::TunnelExit => 80000.0,
            LightCondition::Fog => 5000.0,
            LightCondition::Rain => 8000.0,
            LightCondition::Snow => 15000.0,
        }
    }

    pub fn display_brightness_pct(&self) -> f64 {
        match self {
            LightCondition::BrightSun => 100.0,
            LightCondition::Overcast => 70.0,
            LightCondition::Dusk => 50.0,
            LightCondition::Night => 20.0,
            LightCondition::TunnelEntry => 30.0,
            LightCondition::TunnelExit => 90.0,
            LightCondition::Fog => 60.0,
            LightCondition::Rain => 65.0,
            LightCondition::Snow => 75.0,
        }
    }

    pub fn headlights_required(&self) -> bool {
        matches!(
            self,
            LightCondition::Night
                | LightCondition::TunnelEntry
                | LightCondition::Dusk
                | LightCondition::Fog
                | LightCondition::Rain
        )
    }

    pub fn high_beam_safe(&self) -> bool {
        matches!(self, LightCondition::Night)
    }

    pub fn glare_risk(&self) -> f64 {
        match self {
            LightCondition::TunnelExit => 0.95,
            LightCondition::BrightSun => 0.7,
            LightCondition::Snow => 0.8,
            LightCondition::Dusk => 0.5,
            _ => 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LightSensor {
    pub lux: f64,
    pub uv_index: f64,
    pub color_temp_k: f64,
}

impl LightSensor {
    pub fn new(lux: f64) -> Self {
        Self {
            lux,
            uv_index: 0.0,
            color_temp_k: 5500.0,
        }
    }

    pub fn classify(&self) -> LightCondition {
        match self.lux {
            l if l > 50000.0 => LightCondition::BrightSun,
            l if l > 5000.0 => LightCondition::Overcast,
            l if l > 100.0 => LightCondition::Dusk,
            _ => LightCondition::Night,
        }
    }

    pub fn recommended_brightness(&self) -> f64 {
        self.classify().display_brightness_pct()
    }

    pub fn is_dark(&self) -> bool {
        self.lux < 50.0
    }

    pub fn sun_protection_needed(&self) -> bool {
        self.uv_index > 6.0
    }

    pub fn is_warm_light(&self) -> bool {
        self.color_temp_k < 4000.0
    }
}

#[derive(Debug, Clone)]
pub struct LightTransition {
    pub from: LightCondition,
    pub to: LightCondition,
    pub duration_sec: f64,
}

impl LightTransition {
    pub fn new(from: LightCondition, to: LightCondition) -> Self {
        let duration = match (&from, &to) {
            (LightCondition::BrightSun, LightCondition::TunnelEntry) => 3.0,
            (LightCondition::TunnelEntry, LightCondition::TunnelExit) => 2.0,
            _ => 5.0,
        };
        Self {
            from,
            to,
            duration_sec: duration,
        }
    }

    pub fn brightness_change(&self) -> f64 {
        (self.to.display_brightness_pct() - self.from.display_brightness_pct()).abs()
    }

    pub fn adaptation_difficulty(&self) -> f64 {
        let lux_ratio = if self.from.ambient_lux() > 0.0 && self.to.ambient_lux() > 0.0 {
            (self.from.ambient_lux() / self.to.ambient_lux())
                .max(self.to.ambient_lux() / self.from.ambient_lux())
        } else {
            1.0
        };
        (lux_ratio.log10() * 25.0).clamp(0.0, 100.0)
    }

    pub fn is_dangerous(&self) -> bool {
        self.adaptation_difficulty() > 60.0
    }

    pub fn recommended_speed_reduction_pct(&self) -> f64 {
        let diff = self.adaptation_difficulty();
        if diff > 70.0 {
            40.0
        } else if diff > 50.0 {
            25.0
        } else if diff > 30.0 {
            10.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ambient_lux() {
        assert!(LightCondition::BrightSun.ambient_lux() > LightCondition::Night.ambient_lux());
    }

    #[test]
    fn test_brightness_range() {
        let b = LightCondition::Night.display_brightness_pct();
        assert!((0.0..=100.0).contains(&b));
    }

    #[test]
    fn test_headlights_night() {
        assert!(LightCondition::Night.headlights_required());
        assert!(!LightCondition::BrightSun.headlights_required());
    }

    #[test]
    fn test_high_beam() {
        assert!(LightCondition::Night.high_beam_safe());
        assert!(!LightCondition::Fog.high_beam_safe());
    }

    #[test]
    fn test_glare_risk() {
        assert!(LightCondition::TunnelExit.glare_risk() > 0.9);
    }

    #[test]
    fn test_sensor_classify() {
        let s = LightSensor::new(80000.0);
        assert_eq!(s.classify(), LightCondition::BrightSun);
    }

    #[test]
    fn test_sensor_dark() {
        let s = LightSensor::new(10.0);
        assert!(s.is_dark());
    }

    #[test]
    fn test_sensor_not_dark() {
        let s = LightSensor::new(1000.0);
        assert!(!s.is_dark());
    }

    #[test]
    fn test_uv_protection() {
        let mut s = LightSensor::new(80000.0);
        s.uv_index = 8.0;
        assert!(s.sun_protection_needed());
    }

    #[test]
    fn test_warm_light() {
        let mut s = LightSensor::new(500.0);
        s.color_temp_k = 3000.0;
        assert!(s.is_warm_light());
    }

    #[test]
    fn test_transition_brightness() {
        let t = LightTransition::new(LightCondition::BrightSun, LightCondition::TunnelEntry);
        assert!(t.brightness_change() > 50.0);
    }

    #[test]
    fn test_transition_dangerous() {
        let t = LightTransition::new(LightCondition::BrightSun, LightCondition::TunnelEntry);
        assert!(t.adaptation_difficulty() > 50.0);
    }

    #[test]
    fn test_speed_reduction() {
        let t = LightTransition::new(LightCondition::BrightSun, LightCondition::TunnelEntry);
        assert!(t.recommended_speed_reduction_pct() > 0.0);
    }

    #[test]
    fn test_no_speed_reduction() {
        let t = LightTransition::new(LightCondition::Overcast, LightCondition::Rain);
        assert!(t.recommended_speed_reduction_pct() < 15.0);
    }

    #[test]
    fn test_recommended_brightness() {
        let s = LightSensor::new(80000.0);
        assert!((s.recommended_brightness() - 100.0).abs() < 0.01);
    }
}
