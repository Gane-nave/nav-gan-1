/// Roundabout: entry, exit, lane, yield, navigation
/// Phase 934

#[derive(Debug, Clone)]
pub struct Roundabout {
    pub entry_ok: bool,
    pub exit_ok: bool,
    pub lane_ok: bool,
    pub yield_ok: bool,
    pub nav_ok: bool,
}

impl Default for Roundabout {
    fn default() -> Self {
        Self::new()
    }
}

impl Roundabout {
    pub fn new() -> Self {
        Self {
            entry_ok: true,
            exit_ok: true,
            lane_ok: true,
            yield_ok: true,
            nav_ok: true,
        }
    }

    pub fn approach_ok(&self) -> bool {
        self.entry_ok && self.yield_ok && self.nav_ok
    }

    pub fn traverse_ok(&self) -> bool {
        self.lane_ok && self.exit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.approach_ok() && self.traverse_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.nav_ok || !self.entry_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.nav_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approach() {
        let c = Roundabout::new();
        assert!(c.approach_ok());
    }

    #[test]
    fn test_traverse() {
        let c = Roundabout::new();
        assert!(c.traverse_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Roundabout::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = Roundabout::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_nav() {
        let mut c = Roundabout::new();
        c.nav_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = Roundabout::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
