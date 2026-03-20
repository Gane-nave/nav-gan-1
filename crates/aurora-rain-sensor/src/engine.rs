/// Rain sensor: precipitation detection, wiper speed control, intensity
/// Phase 231

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RainIntensity {
    None,
    Light,
    Moderate,
    Heavy,
    Torrential,
}

#[derive(Debug, Clone)]
pub struct RainSensor {
    pub intensity: RainIntensity,
    pub moisture_pct: f64,
    pub droplet_count: u32,
    pub sensor_ok: bool,
    pub auto_wiper_on: bool,
}

impl Default for RainSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl RainSensor {
    pub fn new() -> Self {
        Self {
            intensity: RainIntensity::None,
            moisture_pct: 0.0,
            droplet_count: 0,
            sensor_ok: true,
            auto_wiper_on: true,
        }
    }

    pub fn is_raining(&self) -> bool {
        self.intensity != RainIntensity::None
    }

    pub fn wiper_speed(&self) -> u8 {
        match self.intensity {
            RainIntensity::None => 0,
            RainIntensity::Light => 1,
            RainIntensity::Moderate => 2,
            RainIntensity::Heavy => 3,
            RainIntensity::Torrential => 4,
        }
    }

    pub fn reduce_speed_advisory(&self) -> bool {
        matches!(
            self.intensity,
            RainIntensity::Heavy | RainIntensity::Torrential
        )
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_raining() {
        let r = RainSensor::new();
        assert!(!r.is_raining());
    }

    #[test]
    fn test_wiper_off() {
        let r = RainSensor::new();
        assert_eq!(r.wiper_speed(), 0);
    }

    #[test]
    fn test_no_advisory() {
        let r = RainSensor::new();
        assert!(!r.reduce_speed_advisory());
    }

    #[test]
    fn test_heavy_rain() {
        let mut r = RainSensor::new();
        r.intensity = RainIntensity::Heavy;
        assert!(r.reduce_speed_advisory());
    }

    #[test]
    fn test_raining() {
        let mut r = RainSensor::new();
        r.intensity = RainIntensity::Light;
        assert!(r.is_raining());
    }

    #[test]
    fn test_health() {
        let r = RainSensor::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
