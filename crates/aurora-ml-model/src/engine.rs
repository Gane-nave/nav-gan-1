/// ml model: create, train, evaluate, deploy, log
/// Phase 1953

#[derive(Debug, Clone)]
pub struct MlModel {
    pub create_ok: bool,
    pub train_ok: bool,
    pub evaluate_ok: bool,
    pub deploy_ok: bool,
    pub log_ok: bool,
}

impl Default for MlModel {
    fn default() -> Self {
        Self::new()
    }
}

impl MlModel {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            train_ok: true,
            evaluate_ok: true,
            deploy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.train_ok && self.evaluate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.deploy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.train_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = MlModel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlModel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlModel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlModel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlModel::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlModel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
