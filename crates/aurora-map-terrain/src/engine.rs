/// map terrain: fetch, mesh, shade, contour, log
/// Phase 1440

#[derive(Debug, Clone)]
pub struct MapTerrain {
    pub fetch_ok: bool,
    pub mesh_ok: bool,
    pub shade_ok: bool,
    pub contour_ok: bool,
    pub log_ok: bool,
}

impl Default for MapTerrain {
    fn default() -> Self {
        Self::new()
    }
}

impl MapTerrain {
    pub fn new() -> Self {
        Self {
            fetch_ok: true,
            mesh_ok: true,
            shade_ok: true,
            contour_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fetch_ok && self.mesh_ok && self.shade_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.contour_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fetch_ok || !self.mesh_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fetch_ok {
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
        let c = MapTerrain::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapTerrain::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapTerrain::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapTerrain::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapTerrain::new();
        c.fetch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapTerrain::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
