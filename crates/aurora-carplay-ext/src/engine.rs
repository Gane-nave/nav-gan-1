/// CarPlay extension: template, session, audio, phone, map
/// Phase 991

#[derive(Debug, Clone)]
pub struct CarplayExt {
    pub template_ok: bool,
    pub session_ok: bool,
    pub audio_ok: bool,
    pub phone_ok: bool,
    pub map_ok: bool,
}

impl Default for CarplayExt {
    fn default() -> Self {
        Self::new()
    }
}

impl CarplayExt {
    pub fn new() -> Self {
        Self {
            template_ok: true,
            session_ok: true,
            audio_ok: true,
            phone_ok: true,
            map_ok: true,
        }
    }

    pub fn interface_ok(&self) -> bool {
        self.template_ok && self.session_ok && self.map_ok
    }

    pub fn media_ok(&self) -> bool {
        self.audio_ok && self.phone_ok
    }

    pub fn all_ok(&self) -> bool {
        self.interface_ok() && self.media_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.template_ok || !self.session_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.template_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface() {
        let c = CarplayExt::new();
        assert!(c.interface_ok());
    }

    #[test]
    fn test_media() {
        let c = CarplayExt::new();
        assert!(c.media_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CarplayExt::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = CarplayExt::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_template() {
        let mut c = CarplayExt::new();
        c.template_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = CarplayExt::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
