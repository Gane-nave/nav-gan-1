/// Clustering: distance, assign, centroid, evaluate, visualize
/// Phase 1030

#[derive(Debug, Clone)]
pub struct ClusterMl {
    pub distance_ok: bool,
    pub assign_ok: bool,
    pub centroid_ok: bool,
    pub evaluate_ok: bool,
    pub visualize_ok: bool,
}

impl Default for ClusterMl {
    fn default() -> Self {
        Self::new()
    }
}

impl ClusterMl {
    pub fn new() -> Self {
        Self {
            distance_ok: true,
            assign_ok: true,
            centroid_ok: true,
            evaluate_ok: true,
            visualize_ok: true,
        }
    }

    pub fn grouping_ok(&self) -> bool {
        self.distance_ok && self.assign_ok && self.centroid_ok
    }

    pub fn analysis_ok(&self) -> bool {
        self.evaluate_ok && self.visualize_ok
    }

    pub fn all_ok(&self) -> bool {
        self.grouping_ok() && self.analysis_ok()
    }

    pub fn needs_init(&self) -> bool {
        !self.centroid_ok || !self.distance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.distance_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grouping() {
        let c = ClusterMl::new();
        assert!(c.grouping_ok());
    }

    #[test]
    fn test_analysis() {
        let c = ClusterMl::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClusterMl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_init() {
        let c = ClusterMl::new();
        assert!(!c.needs_init());
    }

    #[test]
    fn test_centroid() {
        let mut c = ClusterMl::new();
        c.centroid_ok = false;
        assert!(c.needs_init());
    }

    #[test]
    fn test_health() {
        let c = ClusterMl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
