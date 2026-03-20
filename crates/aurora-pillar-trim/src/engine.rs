/// Pillar trim: A-pillar, B-pillar, C-pillar, clip
/// Phase 779

#[derive(Debug, Clone)]
pub struct PillarTrim {
    pub a_pillar_ok: bool,
    pub b_pillar_ok: bool,
    pub c_pillar_ok: bool,
    pub clip_ok: bool,
    pub airbag_ok: bool,
}

impl Default for PillarTrim {
    fn default() -> Self {
        Self::new()
    }
}

impl PillarTrim {
    pub fn new() -> Self {
        Self {
            a_pillar_ok: true,
            b_pillar_ok: true,
            c_pillar_ok: true,
            clip_ok: true,
            airbag_ok: true,
        }
    }

    pub fn panels_ok(&self) -> bool {
        self.a_pillar_ok && self.b_pillar_ok && self.c_pillar_ok
    }

    pub fn mounting_ok(&self) -> bool {
        self.clip_ok && self.airbag_ok
    }

    pub fn all_ok(&self) -> bool {
        self.panels_ok() && self.mounting_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.clip_ok || !self.airbag_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.airbag_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panels() {
        let c = PillarTrim::new();
        assert!(c.panels_ok());
    }

    #[test]
    fn test_mounting() {
        let c = PillarTrim::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PillarTrim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = PillarTrim::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_airbag() {
        let mut c = PillarTrim::new();
        c.airbag_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = PillarTrim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
