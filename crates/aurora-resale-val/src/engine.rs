/// Resale value: market, condition, mileage, history, predict
/// Phase 975

#[derive(Debug, Clone)]
pub struct ResaleVal {
    pub market_ok: bool,
    pub condition_ok: bool,
    pub mileage_ok: bool,
    pub history_ok: bool,
    pub predict_ok: bool,
}

impl Default for ResaleVal {
    fn default() -> Self {
        Self::new()
    }
}

impl ResaleVal {
    pub fn new() -> Self {
        Self {
            market_ok: true,
            condition_ok: true,
            mileage_ok: true,
            history_ok: true,
            predict_ok: true,
        }
    }

    pub fn valuation_ok(&self) -> bool {
        self.market_ok && self.condition_ok && self.mileage_ok
    }

    pub fn analysis_ok(&self) -> bool {
        self.history_ok && self.predict_ok
    }

    pub fn all_ok(&self) -> bool {
        self.valuation_ok() && self.analysis_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.market_ok || !self.history_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.market_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valuation() {
        let c = ResaleVal::new();
        assert!(c.valuation_ok());
    }

    #[test]
    fn test_analysis() {
        let c = ResaleVal::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ResaleVal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = ResaleVal::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_market() {
        let mut c = ResaleVal::new();
        c.market_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = ResaleVal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
