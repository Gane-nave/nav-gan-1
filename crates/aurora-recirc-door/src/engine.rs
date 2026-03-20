/// Recirculation door: actuator, filter, fresh air
/// Phase 632

#[derive(Debug, Clone)]
pub struct RecircDoor {
    pub actuator_ok: bool,
    pub filter_ok: bool,
    pub fresh_air_ok: bool,
    pub position_ok: bool,
    pub seal_ok: bool,
}

impl Default for RecircDoor {
    fn default() -> Self {
        Self::new()
    }
}

impl RecircDoor {
    pub fn new() -> Self {
        Self {
            actuator_ok: true,
            filter_ok: true,
            fresh_air_ok: true,
            position_ok: true,
            seal_ok: true,
        }
    }

    pub fn door_ok(&self) -> bool {
        self.actuator_ok && self.position_ok
    }

    pub fn air_quality_ok(&self) -> bool {
        self.filter_ok && self.fresh_air_ok
    }

    pub fn all_ok(&self) -> bool {
        self.door_ok() && self.air_quality_ok() && self.seal_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.actuator_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_door() {
        let c = RecircDoor::new();
        assert!(c.door_ok());
    }

    #[test]
    fn test_air() {
        let c = RecircDoor::new();
        assert!(c.air_quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RecircDoor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RecircDoor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_actuator() {
        let mut c = RecircDoor::new();
        c.actuator_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RecircDoor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
