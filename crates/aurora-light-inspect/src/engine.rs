/// Light inspection: aim, brightness, pattern, color
/// Phase 831

#[derive(Debug, Clone)]
pub struct LightInspect {
    pub aim_ok: bool,
    pub brightness_ok: bool,
    pub pattern_ok: bool,
    pub color_ok: bool,
    pub lens_ok: bool,
}

impl Default for LightInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl LightInspect {
    pub fn new() -> Self {
        Self {
            aim_ok: true,
            brightness_ok: true,
            pattern_ok: true,
            color_ok: true,
            lens_ok: true,
        }
    }

    pub fn output_ok(&self) -> bool {
        self.aim_ok && self.brightness_ok && self.pattern_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.color_ok && self.lens_ok
    }

    pub fn all_ok(&self) -> bool {
        self.output_ok() && self.condition_ok()
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.aim_ok || !self.brightness_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.aim_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let c = LightInspect::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_condition() {
        let c = LightInspect::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LightInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let c = LightInspect::new();
        assert!(!c.needs_adjustment());
    }

    #[test]
    fn test_aim() {
        let mut c = LightInspect::new();
        c.aim_ok = false;
        assert!(c.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let c = LightInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
