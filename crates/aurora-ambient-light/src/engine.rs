/// Ambient light sensor: auto headlights, dashboard dimming, tunnel detection
/// Phase 232

#[derive(Debug, Clone)]
pub struct AmbientLightSensor {
    pub lux: f64,
    pub auto_lights_on: bool,
    pub dashboard_brightness_pct: f64,
    pub tunnel_detected: bool,
    pub sensor_ok: bool,
}

impl Default for AmbientLightSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl AmbientLightSensor {
    pub fn new() -> Self {
        Self {
            lux: 500.0,
            auto_lights_on: false,
            dashboard_brightness_pct: 80.0,
            tunnel_detected: false,
            sensor_ok: true,
        }
    }

    pub fn is_dark(&self) -> bool {
        self.lux < 50.0
    }

    pub fn is_twilight(&self) -> bool {
        self.lux >= 50.0 && self.lux < 200.0
    }

    pub fn headlights_needed(&self) -> bool {
        self.is_dark() || self.tunnel_detected
    }

    pub fn recommended_brightness(&self) -> f64 {
        if self.lux < 10.0 {
            return 20.0;
        }
        if self.lux < 100.0 {
            return 50.0;
        }
        if self.lux < 1000.0 {
            return 80.0;
        }
        100.0
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
    fn test_not_dark() {
        let a = AmbientLightSensor::new();
        assert!(!a.is_dark());
    }

    #[test]
    fn test_not_twilight() {
        let a = AmbientLightSensor::new();
        assert!(!a.is_twilight());
    }

    #[test]
    fn test_no_headlights() {
        let a = AmbientLightSensor::new();
        assert!(!a.headlights_needed());
    }

    #[test]
    fn test_brightness() {
        let a = AmbientLightSensor::new();
        assert!((a.recommended_brightness() - 80.0).abs() < 0.1);
    }

    #[test]
    fn test_dark() {
        let mut a = AmbientLightSensor::new();
        a.lux = 5.0;
        assert!(a.headlights_needed());
    }

    #[test]
    fn test_health() {
        let a = AmbientLightSensor::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
