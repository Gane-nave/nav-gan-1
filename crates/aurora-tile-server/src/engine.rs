/// Tile server: render, cache, serve, style, source
/// Phase 1091

#[derive(Debug, Clone)]
pub struct TileServer {
    pub render_ok: bool,
    pub cache_ok: bool,
    pub serve_ok: bool,
    pub style_ok: bool,
    pub source_ok: bool,
}

impl Default for TileServer {
    fn default() -> Self {
        Self::new()
    }
}

impl TileServer {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            cache_ok: true,
            serve_ok: true,
            style_ok: true,
            source_ok: true,
        }
    }

    pub fn generation_ok(&self) -> bool {
        self.render_ok && self.cache_ok && self.serve_ok
    }

    pub fn configuration_ok(&self) -> bool {
        self.style_ok && self.source_ok
    }

    pub fn all_ok(&self) -> bool {
        self.generation_ok() && self.configuration_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.render_ok || !self.cache_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.render_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let c = TileServer::new();
        assert!(c.generation_ok());
    }

    #[test]
    fn test_configuration() {
        let c = TileServer::new();
        assert!(c.configuration_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TileServer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = TileServer::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_render() {
        let mut c = TileServer::new();
        c.render_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = TileServer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
