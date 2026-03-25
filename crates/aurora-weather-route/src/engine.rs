/// Weather routing: forecast, hazard, visibility, wind
/// Phase 912

#[derive(Debug, Clone)]
pub struct WeatherRoute {
    pub forecast_ok: bool,
    pub hazard_ok: bool,
    pub visibility_ok: bool,
    pub wind_ok: bool,
    pub update_ok: bool,
}

impl Default for WeatherRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl WeatherRoute {
    pub fn new() -> Self {
        Self {
            forecast_ok: true,
            hazard_ok: true,
            visibility_ok: true,
            wind_ok: true,
            update_ok: true,
        }
    }

    pub fn prediction_ok(&self) -> bool {
        self.forecast_ok && self.hazard_ok && self.update_ok
    }

    pub fn conditions_ok(&self) -> bool {
        self.visibility_ok && self.wind_ok
    }

    pub fn all_ok(&self) -> bool {
        self.prediction_ok() && self.conditions_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.update_ok || !self.forecast_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.forecast_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction() {
        let c = WeatherRoute::new();
        assert!(c.prediction_ok());
    }

    #[test]
    fn test_conditions() {
        let c = WeatherRoute::new();
        assert!(c.conditions_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WeatherRoute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = WeatherRoute::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_update() {
        let mut c = WeatherRoute::new();
        c.update_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = WeatherRoute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
