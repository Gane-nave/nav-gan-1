/// report pdf: layout, render, paginate, export, log
/// Phase 1564

#[derive(Debug, Clone)]
pub struct ReportPdf {
    pub layout_ok: bool,
    pub render_ok: bool,
    pub paginate_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for ReportPdf {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportPdf {
    pub fn new() -> Self {
        Self {
            layout_ok: true,
            render_ok: true,
            paginate_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.layout_ok && self.render_ok && self.paginate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.layout_ok || !self.render_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.layout_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ReportPdf::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReportPdf::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReportPdf::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReportPdf::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReportPdf::new();
        c.layout_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReportPdf::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
