/// sunroof ctrl: open, tilt, close, shade, rain
/// Phase 1317

#[derive(Debug, Clone)]
pub struct SunroofCtrl {
    pub open_ok: bool,
    pub tilt_ok: bool,
    pub close_ok: bool,
    pub shade_ok: bool,
    pub rain_ok: bool,
}

impl Default for SunroofCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl SunroofCtrl {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            tilt_ok: true,
            close_ok: true,
            shade_ok: true,
            rain_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.tilt_ok && self.close_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.shade_ok && self.rain_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.tilt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SunroofCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SunroofCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SunroofCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SunroofCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SunroofCtrl::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SunroofCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
