/// Dimensionality reduction: project, embed, compress, reconstruct, evaluate
/// Phase 1031

#[derive(Debug, Clone)]
pub struct DimReduce {
    pub project_ok: bool,
    pub embed_ok: bool,
    pub compress_ok: bool,
    pub reconstruct_ok: bool,
    pub evaluate_ok: bool,
}

impl Default for DimReduce {
    fn default() -> Self {
        Self::new()
    }
}

impl DimReduce {
    pub fn new() -> Self {
        Self {
            project_ok: true,
            embed_ok: true,
            compress_ok: true,
            reconstruct_ok: true,
            evaluate_ok: true,
        }
    }

    pub fn reduction_ok(&self) -> bool {
        self.project_ok && self.embed_ok && self.compress_ok
    }

    pub fn validation_ok(&self) -> bool {
        self.reconstruct_ok && self.evaluate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.reduction_ok() && self.validation_ok()
    }

    pub fn needs_fit(&self) -> bool {
        !self.project_ok || !self.embed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.project_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduction() {
        let c = DimReduce::new();
        assert!(c.reduction_ok());
    }

    #[test]
    fn test_validation() {
        let c = DimReduce::new();
        assert!(c.validation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DimReduce::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_fit() {
        let c = DimReduce::new();
        assert!(!c.needs_fit());
    }

    #[test]
    fn test_project() {
        let mut c = DimReduce::new();
        c.project_ok = false;
        assert!(c.needs_fit());
    }

    #[test]
    fn test_health() {
        let c = DimReduce::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
