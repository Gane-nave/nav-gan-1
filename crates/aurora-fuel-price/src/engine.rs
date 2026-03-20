/// Fuel price: station, compare, trend, alert, loyalty
/// Phase 916

#[derive(Debug, Clone)]
pub struct FuelPrice {
    pub station_ok: bool,
    pub compare_ok: bool,
    pub trend_ok: bool,
    pub alert_ok: bool,
    pub loyalty_ok: bool,
}

impl Default for FuelPrice {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelPrice {
    pub fn new() -> Self {
        Self {
            station_ok: true,
            compare_ok: true,
            trend_ok: true,
            alert_ok: true,
            loyalty_ok: true,
        }
    }

    pub fn pricing_ok(&self) -> bool {
        self.station_ok && self.compare_ok && self.trend_ok
    }

    pub fn rewards_ok(&self) -> bool {
        self.alert_ok && self.loyalty_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pricing_ok() && self.rewards_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.station_ok || !self.compare_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.station_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pricing() {
        let c = FuelPrice::new();
        assert!(c.pricing_ok());
    }

    #[test]
    fn test_rewards() {
        let c = FuelPrice::new();
        assert!(c.rewards_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelPrice::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = FuelPrice::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_station() {
        let mut c = FuelPrice::new();
        c.station_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = FuelPrice::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
