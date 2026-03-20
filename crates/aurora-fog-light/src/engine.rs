/// Fog light: front/rear fog, auto activation, visibility sensing
/// Phase 244

#[derive(Debug, Clone)]
pub struct FogLight {
    pub front_on: bool,
    pub rear_on: bool,
    pub front_ok: bool,
    pub rear_ok: bool,
    pub visibility_m: f64,
    pub auto_mode: bool,
}

impl Default for FogLight {
    fn default() -> Self {
        Self::new()
    }
}

impl FogLight {
    pub fn new() -> Self {
        Self {
            front_on: false,
            rear_on: false,
            front_ok: true,
            rear_ok: true,
            visibility_m: 500.0,
            auto_mode: true,
        }
    }

    pub fn fog_conditions(&self) -> bool {
        self.visibility_m < 100.0
    }

    pub fn should_activate(&self) -> bool {
        self.auto_mode && self.fog_conditions()
    }

    pub fn all_ok(&self) -> bool {
        self.front_ok && self.rear_ok
    }

    pub fn any_on(&self) -> bool {
        self.front_on || self.rear_on
    }

    pub fn health_score(&self) -> f64 {
        if !self.front_ok && !self.rear_ok {
            return 0.0;
        }
        if !self.all_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_fog() {
        let f = FogLight::new();
        assert!(!f.fog_conditions());
    }

    #[test]
    fn test_no_activate() {
        let f = FogLight::new();
        assert!(!f.should_activate());
    }

    #[test]
    fn test_all_ok() {
        let f = FogLight::new();
        assert!(f.all_ok());
    }

    #[test]
    fn test_none_on() {
        let f = FogLight::new();
        assert!(!f.any_on());
    }

    #[test]
    fn test_fog() {
        let mut f = FogLight::new();
        f.visibility_m = 50.0;
        assert!(f.should_activate());
    }

    #[test]
    fn test_health() {
        let f = FogLight::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
