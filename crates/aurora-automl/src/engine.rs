/// AutoML: search, tune, select, ensemble, deploy
/// Phase 1020

#[derive(Debug, Clone)]
pub struct AutoMl {
    pub search_ok: bool,
    pub tune_ok: bool,
    pub select_ok: bool,
    pub ensemble_ok: bool,
    pub deploy_ok: bool,
}

impl Default for AutoMl {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoMl {
    pub fn new() -> Self {
        Self {
            search_ok: true,
            tune_ok: true,
            select_ok: true,
            ensemble_ok: true,
            deploy_ok: true,
        }
    }

    pub fn optimization_ok(&self) -> bool {
        self.search_ok && self.tune_ok && self.select_ok
    }

    pub fn production_ok(&self) -> bool {
        self.ensemble_ok && self.deploy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.optimization_ok() && self.production_ok()
    }

    pub fn needs_search(&self) -> bool {
        !self.search_ok || !self.tune_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.search_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization() {
        let c = AutoMl::new();
        assert!(c.optimization_ok());
    }

    #[test]
    fn test_production() {
        let c = AutoMl::new();
        assert!(c.production_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AutoMl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_search() {
        let c = AutoMl::new();
        assert!(!c.needs_search());
    }

    #[test]
    fn test_search() {
        let mut c = AutoMl::new();
        c.search_ok = false;
        assert!(c.needs_search());
    }

    #[test]
    fn test_health() {
        let c = AutoMl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
