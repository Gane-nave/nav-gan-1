/// paint inspect: scan, thickness, color, defect, log
/// Phase 1393

#[derive(Debug, Clone)]
pub struct PaintInspect {
    pub scan_ok: bool,
    pub thickness_ok: bool,
    pub color_ok: bool,
    pub defect_ok: bool,
    pub log_ok: bool,
}

impl Default for PaintInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl PaintInspect {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            thickness_ok: true,
            color_ok: true,
            defect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.thickness_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.defect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.thickness_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = PaintInspect::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PaintInspect::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PaintInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PaintInspect::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PaintInspect::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PaintInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
