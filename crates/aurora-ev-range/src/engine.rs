/// EV range estimation: battery, terrain, weather, traffic, predict
/// Phase 1099

#[derive(Debug, Clone)]
pub struct EvRange {
    pub battery_ok: bool,
    pub terrain_ok: bool,
    pub weather_ok: bool,
    pub traffic_ok: bool,
    pub predict_ok: bool,
}

impl Default for EvRange {
    fn default() -> Self {
        Self::new()
    }
}

impl EvRange {
    pub fn new() -> Self {
        Self {
            battery_ok: true,
            terrain_ok: true,
            weather_ok: true,
            traffic_ok: true,
            predict_ok: true,
        }
    }

    pub fn factors_ok(&self) -> bool {
        self.battery_ok && self.terrain_ok && self.weather_ok
    }

    pub fn estimation_ok(&self) -> bool {
        self.traffic_ok && self.predict_ok
    }

    pub fn all_ok(&self) -> bool {
        self.factors_ok() && self.estimation_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.battery_ok || !self.terrain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.battery_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        let c = EvRange::new();
        assert!(c.factors_ok());
    }

    #[test]
    fn test_estimation() {
        let c = EvRange::new();
        assert!(c.estimation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EvRange::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = EvRange::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_battery() {
        let mut c = EvRange::new();
        c.battery_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = EvRange::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
