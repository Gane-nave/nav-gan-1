/// aero sim: mesh, flow, pressure, drag, log
/// Phase 1407

#[derive(Debug, Clone)]
pub struct AeroSim {
    pub mesh_ok: bool,
    pub flow_ok: bool,
    pub pressure_ok: bool,
    pub drag_ok: bool,
    pub log_ok: bool,
}

impl Default for AeroSim {
    fn default() -> Self {
        Self::new()
    }
}

impl AeroSim {
    pub fn new() -> Self {
        Self {
            mesh_ok: true,
            flow_ok: true,
            pressure_ok: true,
            drag_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mesh_ok && self.flow_ok && self.pressure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.drag_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mesh_ok || !self.flow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mesh_ok {
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
        let c = AeroSim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AeroSim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AeroSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AeroSim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AeroSim::new();
        c.mesh_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AeroSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
