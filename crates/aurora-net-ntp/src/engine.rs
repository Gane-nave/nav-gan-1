/// net ntp: query, offset, adjust, sync, log
/// Phase 1547

#[derive(Debug, Clone)]
pub struct NetNtp {
    pub query_ok: bool,
    pub offset_ok: bool,
    pub adjust_ok: bool,
    pub sync_ok: bool,
    pub log_ok: bool,
}

impl Default for NetNtp {
    fn default() -> Self {
        Self::new()
    }
}

impl NetNtp {
    pub fn new() -> Self {
        Self {
            query_ok: true,
            offset_ok: true,
            adjust_ok: true,
            sync_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.query_ok && self.offset_ok && self.adjust_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sync_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.query_ok || !self.offset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.query_ok {
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
        let c = NetNtp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetNtp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetNtp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetNtp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetNtp::new();
        c.query_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetNtp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
