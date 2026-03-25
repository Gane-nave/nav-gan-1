/// Supply chain: inventory, supplier, logistics, forecast
/// Phase 968

#[derive(Debug, Clone)]
pub struct SupplyChain {
    pub inventory_ok: bool,
    pub supplier_ok: bool,
    pub logistics_ok: bool,
    pub forecast_ok: bool,
    pub tracking_ok: bool,
}

impl Default for SupplyChain {
    fn default() -> Self {
        Self::new()
    }
}

impl SupplyChain {
    pub fn new() -> Self {
        Self {
            inventory_ok: true,
            supplier_ok: true,
            logistics_ok: true,
            forecast_ok: true,
            tracking_ok: true,
        }
    }

    pub fn sourcing_ok(&self) -> bool {
        self.inventory_ok && self.supplier_ok && self.tracking_ok
    }

    pub fn planning_ok(&self) -> bool {
        self.logistics_ok && self.forecast_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sourcing_ok() && self.planning_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.inventory_ok || !self.forecast_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inventory_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sourcing() {
        let c = SupplyChain::new();
        assert!(c.sourcing_ok());
    }

    #[test]
    fn test_planning() {
        let c = SupplyChain::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SupplyChain::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = SupplyChain::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_inventory() {
        let mut c = SupplyChain::new();
        c.inventory_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = SupplyChain::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
