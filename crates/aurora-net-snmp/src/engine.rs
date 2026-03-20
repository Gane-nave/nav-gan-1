/// net snmp: query, walk, trap, inform, log
/// Phase 1548

#[derive(Debug, Clone)]
pub struct NetSnmp {
    pub query_ok: bool,
    pub walk_ok: bool,
    pub trap_ok: bool,
    pub inform_ok: bool,
    pub log_ok: bool,
}

impl Default for NetSnmp {
    fn default() -> Self {
        Self::new()
    }
}

impl NetSnmp {
    pub fn new() -> Self {
        Self {
            query_ok: true,
            walk_ok: true,
            trap_ok: true,
            inform_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.query_ok && self.walk_ok && self.trap_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.inform_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.query_ok || !self.walk_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.query_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = NetSnmp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetSnmp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetSnmp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetSnmp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetSnmp::new();
        c.query_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetSnmp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
