/// Interior LED: dome, map, footwell, vanity mirror, courtesy lights
/// Phase 425

#[derive(Debug, Clone)]
pub struct InteriorLed {
    pub dome_ok: bool,
    pub map_ok: bool,
    pub footwell_ok: bool,
    pub vanity_ok: bool,
    pub brightness_pct: f64,
}

impl Default for InteriorLed {
    fn default() -> Self {
        Self::new()
    }
}

impl InteriorLed {
    pub fn new() -> Self {
        Self {
            dome_ok: true,
            map_ok: true,
            footwell_ok: true,
            vanity_ok: true,
            brightness_pct: 80.0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.dome_ok && self.map_ok && self.footwell_ok && self.vanity_ok
    }

    pub fn dimmable(&self) -> bool {
        self.brightness_pct < 100.0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.dome_ok || !self.map_ok
    }

    pub fn working_count(&self) -> u8 {
        [self.dome_ok, self.map_ok, self.footwell_ok, self.vanity_ok]
            .iter()
            .filter(|&&x| x)
            .count() as u8
    }

    pub fn health_score(&self) -> f64 {
        if !self.dome_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let i = InteriorLed::new();
        assert!(i.all_ok());
    }

    #[test]
    fn test_dimmable() {
        let i = InteriorLed::new();
        assert!(i.dimmable());
    }

    #[test]
    fn test_no_replace() {
        let i = InteriorLed::new();
        assert!(!i.needs_replacement());
    }

    #[test]
    fn test_count() {
        let i = InteriorLed::new();
        assert_eq!(i.working_count(), 4);
    }

    #[test]
    fn test_dome_out() {
        let mut i = InteriorLed::new();
        i.dome_ok = false;
        assert!(i.needs_replacement());
    }

    #[test]
    fn test_health() {
        let i = InteriorLed::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
