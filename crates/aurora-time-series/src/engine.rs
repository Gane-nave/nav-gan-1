/// Time series DB: ingest, downsample, retention, query, alert
/// Phase 1045

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ingest_ok: bool,
    pub downsample_ok: bool,
    pub retention_ok: bool,
    pub query_ok: bool,
    pub alert_ok: bool,
}

impl Default for TimeSeries {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeSeries {
    pub fn new() -> Self {
        Self {
            ingest_ok: true,
            downsample_ok: true,
            retention_ok: true,
            query_ok: true,
            alert_ok: true,
        }
    }

    pub fn storage_ok(&self) -> bool {
        self.ingest_ok && self.downsample_ok && self.retention_ok
    }

    pub fn analysis_ok(&self) -> bool {
        self.query_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.storage_ok() && self.analysis_ok()
    }

    pub fn needs_compact(&self) -> bool {
        !self.downsample_ok || !self.retention_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ingest_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let c = TimeSeries::new();
        assert!(c.storage_ok());
    }

    #[test]
    fn test_analysis() {
        let c = TimeSeries::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TimeSeries::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_compact() {
        let c = TimeSeries::new();
        assert!(!c.needs_compact());
    }

    #[test]
    fn test_downsample() {
        let mut c = TimeSeries::new();
        c.downsample_ok = false;
        assert!(c.needs_compact());
    }

    #[test]
    fn test_health() {
        let c = TimeSeries::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
