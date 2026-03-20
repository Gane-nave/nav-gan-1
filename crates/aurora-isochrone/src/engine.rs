/// Isochrone: compute, polygon, intersect, cache, visualize
/// Phase 1088

#[derive(Debug, Clone)]
pub struct Isochrone {
    pub compute_ok: bool,
    pub polygon_ok: bool,
    pub intersect_ok: bool,
    pub cache_ok: bool,
    pub visualize_ok: bool,
}

impl Default for Isochrone {
    fn default() -> Self {
        Self::new()
    }
}

impl Isochrone {
    pub fn new() -> Self {
        Self {
            compute_ok: true,
            polygon_ok: true,
            intersect_ok: true,
            cache_ok: true,
            visualize_ok: true,
        }
    }

    pub fn calculation_ok(&self) -> bool {
        self.compute_ok && self.polygon_ok && self.intersect_ok
    }

    pub fn presentation_ok(&self) -> bool {
        self.cache_ok && self.visualize_ok
    }

    pub fn all_ok(&self) -> bool {
        self.calculation_ok() && self.presentation_ok()
    }

    pub fn needs_recompute(&self) -> bool {
        !self.compute_ok || !self.polygon_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compute_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation() {
        let c = Isochrone::new();
        assert!(c.calculation_ok());
    }

    #[test]
    fn test_presentation() {
        let c = Isochrone::new();
        assert!(c.presentation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Isochrone::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_recompute() {
        let c = Isochrone::new();
        assert!(!c.needs_recompute());
    }

    #[test]
    fn test_compute() {
        let mut c = Isochrone::new();
        c.compute_ok = false;
        assert!(c.needs_recompute());
    }

    #[test]
    fn test_health() {
        let c = Isochrone::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
