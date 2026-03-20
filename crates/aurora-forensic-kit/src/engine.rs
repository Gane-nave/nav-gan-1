/// Forensic kit: image, timeline, artifact, chain, report
/// Phase 1012

#[derive(Debug, Clone)]
pub struct ForensicKit {
    pub image_ok: bool,
    pub timeline_ok: bool,
    pub artifact_ok: bool,
    pub chain_ok: bool,
    pub report_ok: bool,
}

impl Default for ForensicKit {
    fn default() -> Self {
        Self::new()
    }
}

impl ForensicKit {
    pub fn new() -> Self {
        Self {
            image_ok: true,
            timeline_ok: true,
            artifact_ok: true,
            chain_ok: true,
            report_ok: true,
        }
    }

    pub fn collection_ok(&self) -> bool {
        self.image_ok && self.timeline_ok && self.artifact_ok
    }

    pub fn documentation_ok(&self) -> bool {
        self.chain_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.collection_ok() && self.documentation_ok()
    }

    pub fn needs_review(&self) -> bool {
        !self.chain_ok || !self.image_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.image_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection() {
        let c = ForensicKit::new();
        assert!(c.collection_ok());
    }

    #[test]
    fn test_documentation() {
        let c = ForensicKit::new();
        assert!(c.documentation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ForensicKit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_review() {
        let c = ForensicKit::new();
        assert!(!c.needs_review());
    }

    #[test]
    fn test_chain() {
        let mut c = ForensicKit::new();
        c.chain_ok = false;
        assert!(c.needs_review());
    }

    #[test]
    fn test_health() {
        let c = ForensicKit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
