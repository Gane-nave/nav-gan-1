/// report chart: define, render, animate, export, log
/// Phase 1570

#[derive(Debug, Clone)]
pub struct ReportChart {
    pub define_ok: bool,
    pub render_ok: bool,
    pub animate_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for ReportChart {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportChart {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            render_ok: true,
            animate_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.render_ok && self.animate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.render_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ReportChart::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReportChart::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReportChart::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReportChart::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReportChart::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReportChart::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
