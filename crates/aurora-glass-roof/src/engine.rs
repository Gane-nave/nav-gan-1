/// Glass roof: electrochromic, UV filter, insulation, shade
/// Phase 884

#[derive(Debug, Clone)]
pub struct GlassRoof {
    pub electro_ok: bool,
    pub uv_ok: bool,
    pub insulation_ok: bool,
    pub shade_ok: bool,
    pub seal_ok: bool,
}

impl Default for GlassRoof {
    fn default() -> Self {
        Self::new()
    }
}

impl GlassRoof {
    pub fn new() -> Self {
        Self {
            electro_ok: true,
            uv_ok: true,
            insulation_ok: true,
            shade_ok: true,
            seal_ok: true,
        }
    }

    pub fn comfort_ok(&self) -> bool {
        self.electro_ok && self.shade_ok && self.insulation_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.uv_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.comfort_ok() && self.protection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.electro_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.electro_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comfort() {
        let c = GlassRoof::new();
        assert!(c.comfort_ok());
    }

    #[test]
    fn test_protection() {
        let c = GlassRoof::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GlassRoof::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = GlassRoof::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_electro() {
        let mut c = GlassRoof::new();
        c.electro_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = GlassRoof::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
