/// Paint thickness: DFT measurement, orange peel, color match
/// Phase 390

#[derive(Debug, Clone)]
pub struct PaintThick {
    pub thickness_um: f64,
    pub min_um: f64,
    pub max_um: f64,
    pub orange_peel_ok: bool,
    pub color_match_ok: bool,
}

impl Default for PaintThick {
    fn default() -> Self {
        Self::new()
    }
}

impl PaintThick {
    pub fn new() -> Self {
        Self {
            thickness_um: 120.0,
            min_um: 80.0,
            max_um: 180.0,
            orange_peel_ok: true,
            color_match_ok: true,
        }
    }

    pub fn in_spec(&self) -> bool {
        self.thickness_um >= self.min_um && self.thickness_um <= self.max_um
    }

    pub fn quality_ok(&self) -> bool {
        self.in_spec() && self.orange_peel_ok && self.color_match_ok
    }

    pub fn needs_respray(&self) -> bool {
        !self.in_spec() || !self.color_match_ok
    }

    pub fn thickness_pct(&self) -> f64 {
        if self.max_um <= self.min_um {
            return 0.0;
        }
        ((self.thickness_um - self.min_um) / (self.max_um - self.min_um) * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.in_spec() {
            return 20.0;
        }
        if !self.color_match_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_spec() {
        let p = PaintThick::new();
        assert!(p.in_spec());
    }

    #[test]
    fn test_quality() {
        let p = PaintThick::new();
        assert!(p.quality_ok());
    }

    #[test]
    fn test_no_respray() {
        let p = PaintThick::new();
        assert!(!p.needs_respray());
    }

    #[test]
    fn test_pct() {
        let p = PaintThick::new();
        assert!(p.thickness_pct() > 30.0);
    }

    #[test]
    fn test_too_thin() {
        let mut p = PaintThick::new();
        p.thickness_um = 50.0;
        assert!(p.needs_respray());
    }

    #[test]
    fn test_health() {
        let p = PaintThick::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
