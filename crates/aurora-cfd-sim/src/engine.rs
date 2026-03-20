/// cfd sim: mesh, solve, visualize, optimize, log
/// Phase 1403

#[derive(Debug, Clone)]
pub struct CfdSim {
    pub mesh_ok: bool,
    pub solve_ok: bool,
    pub visualize_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for CfdSim {
    fn default() -> Self {
        Self::new()
    }
}

impl CfdSim {
    pub fn new() -> Self {
        Self {
            mesh_ok: true,
            solve_ok: true,
            visualize_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mesh_ok && self.solve_ok && self.visualize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mesh_ok || !self.solve_ok
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
        let c = CfdSim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CfdSim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CfdSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CfdSim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CfdSim::new();
        c.mesh_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CfdSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
