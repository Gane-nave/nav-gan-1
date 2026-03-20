/// deploy ab: split, route, measure, promote, log
/// Phase 1593

#[derive(Debug, Clone)]
pub struct DeployAb2 {
    pub split_ok: bool,
    pub route_ok: bool,
    pub measure_ok: bool,
    pub promote_ok: bool,
    pub log_ok: bool,
}

impl Default for DeployAb2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DeployAb2 {
    pub fn new() -> Self {
        Self {
            split_ok: true,
            route_ok: true,
            measure_ok: true,
            promote_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.split_ok && self.route_ok && self.measure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.promote_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.split_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.split_ok {
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
        let c = DeployAb2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DeployAb2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DeployAb2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DeployAb2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DeployAb2::new();
        c.split_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DeployAb2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
