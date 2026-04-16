/// Underbody: aerodynamic panel, stone guard, drainage
/// Phase 550

#[derive(Debug, Clone)]
pub struct Underbody {
    pub panel_ok: bool,
    pub stone_guard_ok: bool,
    pub drain_clear: bool,
    pub rust_free: bool,
    pub fasteners_ok: bool,
}

impl Default for Underbody {
    fn default() -> Self {
        Self::new()
    }
}

impl Underbody {
    pub fn new() -> Self {
        Self {
            panel_ok: true,
            stone_guard_ok: true,
            drain_clear: true,
            rust_free: true,
            fasteners_ok: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.panel_ok && self.stone_guard_ok
    }

    pub fn drainage_ok(&self) -> bool {
        self.drain_clear
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.drainage_ok() && self.rust_free && self.fasteners_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.panel_ok || !self.rust_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.rust_free {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = Underbody::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_drainage() {
        let c = Underbody::new();
        assert!(c.drainage_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Underbody::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Underbody::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_rust() {
        let mut c = Underbody::new();
        c.rust_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Underbody::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
