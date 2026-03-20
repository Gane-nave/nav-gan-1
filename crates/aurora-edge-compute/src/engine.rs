/// Edge compute: inference, cache, offload, priority, power
/// Phase 979

#[derive(Debug, Clone)]
pub struct EdgeCompute {
    pub inference_ok: bool,
    pub cache_ok: bool,
    pub offload_ok: bool,
    pub priority_ok: bool,
    pub power_ok: bool,
}

impl Default for EdgeCompute {
    fn default() -> Self {
        Self::new()
    }
}

impl EdgeCompute {
    pub fn new() -> Self {
        Self {
            inference_ok: true,
            cache_ok: true,
            offload_ok: true,
            priority_ok: true,
            power_ok: true,
        }
    }

    pub fn processing_ok(&self) -> bool {
        self.inference_ok && self.cache_ok && self.power_ok
    }

    pub fn management_ok(&self) -> bool {
        self.offload_ok && self.priority_ok
    }

    pub fn all_ok(&self) -> bool {
        self.processing_ok() && self.management_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.inference_ok || !self.cache_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inference_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing() {
        let c = EdgeCompute::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_management() {
        let c = EdgeCompute::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EdgeCompute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = EdgeCompute::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_inference() {
        let mut c = EdgeCompute::new();
        c.inference_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = EdgeCompute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
