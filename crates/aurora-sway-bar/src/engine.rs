/// Sway bar: anti-roll bar, end links, bushings
/// Phase 477

#[derive(Debug, Clone)]
pub struct SwayBar {
    pub stiffness_nmm: f64,
    pub end_link_ok: bool,
    pub bushing_ok: bool,
    pub bent: bool,
    pub corroded: bool,
}

impl Default for SwayBar {
    fn default() -> Self {
        Self::new()
    }
}

impl SwayBar {
    pub fn new() -> Self {
        Self {
            stiffness_nmm: 25.0,
            end_link_ok: true,
            bushing_ok: true,
            bent: false,
            corroded: false,
        }
    }

    pub fn effective_stiffness(&self) -> f64 {
        if self.bushing_ok { self.stiffness_nmm } else { self.stiffness_nmm * 0.6 }
    }

    pub fn linkage_ok(&self) -> bool {
        self.end_link_ok && self.bushing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.linkage_ok() && !self.bent && !self.corroded
    }

    pub fn needs_replacement(&self) -> bool {
        self.bent
    }

    pub fn health_score(&self) -> f64 {
        if self.bent { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stiffness() {
        let c = SwayBar::new();
        assert!(c.effective_stiffness() > 20.0);
    }

    #[test]
    fn test_linkage() {
        let c = SwayBar::new();
        assert!(c.linkage_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SwayBar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = SwayBar::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bent() {
        let mut c = SwayBar::new();
        c.bent = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = SwayBar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
