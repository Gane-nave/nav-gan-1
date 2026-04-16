/// Toll calculator: price, route, transponder, payment
/// Phase 915

#[derive(Debug, Clone)]
pub struct TollCalc {
    pub price_ok: bool,
    pub route_ok: bool,
    pub transponder_ok: bool,
    pub payment_ok: bool,
    pub database_ok: bool,
}

impl Default for TollCalc {
    fn default() -> Self {
        Self::new()
    }
}

impl TollCalc {
    pub fn new() -> Self {
        Self {
            price_ok: true,
            route_ok: true,
            transponder_ok: true,
            payment_ok: true,
            database_ok: true,
        }
    }

    pub fn calculation_ok(&self) -> bool {
        self.price_ok && self.route_ok && self.database_ok
    }

    pub fn payment_process_ok(&self) -> bool {
        self.transponder_ok && self.payment_ok
    }

    pub fn all_ok(&self) -> bool {
        self.calculation_ok() && self.payment_process_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.price_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.database_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation() {
        let c = TollCalc::new();
        assert!(c.calculation_ok());
    }

    #[test]
    fn test_payment() {
        let c = TollCalc::new();
        assert!(c.payment_process_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TollCalc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = TollCalc::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = TollCalc::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = TollCalc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
