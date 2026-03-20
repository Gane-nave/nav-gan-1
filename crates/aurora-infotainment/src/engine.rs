/// Infotainment system: display, audio, navigation, connectivity
/// Phase 714

#[derive(Debug, Clone)]
pub struct Infotainment {
    pub display_ok: bool,
    pub audio_ok: bool,
    pub nav_ok: bool,
    pub connect_ok: bool,
    pub touch_ok: bool,
}

impl Default for Infotainment {
    fn default() -> Self {
        Self::new()
    }
}

impl Infotainment {
    pub fn new() -> Self {
        Self {
            display_ok: true,
            audio_ok: true,
            nav_ok: true,
            connect_ok: true,
            touch_ok: true,
        }
    }

    pub fn media_ok(&self) -> bool {
        self.display_ok && self.audio_ok && self.touch_ok
    }

    pub fn services_ok(&self) -> bool {
        self.nav_ok && self.connect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.media_ok() && self.services_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.display_ok || !self.touch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.display_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media() {
        let c = Infotainment::new();
        assert!(c.media_ok());
    }

    #[test]
    fn test_services() {
        let c = Infotainment::new();
        assert!(c.services_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Infotainment::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Infotainment::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_display() {
        let mut c = Infotainment::new();
        c.display_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Infotainment::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
