/// Sway bar: link, bushing, bar integrity
/// Phase 641

#[derive(Debug, Clone)]
pub struct SwayBar {
    pub link_ok: bool,
    pub bushing_ok: bool,
    pub bar_ok: bool,
    pub mount_ok: bool,
    pub noise_free: bool,
}

impl Default for SwayBar {
    fn default() -> Self {
        Self::new()
    }
}

impl SwayBar {
    pub fn new() -> Self {
        Self {
            link_ok: true,
            bushing_ok: true,
            bar_ok: true,
            mount_ok: true,
            noise_free: true,
        }
    }

    pub fn linkage_ok(&self) -> bool {
        self.link_ok && self.bushing_ok
    }

    pub fn structure_ok(&self) -> bool {
        self.bar_ok && self.mount_ok
    }

    pub fn all_ok(&self) -> bool {
        self.linkage_ok() && self.structure_ok() && self.noise_free
    }

    pub fn needs_service(&self) -> bool {
        !self.link_ok || !self.bushing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bar_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linkage() {
        let c = SwayBar::new();
        assert!(c.linkage_ok());
    }

    #[test]
    fn test_structure() {
        let c = SwayBar::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SwayBar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SwayBar::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_link() {
        let mut c = SwayBar::new();
        c.link_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SwayBar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
