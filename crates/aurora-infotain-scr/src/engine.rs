/// Infotainment screen: touchscreen, display resolution, response time
/// Phase 429

#[derive(Debug, Clone)]
pub struct InfotainScr {
    pub touch_ok: bool,
    pub display_ok: bool,
    pub response_ms: f64,
    pub max_response_ms: f64,
    pub dead_pixels: u32,
}

impl Default for InfotainScr {
    fn default() -> Self {
        Self::new()
    }
}

impl InfotainScr {
    pub fn new() -> Self {
        Self {
            touch_ok: true,
            display_ok: true,
            response_ms: 50.0,
            max_response_ms: 200.0,
            dead_pixels: 0,
        }
    }

    pub fn responsive(&self) -> bool {
        self.response_ms < self.max_response_ms && self.touch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.responsive() && self.display_ok && self.dead_pixels == 0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.display_ok || self.dead_pixels > 5
    }

    pub fn quality_score(&self) -> f64 {
        if !self.display_ok {
            return 0.0;
        }
        if self.dead_pixels > 0 {
            return 60.0;
        }
        100.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.display_ok {
            return 0.0;
        }
        if !self.touch_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive() {
        let i = InfotainScr::new();
        assert!(i.responsive());
    }

    #[test]
    fn test_all_ok() {
        let i = InfotainScr::new();
        assert!(i.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let i = InfotainScr::new();
        assert!(!i.needs_replacement());
    }

    #[test]
    fn test_quality() {
        let i = InfotainScr::new();
        assert!((i.quality_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_dead_pixels() {
        let mut i = InfotainScr::new();
        i.dead_pixels = 10;
        assert!(i.needs_replacement());
    }

    #[test]
    fn test_health() {
        let i = InfotainScr::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
