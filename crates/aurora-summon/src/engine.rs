/// Summon: remote call, path, obstacle, speed limit
/// Phase 889

#[derive(Debug, Clone)]
pub struct Summon {
    pub remote_ok: bool,
    pub path_ok: bool,
    pub obstacle_ok: bool,
    pub speed_ok: bool,
    pub comm_ok: bool,
}

impl Default for Summon {
    fn default() -> Self {
        Self::new()
    }
}

impl Summon {
    pub fn new() -> Self {
        Self {
            remote_ok: true,
            path_ok: true,
            obstacle_ok: true,
            speed_ok: true,
            comm_ok: true,
        }
    }

    pub fn navigation_ok(&self) -> bool {
        self.remote_ok && self.path_ok && self.comm_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.obstacle_ok && self.speed_ok
    }

    pub fn all_ok(&self) -> bool {
        self.navigation_ok() && self.safety_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.path_ok || !self.comm_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.remote_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() {
        let c = Summon::new();
        assert!(c.navigation_ok());
    }

    #[test]
    fn test_safety() {
        let c = Summon::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Summon::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = Summon::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_path() {
        let mut c = Summon::new();
        c.path_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = Summon::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
