/// Google Home link: trait, command, query, report, sync
/// Phase 990

#[derive(Debug, Clone)]
pub struct GoogleLink {
    pub trait_ok: bool,
    pub command_ok: bool,
    pub query_ok: bool,
    pub report_ok: bool,
    pub sync_ok: bool,
}

impl Default for GoogleLink {
    fn default() -> Self {
        Self::new()
    }
}

impl GoogleLink {
    pub fn new() -> Self {
        Self {
            trait_ok: true,
            command_ok: true,
            query_ok: true,
            report_ok: true,
            sync_ok: true,
        }
    }

    pub fn control_ok(&self) -> bool {
        self.trait_ok && self.command_ok && self.query_ok
    }

    pub fn state_ok(&self) -> bool {
        self.report_ok && self.sync_ok
    }

    pub fn all_ok(&self) -> bool {
        self.control_ok() && self.state_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.sync_ok || !self.trait_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.trait_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control() {
        let c = GoogleLink::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_state() {
        let c = GoogleLink::new();
        assert!(c.state_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GoogleLink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = GoogleLink::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_sync() {
        let mut c = GoogleLink::new();
        c.sync_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = GoogleLink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
