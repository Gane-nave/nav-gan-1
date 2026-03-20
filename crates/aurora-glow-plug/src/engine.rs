/// Glow plug: preheat temp, current draw, resistance
/// Phase 520

#[derive(Debug, Clone)]
pub struct GlowPlug {
    pub tip_temp_c: f64,
    pub target_temp_c: f64,
    pub current_a: f64,
    pub resistance_ok: bool,
    pub burned_out: bool,
}

impl Default for GlowPlug {
    fn default() -> Self {
        Self::new()
    }
}

impl GlowPlug {
    pub fn new() -> Self {
        Self {
            tip_temp_c: 900.0,
            target_temp_c: 850.0,
            current_a: 8.0,
            resistance_ok: true,
            burned_out: false,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.tip_temp_c >= self.target_temp_c
    }

    pub fn current_ok(&self) -> bool {
        self.current_a > 4.0 && self.current_a < 15.0
    }

    pub fn all_ok(&self) -> bool {
        self.temp_ok() && self.current_ok() && self.resistance_ok && !self.burned_out
    }

    pub fn needs_replacement(&self) -> bool {
        self.burned_out
    }

    pub fn health_score(&self) -> f64 {
        if self.burned_out { return 0.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp() {
        let c = GlowPlug::new();
        assert!(c.temp_ok());
    }

    #[test]
    fn test_current() {
        let c = GlowPlug::new();
        assert!(c.current_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GlowPlug::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = GlowPlug::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_burned() {
        let mut c = GlowPlug::new();
        c.burned_out = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = GlowPlug::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
