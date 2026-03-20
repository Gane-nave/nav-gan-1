/// zigbee radio: join, send, receive, mesh, log
/// Phase 1345

#[derive(Debug, Clone)]
pub struct ZigbeeRadio {
    pub join_ok: bool,
    pub send_ok: bool,
    pub receive_ok: bool,
    pub mesh_ok: bool,
    pub log_ok: bool,
}

impl Default for ZigbeeRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl ZigbeeRadio {
    pub fn new() -> Self {
        Self {
            join_ok: true,
            send_ok: true,
            receive_ok: true,
            mesh_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.join_ok && self.send_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.mesh_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.join_ok || !self.send_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.join_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ZigbeeRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ZigbeeRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ZigbeeRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ZigbeeRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ZigbeeRadio::new();
        c.join_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ZigbeeRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
