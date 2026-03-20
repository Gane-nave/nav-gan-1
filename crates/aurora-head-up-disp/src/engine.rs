/// Head-up display: windshield projection, brightness, content selection
/// Phase 261

#[derive(Debug, Clone)]
pub struct HeadUpDisplay {
    pub active: bool,
    pub brightness_pct: f64,
    pub show_speed: bool,
    pub show_nav: bool,
    pub show_warnings: bool,
    pub projector_ok: bool,
}

impl Default for HeadUpDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadUpDisplay {
    pub fn new() -> Self {
        Self {
            active: true,
            brightness_pct: 70.0,
            show_speed: true,
            show_nav: true,
            show_warnings: true,
            projector_ok: true,
        }
    }

    pub fn is_displaying(&self) -> bool {
        self.active && self.projector_ok
    }

    pub fn content_count(&self) -> u8 {
        let mut count: u8 = 0;
        if self.show_speed {
            count += 1;
        }
        if self.show_nav {
            count += 1;
        }
        if self.show_warnings {
            count += 1;
        }
        count
    }

    pub fn too_bright(&self) -> bool {
        self.brightness_pct > 90.0
    }

    pub fn too_dim(&self) -> bool {
        self.brightness_pct < 20.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.projector_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displaying() {
        let h = HeadUpDisplay::new();
        assert!(h.is_displaying());
    }

    #[test]
    fn test_content_count() {
        let h = HeadUpDisplay::new();
        assert_eq!(h.content_count(), 3);
    }

    #[test]
    fn test_not_bright() {
        let h = HeadUpDisplay::new();
        assert!(!h.too_bright());
    }

    #[test]
    fn test_not_dim() {
        let h = HeadUpDisplay::new();
        assert!(!h.too_dim());
    }

    #[test]
    fn test_off() {
        let mut h = HeadUpDisplay::new();
        h.active = false;
        assert!(!h.is_displaying());
    }

    #[test]
    fn test_health() {
        let h = HeadUpDisplay::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
