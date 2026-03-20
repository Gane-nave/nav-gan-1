/// ml model: define, compile, optimize, quantize, log
/// Phase 1468

#[derive(Debug, Clone)]
pub struct MlModel {
    pub define_ok: bool,
    pub compile_ok: bool,
    pub optimize_ok: bool,
    pub quantize_ok: bool,
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
            define_ok: true,
            compile_ok: true,
            optimize_ok: true,
            quantize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.compile_ok && self.optimize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.quantize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.compile_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlModel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
