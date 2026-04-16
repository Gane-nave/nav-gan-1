/// Interior light: dome, map, ambient, dimmer
/// Phase 676

#[derive(Debug, Clone)]
pub struct InteriorLight {
    pub dome_ok: bool,
    pub map_ok: bool,
    pub ambient_ok: bool,
    pub dimmer_ok: bool,
    pub door_switch_ok: bool,
}

impl Default for InteriorLight {
    fn default() -> Self {
        Self::new()
    }
}

impl InteriorLight {
    pub fn new() -> Self {
        Self {
            dome_ok: true,
            map_ok: true,
            ambient_ok: true,
            dimmer_ok: true,
            door_switch_ok: true,
        }
    }

    pub fn lighting_ok(&self) -> bool {
        self.dome_ok && self.map_ok && self.ambient_ok
    }

    pub fn control_ok(&self) -> bool {
        self.dimmer_ok && self.door_switch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.lighting_ok() && self.control_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.dome_ok || !self.dimmer_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.dome_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lighting() {
        let c = InteriorLight::new();
        assert!(c.lighting_ok());
    }

    #[test]
    fn test_control() {
        let c = InteriorLight::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InteriorLight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = InteriorLight::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_dome() {
        let mut c = InteriorLight::new();
        c.dome_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = InteriorLight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
