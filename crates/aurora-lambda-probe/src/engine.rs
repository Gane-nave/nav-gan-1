/// lambda probe: sense, heat, trim, age, check
/// Phase 1243

#[derive(Debug, Clone)]
pub struct LambdaProbe {
    pub sense_ok: bool,
    pub heat_ok: bool,
    pub trim_ok: bool,
    pub age_ok: bool,
    pub check_ok: bool,
}

impl Default for LambdaProbe {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaProbe {
    pub fn new() -> Self {
        Self {
            sense_ok: true,
            heat_ok: true,
            trim_ok: true,
            age_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sense_ok && self.heat_ok && self.trim_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.age_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sense_ok || !self.heat_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sense_ok {
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
        let c = LambdaProbe::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LambdaProbe::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LambdaProbe::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LambdaProbe::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LambdaProbe::new();
        c.sense_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LambdaProbe::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
