/// infotain disp: media, nav, phone, settings, update
/// Phase 1304

#[derive(Debug, Clone)]
pub struct InfotainDisp {
    pub media_ok: bool,
    pub nav_ok: bool,
    pub phone_ok: bool,
    pub settings_ok: bool,
    pub update_ok: bool,
}

impl Default for InfotainDisp {
    fn default() -> Self {
        Self::new()
    }
}

impl InfotainDisp {
    pub fn new() -> Self {
        Self {
            media_ok: true,
            nav_ok: true,
            phone_ok: true,
            settings_ok: true,
            update_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.media_ok && self.nav_ok && self.phone_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.settings_ok && self.update_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.media_ok || !self.nav_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.media_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = InfotainDisp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfotainDisp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfotainDisp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfotainDisp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfotainDisp::new();
        c.media_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfotainDisp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
