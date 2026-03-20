/// Scenic route: vista, landmark, photo spot, detour
/// Phase 918

#[derive(Debug, Clone)]
pub struct ScenicRoute {
    pub vista_ok: bool,
    pub landmark_ok: bool,
    pub photo_ok: bool,
    pub detour_ok: bool,
    pub database_ok: bool,
}

impl Default for ScenicRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl ScenicRoute {
    pub fn new() -> Self {
        Self {
            vista_ok: true,
            landmark_ok: true,
            photo_ok: true,
            detour_ok: true,
            database_ok: true,
        }
    }

    pub fn discovery_ok(&self) -> bool {
        self.vista_ok && self.landmark_ok && self.database_ok
    }

    pub fn experience_ok(&self) -> bool {
        self.photo_ok && self.detour_ok
    }

    pub fn all_ok(&self) -> bool {
        self.discovery_ok() && self.experience_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.vista_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.database_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery() {
        let c = ScenicRoute::new();
        assert!(c.discovery_ok());
    }

    #[test]
    fn test_experience() {
        let c = ScenicRoute::new();
        assert!(c.experience_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ScenicRoute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = ScenicRoute::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = ScenicRoute::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = ScenicRoute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
