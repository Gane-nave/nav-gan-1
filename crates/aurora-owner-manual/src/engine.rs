/// Owner manual: search, diagram, video, FAQ, update
/// Phase 974

#[derive(Debug, Clone)]
pub struct OwnerManual {
    pub search_ok: bool,
    pub diagram_ok: bool,
    pub video_ok: bool,
    pub faq_ok: bool,
    pub update_ok: bool,
}

impl Default for OwnerManual {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnerManual {
    pub fn new() -> Self {
        Self {
            search_ok: true,
            diagram_ok: true,
            video_ok: true,
            faq_ok: true,
            update_ok: true,
        }
    }

    pub fn content_ok(&self) -> bool {
        self.search_ok && self.diagram_ok && self.video_ok
    }

    pub fn support_ok(&self) -> bool {
        self.faq_ok && self.update_ok
    }

    pub fn all_ok(&self) -> bool {
        self.content_ok() && self.support_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.update_ok || !self.search_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.search_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content() {
        let c = OwnerManual::new();
        assert!(c.content_ok());
    }

    #[test]
    fn test_support() {
        let c = OwnerManual::new();
        assert!(c.support_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OwnerManual::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = OwnerManual::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_update() {
        let mut c = OwnerManual::new();
        c.update_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = OwnerManual::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
