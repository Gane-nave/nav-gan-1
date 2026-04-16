/// Fuel economy: instantaneous/average consumption, eco scoring
/// Phase 287

#[derive(Debug, Clone)]
pub struct FuelEconomy {
    pub instant_l100km: f64,
    pub average_l100km: f64,
    pub eco_score: f64,
    pub distance_to_empty_km: f64,
    pub fuel_level_pct: f64,
}

impl Default for FuelEconomy {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelEconomy {
    pub fn new() -> Self {
        Self {
            instant_l100km: 7.0,
            average_l100km: 7.5,
            eco_score: 75.0,
            distance_to_empty_km: 500.0,
            fuel_level_pct: 70.0,
        }
    }

    pub fn efficient(&self) -> bool {
        self.average_l100km < 8.0
    }

    pub fn fuel_low(&self) -> bool {
        self.fuel_level_pct < 15.0
    }

    pub fn range_ok(&self) -> bool {
        self.distance_to_empty_km > 50.0
    }

    pub fn eco_driving(&self) -> bool {
        self.eco_score > 80.0
    }

    pub fn health_score(&self) -> f64 {
        if self.fuel_low() {
            return 30.0;
        }
        self.eco_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efficient() {
        let f = FuelEconomy::new();
        assert!(f.efficient());
    }

    #[test]
    fn test_not_low() {
        let f = FuelEconomy::new();
        assert!(!f.fuel_low());
    }

    #[test]
    fn test_range_ok() {
        let f = FuelEconomy::new();
        assert!(f.range_ok());
    }

    #[test]
    fn test_not_eco() {
        let f = FuelEconomy::new();
        assert!(!f.eco_driving());
    }

    #[test]
    fn test_low_fuel() {
        let mut f = FuelEconomy::new();
        f.fuel_level_pct = 10.0;
        assert!(f.fuel_low());
    }

    #[test]
    fn test_health() {
        let f = FuelEconomy::new();
        assert!((f.health_score() - 75.0).abs() < 0.1);
    }
}
