/// Head-up display: projector, combiner, brightness, align
/// Phase 742

#[derive(Debug, Clone)]
pub struct HeadUp {
    pub projector_ok: bool,
    pub combiner_ok: bool,
    pub brightness_ok: bool,
    pub aligned: bool,
    pub content_ok: bool,
}

impl Default for HeadUp {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadUp {
    pub fn new() -> Self {
        Self {
            projector_ok: true,
            combiner_ok: true,
            brightness_ok: true,
            aligned: true,
            content_ok: true,
        }
    }

    pub fn display_ok(&self) -> bool {
        self.projector_ok && self.combiner_ok && self.brightness_ok
    }

    pub fn setup_ok(&self) -> bool {
        self.aligned && self.content_ok
    }

    pub fn all_ok(&self) -> bool {
        self.display_ok() && self.setup_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.projector_ok || !self.aligned
    }

    pub fn health_score(&self) -> f64 {
        if !self.projector_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let c = HeadUp::new();
        assert!(c.display_ok());
    }

    #[test]
    fn test_setup() {
        let c = HeadUp::new();
        assert!(c.setup_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeadUp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HeadUp::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_projector() {
        let mut c = HeadUp::new();
        c.projector_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HeadUp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
