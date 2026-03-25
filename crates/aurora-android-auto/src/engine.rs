/// Android Auto: notification, media, messaging, nav, voice
/// Phase 992

#[derive(Debug, Clone)]
pub struct AndroidAuto {
    pub notif_ok: bool,
    pub media_ok: bool,
    pub message_ok: bool,
    pub nav_ok: bool,
    pub voice_ok: bool,
}

impl Default for AndroidAuto {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidAuto {
    pub fn new() -> Self {
        Self {
            notif_ok: true,
            media_ok: true,
            message_ok: true,
            nav_ok: true,
            voice_ok: true,
        }
    }

    pub fn display_ok(&self) -> bool {
        self.notif_ok && self.media_ok && self.nav_ok
    }

    pub fn interaction_ok(&self) -> bool {
        self.message_ok && self.voice_ok
    }

    pub fn all_ok(&self) -> bool {
        self.display_ok() && self.interaction_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.notif_ok || !self.nav_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.notif_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let c = AndroidAuto::new();
        assert!(c.display_ok());
    }

    #[test]
    fn test_interaction() {
        let c = AndroidAuto::new();
        assert!(c.interaction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AndroidAuto::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = AndroidAuto::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_notif() {
        let mut c = AndroidAuto::new();
        c.notif_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = AndroidAuto::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
