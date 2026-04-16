/// aurora-sim-env: sim env
/// Phase 2527

#[derive(Debug, Clone)]
pub struct SimEnv {
    pub terrain_ok: bool,
    pub obstacle_ok: bool,
    pub road_ok: bool,
    pub light_ok: bool,
    pub weather_ok: bool,
}

impl Default for SimEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl SimEnv {
    pub fn new() -> Self {
        Self {
            terrain_ok: true,
            obstacle_ok: true,
            road_ok: true,
            light_ok: true,
            weather_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.terrain_ok && self.obstacle_ok && self.road_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.light_ok && self.weather_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.terrain_ok || !self.obstacle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.terrain_ok {
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
        let c = SimEnv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimEnv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimEnv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimEnv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimEnv::new();
        c.terrain_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimEnv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimEnv::default();
        assert!(c.all_ok());
    }
}
