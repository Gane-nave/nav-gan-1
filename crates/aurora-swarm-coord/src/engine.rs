/// swarm coordination: elect, assign, sync, migrate, disband
/// Phase 1139

#[derive(Debug, Clone)]
pub struct SwarmCoord {
    pub elect_ok: bool,
    pub assign_ok: bool,
    pub sync_ok: bool,
    pub migrate_ok: bool,
    pub disband_ok: bool,
}

impl Default for SwarmCoord {
    fn default() -> Self {
        Self::new()
    }
}

impl SwarmCoord {
    pub fn new() -> Self {
        Self {
            elect_ok: true,
            assign_ok: true,
            sync_ok: true,
            migrate_ok: true,
            disband_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.elect_ok && self.assign_ok && self.sync_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.migrate_ok && self.disband_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.elect_ok || !self.assign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.elect_ok {
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
        let c = SwarmCoord::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SwarmCoord::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SwarmCoord::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SwarmCoord::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SwarmCoord::new();
        c.elect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SwarmCoord::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
