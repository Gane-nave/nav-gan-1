/// report excel: workbook, sheet, formula, export, log
/// Phase 1569

#[derive(Debug, Clone)]
pub struct ReportExcel {
    pub workbook_ok: bool,
    pub sheet_ok: bool,
    pub formula_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for ReportExcel {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportExcel {
    pub fn new() -> Self {
        Self {
            workbook_ok: true,
            sheet_ok: true,
            formula_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.workbook_ok && self.sheet_ok && self.formula_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.workbook_ok || !self.sheet_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.workbook_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ReportExcel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReportExcel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReportExcel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReportExcel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReportExcel::new();
        c.workbook_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReportExcel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
