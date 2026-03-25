/// Terrain mapping: elevation, slope, surface, off-road
/// Phase 913

#[derive(Debug, Clone)]
pub struct TerrainMap {
    pub elevation_ok: bool,
    pub slope_ok: bool,
    pub surface_ok: bool,
    pub offroad_ok: bool,
    pub render_ok: bool,
}

impl Default for TerrainMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainMap {
    pub fn new() -> Self {
        Self {
            elevation_ok: true,
            slope_ok: true,
            surface_ok: true,
            offroad_ok: true,
            render_ok: true,
        }
    }

    pub fn analysis_ok(&self) -> bool {
        self.elevation_ok && self.slope_ok && self.surface_ok
    }

    pub fn display_ok(&self) -> bool {
        self.offroad_ok && self.render_ok
    }

    pub fn all_ok(&self) -> bool {
        self.analysis_ok() && self.display_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.elevation_ok || !self.surface_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.elevation_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis() {
        let c = TerrainMap::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_display() {
        let c = TerrainMap::new();
        assert!(c.display_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TerrainMap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = TerrainMap::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_elevation() {
        let mut c = TerrainMap::new();
        c.elevation_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = TerrainMap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
