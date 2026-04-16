/// ETL pipeline: extract, transform, load, validate, schedule
/// Phase 1032

#[derive(Debug, Clone)]
pub struct EtlPipeline {
    pub extract_ok: bool,
    pub transform_ok: bool,
    pub load_ok: bool,
    pub validate_ok: bool,
    pub schedule_ok: bool,
}

impl Default for EtlPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl EtlPipeline {
    pub fn new() -> Self {
        Self {
            extract_ok: true,
            transform_ok: true,
            load_ok: true,
            validate_ok: true,
            schedule_ok: true,
        }
    }

    pub fn ingestion_ok(&self) -> bool {
        self.extract_ok && self.transform_ok && self.load_ok
    }

    pub fn control_ok(&self) -> bool {
        self.validate_ok && self.schedule_ok
    }

    pub fn all_ok(&self) -> bool {
        self.ingestion_ok() && self.control_ok()
    }

    pub fn needs_restart(&self) -> bool {
        !self.extract_ok || !self.load_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.extract_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingestion() {
        let c = EtlPipeline::new();
        assert!(c.ingestion_ok());
    }

    #[test]
    fn test_control() {
        let c = EtlPipeline::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EtlPipeline::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_restart() {
        let c = EtlPipeline::new();
        assert!(!c.needs_restart());
    }

    #[test]
    fn test_extract() {
        let mut c = EtlPipeline::new();
        c.extract_ok = false;
        assert!(c.needs_restart());
    }

    #[test]
    fn test_health() {
        let c = EtlPipeline::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
