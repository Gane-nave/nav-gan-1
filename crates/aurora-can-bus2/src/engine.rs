/// can bus: transmit, receive, filter, arbitrate, check
/// Phase 1269

#[derive(Debug, Clone)]
pub struct CanBus2 {
    pub transmit_ok: bool,
    pub receive_ok: bool,
    pub filter_ok: bool,
    pub arbitrate_ok: bool,
    pub check_ok: bool,
}

impl Default for CanBus2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CanBus2 {
    pub fn new() -> Self {
        Self {
            transmit_ok: true,
            receive_ok: true,
            filter_ok: true,
            arbitrate_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.transmit_ok && self.receive_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.arbitrate_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.transmit_ok || !self.receive_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.transmit_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CanBus2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CanBus2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CanBus2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CanBus2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CanBus2::new();
        c.transmit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CanBus2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
