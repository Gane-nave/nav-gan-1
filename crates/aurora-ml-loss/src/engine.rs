/// ml loss: compute, backward, reduce, weight, log
/// Phase 1964

#[derive(Debug, Clone)]
pub struct MlLoss {
    pub compute_ok: bool,
    pub backward_ok: bool,
    pub reduce_ok: bool,
    pub weight_ok: bool,
    pub log_ok: bool,
}

impl Default for MlLoss {
    fn default() -> Self {
        Self::new()
    }
}

impl MlLoss {
    pub fn new() -> Self {
        Self {
            compute_ok: true,
            backward_ok: true,
            reduce_ok: true,
            weight_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compute_ok && self.backward_ok && self.reduce_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.weight_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compute_ok || !self.backward_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compute_ok {
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
        let c = MlLoss::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlLoss::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlLoss::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlLoss::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlLoss::new();
        c.compute_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlLoss::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
