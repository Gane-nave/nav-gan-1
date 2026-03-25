/// Headliner: fabric, adhesive, sag, stain
/// Phase 776

#[derive(Debug, Clone)]
pub struct Headliner {
    pub fabric_ok: bool,
    pub adhesive_ok: bool,
    pub sag_free: bool,
    pub stain_free: bool,
    pub fit_ok: bool,
}

impl Default for Headliner {
    fn default() -> Self {
        Self::new()
    }
}

impl Headliner {
    pub fn new() -> Self {
        Self {
            fabric_ok: true,
            adhesive_ok: true,
            sag_free: true,
            stain_free: true,
            fit_ok: true,
        }
    }

    pub fn appearance_ok(&self) -> bool {
        self.fabric_ok && self.stain_free
    }

    pub fn attachment_ok(&self) -> bool {
        self.adhesive_ok && self.sag_free && self.fit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.appearance_ok() && self.attachment_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.sag_free || !self.adhesive_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sag_free {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance() {
        let c = Headliner::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_attachment() {
        let c = Headliner::new();
        assert!(c.attachment_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Headliner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Headliner::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_sag() {
        let mut c = Headliner::new();
        c.sag_free = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Headliner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
