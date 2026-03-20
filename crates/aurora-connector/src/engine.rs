/// connector: mate, seal, lock, pin, check
/// Phase 1267

#[derive(Debug, Clone)]
pub struct Connector {
    pub mate_ok: bool,
    pub seal_ok: bool,
    pub lock_ok: bool,
    pub pin_ok: bool,
    pub check_ok: bool,
}

impl Default for Connector {
    fn default() -> Self {
        Self::new()
    }
}

impl Connector {
    pub fn new() -> Self {
        Self {
            mate_ok: true,
            seal_ok: true,
            lock_ok: true,
            pin_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mate_ok && self.seal_ok && self.lock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.pin_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mate_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mate_ok {
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
        let c = Connector::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Connector::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Connector::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Connector::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Connector::new();
        c.mate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Connector::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
