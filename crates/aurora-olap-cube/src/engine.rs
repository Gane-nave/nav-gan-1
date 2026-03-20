/// OLAP cube: dimension, measure, aggregate, slice, drill
/// Phase 1043

#[derive(Debug, Clone)]
pub struct OlapCube {
    pub dimension_ok: bool,
    pub measure_ok: bool,
    pub aggregate_ok: bool,
    pub slice_ok: bool,
    pub drill_ok: bool,
}

impl Default for OlapCube {
    fn default() -> Self {
        Self::new()
    }
}

impl OlapCube {
    pub fn new() -> Self {
        Self {
            dimension_ok: true,
            measure_ok: true,
            aggregate_ok: true,
            slice_ok: true,
            drill_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.dimension_ok && self.measure_ok && self.aggregate_ok
    }

    pub fn navigation_ok(&self) -> bool {
        self.slice_ok && self.drill_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.navigation_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.dimension_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.dimension_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = OlapCube::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_navigation() {
        let c = OlapCube::new();
        assert!(c.navigation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OlapCube::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = OlapCube::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_dimension() {
        let mut c = OlapCube::new();
        c.dimension_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = OlapCube::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
