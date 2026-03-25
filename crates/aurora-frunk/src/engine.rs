/// Frunk: latch, strut, light, sensor, seal
/// Phase 883

#[derive(Debug, Clone)]
pub struct Frunk {
    pub latch_ok: bool,
    pub strut_ok: bool,
    pub light_ok: bool,
    pub sensor_ok: bool,
    pub seal_ok: bool,
}

impl Default for Frunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Frunk {
    pub fn new() -> Self {
        Self {
            latch_ok: true,
            strut_ok: true,
            light_ok: true,
            sensor_ok: true,
            seal_ok: true,
        }
    }

    pub fn mechanism_ok(&self) -> bool {
        self.latch_ok && self.strut_ok
    }

    pub fn features_ok(&self) -> bool {
        self.light_ok && self.sensor_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanism_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.latch_ok || !self.strut_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.latch_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanism() {
        let c = Frunk::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_features() {
        let c = Frunk::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Frunk::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Frunk::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_latch() {
        let mut c = Frunk::new();
        c.latch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Frunk::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
