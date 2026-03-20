/// report html: template, render, style, export, log
/// Phase 1568

#[derive(Debug, Clone)]
pub struct ReportHtml {
    pub template_ok: bool,
    pub render_ok: bool,
    pub style_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for ReportHtml {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportHtml {
    pub fn new() -> Self {
        Self {
            template_ok: true,
            render_ok: true,
            style_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.template_ok && self.render_ok && self.style_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.template_ok || !self.render_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.template_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ReportHtml::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReportHtml::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReportHtml::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReportHtml::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReportHtml::new();
        c.template_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReportHtml::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
