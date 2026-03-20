/// Intersection: detection, right-of-way, conflict, timing
/// Phase 933

#[derive(Debug, Clone)]
pub struct Intersection {
    pub detect_ok: bool,
    pub right_of_way_ok: bool,
    pub conflict_ok: bool,
    pub timing_ok: bool,
    pub map_ok: bool,
}

impl Default for Intersection {
    fn default() -> Self {
        Self::new()
    }
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            right_of_way_ok: true,
            conflict_ok: true,
            timing_ok: true,
            map_ok: true,
        }
    }

    pub fn awareness_ok(&self) -> bool {
        self.detect_ok && self.map_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.right_of_way_ok && self.conflict_ok && self.timing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.awareness_ok() && self.safety_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.map_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awareness() {
        let c = Intersection::new();
        assert!(c.awareness_ok());
    }

    #[test]
    fn test_safety() {
        let c = Intersection::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Intersection::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = Intersection::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_map() {
        let mut c = Intersection::new();
        c.map_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = Intersection::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
