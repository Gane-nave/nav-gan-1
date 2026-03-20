/// fea sim: mesh, constrain, solve, visualize, log
/// Phase 1404

#[derive(Debug, Clone)]
pub struct FeaSim {
    pub mesh_ok: bool,
    pub constrain_ok: bool,
    pub solve_ok: bool,
    pub visualize_ok: bool,
    pub log_ok: bool,
}

impl Default for FeaSim {
    fn default() -> Self {
        Self::new()
    }
}

impl FeaSim {
    pub fn new() -> Self {
        Self {
            mesh_ok: true,
            constrain_ok: true,
            solve_ok: true,
            visualize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mesh_ok && self.constrain_ok && self.solve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.visualize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mesh_ok || !self.constrain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mesh_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FeaSim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FeaSim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FeaSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FeaSim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FeaSim::new();
        c.mesh_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FeaSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
