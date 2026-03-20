/// Moonroof: glass panel, slide, tilt, shade
/// Phase 784

#[derive(Debug, Clone)]
pub struct Moonroof {
    pub glass_ok: bool,
    pub slide_ok: bool,
    pub tilt_ok: bool,
    pub shade_ok: bool,
    pub drain_ok: bool,
}

impl Default for Moonroof {
    fn default() -> Self {
        Self::new()
    }
}

impl Moonroof {
    pub fn new() -> Self {
        Self {
            glass_ok: true,
            slide_ok: true,
            tilt_ok: true,
            shade_ok: true,
            drain_ok: true,
        }
    }

    pub fn panel_ok(&self) -> bool {
        self.glass_ok && self.shade_ok
    }

    pub fn movement_ok(&self) -> bool {
        self.slide_ok && self.tilt_ok && self.drain_ok
    }

    pub fn all_ok(&self) -> bool {
        self.panel_ok() && self.movement_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.slide_ok || !self.drain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.drain_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel() {
        let c = Moonroof::new();
        assert!(c.panel_ok());
    }

    #[test]
    fn test_movement() {
        let c = Moonroof::new();
        assert!(c.movement_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Moonroof::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Moonroof::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_drain() {
        let mut c = Moonroof::new();
        c.drain_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Moonroof::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
