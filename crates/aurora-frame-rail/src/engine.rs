/// Frame rail: main rail, support, gusset, mount
/// Phase 797

#[derive(Debug, Clone)]
pub struct FrameRail {
    pub main_rail_ok: bool,
    pub support_ok: bool,
    pub gusset_ok: bool,
    pub mount_ok: bool,
    pub weld_ok: bool,
}

impl Default for FrameRail {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameRail {
    pub fn new() -> Self {
        Self {
            main_rail_ok: true,
            support_ok: true,
            gusset_ok: true,
            mount_ok: true,
            weld_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.main_rail_ok && self.support_ok && self.gusset_ok
    }

    pub fn attachment_ok(&self) -> bool {
        self.mount_ok && self.weld_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.attachment_ok()
    }

    pub fn needs_inspection(&self) -> bool {
        !self.main_rail_ok || !self.weld_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.main_rail_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = FrameRail::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_attachment() {
        let c = FrameRail::new();
        assert!(c.attachment_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FrameRail::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_inspect() {
        let c = FrameRail::new();
        assert!(!c.needs_inspection());
    }

    #[test]
    fn test_rail() {
        let mut c = FrameRail::new();
        c.main_rail_ok = false;
        assert!(c.needs_inspection());
    }

    #[test]
    fn test_health() {
        let c = FrameRail::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
