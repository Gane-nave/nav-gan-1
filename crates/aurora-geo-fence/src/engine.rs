/// Geo-fence: virtual boundary, speed limit zones, restricted areas
/// Phase 285

#[derive(Debug, Clone)]
pub struct GeoFence {
    pub active: bool,
    pub fence_count: u16,
    pub inside_fence: bool,
    pub speed_limited: bool,
    pub max_speed_kmh: f64,
    pub alert_on_exit: bool,
}

impl Default for GeoFence {
    fn default() -> Self {
        Self::new()
    }
}

impl GeoFence {
    pub fn new() -> Self {
        Self {
            active: true,
            fence_count: 0,
            inside_fence: false,
            speed_limited: false,
            max_speed_kmh: 200.0,
            alert_on_exit: true,
        }
    }

    pub fn has_fences(&self) -> bool {
        self.fence_count > 0
    }

    pub fn speed_ok(&self, current_kmh: f64) -> bool {
        !self.speed_limited || current_kmh <= self.max_speed_kmh
    }

    pub fn violation(&self, current_kmh: f64) -> bool {
        self.speed_limited && current_kmh > self.max_speed_kmh
    }

    pub fn boundary_alert(&self) -> bool {
        self.alert_on_exit && !self.inside_fence && self.has_fences()
    }

    pub fn health_score(&self) -> f64 {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_fences() {
        let g = GeoFence::new();
        assert!(!g.has_fences());
    }

    #[test]
    fn test_speed_ok() {
        let g = GeoFence::new();
        assert!(g.speed_ok(100.0));
    }

    #[test]
    fn test_no_violation() {
        let g = GeoFence::new();
        assert!(!g.violation(100.0));
    }

    #[test]
    fn test_no_boundary() {
        let g = GeoFence::new();
        assert!(!g.boundary_alert());
    }

    #[test]
    fn test_violation() {
        let mut g = GeoFence::new();
        g.speed_limited = true;
        g.max_speed_kmh = 50.0;
        assert!(g.violation(80.0));
    }

    #[test]
    fn test_health() {
        let g = GeoFence::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
