/// chain drive: mesh, tension, lubricate, guide, check
/// Phase 1227

#[derive(Debug, Clone)]
pub struct ChainDrive {
    pub mesh_ok: bool,
    pub tension_ok: bool,
    pub lubricate_ok: bool,
    pub guide_ok: bool,
    pub check_ok: bool,
}

impl Default for ChainDrive {
    fn default() -> Self {
        Self::new()
    }
}

impl ChainDrive {
    pub fn new() -> Self {
        Self {
            mesh_ok: true,
            tension_ok: true,
            lubricate_ok: true,
            guide_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mesh_ok && self.tension_ok && self.lubricate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.guide_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mesh_ok || !self.tension_ok
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
        let c = ChainDrive::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChainDrive::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChainDrive::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChainDrive::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChainDrive::new();
        c.mesh_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChainDrive::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
