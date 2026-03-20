/// Elevation service: query, profile, gradient, contour, cache
/// Phase 1089

#[derive(Debug, Clone)]
pub struct Elevation {
    pub query_ok: bool,
    pub profile_ok: bool,
    pub gradient_ok: bool,
    pub contour_ok: bool,
    pub cache_ok: bool,
}

impl Default for Elevation {
    fn default() -> Self {
        Self::new()
    }
}

impl Elevation {
    pub fn new() -> Self {
        Self {
            query_ok: true,
            profile_ok: true,
            gradient_ok: true,
            contour_ok: true,
            cache_ok: true,
        }
    }

    pub fn terrain_ok(&self) -> bool {
        self.query_ok && self.profile_ok && self.gradient_ok
    }

    pub fn rendering_ok(&self) -> bool {
        self.contour_ok && self.cache_ok
    }

    pub fn all_ok(&self) -> bool {
        self.terrain_ok() && self.rendering_ok()
    }

    pub fn needs_refresh(&self) -> bool {
        !self.query_ok || !self.profile_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.query_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain() {
        let c = Elevation::new();
        assert!(c.terrain_ok());
    }

    #[test]
    fn test_rendering() {
        let c = Elevation::new();
        assert!(c.rendering_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Elevation::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refresh() {
        let c = Elevation::new();
        assert!(!c.needs_refresh());
    }

    #[test]
    fn test_query() {
        let mut c = Elevation::new();
        c.query_ok = false;
        assert!(c.needs_refresh());
    }

    #[test]
    fn test_health() {
        let c = Elevation::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
