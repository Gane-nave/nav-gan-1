/// Glow plug monitoring: preheat control, resistance, diesel cold start
/// Phase 221

#[derive(Debug, Clone)]
pub struct GlowPlug {
    pub cylinder: u8,
    pub resistance_ohm: f64,
    pub current_a: f64,
    pub temp_c: f64,
    pub preheat_complete: bool,
    pub fault: bool,
}

impl Default for GlowPlug {
    fn default() -> Self {
        Self::new()
    }
}

impl GlowPlug {
    pub fn new() -> Self {
        Self {
            cylinder: 1,
            resistance_ohm: 0.8,
            current_a: 15.0,
            temp_c: 900.0,
            preheat_complete: true,
            fault: false,
        }
    }

    pub fn resistance_ok(&self) -> bool {
        self.resistance_ohm > 0.3 && self.resistance_ohm < 2.0
    }

    pub fn current_ok(&self) -> bool {
        self.current_a > 5.0 && self.current_a < 25.0
    }

    pub fn at_temp(&self) -> bool {
        self.temp_c > 800.0
    }

    pub fn needs_replacement(&self) -> bool {
        self.fault || !self.resistance_ok()
    }

    pub fn health_score(&self) -> f64 {
        if self.fault {
            return 0.0;
        }
        let mut score: f64 = 100.0;
        if !self.resistance_ok() {
            score -= 40.0;
        }
        if !self.current_ok() {
            score -= 30.0;
        }
        if !self.preheat_complete {
            score -= 15.0;
        }
        score.max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct GlowPlugSystem {
    pub plugs: Vec<GlowPlug>,
    pub preheat_time_s: f64,
    pub ambient_temp_c: f64,
}

impl Default for GlowPlugSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl GlowPlugSystem {
    pub fn new() -> Self {
        Self {
            plugs: vec![GlowPlug::new()],
            preheat_time_s: 3.0,
            ambient_temp_c: 10.0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.plugs.iter().all(|p| !p.fault && p.resistance_ok())
    }

    pub fn fault_count(&self) -> usize {
        self.plugs.iter().filter(|p| p.fault).count()
    }

    pub fn cold_start_needed(&self) -> bool {
        self.ambient_temp_c < 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resistance_ok() {
        let g = GlowPlug::new();
        assert!(g.resistance_ok());
    }

    #[test]
    fn test_current_ok() {
        let g = GlowPlug::new();
        assert!(g.current_ok());
    }

    #[test]
    fn test_at_temp() {
        let g = GlowPlug::new();
        assert!(g.at_temp());
    }

    #[test]
    fn test_no_replacement() {
        let g = GlowPlug::new();
        assert!(!g.needs_replacement());
    }

    #[test]
    fn test_system_ok() {
        let s = GlowPlugSystem::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_cold_start() {
        let s = GlowPlugSystem::new();
        assert!(!s.cold_start_needed());
    }

    #[test]
    fn test_health() {
        let g = GlowPlug::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
