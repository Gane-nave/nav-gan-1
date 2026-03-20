/// map label: place, collide, prioritize, render, log
/// Phase 1430

#[derive(Debug, Clone)]
pub struct MapLabel {
    pub place_ok: bool,
    pub collide_ok: bool,
    pub prioritize_ok: bool,
    pub render_ok: bool,
    pub log_ok: bool,
}

impl Default for MapLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl MapLabel {
    pub fn new() -> Self {
        Self {
            place_ok: true,
            collide_ok: true,
            prioritize_ok: true,
            render_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.place_ok && self.collide_ok && self.prioritize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.render_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.place_ok || !self.collide_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.place_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MapLabel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapLabel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapLabel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapLabel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapLabel::new();
        c.place_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapLabel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
