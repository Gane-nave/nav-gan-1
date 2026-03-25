/// Cargo optimization: weight, volume, route, stack, report
/// Phase 1098

#[derive(Debug, Clone)]
pub struct CargoOptim {
    pub weight_ok: bool,
    pub volume_ok: bool,
    pub route_ok: bool,
    pub stack_ok: bool,
    pub report_ok: bool,
}

impl Default for CargoOptim {
    fn default() -> Self {
        Self::new()
    }
}

impl CargoOptim {
    pub fn new() -> Self {
        Self {
            weight_ok: true,
            volume_ok: true,
            route_ok: true,
            stack_ok: true,
            report_ok: true,
        }
    }

    pub fn loading_ok(&self) -> bool {
        self.weight_ok && self.volume_ok && self.stack_ok
    }

    pub fn logistics_ok(&self) -> bool {
        self.route_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.loading_ok() && self.logistics_ok()
    }

    pub fn needs_replan(&self) -> bool {
        !self.weight_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.weight_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading() {
        let c = CargoOptim::new();
        assert!(c.loading_ok());
    }

    #[test]
    fn test_logistics() {
        let c = CargoOptim::new();
        assert!(c.logistics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CargoOptim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replan() {
        let c = CargoOptim::new();
        assert!(!c.needs_replan());
    }

    #[test]
    fn test_weight() {
        let mut c = CargoOptim::new();
        c.weight_ok = false;
        assert!(c.needs_replan());
    }

    #[test]
    fn test_health() {
        let c = CargoOptim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
