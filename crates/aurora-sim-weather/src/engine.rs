/// aurora-sim-weather: sim weather
/// Phase 2526

#[derive(Debug, Clone)]
pub struct SimWeather {
    pub rain_ok: bool,
    pub fog_ok: bool,
    pub wind_ok: bool,
    pub snow_ok: bool,
    pub temperature_ok: bool,
}

impl Default for SimWeather {
    fn default() -> Self {
        Self::new()
    }
}

impl SimWeather {
    pub fn new() -> Self {
        Self {
            rain_ok: true,
            fog_ok: true,
            wind_ok: true,
            snow_ok: true,
            temperature_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.rain_ok && self.fog_ok && self.wind_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.snow_ok && self.temperature_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.rain_ok || !self.fog_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rain_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SimWeather::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimWeather::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimWeather::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimWeather::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimWeather::new();
        c.rain_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimWeather::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimWeather::default();
        assert!(c.all_ok());
    }
}
