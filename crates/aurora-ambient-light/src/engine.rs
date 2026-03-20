/// ambient light: color, brightness, zone, animate, sync
/// Phase 1185

#[derive(Debug, Clone)]
pub struct AmbientLight {
    pub color_ok: bool,
    pub brightness_ok: bool,
    pub zone_ok: bool,
    pub animate_ok: bool,
    pub sync_ok: bool,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self::new()
    }
}

impl AmbientLight {
    pub fn new() -> Self {
        Self {
            color_ok: true,
            brightness_ok: true,
            zone_ok: true,
            animate_ok: true,
            sync_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.color_ok && self.brightness_ok && self.zone_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.animate_ok && self.sync_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.color_ok || !self.brightness_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.color_ok {
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
        let c = AmbientLight::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AmbientLight::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AmbientLight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AmbientLight::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AmbientLight::new();
        c.color_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AmbientLight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
